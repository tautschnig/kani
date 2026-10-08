// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Checks that zero-count copy intrinsics validate pointer provenance.
#![allow(invalid_null_arguments)]

#[kani::proof]
fn copy_null_zero() {
    unsafe {
        std::ptr::copy(std::ptr::null::<u32>(), std::ptr::null_mut::<u32>(), 0);
    }
}

#[kani::proof]
fn copy_nonoverlapping_null_zero() {
    unsafe {
        std::ptr::copy_nonoverlapping(
            std::ptr::null::<u32>(),
            std::ptr::null_mut::<u32>(),
            0,
        );
    }
}

#[kani::proof]
fn copy_misaligned_zero() {
    let mut value = 0u32;
    let aligned = &mut value as *mut u32;
    let unaligned = unsafe { (aligned as *mut u8).add(1) as *mut u32 };
    unsafe {
        std::ptr::copy(unaligned, unaligned, 0);
    }
}

#[kani::proof]
fn copy_nonoverlapping_misaligned_zero() {
    let mut value = 0u32;
    let aligned = &mut value as *mut u32;
    let unaligned = unsafe { (aligned as *mut u8).add(1) as *mut u32 };
    unsafe {
        std::ptr::copy_nonoverlapping(unaligned, unaligned, 0);
    }
}
