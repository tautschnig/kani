// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

union BytesOrBool {
    byte: u8,
    boolean: bool,
}

#[kani::proof]
fn union_invalid_bool_read_must_not_be_valid() {
    let value = BytesOrBool { byte: 2 };
    let boolean = unsafe { value.boolean };
    std::mem::forget(boolean);
    assert!(true);
}
