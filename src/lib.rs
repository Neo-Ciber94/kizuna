/// Provides a mechanism for insert and get dependencies that may fail.
pub mod try_locator;

//
mod error;
mod from_locator;
mod invoke;
mod locator;

pub use {error::*, from_locator::*, invoke::*, locator::*};
