// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(core_intrinsics)]
#![allow(internal_features)]

#[kani::proof]
fn typed_swap_uninitialized_reference_must_not_be_valid() {
    let mut first = std::mem::MaybeUninit::<&u8>::uninit();
    let mut second = std::mem::MaybeUninit::<&u8>::uninit();
    unsafe {
        std::intrinsics::typed_swap_nonoverlapping(first.as_mut_ptr(), second.as_mut_ptr());
    }
    assert!(true);
}
