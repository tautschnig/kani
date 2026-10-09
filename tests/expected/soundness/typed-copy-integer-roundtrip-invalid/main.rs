// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn integer_roundtrip_pointer_then_copy() {
    let source = 2u8;
    let mut destination = false;
    let destination_ptr = (&mut destination as *mut bool) as usize as *mut u8;
    unsafe {
        std::ptr::copy_nonoverlapping(&source, destination_ptr, 1);
    }
    let _ = destination;
}
