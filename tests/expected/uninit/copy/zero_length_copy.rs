// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z uninit-checks

/// A zero-length copy must not initialize the destination.
#[kani::proof]
unsafe fn zero_length_copy_does_not_initialize() {
    let layout = std::alloc::Layout::from_size_align(1, 1).unwrap();
    let ptr = std::alloc::alloc(layout);
    assert!(!ptr.is_null());

    std::ptr::copy(ptr, ptr, 0);
    let _value = std::ptr::read(ptr);
}
