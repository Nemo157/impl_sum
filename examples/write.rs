#![feature(proc_macro)]
#![feature(use_extern_macros)]

extern crate impl_sum;

use std::{env, io, fs};
use std::io::Write;

use impl_sum::impl_sum;

#[impl_sum]
fn find_output_stream() -> io::Result<impl io::Write> {
    if let Some(file) = env::args().skip(1).next() {
        println!("Going to write to {}", file);
        fs::File::create(file).map(|f| impl_sum!(f))
    } else {
        println!("Going to write to stdout");
        Ok(impl_sum!(io::stdout()))
    }
}

fn main() -> io::Result<()> {
    let mut output = find_output_stream()?;
    writeln!(output, "Hello world")?;
    output.flush()?;
    Ok(())
}
