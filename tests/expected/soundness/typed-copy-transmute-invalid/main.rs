// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn transmute_mutable_pointer_then_copy() {
    let source = 2u8;
    let mut destination = false;
    let destination_ptr: *mut u8 = unsafe {
        std::mem::transmute::<*mut bool, *mut u8>(&mut destination)
    };
    unsafe {
        std::ptr::copy_nonoverlapping(&source, destination_ptr, 1);
    }
    let _ = destination;
}
