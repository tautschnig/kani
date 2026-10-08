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
  feature that may change.
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
- **Transmute to invalid values**: `kani::any()` always produces valid values,
  but `transmute` of invalid bit patterns is not detected unless the unstable
  `-Z valid-value-checks` flag is enabled.

## Soundness Caveats

### CBMC backend
Kani uses [CBMC](https://github.com/diffblue/cbmc) as its verification backend.
CBMC is a mature tool but, like any complex software, may contain bugs. Kani
pins a specific CBMC version and runs CBMC's regression suite as part of its CI.

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
invariants/decreases expressions must be pure: Kani does not generally reject
side effects in them, and side effects can change the state on which the
verification condition is checked. A `modifies` expression that can panic is
also not soundly supported. Treat proofs using such clauses as outside the
soundness contract.

Loop-contract instrumentation currently disables CBMC's generic side-effect
check because Kani's generated closure wrappers are conservatively classified
as side-effecting. This does not make side-effecting user clauses safe; the
user remains responsible for keeping them pure.

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

## Reporting Soundness Issues

If you believe Kani has failed to detect a genuine issue (false negative),
please file a bug report with the `[F] Soundness` label at the
[soundness issues tracker](https://github.com/model-checking/kani/issues?q=label%3A%22%5BF%5D+Soundness%22).

### Intentional under-approximations

Some opt-in workflows are explicitly narrower than the set of all executions
of the source function. Their results must be read conditionally:

- `kani autoharness --bounded-arguments` limits slice, string, and bounded
  arbitrary inputs.
- `kani autoharness --constructor-args` and mined invariants generate only
  values reachable through a selected constructor or heuristic invariant.
- Automatic harnesses may verify a selected generic instantiation rather than
  all instantiations.
- Stubs, contracts, `kani::assume`, and `--restrict-vtable` replace or exclude
  behaviors according to their specifications.
- Disabling unwinding assertions, default checks, or relevant safety checks
  removes part of the stated proof obligation and must not be described as an
  unrestricted sound proof.

These modes may be useful and may be sound relative to their reduced
assumption set, but they cannot establish the corresponding unrestricted
property. In particular, an unsatisfiable assumption or constructor must not
be allowed to turn an empty set of executions into an unconditional successful
proof.
