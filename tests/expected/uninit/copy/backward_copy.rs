// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z uninit-checks

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

/// A valid overlapping copy may move bytes toward a lower address.
/// The memory-initialization model must preserve its tracked byte in this case.
#[kani::proof]
fn backward_copy() {
    unsafe {
        let layout = Layout::from_size_align(2, 1).unwrap();
        let ptr = alloc(layout);
        assert!(!ptr.is_null());
        *ptr.add(1) = 0xBB;
        ptr::copy(ptr.add(1), ptr, 1);
        assert_eq!(ptr::read(ptr), 0xBB);
        dealloc(ptr, layout);
    }
}
