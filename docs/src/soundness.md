# Soundness

This page documents Kani's soundness guarantees and known limitations.

## Operational soundness contract

For a particular harness and configuration, Kani is sound when the following
implication holds:

> If Kani reports `Success`, then every execution represented by the configured
> Rust/MIR model, satisfying the harness's explicit assumptions, contracts, and
> stubs, satisfies every checked property.

The claim is conditional on the verification scope. In particular, a proof
does not establish properties for values excluded by `kani::assume`, function
contracts, verified or user-provided stubs, or other explicit preconditions.
The same applies to a harness generated from a restricted input domain.

Soundness here means no false negative: Kani must not report `Success` while a
represented execution violates a checked property. Incompleteness and false
positives are permitted. For example, Kani may reject a program because a
construct is unsupported or because its model is more restrictive than Rust's
semantics, but it must not silently omit a reachable execution and then report
success.

An invocation that checks no proof harnesses is not a successful proof. Regular
verification commands therefore fail when no harness was found; informational
autoharness runs may instead report that no functions were eligible.

The contract assumes that:

- `rustc` has produced a correct MIR representation of the selected target and
  the target/platform ABI and layout model are the ones intended by the user;
- CBMC, its solver, and Kani's serialization and result-processing pipeline
  correctly implement the submitted verification conditions;
- all loops and recursion are either completely unwound with their unwinding
  assertions passing or are justified by a sound loop/recursion contract;
- every reachable unsupported, undefined, or inconclusive condition is
  reported as a failure or as an undetermined result, never as success; and
- the user has not enabled a documented under-approximation or disabled a
  required check.

This contract is deliberately separate from completeness. A successful proof
may still cover only one harness, one monomorphization, a bounded input size,
or values reachable through a selected constructor.

## Threat model and proof obligations

The soundness-sensitive boundaries are:

1. **Input and harness generation** must not exclude valid executions while
   presenting the result as an unrestricted proof.
2. **MIR transformation** must preserve behavior, except where an explicit
   contract says that behavior is intentionally assumed or modeled.
3. **Code generation and memory modeling** must over-approximate valid Rust
   executions and must check the validity conditions for unsafe operations.
4. **Unsupported features and backend failures** must remain visible and must
   prevent a successful verdict.
5. **Result processing** must reject incomplete, unknown, solver-error, and
   otherwise inconclusive output.

This threat model allows conservative modeling choices that cause false
alarms. It does not allow assumptions that remove a valid failing execution
from a proof without clearly changing the proof scope.

## What Kani Checks

Kani automatically checks for the following classes of issues:

### Enabled by default
These checks are on by default but can be individually disabled with `--no-XXX-checks` flags.

- **Arithmetic overflow**: Addition, subtraction, multiplication, division,
  remainder, shift, and negation on integer types.
- **Division by zero**: Integer and floating-point division and remainder.
- **Pointer dereference validity**: Null pointers, dangling pointers, pointers
  to deallocated or dead objects, out-of-bounds pointers, misaligned pointers.
- **Array bounds**: Index out of bounds on arrays and slices.
- **Panics**: `assert!`, `unwrap()`, `expect()`, `panic!()`, `unreachable!()`,
  and any other panic path in the code under verification.
- **Shift distance**: Negative shift amounts and shifts exceeding the bit width.
- **Float-to-integer conversion**: Non-finite float values (NaN, infinity)
  are rejected before conversion to integer types.
- **Memory leaks**: Dynamically allocated memory that is never freed (within
  the verification scope).

### Opt-in (unstable)
- **Valid value invariants** (`-Z valid-value-checks`): Checks that values meet
  their type's validity requirements (e.g., `bool` is 0 or 1, `char` is a valid
  Unicode scalar value, enum discriminants are in range). This is an unstable
  feature that may change. Kani conservatively rejects typed byte-copy,
  raw-pointer-load, volatile-load, and byte-write operations targeting
  validity-restricted types because it does not yet track validity through
  those byte-level operations.
- **Uninitialized memory** (`-Z uninit-checks`): Checks that memory is
  initialized before being read through pointers. This is an unstable feature
  that may change.

## What Kani Does NOT Check

The following classes of undefined behavior are **not detected** by Kani:

- **Data races**: Kani verifies sequential code only. Concurrent programs are
  not supported.
- **Pointer aliasing violations** (Stacked Borrows / Tree Borrows): Kani does
  not track reference lifetimes or enforce Rust's aliasing rules. If aliasing
  violations cause a memory safety or assertion failure, Kani will detect the
  symptom but not the root cause.
- **Mutation of immutable data**: Same as aliasing — detected only if it causes
  an observable failure.
- **Incorrect use of inline assembly**: Kani does not support inline assembly.
  Global assembly (`global_asm!`) is ignored with a warning.
- **ABI violations**: Kani relies on `rustc` for ABI checking.
- **Unsupported invalid-value introductions**: `kani::any()` always produces
  valid values. Transmutations to references or other types with
  validity-restricted layouts (including `bool`, `char`, non-zero types, owning
  pointers, and function pointers) are rejected conservatively. Fully
  bit-pattern-valid destinations remain supported. Typed byte-copy,
  raw-pointer-load, volatile-load, and byte-write operations targeting
  validity-restricted types are likewise rejected while validity tracking is
  incomplete. Mutable raw-pointer casts, pointer transmutes,
  integer-to-mutable-pointer casts, and union field accesses that can
  reinterpret a validity-restricted value are also rejected. Typed swaps of
  validity-restricted types are rejected for the same reason. Other
  invalid-value paths still require the unstable `-Z valid-value-checks` flag.

## Soundness Caveats

### CBMC backend
Kani uses [CBMC](https://github.com/diffblue/cbmc) as its verification backend.
CBMC is a mature tool but, like any complex software, may contain bugs. Kani
pins a specific CBMC version and runs CBMC's regression suite as part of its CI.
Kani rejects raw `--cbmc-args` that disable, weaken, or select away verification
properties. Other CBMC arguments remain an advanced escape hatch and are part
of the configured model.

### LLBC backend
The experimental LLBC backend (`-Z lean`) currently stops after generating
LLBC. It does not invoke a verification backend or produce a Kani verification
verdict, so a successful LLBC code-generation run does not establish the
operational soundness contract above.

### Bounded verification
Kani performs bounded model checking. Loops are unwound up to a configurable
bound (`--default-unwind`). If the bound is insufficient, Kani emits an
unwinding assertion failure. Verification results are sound only if all
unwinding assertions pass.

### Floating-point arithmetic
CBMC's floating-point reasoning uses bit-precise IEEE 754 semantics for the SAT
backend. Results are sound for standard floating-point operations. However, some
platform-specific floating-point behavior (e.g., x87 extended precision) may
differ from the verification model.

### C FFI
The experimental `-Z c-ffi` mode trusts Rust declarations for supported
`extern "C"` functions and does not validate the linked C implementation or
check that duplicate declarations and definitions agree. Proofs that call
foreign functions in this mode are conditional on those declarations, ABIs,
and implementations matching.

### Object size limits
CBMC represents pointers with a fixed number of object bits (default: 16,
configurable via `--cbmc-args --object-bits N`). Programs that allocate more
objects than `2^N` may exhibit incorrect wrapping behavior. This is a known
limitation tracked in [#1150](https://github.com/model-checking/kani/issues/1150).

### Function pointers
By default, unresolved function pointers are modeled as nondeterministic calls
to any function with a compatible signature. This is sound but may be imprecise
(reporting spurious failures). An unstable `--restrict-vtable` flag limits
dispatch targets based on vtable analysis, but **this is a known soundness
issue**: the analysis may exclude valid targets, causing Kani to miss real
failures (false negatives). See
[#802](https://github.com/model-checking/kani/issues/802) for details.

### Experimental contracts
Function and loop contracts are still experimental. Contract clauses and loop
invariants/decreases expressions must be pure: direct assignments and
side-effecting calls that the MIR checker can resolve are rejected, but
effects hidden behind unsupported or indirect constructs may remain outside
the checker and can change the state on which the verification condition is
checked. A `modifies` expression that can panic is also not soundly
supported. Treat proofs using such clauses as outside the soundness contract.
`proof_for_contract` additionally requires the generated
contract-check path to be reachable; an unreachable target or unsatisfiable
precondition is reported as a failed proof rather than a successful vacuous
one.

Loop-contract instrumentation currently disables CBMC's generic side-effect
check because Kani's generated closure wrappers are conservatively classified
as side-effecting. Kani performs a separate conservative MIR effect check for
resolved loop-clause calls, but this does not make unsupported or indirect
effects safe; the user remains responsible for keeping clauses pure.

The experimental `for`-loop adapter for a consuming `Vec` is available only
when its element type is `Copy`. Non-`Copy` elements are rejected because a
pointer-only `nth` model cannot faithfully reproduce Rust's move semantics
without invoking user-defined `Clone` behavior.

### Quantified predicates

Quantified predicates containing runtime checks, such as user assertions or
assumptions, overflow, division-by-zero, or finite-value checks, are rejected
as unsupported. CBMC cannot preserve those checks inside a quantified
expression, and silently dropping one could turn a Rust panic, changed
assumption, or failed assertion into a successful proof.

The same fail-closed rule applies to side effects that cannot be represented
in a pure quantified expression, including function calls, memory writes, and
nondeterministic values. Kani may accept calls to a narrowly pure fragment
after inlining; callers must not rely on side effects in quantified
predicates.

### Other scope-reducing models
Some options and APIs intentionally change the model being verified:

- `-Z unstable-options --ignore-global-asm` removes `global_asm!` from the
  model. Kani emits a warning, but a successful result is conditional on the
  assembly having no relevant effect.
- `#[safety_constraint(...)]`, derived `Invariant` implementations, and
  similar validity predicates constrain values generated by `kani::any()`.
  Contradictory predicates can make checks unreachable; predicates with side
  effects can also change the modeled state. These predicates are assumptions,
  not independently verified facts.
- Standard-library and platform/system-call models can be more precise than
  the actual target environment. A proof that depends on such a model is
  conditional on that model matching the deployment platform; replacing a
  system function with an overly precise stub can create false negatives.
- Allocation failure is not modeled: Kani passes CBMC's
  `--no-malloc-may-fail` option and assumes that nonzero-size allocations
  succeed. Code whose correctness depends on handling an allocation failure
  is outside the default proof scope; see [#891](https://github.com/model-checking/kani/issues/891).
- `PointerGenerator::any_alloc_status` is not an unrestricted pointer
  generator. In particular, for zero-sized pointees, some status constraints
  (such as `Dangling`) can be vacuous because the current pointer model cannot
  represent all of the corresponding constructions. Proofs that use this API
  must treat its generated statuses and pointer values as an explicit
  harness model, not as coverage of every Rust pointer execution.
- `kanitool` attributes are compiler-internal implementation details, not
  user-level assumptions. In particular, user-defined items cannot acquire
  Kani's check-disabling, contract-instrumentation, or trusted hook/intrinsic
  behavior merely by copying an internal attribute or marker. Contract and loop
  helpers are accepted only when their attributes originate in Kani's trusted
  procedural-macro expansion; malformed internal marker shapes are rejected or
  treated conservatively.

## Reporting Soundness Issues

If you believe Kani has failed to detect a genuine issue (false negative),
please file a bug report with the `[F] Soundness` label at the
[soundness issues tracker](https://github.com/model-checking/kani/issues?q=label%3A%22%5BF%5D+Soundness%22).

### Intentional under-approximations

Some opt-in workflows are explicitly narrower than the set of all executions
of the source function. Their results must be read conditionally:

- `--prove-safety-only` turns user assertions into assumptions, so a successful
  run establishes only the safety checks that remain enabled.
- `kani autoharness --bounded-arguments` limits slice, string, and bounded
  arbitrary inputs.
- `kani autoharness --constructor-args` and mined invariants generate only
  values reachable through a selected constructor or heuristic invariant.
- Automatic harnesses may verify a selected generic instantiation rather than
  all instantiations.
- Automatic harnesses for `Fn`/`FnMut`/`FnOnce` parameters use generated
  nondeterministic function items whose return values are unconstrained, but
  whose arbitrary argument, captured-state, and global-state side effects are
  not modeled.
- Stubs, contracts, `kani::assume`, and `--restrict-vtable` replace or exclude
  behaviors according to their specifications. Stubs must also be compatible
  with the replaced function's argument/return types, ABI, safety, and
  variadicness; incompatible stubs are rejected.
- Disabling unwinding assertions, default checks, or relevant safety checks
  removes part of the stated proof obligation and must not be described as an
  unrestricted sound proof.
- CBMC diagnostic options used with `--output-format old` (for example
  `--show-loops`) are informational commands, not proof results; a successful
  diagnostic command does not establish any property.

These modes may be useful and may be sound relative to their reduced
assumption set, but they cannot establish the corresponding unrestricted
property. In particular, an unsatisfiable assumption or constructor must not
be allowed to turn an empty set of executions into an unconditional successful
proof.
