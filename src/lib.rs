#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
pub(crate) use core::fmt;

#[cfg(feature = "std")]
pub(crate) use std::fmt;

pub mod option;
pub mod prelude;
pub mod result;
