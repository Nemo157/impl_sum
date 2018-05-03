use impl_sum_macro::impl_sum_impl;

#[impl_sum_impl(::std::iter::Iterator)]
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn size_hint(&self) -> (usize, Option<usize>);
    fn count(self) -> usize;
    fn last(self) -> Option<Self::Item>;
    fn nth(&mut self, n: usize) -> Option<Self::Item>;
}
