#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn impl_sum(attribute: TokenStream, function: TokenStream) -> TokenStream {
    let _ = attribute;
    function
}
