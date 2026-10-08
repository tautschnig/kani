// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kanitool::disable_checks(pointer)]
pub fn bad() {
    let ptr = std::ptr::null::<u32>();
    unsafe {
        let _ = *ptr;
    }
}
