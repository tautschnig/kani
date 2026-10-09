// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z valid-value-checks

#[kani::proof]
fn typed_volatile_load_invalid_bool_is_not_a_valid_execution() {
    let raw = [2u8];
    let value = unsafe { std::ptr::read_volatile(raw.as_ptr().cast::<bool>()) };
    assert!(value);
}
