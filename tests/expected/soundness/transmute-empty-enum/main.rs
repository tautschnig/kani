// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

enum Never {}

#[kani::proof]
fn transmute_to_empty_enum_must_not_be_valid() {
    let value: Never = unsafe { std::mem::transmute(()) };
    std::mem::forget(value);
    assert!(true);
}
