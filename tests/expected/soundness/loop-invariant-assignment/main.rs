// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z loop-contracts

#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

static mut SIDE_EFFECT: u32 = 0;

#[kani::proof]
fn main() {
    #[kani::loop_invariant({
        unsafe {
            SIDE_EFFECT = 1;
        }
        true
    })]
    while unsafe { SIDE_EFFECT == 0 } {
        unsafe {
            SIDE_EFFECT = 2;
        }
    }
    assert!(unsafe { SIDE_EFFECT == 1 });
}
