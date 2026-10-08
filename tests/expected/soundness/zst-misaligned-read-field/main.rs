// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[repr(align(8))]
struct Aligned;

#[repr(C)]
struct Container {
    prefix: u8,
    value: Aligned,
}

#[kani::proof]
fn check_misaligned_zst_field_read() {
    let ptr = 1usize as *const Container;
    let field = unsafe { std::ptr::addr_of!((*ptr).value) };
    unsafe { std::ptr::read(field) };
}
