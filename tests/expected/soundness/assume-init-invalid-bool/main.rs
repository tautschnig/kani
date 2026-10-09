// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn assume_init_invalid_bool() {
    let value = unsafe { std::mem::MaybeUninit::<bool>::uninit().assume_init() };
    let _ = value;
}
