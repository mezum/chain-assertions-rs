/// An extension trait to add the assertion_some methods.
pub trait AssertSomeExt {
    /// Asserts the Option is [`Some`].
    ///
    /// # Panics
    ///
    /// If it is [`None`], the method panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(21);
    /// let x = x.assert_some().map(|v| v * 2);
    /// assert_eq!(x, Some(42));
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = None;
    /// let _ = x.assert_some().map(|v| v * 2);
    /// //        ^-- panics here
    fn assert_some(self) -> Self;

    /// Asserts the Option is [`Some`] only in debug builds.
    ///
    /// # Panics
    ///
    /// If it is [`None`] and `debug_assertions` are enabled, the method panics.
    /// If `debug_assertions` are disabled, the method is a no-op even if it is [`None`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(21);
    /// let x = x.debug_assert_some().map(|v| v * 2);
    /// assert_eq!(x, Some(42));
    /// ```
    fn debug_assert_some(self) -> Self;
}

/// An extension trait to add the assertion_none methods.
pub trait AssertNoneExt {
    /// Asserts the Option is [`None`].
    ///
    /// # Panics
    ///
    /// If it is [`Some`], the method panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = None;
    /// let x = x.assert_none().map(|v| v * 2);
    /// assert_eq!(x, None);
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(21);
    /// let _ = x.assert_none().map(|v| v * 2);
    /// //        ^-- panics here
    /// ```
    fn assert_none(self) -> Self;

    /// Asserts the Option is [`None`] only in debug builds.
    ///
    /// # Panics
    ///
    /// If it is [`Some`] and `debug_assertions` are enabled, the method panics.
    /// If `debug_assertions` are disabled, the method is a no-op even if it is [`Some`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = None;
    /// let x = x.debug_assert_none().map(|v| v * 2);
    /// assert_eq!(x, None);
    /// ```
    fn debug_assert_none(self) -> Self;
}

impl<T> AssertSomeExt for Option<T> {
    #[track_caller]
    #[inline]
    fn assert_some(self) -> Self {
        if self.is_none() {
            panic!("Expected Some(_), got None");
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_some(self) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            if self.is_none() {
                panic!("Expected Some(_), got None");
            }
        }
        self
    }
}

impl<T> AssertNoneExt for Option<T>
where
    T: crate::fmt::Debug,
{
    #[track_caller]
    #[inline]
    fn assert_none(self) -> Self {
        if let Some(ref v) = self {
            panic!("Expected None, got Some({:?})", v);
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_none(self) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            if let Some(ref v) = self {
                panic!("Expected None, got Some({:?})", v);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    struct NonDebuggable;

    #[derive(Debug)]
    struct Debuggable;

    mod assert_some {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_some() {
            let opt: Option<NonDebuggable> = Some(NonDebuggable);
            let target = opt.assert_some();

            assert!(
                matches!(target, Some(NonDebuggable)),
                "Expected Some(NonDebuggable)"
            );
        }

        #[test]
        #[should_panic(expected = "Expected Some(_), got None")]
        fn it_fails_on_none() {
            let opt: Option<NonDebuggable> = None;
            let _ = opt.assert_some();
            //          ^-- should panic here
        }
    }

    mod debug_assert_some {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_some() {
            let opt: Option<NonDebuggable> = Some(NonDebuggable);
            let target = opt.debug_assert_some();

            assert!(
                matches!(target, Some(NonDebuggable)),
                "Expected Some(NonDebuggable)"
            );
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic(expected = "Expected Some(_), got None")
        )]
        fn it_fails_on_none() {
            let opt: Option<NonDebuggable> = None;
            let target = opt.debug_assert_some();
            //               ^-- should panic here only in debug mode
            assert!(matches!(target, None), "Expected None");
        }
    }

    mod assert_none {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_none() {
            let opt: Option<Debuggable> = None;
            let target = opt.assert_none();

            assert!(matches!(target, None), "Expected None");
        }

        #[test]
        #[should_panic(expected = "Expected None, got Some(Debuggable)")]
        fn it_fails_on_some() {
            let opt: Option<Debuggable> = Some(Debuggable);
            let target = opt.assert_none();
            //          ^-- should panic here only in debug mode
            assert!(
                matches!(target, Some(Debuggable)),
                "Expected Some(Debuggable)"
            );
        }
    }

    mod debug_assert_none {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_none() {
            let opt: Option<Debuggable> = None;
            let target = opt.debug_assert_none();

            assert!(matches!(target, None), "Expected None");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic(expected = "Expected None, got Some(Debuggable)")
        )]
        fn it_fails_on_some() {
            let opt: Option<Debuggable> = Some(Debuggable);
            let target = opt.debug_assert_none();
            //               ^-- should panic here only in debug mode
            assert!(
                matches!(target, Some(Debuggable)),
                "Expected Some(Debuggable)"
            );
        }
    }
}
