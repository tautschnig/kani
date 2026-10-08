// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z quantifiers
// kani-verify-fail

//! Checked arithmetic in quantifier predicates is rejected because its runtime
//! checks cannot be represented soundly inside a pure CBMC expression.

#[kani::proof]
fn check_addition_in_forall() {
    assert!(kani::forall!(|i in (0, 10)| i + 1 > 0));
}

#[kani::proof]
fn check_modulo_in_exists() {
    assert!(kani::exists!(|i in (0, 10)| i % 2 == 0));
}

#[kani::proof]
fn check_multiplication_in_forall() {
    assert!(kani::forall!(|i in (1, 5)| i * 2 >= 2));
}

#[kani::proof]
fn check_subtraction_in_exists() {
    assert!(kani::exists!(|i in (5, 10)| i - 5 == 0));
}

#[kani::proof]
fn check_combined_arithmetic() {
    let j: usize = kani::any();
    kani::assume(j % 2 == 0 && j < 2000);
    kani::assert(kani::exists!(|i in (0, 1000)| i + i == j), "");
}
