// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Mine type invariants from a type's own assertions (C14 static form).
//!
//! Source: `&self` methods of the type's inherent impls whose `kani::assert` calls (Kani's
//! macro overrides have rewritten user asserts/panics into these) satisfy all of:
//! - the assert's block *post-dominates* the entry block (the claim holds on every normal
//!   return, structurally rejecting mode-guarded asserts like `if ready { assert!(..) }`);
//! - the condition's backward slice is pure and call-free: leaves are field projections of
//!   `self` or constants, interior nodes single-assignment temporaries combined with
//!   whitelisted operators.
//! Conditions are extracted into a small expression AST ([MinedExpr]), which provides a
//! canonical form for the frequency filter (a conjunct must be asserted in at least
//! [MIN_ASSERTING_METHODS] distinct methods, guarding against method-local preconditions
//! masquerading as type invariants) and is trivially total when re-materialized as MIR.

use rustc_data_structures::fx::FxHashMap;
use rustc_middle::ty::TyCtxt;
use rustc_public::CrateDef;
use rustc_public::mir::mono::Instance;
use rustc_public::mir::{
    BinOp, Body, ConstOperand, Operand, Place, Rvalue, StatementKind, TerminatorKind, UnOp,
};
use rustc_public::ty::{FnDef, RigidTy, Ty, TyKind};

/// A conjunct must be asserted in at least this many distinct methods to be considered a
/// type invariant rather than a method-local precondition.
pub const MIN_ASSERTING_METHODS: usize = 2;

/// A pure expression over the fields of a value of the mined type.
#[derive(Clone, Debug)]
pub enum MinedExpr {
    /// A chain of field projections starting at the value itself, with the field type.
    Field(Vec<(usize, Ty)>),
    /// A constant: the canonical token (for equality/hashing across methods) plus the
    /// original operand (for re-materialization).
    Const(String, ConstOperand),
    BinOp(BinOp, Box<MinedExpr>, Box<MinedExpr>),
    UnOp(UnOp, Box<MinedExpr>),
}

impl PartialEq for MinedExpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MinedExpr::Field(a), MinedExpr::Field(b)) => a == b,
            (MinedExpr::Const(a, _), MinedExpr::Const(b, _)) => a == b,
            (MinedExpr::BinOp(o1, a1, b1), MinedExpr::BinOp(o2, a2, b2)) => {
                o1 == o2 && a1 == a2 && b1 == b2
            }
            (MinedExpr::UnOp(o1, a1), MinedExpr::UnOp(o2, a2)) => o1 == o2 && a1 == a2,
            _ => false,
        }
    }
}
impl Eq for MinedExpr {}
impl std::hash::Hash for MinedExpr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            MinedExpr::Field(path) => path.hash(state),
            MinedExpr::Const(token, _) => token.hash(state),
            MinedExpr::BinOp(op, a, b) => {
                format!("{op:?}").hash(state);
                a.hash(state);
                b.hash(state);
            }
            MinedExpr::UnOp(op, a) => {
                format!("{op:?}").hash(state);
                a.hash(state);
            }
        }
    }
}

impl MinedExpr {
    pub fn ty(&self) -> Option<Ty> {
        match self {
            MinedExpr::Field(path) => path.last().map(|(_, t)| *t),
            MinedExpr::Const(_, c) => Some(c.const_.ty()),
            MinedExpr::BinOp(op, a, _) => match op {
                BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                    Some(Ty::bool_ty())
                }
                _ => a.ty(),
            },
            MinedExpr::UnOp(_, a) => a.ty(),
        }
    }
}

/// A mined invariant conjunct: the boolean expression plus provenance for diagnostics.
#[derive(Clone, Debug)]
pub struct MinedConjunct {
    pub expr: MinedExpr,
    /// Methods (pretty names) that assert this conjunct.
    pub asserted_in: Vec<String>,
}

/// Whether `bb` post-dominates the entry block of `body` w.r.t. normal returns:
/// every path from entry to a `Return` terminator passes through `bb`.
fn postdominates_entry(body: &Body, bb: usize) -> bool {
    // BFS from entry avoiding `bb`; if any Return block is reachable, `bb` does not
    // post-dominate.
    let mut seen = vec![false; body.blocks.len()];
    let mut queue = vec![0usize];
    seen[0] = true;
    if bb == 0 {
        return true;
    }
    while let Some(cur) = queue.pop() {
        let term = &body.blocks[cur].terminator;
        if matches!(term.kind, TerminatorKind::Return) {
            return false;
        }
        let mut push = |t: usize| {
            if t != bb && !seen[t] {
                seen[t] = true;
                queue.push(t);
            }
        };
        match &term.kind {
            TerminatorKind::Goto { target } => push(*target),
            TerminatorKind::SwitchInt { targets, .. } => {
                for (_, t) in targets.branches() {
                    push(t);
                }
                push(targets.otherwise());
            }
            TerminatorKind::Call { target, .. } => {
                if let Some(t) = target {
                    push(*t);
                }
            }
            TerminatorKind::Drop { target, .. } => push(*target),
            TerminatorKind::Assert { target, .. } => push(*target),
            _ => {}
        }
    }
    true
}

/// Extract a [MinedExpr] for `op` in `body`, where local `_1` is `&self` (or `self`).
/// Returns None (bail) when the slice leaves the pure/call-free/single-assignment fragment.
fn extract_expr(body: &Body, op: &Operand, depth: usize) -> Option<MinedExpr> {
    if depth > 24 {
        return None;
    }
    match op {
        Operand::Constant(c) => extract_const(c),
        Operand::Copy(place) | Operand::Move(place) => extract_place(body, place, depth),
        Operand::RuntimeChecks(_) => None,
    }
}

fn extract_const(c: &ConstOperand) -> Option<MinedExpr> {
    // The Debug rendering of the MIR constant is the canonical token for cross-method
    // equality; the operand itself is kept for re-materialization.
    Some(MinedExpr::Const(format!("{:?}", c.const_), c.clone()))
}

/// `self` is local 1; a place rooted at it with only Deref/Field projections is a field path.
fn extract_place(body: &Body, place: &Place, depth: usize) -> Option<MinedExpr> {
    use rustc_public::mir::ProjectionElem;
    if place.local == 1 {
        let mut path = vec![];
        for elem in &place.projection {
            match elem {
                ProjectionElem::Deref => {}
                ProjectionElem::Field(idx, ty) => path.push((*idx, *ty)),
                _ => return None,
            }
        }
        if path.is_empty() {
            return None; // whole-self uses are not field conditions
        }
        return Some(MinedExpr::Field(path));
    }
    // A temporary: find its unique defining assignment.
    if !place.projection.is_empty() {
        return None;
    }
    let mut def: Option<&Rvalue> = None;
    for block in &body.blocks {
        for stmt in &block.statements {
            if let StatementKind::Assign(p, rv) = &stmt.kind
                && p.local == place.local
            {
                if p.projection.is_empty() {
                    if def.is_some() {
                        return None; // multiple assignments (e.g. short-circuit merge)
                    }
                    def = Some(rv);
                } else {
                    return None;
                }
            }
        }
        // Call destinations also define locals.
        if let TerminatorKind::Call { destination, .. } = &block.terminator.kind
            && destination.local == place.local
        {
            return None; // defined by a call: outside the pure fragment
        }
    }
    match def? {
        Rvalue::Use(inner) => extract_expr(body, inner, depth + 1),
        Rvalue::BinaryOp(bop, a, b) => Some(MinedExpr::BinOp(
            *bop,
            Box::new(extract_expr(body, a, depth + 1)?),
            Box::new(extract_expr(body, b, depth + 1)?),
        )),
        Rvalue::UnaryOp(uop, a) => {
            Some(MinedExpr::UnOp(*uop, Box::new(extract_expr(body, a, depth + 1)?)))
        }
        Rvalue::CopyForDeref(p) => extract_place(body, p, depth + 1),
        _ => None,
    }
}

/// Mine the invariant conjuncts of `ty` (a struct ADT) from its inherent `&self` methods.
/// Results are cached by the caller.
pub fn mine_self_assert_conjuncts(tcx: TyCtxt, ty: Ty, kani_assert: FnDef) -> Vec<MinedConjunct> {
    let TyKind::RigidTy(RigidTy::Adt(adt_def, ref adt_args)) = ty.kind() else {
        return vec![];
    };
    if !adt_args.0.is_empty() {
        return vec![];
    }
    let adt_did = rustc_public::rustc_internal::internal(tcx, adt_def.def_id());
    let mut by_expr: FxHashMap<MinedExpr, Vec<String>> = FxHashMap::default();
    for &impl_did in tcx.inherent_impls(adt_did) {
        for &item in tcx.associated_item_def_ids(impl_did) {
            if !tcx.def_kind(item).is_fn_like() || !tcx.associated_item(item).is_method() {
                continue;
            }
            // Skip generic methods; instantiate with no args.
            if tcx
                .generics_of(item)
                .own_params
                .iter()
                .any(|p| !matches!(p.kind, rustc_middle::ty::GenericParamDefKind::Lifetime))
            {
                continue;
            }
            let Some(fn_def) = crate::kani_middle::stable_fn_def(tcx, item) else { continue };
            let Ok(inst) = Instance::resolve(fn_def, &rustc_public::ty::GenericArgs(vec![])) else {
                continue;
            };
            let Some(body) = inst.body() else { continue };
            // First parameter must be self by-ref or by-value of our type.
            let Some(self_decl) = body.arg_locals().first() else { continue };
            let self_ok = self_decl.ty == ty
                || matches!(self_decl.ty.kind(),
                    TyKind::RigidTy(RigidTy::Ref(_, inner, _)) if inner == ty);
            if !self_ok {
                continue;
            }
            let method_name = fn_def.name();
            for (bb_idx, block) in body.blocks.iter().enumerate() {
                let TerminatorKind::Call { func, args, .. } = &block.terminator.kind else {
                    continue;
                };
                let Ok(fn_ty) = func.ty(body.locals()) else { continue };
                let TyKind::RigidTy(RigidTy::FnDef(def, _)) = fn_ty.kind() else { continue };
                if def != kani_assert {
                    continue;
                }
                if !postdominates_entry(&body, bb_idx) {
                    continue;
                }
                let Some(expr) = extract_expr(&body, &args[0], 0) else { continue };
                if expr.ty() != Some(Ty::bool_ty()) {
                    continue;
                }
                let entry = by_expr.entry(expr).or_default();
                if !entry.contains(&method_name) {
                    entry.push(method_name.clone());
                }
            }
        }
    }
    by_expr
        .into_iter()
        .filter(|(_, methods)| methods.len() >= MIN_ASSERTING_METHODS)
        .map(|(expr, asserted_in)| MinedConjunct { expr, asserted_in })
        .collect()
}
