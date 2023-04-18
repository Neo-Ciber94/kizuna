use std::fmt::{Display, Formatter};

/// An error that occurred while resolving a dependency.
#[derive(Debug)]
pub enum LocatorError {
    /// When a dependency is not found.
    NotFound { expected: &'static str },

    /// Other error that occurred while resolving a dependency.
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl LocatorError {
    /// Returns a not found error for the given type.
    pub fn not_found<T>() -> LocatorError {
        LocatorError::NotFound {
            expected: std::any::type_name::<T>(),
        }
    }
}

impl Display for LocatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocatorError::NotFound { expected } => {
                write!(f, "unable to find `{}` in locator", expected)
            }
            LocatorError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for LocatorError {}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for LocatorError {
    fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        LocatorError::Other(err)
    }
}
