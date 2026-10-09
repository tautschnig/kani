// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This macro generates implementations of the `KaniIntoIter` trait for various common types that are used in `for` loops.
//! We use this trait to overwrite the Rust IntoIter trait to reduce call stacks and avoid complicated loop invariant specifications,
//! while maintaining the semantics of the loop.
use crate::{KaniIntoIter, KaniPtrIter};

// The loop-contract lowering calls `nth` on an immutable iterator model. For a
// consuming `Vec` iterator, returning a cloned element is only equivalent to
// moving it when `T: Copy`; a user-defined `Clone` implementation may have
// arbitrary effects or return a different value.
impl<T: Copy> KaniIntoIter for Vec<T> {
    type Iter = KaniPtrIter<T>;
    fn kani_into_iter(self) -> Self::Iter {
        let s = self.iter();
        KaniPtrIter::new(s.as_slice().as_ptr(), s.len())
    }
}
