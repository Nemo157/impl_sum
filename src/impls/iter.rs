macro_rules! impl_iterator {
    ($head:ident) => {
    };

    ($head:ident $head2:ident $($rest:ident)*) => {
        impl<$head2, $($rest),*> Iterator for ::$head<$head2, $($rest),*> where $head2: Iterator, $($rest: Iterator<Item = $head2::Item>),* {
            type Item = $head2::Item;

            fn next(&mut self) -> Option<Self::Item> {
                match *self {
                    ::$head::$head2(ref mut e) => e.next(),
                    $(::$head::$rest(ref mut e) => e.next(),)*
                }
            }
        }
        impl_iterator!($head2 $($rest)*);
    };
}

impl_iterator! {
    Z Y X W V U T S R Q P O N M L K J I H G F E D C B A
}
