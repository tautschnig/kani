// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z quantifiers

#[inline(never)]
fn invalid_write() {
    unsafe {
        let p: *mut u8 = 1usize as *mut u8;
        std::ptr::write(p, 1);
    }
}

#[inline(never)]
fn invalid_bool() -> bool {
    invalid_write();
    true
}

#[kani::proof]
fn main() {
    assert!(kani::forall!(|i in (0, 1)| {
        let _ = i;
        invalid_bool()
    }));
}
