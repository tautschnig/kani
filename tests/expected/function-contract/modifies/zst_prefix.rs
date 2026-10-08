// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

// A zero-sized modifies target must not shift the tuple index of later targets.
#[kani::modifies(z)]
#[kani::modifies(x)]
fn write_x(z: &mut (), x: &mut u32) {
    let _ = z;
    *x = 42;
}

#[kani::proof_for_contract(write_x)]
fn check_write_x() {
    let mut z = ();
    let mut x = kani::any();
    write_x(&mut z, &mut x);
}
