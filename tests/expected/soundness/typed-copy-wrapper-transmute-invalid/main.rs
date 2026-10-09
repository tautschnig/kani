// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[repr(transparent)]
struct PointerWrapper<T>(*mut T);

#[kani::proof]
fn wrapper_transmute_then_copy() {
    let source = 2u8;
    let mut destination = false;
    let pointer = PointerWrapper(&mut destination);
    let byte_pointer: PointerWrapper<u8> =
        unsafe { std::mem::transmute::<PointerWrapper<bool>, PointerWrapper<u8>>(pointer) };
    unsafe {
        std::ptr::copy_nonoverlapping(&source, byte_pointer.0, 1);
    }
    let _ = destination;
}
