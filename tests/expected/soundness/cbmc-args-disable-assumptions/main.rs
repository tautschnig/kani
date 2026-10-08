// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z unstable-options --cbmc-args --no-assumptions

#[kani::proof]
fn cbmc_assumption_disabling_must_be_rejected() {
    let value: u8 = kani::any();
    kani::assume(value == 0);
    assert!(value == 0);
}
