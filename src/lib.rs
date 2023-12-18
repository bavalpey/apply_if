//! # ApplyIf
//! 
//! ApplyIf supplies a trait with one method: ``apply_if(cond, closure)``, that applies the closure
//! on an object if the condition is true, returning the original object otherwise.
//! Very useful for the builder pattern when you want to keep the nice ``.builder1().builder2()``
//! chain, and not interrupt it with if-else blocks.
//!
//! //! Comes with a blanket implementation for all sized types.

/// The ApplyIf trait only works for Sized objects.
pub trait ApplyIf: Sized {
    /// Takes a closure, and calls it on ``self`` if ``cond`` is true.
    /// 
    /// Equivalent to the following:
    /// ```
    /// if cond {
    ///   f(self)
    /// } else {
    ///   self
    /// }
    ///
    /// ```
    fn apply_if<F: Fn(Self) -> Self>(self, cond: bool, f: F) -> Self;
}

impl<T: Sized> ApplyIf for T {
    #[inline]
    fn apply_if<F: Fn(Self) -> Self>(self, cond: bool, f: F) -> Self {
        if cond {
            f(self)
        } else {
            self
        }
    }
}
