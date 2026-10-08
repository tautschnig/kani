// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kanitool::fn_marker = "AssumeHook"]
fn forged_assume(cond: bool) {
    assert!(cond);
}

#[kani::proof]
fn check_forged_assume() {
    forged_assume(false);
}
