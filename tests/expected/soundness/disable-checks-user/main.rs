// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kanitool::disable_checks(pointer)]
fn bad() {
    let ptr = std::ptr::null::<u32>();
    unsafe {
        let _ = *ptr;
    }
}

#[kani::proof]
fn check_null_deref() {
    bad();
}
