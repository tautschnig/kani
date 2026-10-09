// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: -Z loop-contracts
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

struct Item {
    value: u8,
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self { value: self.value + 100 }
    }
}

#[kani::proof]
fn main() {
    let mut index = 0u8;
    let values = vec![Item { value: 1 }, Item { value: 2 }];

    #[kani::loop_invariant(kani::index <= 2)]
    for item in values {
        index += 1;
        if index == 2 {
            assert!(item.value != 2);
        }
    }
}
