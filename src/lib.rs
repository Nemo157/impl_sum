#![feature(proc_macro)]
#![feature(use_extern_macros)]

extern crate impl_sum_macro;

mod impls;

pub use impl_sum_macro::impl_sum;

macro_rules! enums {
    ($head:ident) => {
    };

    ($head:ident $($rest:ident)*) => {
        pub enum $head<$($rest),*> {
            $($rest($rest)),*
        }
        enums!($($rest)*);
    };
}

enums! {
    Z Y X W V U T S R Q P O N M L K J I H G F E D C B A
}
