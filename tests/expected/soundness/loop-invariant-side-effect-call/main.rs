// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z loop-contracts

#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

static mut SIDE_EFFECT: u32 = 0;

fn set_side_effect() {
    unsafe {
        SIDE_EFFECT = 1;
    }
}

#[kani::proof]
fn main() {
    #[kani::loop_invariant({
        set_side_effect();
        true
    })]
    while unsafe { SIDE_EFFECT == 0 } {
        unsafe {
            SIDE_EFFECT = 2;
        }
    }
    assert!(unsafe { SIDE_EFFECT == 1 });
}
