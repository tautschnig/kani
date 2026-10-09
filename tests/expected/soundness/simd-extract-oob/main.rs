// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(core_intrinsics, repr_simd)]

use std::intrinsics::simd::{simd_extract, simd_insert};

#[repr(simd)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
struct i32x2([i32; 2]);

#[kani::proof]
fn simd_extract_index_must_be_in_bounds() {
    let vector = i32x2([10, 20]);
    const INDEX: u32 = 2;
    let _: i32 = unsafe { simd_extract(vector, INDEX) };
}

#[kani::proof]
fn simd_insert_index_must_be_in_bounds() {
    let vector = i32x2([10, 20]);
    const INDEX: u32 = 2;
    let _ = unsafe { simd_insert(vector, INDEX, 30_i32) };
}
