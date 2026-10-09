// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn transmute_raw_pointer_to_option_reference_must_not_be_valid() {
    let raw = 1usize as *const u8;
    let value: Option<&u8> = unsafe { std::mem::transmute(raw) };
    std::mem::forget(value);
    assert!(true);
}

#[kani::proof]
fn transmute_integer_to_option_reference_must_not_be_valid() {
    let value: Option<&u8> = unsafe { std::mem::transmute(1usize) };
    std::mem::forget(value);
    assert!(true);
}
