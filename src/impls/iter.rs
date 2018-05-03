use impl_sum_macro::impl_sum_impl;

#[impl_sum_impl(::std::iter::Iterator)]
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
