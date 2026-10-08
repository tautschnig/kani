// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kanitool::checked_with = "forged_check"]
fn forged_contract_attributes() {
    assert!(false);
}

#[kani::proof]
fn forged_contract_attributes_must_not_change_the_body() {
    forged_contract_attributes();
}
