/// An extension trait to add the assertion_some methods.
pub trait AssertSomeExt {
    /// Asserts the [`Option`] is [`Some`].
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
    /// let x = x.assert_some().map(|x| x * 2);
    /// assert_eq!(x, Some(42));
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = None;
    /// let _ = x.assert_some().map(|x| x * 2);
    /// //        ^-- panics here
    fn assert_some(self) -> Self;

    /// Asserts the [`Option`] is [`Some`] only in debug builds.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`None`]
    /// - `debug_assertions` is enabled
    /// - `passthrough` feature is disabled
    ///
    /// Otherwise, the method returns self as is.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(21);
    /// let x = x.debug_assert_some().map(|x| x * 2);
    /// assert_eq!(x, Some(42));
    /// ```
    fn debug_assert_some(self) -> Self;
}

/// An extension trait to add the assertion_some_and methods.
pub trait AssertSomeAndExt<T> {
    /// Asserts the [`Option`] is [`Some`] and satisfies the condition.
    ///
    /// # Panics
    ///
    /// If it is [`None`] or the condition is not satisfied, the method panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(21);
    /// let x = x.assert_some_and(|x| x >= &20).map(|x| x * 2);
    /// assert_eq!(x, Some(42));
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(19);
    /// let _ = x.assert_some_and(|x| x >= &20).map(|x| x * 2);
    /// //        ^-- panics here
    /// ```
    fn assert_some_and(self, cond: impl FnOnce(&T) -> bool) -> Self;

    /// Asserts the [`Option`] is [`Some`] and satisfies the condition only in debug builds.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`None`] or the condition is not satisfied
    /// - `debug_assertions` is enabled
    /// - `passthrough` feature is disabled
    ///
    /// Otherwise, the method returns self as is.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(21);
    /// let x = x.debug_assert_some_and(|x| x >= &20).map(|x| x * 2);
    /// assert_eq!(x, Some(42));
    /// ```
    ///
    /// ```rust,should_panic,ignore
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(19);
    /// let _ = x.debug_assert_some_and(|x| x >= &20).map
    /// //        ^-- panics here if debug_assertion is enabled
    /// ```
    fn debug_assert_some_and(self, cond: impl FnOnce(&T) -> bool) -> Self;
}

/// An extension trait to add the assertion_none methods.
pub trait AssertNoneExt {
    /// Asserts the [`Option`] is [`None`].
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
    /// let x = x.assert_none().map(|x| x * 2);
    /// assert_eq!(x, None);
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Option<i32> = Some(21);
    /// let _ = x.assert_none().map(|x| x * 2);
    /// //        ^-- panics here
    /// ```
    fn assert_none(self) -> Self;

    /// Asserts the [`Option`] is [`None`] only in debug builds.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`Some`]
    /// - `debug_assertions` is enabled
    /// - `passthrough` feature is disabled
    ///
    /// Otherwise, the method returns self as is.
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

impl<T> AssertSomeAndExt<T> for Option<T>
where
    T: crate::fmt::Debug,
{
    #[track_caller]
    #[inline]
    fn assert_some_and(self, cond: impl FnOnce(&T) -> bool) -> Self {
        match self {
            Some(ref v) if cond(v) => { /* do nothing */ }
            Some(ref v) => panic!("Condition not satisfied for Ok({:?})", v),
            None => panic!("Expected Some(_), got None"),
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_some_and(self, _cond: impl FnOnce(&T) -> bool) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            match self {
                Some(ref v) if _cond(v) => { /* do nothing */ }
                Some(ref v) => panic!("Condition not satisfied for Ok({:?})", v),
                None => panic!("Expected Some(_), got None"),
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
    #[derive(PartialEq)]
    struct NonDebuggable;

    #[derive(Debug, PartialEq)]
    struct Debuggable;

    mod assert_some {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_some() {
            let x: Option<NonDebuggable> = Some(NonDebuggable);
            let x = x.assert_some();

            assert!(
                matches!(x, Some(NonDebuggable)),
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

    mod assert_ok_and {
        use super::super::*;

        #[test]
        fn it_succeeds_on_ok_and_condition_satisfied() {
            let x: Option<i32> = Some(21);
            let x = x.assert_some_and(|x| x >= &20).map(|x| x * 2);
            assert_eq!(x, Some(42));
        }

        #[test]
        #[should_panic(expected = "Condition not satisfied for Ok(19)")]
        fn it_fails_on_ok_and_condition_not_satisfied() {
            let x: Option<i32> = Some(19);
            let _ = x.assert_some_and(|x| x >= &20).map(|x| x * 2);
            //        ^-- should panic here
        }
    }

    mod debug_assert_some {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_some() {
            let x: Option<NonDebuggable> = Some(NonDebuggable);
            let x = x.debug_assert_some();

            assert!(
                matches!(x, Some(NonDebuggable)),
                "Expected Some(NonDebuggable)"
            );
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic(expected = "Expected Some(_), got None")
        )]
        fn it_fails_on_none() {
            let x: Option<NonDebuggable> = None;
            let x = x.debug_assert_some();
            //               ^-- should panic here only in debug mode
            assert!(matches!(x, None), "Expected None");
        }
    }

    mod debug_assert_some_and {
        use super::super::*;

        #[test]
        fn it_succeeds_on_ok_and_condition_satisfied() {
            let x: Option<i32> = Some(21);
            let x = x.debug_assert_some_and(|x| x >= &20).map(|x| x * 2);

            assert_eq!(x, Some(42));
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic(expected = "Condition not satisfied for Ok(19)")
        )]
        fn it_fails_on_ok_and_condition_not_satisfied() {
            let x: Option<i32> = Some(19);
            let x = x.debug_assert_some_and(|x| x >= &20).map(|x| x * 2);
            //        ^-- should panic here only in debug mode

            // for debug builds
            assert_eq!(x, Some(38));
        }
    }

    mod assert_none {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_none() {
            let x: Option<Debuggable> = None;
            let x = x.assert_none();

            assert!(matches!(x, None), "Expected None");
        }

        #[test]
        #[should_panic(expected = "Expected None, got Some(Debuggable)")]
        fn it_fails_on_some() {
            let x: Option<Debuggable> = Some(Debuggable);
            let _ = x.assert_none();
            //        ^-- should panic here
        }
    }

    mod debug_assert_none {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_none() {
            let x: Option<Debuggable> = None;
            let x = x.debug_assert_none();

            assert!(matches!(x, None), "Expected None");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic(expected = "Expected None, got Some(Debuggable)")
        )]
        fn it_fails_on_some() {
            let x: Option<Debuggable> = Some(Debuggable);
            let x = x.debug_assert_none();
            //        ^-- should panic here only in debug mode

            // for debug builds
            assert!(matches!(x, Some(Debuggable)), "Expected Some(Debuggable)");
        }
    }
}
