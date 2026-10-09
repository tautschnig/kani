// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn transmute_zero_to_nonzero_must_not_be_valid() {
    let value: std::num::NonZeroU8 = unsafe { std::mem::transmute(0_u8) };
    std::mem::forget(value);
    assert!(true);
}

#[kani::proof]
fn transmute_invalid_integer_to_char_must_not_be_valid() {
    let value: char = unsafe { std::mem::transmute(0x11_0000_u32) };
    std::mem::forget(value);
    assert!(true);
}

#[kani::proof]
fn transmute_two_to_bool_must_not_be_valid() {
    let value: bool = unsafe { std::mem::transmute(2_u8) };
    std::mem::forget(value);
    assert!(true);
}

#[kani::proof]
fn transmute_null_to_function_pointer_must_not_be_valid() {
    let value: fn() = unsafe { std::mem::transmute(std::ptr::null::<()>()) };
    std::mem::forget(value);
    assert!(true);
}

#[kani::proof]
fn transmute_null_to_box_must_not_be_valid() {
    let value: Box<u8> = unsafe { std::mem::transmute(std::ptr::null_mut::<u8>()) };
    std::mem::forget(value);
    assert!(true);
}
