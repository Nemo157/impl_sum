#![feature(proc_macro)]
#![feature(use_extern_macros)]

extern crate impl_sum;

use std::{env, io, fs};
use std::io::Write;

use impl_sum::impl_sum;

#[impl_sum]
fn find_output_stream() -> impl io::Write {
    if let Some(file) = env::args().skip(1).next() {
        println!("Going to write to {}", file);
        return fs::File::create(file).unwrap();
    } else {
        println!("Going to write to stdout");
        return io::stdout();
    }
}

fn main() {
    let mut output = find_output_stream();
    writeln!(output, "Hello world").unwrap();
    output.flush().unwrap();
}
