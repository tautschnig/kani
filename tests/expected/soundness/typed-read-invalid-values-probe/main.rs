// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn typed_read_invalid_bool_must_not_be_valid() {
    let raw = [2_u8];
    let value = unsafe { std::ptr::read(raw.as_ptr().cast::<bool>()) };
    std::mem::forget(value);
    assert!(true);
}

#[kani::proof]
fn typed_read_unaligned_invalid_char_must_not_be_valid() {
    let raw = [0_u8, 0_u8, 0x11_u8, 0_u8];
    let value = unsafe { std::ptr::read_unaligned(raw.as_ptr().cast::<char>()) };
    std::mem::forget(value);
    assert!(true);
}

#[kani::proof]
fn typed_read_volatile_invalid_bool_must_not_be_valid() {
    let raw = [2_u8];
    let value = unsafe { std::ptr::read_volatile(raw.as_ptr().cast::<bool>()) };
    std::mem::forget(value);
    assert!(true);
}
