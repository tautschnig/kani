// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[repr(align(8))]
struct Aligned;

#[kani::proof]
fn check_misaligned_zst_read_in_aggregate() {
    let ptr = 1usize as *const Aligned;
    let _pair = unsafe { (std::ptr::read(ptr), 0u8) };
}
