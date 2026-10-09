// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// kani-flags: -Z mem-predicates

extern crate kani;

use kani::PointerGenerator;

#[kani::proof]
fn library_model_precondition_must_not_assume_false() {
    let mut generator = PointerGenerator::<1>::new();
    let _ = generator.any_in_bounds::<u16>();
    assert!(false);
}
