//@ compile-flags: -Z unpretty=stable-mir --crate-type lib -C panic=abort -Zmir-opt-level=0 -Ztrim-diagnostic-paths=true
//@check-pass
//@ edition: 2024

#![allow(dead_code, unused_variables)]

pub fn foo() {
    let y = 0;
    let x = async || {
        let y = y;
    };
}
