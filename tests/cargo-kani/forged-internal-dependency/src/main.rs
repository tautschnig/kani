// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn check_dependency_safety() {
    forged_dep::bad();
}

fn main() {}
