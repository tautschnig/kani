// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(core_intrinsics)]

#[kani::proof]
fn write_bytes_invalid_bool() {
    let mut value = false;
    unsafe {
        core::ptr::write_bytes(&mut value as *mut bool, 2, 1);
    }
    assert!(value);
}

#[kani::proof]
fn volatile_set_memory_invalid_bool() {
    let mut value = false;
    unsafe {
        core::intrinsics::volatile_set_memory(&mut value as *mut bool, 2, 1);
    }
    assert!(value);
}
