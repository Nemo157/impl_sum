# `impl_sum`

This is an experimental implementation of automatically generated anonymous sum
types as described in [rust-lang/rfcs#2414][]. The current implementation has
many limitations, some of these are avoidable with a bit more effort, others are
(as far as I'm aware) unavoidable when implementing via a procedural macro.

There are two major limitations with this implementation that could be avoided
by an in-compiler implementation:

 * The biggest unavoidable limitation is that the supported traits need to be
  explicitly specified in this repository. This could potentially be avoided
  via a generic delegation mechanism such as described in
  [rust-lang/rfcs#2393][], although I believe that at a minimum some of the
  proposed extensions to that RFC would also have to be implemented.

 * There needs to be an expression level marker for this implementation, an
   in-compiler implementation could avoid this by instead collecting all points
   at which it fails to unify expression types that end up being unified with
   the return type.

 * Only a single anonymous sum type per function is supported. This is because
   of how the proc-macro walks the function AST and replaces all calls to
   `impl_sum!` by a specific wrapper. There's currently no way for this to
   distinguish between different parts of the return value, and all solutions I
   can think of would be rather cumbersome to use.

Old limitations that have been dropped:

 * ~No implicit returns.~
 * ~Only the top-level of the returned type is supported~

[rust-lang/rfcs#2414]: https://github.com/rust-lang/rfcs/issues/2414
[rust-lang/rfcs#2393]: https://github.com/rust-lang/rfcs/pull/2393
