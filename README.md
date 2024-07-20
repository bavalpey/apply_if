# ApplyIf
[![CI](https://github.com/bavalpey/apply_if/actions/workflows/tests.yml/badge.svg)](https://github.com/bavalpey/apply_if/actions/workflows/tests.yml)

ApplyIf supplies a trait with two methods: ``apply_if(cond, closure)`` and
``apply_if_mut(cond, closure)``. These apply the given closure
on an instance if `cond` is true, returning the original instance otherwise
(or a reference in case of ``apply_if_mut``).  

Very useful for both the immutable and the mutable builder patterns
when you want to keep the nice ``.builder1().builder2()`` chain, and not interrupt
it with if-else blocks.  

Comes with a blanket implementation for all types, where ``apply_if`` is only
implemented for sized types.
