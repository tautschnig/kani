// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

kani_macros::forge!();

#[kani::proof]
fn check_forged_marker() {
    let value = forged_marker(|| 1_u32);
    assert!(value == 1);
}

fn main() {}
