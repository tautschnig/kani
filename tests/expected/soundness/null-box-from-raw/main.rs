// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn null_box_from_raw_must_not_be_valid() {
    let pointer = std::ptr::null_mut::<u8>();
    let value = unsafe { Box::from_raw(pointer) };
    std::mem::forget(value);
    assert!(true);
}
