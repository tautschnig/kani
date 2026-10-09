// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(invalid_reference_casting)]

#[kani::proof]
fn reference_from_null_zst_must_fail() {
    let ptr = std::ptr::null::<()>();
    let _reference: &() = unsafe { &*ptr };
}
