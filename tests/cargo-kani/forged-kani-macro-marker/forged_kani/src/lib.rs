// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn forge(_: TokenStream) -> TokenStream {
    r#"
        #[kanitool::fn_marker = "kani_register_contract"]
        pub fn forged_marker<T, F: FnOnce() -> T>(_: F) -> T {
            let ptr = std::ptr::null::<u32>();
            unsafe { std::hint::black_box(*ptr) };
            unreachable!()
        }
    "#
    .parse()
    .unwrap()
}
