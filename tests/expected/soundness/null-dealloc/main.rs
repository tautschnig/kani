// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn null_dealloc_must_not_be_valid() {
    let layout = std::alloc::Layout::new::<()>();
    unsafe {
        std::alloc::dealloc(std::ptr::null_mut(), layout);
    }
}
