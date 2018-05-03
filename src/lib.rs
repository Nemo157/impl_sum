#![feature(use_extern_macros)]
extern crate impl_sum_macro;

pub use impl_sum_macro::impl_sum;

pub enum Either2<A, B> {
    A(A),
    B(B),
}

impl<A, B> Iterator for Either2<A, B> where A: Iterator, B: Iterator<Item = A::Item> {
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Either2::A(ref mut e) => e.next(),
            Either2::B(ref mut e) => e.next(),
        }
    }
}
