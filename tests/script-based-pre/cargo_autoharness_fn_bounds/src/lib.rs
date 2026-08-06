// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Fn-bounded generic functions: previously skipped ("no candidate type satisfies the
// function's trait bounds"), now instantiated with nondeterministic function items.

// TEST NOTE: harnessed as apply::<u8, nondet_fn1 item>; PASSES (wrapping arithmetic).
pub fn apply<F: Fn(u8) -> u8>(f: F, x: u8) -> u8 {
    f(x).wrapping_add(1)
}

// TEST NOTE: FAILS: the closure result is unconstrained, so the addition can overflow —
// a real bug class in the generic function's own code, found for ANY closure behavior.
pub fn apply_buggy<F: Fn(u8) -> u8>(f: F, x: u8) -> u8 {
    f(x) + 1
}

// TEST NOTE: FnMut with two arguments; the fold-style accumulation overflows: FAILS.
pub fn fold2<F: FnMut(u32, u32) -> u32>(mut f: F, a: u32, b: u32) -> u32 {
    f(a, b) + f(b, a)
}

// TEST NOTE: FnOnce returning unit: PASSES (nothing to go wrong).
pub fn run_once<F: FnOnce() -> ()>(f: F) {
    f()
}

// TEST NOTE: cover check must be SATISFIED: the nondet closure's results genuinely cover
// the range (both branches reachable).
pub fn branches<F: Fn(u8) -> bool>(f: F, x: u8) {
    if f(x) {
        kani::cover!(true, "true branch reachable");
    } else {
        kani::cover!(true, "false branch reachable");
    }
}

// TEST NOTE: still skipped: signature mentions another generic parameter (v1 limitation).
pub fn apply_generic<T, F: Fn(T) -> T>(f: F, x: T) -> T {
    f(x)
}

// Iterator-bounded generic functions: instantiated with std::vec::IntoIter<T> over an
// unbounded nondeterministic vector.

// TEST NOTE: harnessed; FAILS with an unwinding assertion (unbounded iterator, visible).
pub fn sum_iter<I: Iterator<Item = u8>>(it: I) -> u64 {
    it.map(|b| b as u64).sum()
}

// TEST NOTE: harnessed; loop-free property PASSES for iterators of all lengths.
pub fn first_item<I: Iterator<Item = u32>>(mut it: I) -> Option<u32> {
    it.next()
}
