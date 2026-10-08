// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

static mut ESCAPED: Option<Box<dyn FnOnce() -> i32>> = None;

#[kanitool::fn_marker = "kani_force_fn_once"]
#[inline(never)]
fn forged_force_fn_once<F, T>(f: F) -> F
where
    F: FnOnce() -> T,
{
    f
}

#[kani::ensures(|_| true)]
fn stash() {
    let x: i32 = 42;
    let owned = Box::new(0u8);
    let escaping = forged_force_fn_once(|| {
        let _consume = owned;
        x
    });
    unsafe {
        ESCAPED = Some(std::mem::transmute::<
            Box<dyn FnOnce() -> i32 + '_>,
            Box<dyn FnOnce() -> i32 + 'static>,
        >(Box::new(escaping)));
    }
}

#[kani::proof]
fn forged_helper_must_not_suppress_dangling_capture_check() {
    stash();
    let escaped = unsafe { ESCAPED.take().unwrap() };
    let _ = escaped();
}
