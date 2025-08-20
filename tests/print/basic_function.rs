//@ compile-flags: -Z unpretty=stable-mir -Zmir-opt-level=0 -Ztrim-diagnostic-paths=true
//@check-pass

#![allow(dead_code, unused_variables)]

fn foo(i: i32) -> i32 {
    i + 1
}

fn bar(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone();
    new_vec.push(1);
    new_vec
}

pub fn demux(input: u8) -> u8 {
    match input {
        0 => 10,
        1 => 6,
        2 => 8,
        _ => 0,
    }
}

fn main() {}
