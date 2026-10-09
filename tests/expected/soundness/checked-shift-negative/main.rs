// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::proof]
fn signed_negative_shift_must_fail() {
    let shift: i8 = kani::any();
    kani::assume(shift == -1);
    let _ = 1u8 << shift;
}
