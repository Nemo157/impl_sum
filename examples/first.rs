#![feature(proc_macro)]
#![feature(use_extern_macros)]

extern crate impl_sum;

use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

use impl_sum::{impl_sum, C};

fn foo() -> impl Iterator<Item = u32> {
    vec![1, 2, 3].into_iter()
}

fn bar(choose: bool) -> impl Iterator<Item = u32> {
    if choose {
        C::A(vec![1, 2, 3].into_iter())
    } else {
        C::B([4, 5, 6].iter().cloned())
    }
}

#[impl_sum]
fn bar2(choose: bool) -> impl Iterator<Item = u32> {
    if choose {
        impl_sum!(vec![1, 2, 3].into_iter())
    } else {
        impl_sum!([4, 5, 6].iter().cloned())
    }
}

#[impl_sum]
fn bar4(choose: usize) -> impl Iterator<Item = u32> {
    match choose {
        1 => impl_sum!(vec![1, 2, 3].into_iter()),
        2 => impl_sum!(HashSet::<u32, RandomState>::from_iter([1, 2, 3].iter().cloned()).into_iter()),
        3 => impl_sum!([4, 5, 6].iter().cloned()),
        4 => impl_sum!(Some(5).into_iter()),
        _ => unimplemented!(),
    }
}

fn main() {
    println!("foo");
    for i in foo() {
        println!("{}", i);
    }

    println!("bar(true)");
    for i in bar(true) {
        println!("{}", i);
    }

    println!("bar(false)");
    for i in bar(false) {
        println!("{}", i);
    }

    println!("bar2(true)");
    for i in bar2(true) {
        println!("{}", i);
    }

    println!("bar2(false)");
    for i in bar2(false) {
        println!("{}", i);
    }

    println!("bar4(1)");
    for i in bar4(1) {
        println!("{}", i);
    }

    println!("bar4(2)");
    for i in bar4(2) {
        println!("{}", i);
    }

    println!("bar4(3)");
    for i in bar4(3) {
        println!("{}", i);
    }

    println!("bar4(4)");
    for i in bar4(4) {
        println!("{}", i);
    }
}
