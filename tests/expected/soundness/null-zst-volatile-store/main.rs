// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(invalid_null_arguments)]
#![feature(core_intrinsics)]

#[kani::proof]
fn volatile_store_null_unit() {
    unsafe { std::intrinsics::volatile_store(std::ptr::null_mut::<()>(), ()) };
}

#[kani::proof]
fn unaligned_volatile_store_null_unit() {
    unsafe { std::intrinsics::unaligned_volatile_store(std::ptr::null_mut::<()>(), ()) };
}
