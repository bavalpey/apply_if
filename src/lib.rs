//! # ApplyIf
//!
//! ApplyIf supplies a trait with one method: ``apply_if(cond, closure)``, that applies the closure
//! on an object if the condition is true, returning the original object otherwise.
//! Very useful for the builder pattern when you want to keep the nice ``.builder1().builder2()``
//! chain, and not interrupt it with if-else blocks. It works for both the
//! immutable builder pattern and the mutable builder pattern.
//!
//! Comes with a blanket implementation for all types

#![no_std]

#[cfg(test)]
mod test;

pub trait ApplyIf {
    /// Takes a closure, and calls it on ``self`` if ``cond`` is true.
    ///
    /// Equivalent to the following:
    /// ```
    /// # struct Foo;
    /// # impl Foo {
    /// # fn foo(self, cond: bool) -> Self {
    /// # let f = |x| x;
    /// if cond {
    ///   f(self)
    /// } else {
    ///   self
    /// }
    /// # }}
    /// ```
    /// **Note** this method only works on sized objects
    fn apply_if<F>(self, cond: bool, f: F) -> Self
    where
        Self: Sized,
        F: Fn(Self) -> Self;

    /// Takes a closure, and calls it a mutable reference to `Self`.
    /// Use this method with the mutable builder pattern
    ///
    /// Equivalent to the following for `self : &mut Self`:
    /// ```
    /// # struct Foo;
    /// # impl Foo {
    /// # fn foo(&mut self, cond:bool) -> &mut Self {
    /// # let f = |x| x;
    /// if cond {
    ///   f(self)
    /// } else {
    ///   self
    /// }
    /// # }}
    /// ```
    fn apply_if_mut<F>(&mut self, cond: bool, f: F) -> &mut Self
    where
        F: Fn(&mut Self) -> &mut Self;
}

impl<T> ApplyIf for T {
    #[inline]
    fn apply_if<F: Fn(Self) -> Self>(self, cond: bool, f: F) -> Self
    where
        Self: Sized,
    {
        if cond {
            f(self)
        } else {
            self
        }
    }

    #[inline]
    fn apply_if_mut<F>(&mut self, cond: bool, f: F) -> &mut Self
    where
        F: Fn(&mut Self) -> &mut Self,
    {
        if cond {
            f(self)
        } else {
            self
        }
    }
}
