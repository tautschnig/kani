// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn transmute_null_pointer_to_reference_is_not_valid() {
    let pointer = std::ptr::null::<u8>();
    let _reference: &u8 = unsafe { std::mem::transmute(pointer) };
    assert!(true);
}
