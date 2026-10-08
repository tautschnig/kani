// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Checks that `write_bytes` validates `dst` even when the count is zero.
#![feature(core_intrinsics)]

use std::intrinsics::write_bytes;

#[kani::proof]
fn zero_count_null() {
    unsafe {
        write_bytes(std::ptr::null_mut::<u32>(), 0, 0);
    }
}

#[kani::proof]
fn zero_count_misaligned() {
    let mut value = 0u32;
    let aligned = &mut value as *mut u32;
    let unaligned = unsafe { (aligned as *mut u8).add(1) as *mut u32 };
    unsafe {
        write_bytes(unaligned, 0, 0);
    }
}
