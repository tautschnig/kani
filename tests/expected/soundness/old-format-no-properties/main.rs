// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// A proof with no verification obligations must not be reported as successful in the legacy
// terminal output mode.
// kani-flags: --output-format old

#[kani::proof]
fn main() {}
