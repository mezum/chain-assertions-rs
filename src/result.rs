/// An extension trait to add the assertion_ok methods.
pub trait AssertOkExt {
    /// Asserts the [`Result`] is [`Ok`].
    ///
    /// # Panics
    ///
    /// If it is [`Err`], the method panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<i32, &str> = Ok(21);
    /// let x = x.assert_ok().map(|v| v * 2);
    /// assert_eq!(x, Ok(42));
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<i32, &str> = Err("oops");
    /// let _ = x.assert_ok().map(|v| v * 2);
    /// //        ^-- panics here
    /// ```
    fn assert_ok(self) -> Self;

    /// Asserts the [`Result`] is [`Ok`] only in debug builds.
    ///
    /// # Panics
    ///
    /// If it is [`Err`] and `debug_assertions` are enabled, the method panics.
    /// If `debug_assertions` are disabled, the method is a no-op even if it is [`Err`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<i32, &str> = Ok(21);
    /// let x = x.debug_assert_ok().map(|v| v * 2);
    /// assert_eq!(x, Ok(42));
    /// ```
    fn debug_assert_ok(self) -> Self;
}

/// An extension trait to add the assertion_err methods.
pub trait AssertErrExt {
    /// Asserts the [`Result`] is [`Err`].
    ///
    /// # Panics
    ///
    /// If it is [`Ok`], the method panics.
    ///
    /// # Examples
    ///
    /// This sample
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Err(21);
    /// let x = x.assert_err().map_err(|v| v * 2);
    /// assert_eq!(x, Err(42));
    /// ```
    ///
    /// ```rust,should_panic
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Ok("success");
    /// let _ = x.assert_err().map_err(|v| v * 2);
    /// //        ^-- panics here
    fn assert_err(self) -> Self;

    /// Asserts the [`Result`] is [`Err`] only in debug builds.
    ///
    /// # Panics
    ///
    /// If it is [`Ok`] and `debug_assertions` are enabled, the method panics.
    /// If `debug_assertions` are disabled, the method is a no-op even if it is [`Ok`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chain_assertions::prelude::*;
    ///
    /// let x: Result<&str, i32> = Err(42);
    /// let _ = x.debug_assert_err();
    /// ```
    fn debug_assert_err(self) -> Self;
}

impl<T, E> AssertOkExt for Result<T, E>
where
    E: crate::fmt::Debug,
{
    #[track_caller]
    #[inline]
    fn assert_ok(self) -> Self {
        if let Err(ref e) = self {
            panic!("Expected Ok(_), got Err({:?})", e);
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_ok(self) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            if let Err(ref e) = self {
                panic!("Expected Ok(_), got Err({:?})", e);
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
        if let Ok(ref v) = self {
            panic!("Expected Err(_), got Ok({:?})", v);
        }
        self
    }

    #[track_caller]
    #[inline]
    fn debug_assert_err(self) -> Self {
        #[cfg(all(debug_assertions, not(feature = "passthrough")))]
        {
            if let Ok(ref v) = self {
                panic!("Expected Err(_), got Ok({:?})", v);
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

    mod assert_ok {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_ok() {
            let res: Result<NonDebuggable, Debuggable> = Ok(NonDebuggable);
            let target = res.assert_ok();

            assert!(
                matches!(target, Ok(NonDebuggable)),
                "Expected Ok(NonDebuggable)"
            );
        }

        #[test]
        #[should_panic(expected = "Expected Ok(_), got Err(Debuggable)")]
        fn it_fails_on_err() {
            let res: Result<NonDebuggable, Debuggable> = Err(Debuggable);
            let _ = res.assert_ok();
            //          ^-- should panic here
        }
    }

    mod debug_assert_ok {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_ok() {
            let res: Result<NonDebuggable, Debuggable> = Ok(NonDebuggable);
            let target = res.debug_assert_ok();

            assert!(matches!(target, Ok(NonDebuggable)), "Expected Ok(_)");
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic = "Expected Ok(_), got Err(Debuggable)"
        )]
        fn it_fails_on_err_only_in_debug_mode() {
            let res: Result<NonDebuggable, Debuggable> = Err(Debuggable);
            let target = res.debug_assert_ok();
            //               ^-- panic here only in debug builds

            assert!(
                matches!(target, Err(Debuggable)),
                "Expected Err(Debuggable)"
            );
        }
    }

    mod assert_err {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_err() {
            let res: Result<Debuggable, NonDebuggable> = Err(NonDebuggable);
            let target = res.assert_err();
            assert!(
                matches!(target, Err(NonDebuggable)),
                "Expected Err(NonDebuggable)"
            );
        }

        #[test]
        #[should_panic(expected = "Expected Err(_), got Ok(Debuggable)")]
        fn it_fails_on_ok() {
            let res: Result<Debuggable, NonDebuggable> = Ok(Debuggable);
            let _ = res.assert_err();
            //          ^-- should panic here
        }
    }

    mod debug_assert_err {
        use super::{super::*, *};

        #[test]
        fn it_succeeds_on_err() {
            let res: Result<Debuggable, NonDebuggable> = Err(NonDebuggable);
            let target = res.debug_assert_err();
            assert!(
                matches!(target, Err(NonDebuggable)),
                "Expected Err(NonDebuggable)"
            );
        }

        #[test]
        #[cfg_attr(
            all(debug_assertions, not(feature = "passthrough")),
            should_panic = "Expected Err(_), got Ok(Debuggable)"
        )]
        fn it_fails_on_ok_only_in_debug_mode() {
            let res: Result<Debuggable, NonDebuggable> = Ok(Debuggable);
            let target = res.debug_assert_err();
            //               ^-- panic here only in debug builds
            assert!(matches!(target, Ok(Debuggable)), "Expected Ok(Debuggable)");
        }
    }
}
