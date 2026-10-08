// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// `typed_swap_nonoverlapping` requires the complete T-sized regions to be
// disjoint, not merely the pointers themselves.
#![feature(core_intrinsics)]
#![allow(internal_features)]

#[repr(C)]
struct Four([u8; 4]);

#[kani::proof]
fn overlapping_typed_swap() {
    let mut bytes = [0u8; 8];
    let x = bytes.as_mut_ptr() as *mut Four;
    let y = unsafe { bytes.as_mut_ptr().add(2) as *mut Four };
    unsafe { std::intrinsics::typed_swap_nonoverlapping(x, y) };
}
