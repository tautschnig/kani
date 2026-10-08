// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z unstable-options --cbmc-args --no-assertions

#[kani::proof]
fn cbmc_assertion_disabling_must_be_rejected() {
    assert!(false);
}
