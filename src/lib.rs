#![doc=include_str!("../README.md")]
#![no_std]
#[warn(missing_docs)]
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
        F: FnOnce(Self) -> Self;

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
        F: FnOnce(&mut Self) -> &mut Self;

    /// Apply the closure only if the given optional argument contains
    /// a value.
    /// Use this method with the immutable builder pattern
    ///
    /// ```
    /// # struct Foo;
    /// # impl Foo {
    /// # fn foo(self, cond:bool) -> Self {
    /// # let f = |x,u| x;
    /// # let optional_u = Some(1);
    /// if let Some(u) = optional_u {
    ///   f(self,u)
    /// } else {
    ///   self
    /// }
    /// # }}
    /// ```
    fn apply_if_some<F, U>(self, u: Option<U>, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(Self, U) -> Self;

    /// Apply the closure only if the given optional argument contains
    /// a value.
    /// Use this method with the mutable builder pattern
    ///
    /// ```
    /// # struct Foo;
    /// # impl Foo {
    /// # fn foo(&mut self, cond:bool) -> &mut Self {
    /// # let f = |x,u| x;
    /// # let optional_u = Some(1);
    /// if let Some(u) = optional_u {
    ///   f(self,u)
    /// } else {
    ///   self
    /// }
    /// # }}
    /// ```
    fn apply_if_some_mut<F, U>(&mut self, u: Option<U>, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self, U) -> &mut Self;
}

impl<T> ApplyIf for T {
    #[inline]
    fn apply_if<F: FnOnce(Self) -> Self>(self, cond: bool, f: F) -> Self
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
        F: FnOnce(&mut Self) -> &mut Self,
    {
        if cond {
            f(self)
        } else {
            self
        }
    }

    #[inline]
    fn apply_if_some<F, U>(self, u: Option<U>, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(Self, U) -> Self,
    {
        if let Some(u) = u {
            f(self, u)
        } else {
            self
        }
    }

    #[inline]
    fn apply_if_some_mut<F, U>(&mut self, u: Option<U>, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self, U) -> &mut Self,
    {
        if let Some(u) = u {
            f(self, u)
        } else {
            self
        }
    }
}
