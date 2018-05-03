# `impl_sum`

This is an experimental implementation of automatically generated anonymous sum
types as described in [rust-lang/rfcs#2414][]. The current implementation has
many limitations, some of these are avoidable with a bit more effort, others are
(as far as I'm aware) unavoidable when implementing via a procedural macro.

The biggest unavoidable limitation is that the supported traits need to be
explicitly specified in this repository. This could potentially be avoided via a
generic delegation mechanism such as described in [rust-lang/rfcs#2393][],
although I believe that at a minimum some of the proposed extensions to that RFC
would also have to be implemented.

Some limitations that could potentially be avoided in this implementation:

 * No implicit returns.

   The current implementation is using a single procedural
   macro attribute applied to a function. This then walks the AST and injects
   the necessary code at all explicit return sites. For example

   ```rust
   #[impl_sum]
   fn bar2(choose: bool) -> impl Iterator<Item = u32> {
       if choose {
           return vec![1, 2, 3].into_iter();
       } else {
           return [4, 5, 6].iter().cloned();
       }
   }
   ```

   This could be changed to use something like an implicitly defined expression
   macro, which would take the place of the explicit `return` in telling the
   macro where the code must be injected and allow using implicit returns:

   ```rust
   #[impl_sum]
   fn bar2(choose: bool) -> impl Iterator<Item = u32> {
       if choose {
           impl_sum!(vec![1, 2, 3].into_iter())
       } else {
           impl_sum!([4, 5, 6].iter().cloned())
       }
   }
   ```

   Either way there needs to be some marker for this implementation, an
   in-compiler implementation could avoid this by instead collecting all points
   at which it fails to unify expression types that end up being unified with
   the return type.

 * Only the top-level of the returned type is supported

   This is for a similar reason to the last limitation, if you have a return
   type like `io::Error<impl Iterator<Item = u32>>` there's no simple way for
   this macro to find the part of a return expression that corresponds to the
   `impl Iterator` part of the return type.

   This would be fixed by having an expression level macro as well

   ```rust
   #[impl_sum]
   fn bar2(choose: Option<bool>) -> Err<impl Iterator<Item = u32>, &'static str> {
       match choose {
           Some(true) => Ok(impl_sum!(vec![1, 2, 3].into_iter())),
           Some(false) => Ok(impl_sum!([4, 5, 6].iter().cloned())),
           None => Err("none"),
       }
   }
   ```

[rust-lang/rfcs#2414]: https://github.com/rust-lang/rfcs/issues/2414
[rust-lang/rfcs#2393]: https://github.com/rust-lang/rfcs/pull/2393
