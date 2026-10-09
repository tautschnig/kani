// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

struct TestIter {
    valid: bool,
}

impl kani::KaniIter for TestIter {
    type Item = ();

    fn nth(&self, _i: usize) {}

    fn first(&self) {}

    fn assumption(&self) -> bool {
        self.valid
    }

    fn len(&self) -> usize {
        0
    }
}

#[kani::proof]
fn chain_requires_both_iterators() {
    let first = TestIter { valid: true };
    let second = TestIter { valid: false };
    let chain = kani::KaniChainIter::new(first, second);
    assert!(!kani::KaniIter::assumption(&chain));
}
