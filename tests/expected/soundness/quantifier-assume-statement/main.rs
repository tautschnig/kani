// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z quantifiers

// Assumptions in quantified predicates are statements that the pure-expression
// lowering cannot preserve. They must not be silently dropped.
#[kani::proof]
fn main() {
    assert!(kani::forall!(|i in (0, 4)| {
        kani::assume(i != 2);
        true
    }));
}
