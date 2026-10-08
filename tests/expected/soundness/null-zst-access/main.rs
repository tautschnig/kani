// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(invalid_null_arguments)]

#[kani::proof]
fn read_null_unit() {
    unsafe { std::ptr::read(std::ptr::null::<()>()) };
}

#[kani::proof]
fn write_null_unit() {
    unsafe { std::ptr::write(std::ptr::null_mut::<()>(), ()) };
}
