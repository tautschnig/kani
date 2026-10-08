// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z function-contracts

#[kani::requires(false)]
fn bad_contract() {
    let ptr = std::ptr::null::<u32>();
    unsafe { assert_eq!(*ptr, 0); }
}

#[kani::proof_for_contract(bad_contract)]
fn check_bad_contract() {
    let call_target: bool = kani::any();
    kani::assume(!call_target);
    if call_target {
        bad_contract();
    }
}
