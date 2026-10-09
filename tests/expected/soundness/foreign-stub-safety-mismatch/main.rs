// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z stubbing

unsafe extern "C" {
    fn foreign_fn(value: u32) -> u32;
}

extern "C" fn mismatched_stub(_value: u32) -> u32 {
    7
}

#[kani::proof]
#[kani::stub(foreign_fn, mismatched_stub)]
fn main() {
    let value = unsafe { foreign_fn(7) };
    assert!(value == 7);
}
