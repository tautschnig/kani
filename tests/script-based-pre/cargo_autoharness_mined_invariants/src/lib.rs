// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// A type whose invariant (value in 1..=366) is stated by asserts in TWO methods:
// mined as an invariant (frequency filter passes) and assumed for generated values.
pub struct Day {
    value: u16,
}

impl Day {
    pub fn ordinal0(&self) -> u16 {
        assert!(self.value >= 1);
        self.value - 1
    }

    pub fn ordinal(&self) -> u16 {
        assert!(self.value >= 1);
        self.value
    }
}

// TEST NOTE: previously a false alarm (raw field synthesis generates value == 0);
// with mined-invariant assumption, PASSES.
pub fn day_user(d: Day) -> u16 {
    d.ordinal0()
}

// A method-local precondition asserted in only ONE method: must NOT be mined
// (frequency filter), so the false alarm on prec_user remains — honest behavior.
pub struct Gauge {
    level: u8,
}

impl Gauge {
    pub fn drain(&self) -> u8 {
        assert!(self.level >= 10, "drain requires level >= 10");
        self.level - 10
    }
}

// TEST NOTE: still FAILS (assert in drain is not mined as an invariant).
pub fn prec_user(g: Gauge) -> u8 {
    g.drain()
}

// TEST NOTE (--check-invariants): makes an INVALID Day (value == 0) — the mined-invariant
// output check must FAIL on this function.
pub fn buggy_make_day(seed: u16) -> Day {
    Day { value: seed % 366 } // BUG: yields 0 when seed % 366 == 0; invariant needs 1..=366
}

// TEST NOTE (--check-invariants): correct producer — output check must PASS.
pub fn good_make_day(seed: u16) -> Day {
    Day { value: (seed % 366) + 1 }
}
