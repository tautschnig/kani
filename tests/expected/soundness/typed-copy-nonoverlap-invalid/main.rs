// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn copy_u8_into_bool_then_use() {
    let source = 2u8;
    let mut destination = false;
    unsafe {
        std::ptr::copy_nonoverlapping(
            &source,
            (&mut destination as *mut bool).cast::<u8>(),
            1,
        );
    }
    let _ = destination;
}
