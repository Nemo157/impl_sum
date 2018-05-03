extern crate impl_sum;

use impl_sum::Either2;

fn foo() -> impl Iterator<Item = u32> {
    vec![1, 2, 3].into_iter()
}

fn bar(choose: bool) -> impl Iterator<Item = u32> {
    if choose {
        Either2::A(vec![1, 2, 3].into_iter())
    } else {
        Either2::B([4, 5, 6].iter().cloned())
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
}
