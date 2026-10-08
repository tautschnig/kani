// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z uninit-checks

#![feature(core_intrinsics)]

use std::intrinsics::raw_eq;

#[repr(C)]
#[derive(kani::Arbitrary)]
struct Padded {
    byte: u8,
    word: u16,
}

#[inline(never)]
unsafe fn compare<T>(lhs: &T, rhs: &T) -> bool {
    raw_eq(lhs, rhs)
}

#[kani::proof]
fn check_raw_eq_padding() {
    let lhs: Padded = kani::any();
    unsafe {
        assert!(compare(&lhs, &lhs));
    }
}
