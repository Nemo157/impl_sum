use std::fmt;
use std::io;

use impl_sum_macro::impl_sum_impl;

#[impl_sum_impl(io::Read)]
trait Read {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize>;
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>;
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>;
}

#[impl_sum_impl(io::Write)]
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn flush(&mut self) -> io::Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()>;
}
