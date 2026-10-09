// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

union PointerView {
    bool_ptr: *const bool,
    byte_ptr: *mut u8,
}

#[kani::proof]
fn union_pointer_reinterpretation_then_copy() {
    let source = 2u8;
    let mut destination = false;
    let pointer = PointerView { bool_ptr: &mut destination };
    let destination_ptr = unsafe { pointer.byte_ptr };
    unsafe {
        std::ptr::copy_nonoverlapping(&source, destination_ptr, 1);
    }
    let _ = destination;
}
