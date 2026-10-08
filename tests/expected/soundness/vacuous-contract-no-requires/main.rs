// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z function-contracts

#[kani::ensures(|_| true)]
fn bad_contract() {
    assert!(false);
}

#[kani::proof_for_contract(bad_contract)]
fn check_bad_contract() {
    let call_target: bool = kani::any();
    kani::assume(!call_target);
    if call_target {
        bad_contract();
    }
}
