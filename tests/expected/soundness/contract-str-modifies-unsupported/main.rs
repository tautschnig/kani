// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z function-contracts --prove-safety-only

#[kani::modifies(value)]
fn overwrite(value: &mut str) {}

#[kani::proof_for_contract(overwrite)]
fn check_overwrite() {
    let mut bytes = [b'x'];
    let value = unsafe { std::str::from_utf8_unchecked_mut(&mut bytes) };
    overwrite(value);
}
