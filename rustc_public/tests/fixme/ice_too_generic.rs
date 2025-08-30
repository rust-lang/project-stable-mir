//@compile-flags: -Copt-level=1

#![allow(dead_code, unused_variables)]
use std::fmt::Debug;

pub trait Meow<A: Clone + Debug> {
    fn foo(&self, a: Option<&A>) -> A;

    fn fzz(&self) -> A {
        self.foo(None)
    }
}

fn main() {}