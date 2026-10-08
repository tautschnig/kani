// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Checks that `compare_bytes` validates pointers even when the length is zero.
#![feature(core_intrinsics)]

use std::intrinsics::compare_bytes;

#[kani::proof]
fn zero_count_null() {
    unsafe {
        compare_bytes(std::ptr::null::<u8>(), std::ptr::null::<u8>(), 0);
    }
}
