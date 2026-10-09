// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[repr(C)]
struct Pair {
    boolean: bool,
}

#[kani::proof]
fn typed_read_aggregate_invalid_bool_must_not_be_valid() {
    let raw = [2_u8];
    let value = unsafe { std::ptr::read(raw.as_ptr().cast::<Pair>()) };
    std::mem::forget(value);
    assert!(true);
}

#[kani::proof]
fn typed_field_read_through_raw_pointer_invalid_bool_must_not_be_valid() {
    let raw = [2_u8];
    let boolean = unsafe { (*raw.as_ptr().cast::<Pair>()).boolean };
    std::mem::forget(boolean);
    assert!(true);
}

#[kani::proof]
fn typed_field_reference_through_raw_pointer_invalid_bool_must_not_be_valid() {
    let raw = [2_u8];
    let boolean = unsafe { &(*raw.as_ptr().cast::<Pair>()).boolean };
    std::hint::black_box(boolean);
    assert!(true);
}
