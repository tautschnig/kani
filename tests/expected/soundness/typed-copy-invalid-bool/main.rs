// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z valid-value-checks

#[kani::proof]
fn typed_copy_invalid_bool_is_not_a_valid_execution() {
    let raw = [2u8];
    let mut value = false;
    unsafe {
        std::ptr::copy(raw.as_ptr().cast::<bool>(), &mut value, 1);
    }
    assert!(value);
}
