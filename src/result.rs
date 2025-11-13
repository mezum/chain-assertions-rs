/// An extension trait to add the assertion_ok methods.
pub trait AssertOkExt {
    /// Asserts the [`Result`] is [`Ok`].
    ///
    /// # Panics
    ///
    /// The method panics if it is [`Err`] and `passthrough` feature is disabled.
    /// Otherwise, the method return self as is.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<i32, &str> = Ok(21);
    /// let x = x.assert_ok().map(|x| x * 2);
    /// assert_eq!(x, Ok(42));
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<i32, &str> = Err("oops");
    /// let _ = x.assert_ok().map(|x| x * 2);
    /// //        ^-- panics here
    /// ```
    fn assert_ok(self) -> Self;

    /// Asserts the [`Result`] is [`Ok`] but check only in debug builds.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`Err`]
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
    /// let x: Result<i32, &str> = Ok(21);
    /// let x = x.debug_assert_ok().map(|x| x * 2);
    /// assert_eq!(x, Ok(42));
    /// ```
    fn debug_assert_ok(self) -> Self;
}

/// An extension trait to add the assertion_ok_and methods.
pub trait AssertOkAndExt<T> {
    /// Asserts the [`Result`] is [`Ok`] and satisfies the condition.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`Err`], or [`Ok`] but user-provided condition returns `false`
    /// - `debug_assertions` is enabled
    /// - `passthrough` feature is disabled
    ///
    /// Otherwise, the method return self as is.
    ///
    /// # Examples
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<i32, &str> = Ok(21);
    /// let x = x.assert_ok_and(|x| x == &21).map(|x| x * 2);
    /// assert_eq!(x, Ok(42), "Expected Ok(42)");
    /// ```
    fn assert_ok_and(self, cond: impl FnOnce(&T) -> bool) -> Self;

    /// Asserts the [`Result`] is [`Ok`] and satisfies the condition only in debug builds.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`Err`], or [`Ok`] but user-provided condition returns `false`
    /// - `debug_assertions` is enabled
    /// - `passthrough` feature is disabled
    ///
    /// Otherwise, the method return self as is.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<i32, &str> = Ok(21);
    /// let x = x.debug_assert_ok_and(|x| x == &21).map(|x| x * 2);
    /// assert_eq!(x, Ok(42), "Expected Ok(42)");
    /// ```
    fn debug_assert_ok_and(self, cond: impl FnOnce(&T) -> bool) -> Self;
}

/// An extension trait to add the assertion_err methods.
pub trait AssertErrExt {
    /// Asserts the [`Result`] is [`Err`].
    ///
    /// # Panics
    ///
    /// The method panics if it is [`Ok`] and `passthrough` feature is disabled.
    /// Otherwise, the method return self as is.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Err(21);
    /// let x = x.assert_err().map_err(|x| x * 2);
    /// assert_eq!(x, Err(42));
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Ok("success");
    /// let x = x.assert_err().map_err(|x| x * 2);
    /// //        ^-- panics here
    /// ```
    fn assert_err(self) -> Self;

    /// Asserts the [`Result`] is [`Err`] but check only in debug builds.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`Err`].
    /// - `debug_assertions` is enabled.
    /// - `passthrough` feature is disabled.
    ///
    /// Otherwise, the method returns self as is.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Err(21);
    /// let x = x.debug_assert_err().map_err(|x| x * 2);
    /// assert_eq!(x, Err(42), "Expected Err(42)");
    /// ```
    ///
    /// ```rust,should_panic,ignore
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Ok("success");
    /// let _ = x.debug_assert_err();
    /// //        ^-- panics here only in debug builds
    ///
    /// ```
    fn debug_assert_err(self) -> Self;
}

pub trait AssertErrAndExt<T, E> {
    /// Asserts the [`Result`] is [`Err`] and satisfies the condition.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`Ok`], or [`Err`] but user-provided condition returns `false`
    /// - `debug_assertions` is enabled
    /// - `passthrough` feature is disabled
    ///
    /// Otherwise, the method return self as is.
    ///
    /// # Examples
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Err(21);
    /// let x = x.assert_err_and(|x| x == &21).map_err(|x| x * 2);
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Ok("debuggable");
    /// let x = x.assert_err_and(|x| x == &42).map_err(|x| x + 1);
    /// //        ^-- panics here
    /// ```
    fn assert_err_and(self, cond: impl FnOnce(&E) -> bool) -> Self;

    /// Asserts the [`Result`] is [`Err`] and satisfies the condition only in debug builds.
    ///
    /// # Panics
    ///
    /// The method panics if all following conditions are satisfied:
    ///
    /// - It is [`Ok`], or [`Err`] but user-provided condition returns `false`
    /// - `debug_assertions` is enabled
    /// - `passthrough` feature is disabled
    ///
    /// Otherwise, the method return self as is.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Err(21);
    /// let x = x.debug_assert_err_and(|x| x == &21).map_err(|x| x * 2);
    /// assert_eq!(x, Err(42), "Expected Err(42)");
    /// ```
    ///
    /// ```rust,should_panic,ignore
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Ok("debuggable");
    /// let x = x.debug_assert_err_and(|x| x == &42).map_err(|x| x + 1);
    /// //        ^-- panics here only in debug builds
    /// ```
    fn debug_assert_err_and(self, cond: impl FnOnce(&E) -> bool) -> Self;
}

impl<T, E> AssertOkExt for Result<T, E>
where
    E: crate::fmt::Debug,
{
    #[track_caller]
    #[inline]
    fn assert_ok(self) -> Self {
        if let Err(ref x) = self {
            panic!("Expected Ok(_), got Err({:?})", x);
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_ok(self) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            if let Err(ref x) = self {
                panic!("Expected Ok(_), got Err({:?})", x);
            }
        }
        self
    }
}

impl<T, E> AssertOkAndExt<T> for Result<T, E>
where
    T: crate::fmt::Debug,
    E: crate::fmt::Debug,
{
    #[track_caller]
    #[inline]
    fn assert_ok_and(self, cond: impl FnOnce(&T) -> bool) -> Self {
        match self {
            Ok(ref x) if cond(x) => { /* do nothing */ }
            Ok(ref x) => panic!("Condition not satisfied for Ok({:?})", x),
            Err(ref x) => panic!("Expected Ok(_), got Err({:?})", x),
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_ok_and(self, _cond: impl FnOnce(&T) -> bool) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            match self {
                Ok(ref x) if _cond(x) => { /* do nothing */ }
                Ok(ref x) => panic!("Condition not satisfied for Ok({:?})", x),
                Err(ref x) => panic!("Expected Ok(_), got Err({:?})", x),
            }
        }
        self
    }
}

impl<T, E> AssertErrExt for Result<T, E>
where
    T: crate::fmt::Debug,
{
    #[track_caller]
    #[inline]
    fn assert_err(self) -> Self {
        if let Ok(ref x) = self {
            panic!("Expected Err(_), got Ok({:?})", x);
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_err(self) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            if let Ok(ref x) = self {
                panic!("Expected Err(_), got Ok({:?})", x);
            }
        }
        self
    }
}

impl<T, E> AssertErrAndExt<T, E> for Result<T, E>
where
    T: crate::fmt::Debug,
    E: crate::fmt::Debug,
{
    #[track_caller]
    #[inline]
    fn assert_err_and(self, cond: impl FnOnce(&E) -> bool) -> Self {
        match self {
            Err(ref x) if cond(x) => { /* do nothing */ }
            Err(ref x) => panic!("Condition not satisfied for Err({:?})", x),
            Ok(ref x) => panic!("Expected Err(_), got Ok({:?})", x),
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_err_and(self, _cond: impl FnOnce(&E) -> bool) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            match self {
                Err(ref x) if _cond(x) => { /* do nothing */ }
                Err(ref x) => panic!("Condition not satisfied for Err({:?})", x),
                Ok(ref x) => panic!("Expected Err(_), got Ok({:?})", x),
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

    mod assert_ok {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_ok() {
            let x: Result<NonDebuggable, Debuggable> = Ok(NonDebuggable);
            let x = x.assert_ok();

            assert!(matches!(x, Ok(NonDebuggable)), "Expected Ok(NonDebuggable)");
        }

        #[test]
        #[should_panic(expected = "Expected Ok(_), got Err(Debuggable)")]
        fn it_fails_on_err() {
            let x: Result<NonDebuggable, Debuggable> = Err(Debuggable);
            let _ = x.assert_ok();
            //        ^-- should panic here
        }
    }

    mod assert_ok_and {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_ok_and_condition_satisfied() {
            let x: Result<i32, Debuggable> = Ok(42);
            let x = x.assert_ok_and(|x| x == &42);
            assert_eq!(x, Ok(42), "Expected Ok(42)");
        }

        #[test]
        #[should_panic(expected = "Condition not satisfied for Ok(42)")]
        fn it_fails_on_ok_but_condition_not_satisfied() {
            let x: Result<i32, Debuggable> = Ok(42);
            let _ = x.assert_ok_and(|x| x == &21);
            //        ^-- should panic here
        }

        #[test]
        #[should_panic(expected = "Expected Ok(_), got Err(Debuggable)")]
        fn it_fails_on_err() {
            let x: Result<i32, Debuggable> = Err(Debuggable);
            let _ = x.assert_ok_and(|x| x == &21).map(|x| x * 2);
            //        ^-- should panic here
        }
    }

    mod debug_assert_ok {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_ok() {
            let x: Result<i32, Debuggable> = Ok(41);
            let x = x.debug_assert_ok().map(|x| x + 1);

            assert!(matches!(x, Ok(42)), "Expected Ok(42)");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic = "Expected Ok(_), got Err(Debuggable)"
        )]
        fn it_fails_on_err() {
            let x: Result<i32, Debuggable> = Err(Debuggable);
            let x = x.debug_assert_ok().map(|x| x + 1);
            //        ^-- panic here only in debug builds

            // for debug builds
            assert!(matches!(x, Err(Debuggable)), "Expected Err(Debuggable)");
        }
    }

    mod debug_assert_ok_and {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_satisfied_ok() {
            let x: Result<i32, Debuggable> = Ok(21);
            let x = x.debug_assert_ok_and(|x| x == &21).map(|x| x * 2);

            assert_eq!(x, Ok(42), "Expected Ok(42)");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic = "Condition not satisfied for Ok(21)"
        )]
        fn it_fails_on_invalid_ok() {
            let x: Result<i32, Debuggable> = Ok(21);
            let x = x.debug_assert_ok_and(|_| false).map(|x| x * 2);
            //        ^-- should panic here only in debug builds

            // for debug builds
            assert_eq!(x, Ok(42), "Expected Ok(42)");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic = "Expected Ok(_), got Err(Debuggable)"
        )]
        fn it_fails_on_err() {
            let x: Result<i32, Debuggable> = Err(Debuggable);
            let x = x.debug_assert_ok_and(|x| x >= &20).map(|x| x);
            //        ^-- should panic here only in debug builds

            // for debug builds
            assert!(matches!(x, Err(Debuggable)), "Expected Err(Debuggable)");
        }
    }

    mod assert_err {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_err() {
            let x: Result<Debuggable, NonDebuggable> = Err(NonDebuggable);
            let x = x.assert_err();
            assert!(
                matches!(x, Err(NonDebuggable)),
                "Expected Err(NonDebuggable)"
            );
        }

        #[test]
        #[should_panic(expected = "Expected Err(_), got Ok(Debuggable)")]
        fn it_fails_on_ok() {
            let x: Result<Debuggable, i32> = Ok(Debuggable);
            let _ = x.assert_err().map_err(|x| x * 2);
            //        ^-- should panic here
        }
    }

    mod assert_err_and {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_err_and_condition_satisfied() {
            let x: Result<Debuggable, i32> = Err(21);
            let x = x.assert_err_and(|x| x == &21).map_err(|x| x * 2);
            assert_eq!(x, Err(42), "Expected Err(42)");
        }

        #[test]
        #[should_panic(expected = "Condition not satisfied for Err(41)")]
        fn it_fails_on_err_but_condition_not_satisfied() {
            let x: Result<Debuggable, i32> = Err(41);
            let _ = x.assert_err_and(|x| x == &21).map_err(|x| x + 1);
            //        ^-- should panic here
        }

        #[test]
        #[should_panic(expected = "Expected Err(_), got Ok(Debuggable)")]
        fn it_fails_on_ok() {
            let x: Result<Debuggable, i32> = Ok(Debuggable);
            let _ = x.assert_err_and(|x| x == &21).map_err(|x| x * 2);
            //        ^-- should panic here
        }
    }

    mod debug_assert_err {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_err() {
            let x: Result<Debuggable, i32> = Err(41);
            let x = x.debug_assert_err().map_err(|x| x + 1);
            assert!(matches!(x, Err(42)), "Expected Err(42)");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic = "Expected Err(_), got Ok(Debuggable)"
        )]

        fn it_fails_on_ok_only_in_debug_mode() {
            let x: Result<Debuggable, i32> = Ok(Debuggable);
            let x = x.debug_assert_err().map_err(|x| x + 1);
            //        ^-- panic here only in debug builds

            // for debug builds
            assert!(matches!(x, Ok(Debuggable)), "Expected Ok(Debuggable)");
        }
    }

    mod debug_assert_err_and {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_satisfied_err() {
            let x: Result<Debuggable, i32> = Err(21);
            let x = x.debug_assert_err_and(|x| x == &21).map_err(|x| x * 2);

            assert_eq!(x, Err(42), "Expected Err(42)");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic = "Condition not satisfied for Err(21)"
        )]
        fn it_fails_on_invalid_err() {
            let x: Result<Debuggable, i32> = Err(21);
            let x = x.debug_assert_err_and(|x| x < &21).map_err(|x| x * 2);
            //        ^-- should panic here only in debug builds

            assert_eq!(x, Err(42), "Expected Err(42)");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic = "Expected Err(_), got Ok(Debuggable)"
        )]
        fn it_fails_on_ok() {
            let x: Result<Debuggable, i32> = Ok(Debuggable);
            let x = x.debug_assert_err_and(|x| x >= &20).map_err(|x| x + 1);
            //        ^-- should panic here only in debug builds

            // for debug builds
            assert!(matches!(x, Ok(Debuggable)), "Expected Ok(Debuggable)");
        }
    }
}
