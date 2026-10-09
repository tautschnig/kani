// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn typed_copy_default_invalid_bool_must_not_be_valid() {
    let raw = [2_u8];
    let mut value = false;
    unsafe {
        std::ptr::copy(raw.as_ptr().cast::<bool>(), &mut value, 1);
    }
    std::mem::forget(value);
    assert!(true);
}
