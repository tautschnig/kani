// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn check_dependency_helper_marker() {
    let _ = fake_kani::fake_register(|| 0_u32);
    kani::assume(false);
    assert!(false);
}

fn main() {}
