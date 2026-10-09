// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

//! Test that Kani detects mutual recursion through more than one intermediate
//! function when all functions use contract recursion.

#[kani::requires(x < 100)]
#[kani::ensures(|&result| result <= x)]
#[kani::recursion]
fn mutual_a(x: u32) -> u32 {
    if x == 0 { 0 } else { mutual_b(x - 1) }
}

#[kani::requires(x < 100)]
#[kani::ensures(|&result| result <= x)]
#[kani::recursion]
fn mutual_b(x: u32) -> u32 {
    if x == 0 { 0 } else { mutual_c(x - 1) }
}

#[kani::requires(x < 100)]
#[kani::ensures(|&result| result <= x)]
#[kani::recursion]
fn mutual_c(x: u32) -> u32 {
    if x == 0 { 0 } else { mutual_a(x - 1) }
}

#[kani::proof_for_contract(mutual_a)]
fn check_mutual_a() {
    let x: u32 = kani::any();
    mutual_a(x);
}
