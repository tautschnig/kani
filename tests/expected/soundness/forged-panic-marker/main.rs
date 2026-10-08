// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kanitool::fn_marker = "PanicStub"]
fn forged_panic_stub() {
    unsafe {
        let ptr = std::ptr::null::<u8>();
        std::hint::black_box(*ptr);
    }
}

#[kani::proof]
fn forged_panic_marker_must_not_replace_the_body() {
    forged_panic_stub();
}
