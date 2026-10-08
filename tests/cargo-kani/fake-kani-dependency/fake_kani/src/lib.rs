// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kanitool::fn_marker = "kani_register_contract"]
pub fn fake_register<T, F: FnOnce() -> T>(f: F) -> T {
    f()
}
