// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z quantifiers

// A checked operation in a quantified predicate cannot be represented as a pure
// CBMC expression. It must not be silently dropped, since that could turn a
// Rust panic into a successful proof.
#[kani::proof]
fn main() {
    assert!(kani::forall!(|i: u8 in (0, 255)| i * 2 >= 0));
}
