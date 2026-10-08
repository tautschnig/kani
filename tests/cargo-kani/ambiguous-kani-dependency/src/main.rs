// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn check_ambiguous_kani() {
    fake_kani::fake_assume(true);
    kani::assume(false);
    assert!(false);
}

fn main() {}
