use crate::{LocatorError, Locator};

/// A type that can be constructed from a `Locator`.
pub trait FromLocator : Sized {
    /// Constructs this type from the given `Locator`.
    fn from_locator(locator: &Locator) -> Result<Self, LocatorError>;
}

macro_rules! impl_from_locator_for_tuple {
    ( $($ty:ident),* ) => {
        impl<$($ty),*> FromLocator for ($($ty,)*) 
            where $($ty: Send + Sync + 'static),* {

            fn from_locator(locator: &Locator) -> Result<Self, LocatorError> {
                Ok((
                    $(
                        locator.get::<$ty>().ok_or(LocatorError::NotFound { expected: std::any::type_name::<$ty>() })?
                    ,)*
                ))
            }
        }
    };
}

impl_from_locator_for_tuple!(A);
impl_from_locator_for_tuple!(A, B);
impl_from_locator_for_tuple!(A, B, C);
impl_from_locator_for_tuple!(A, B, C, D);
impl_from_locator_for_tuple!(A, B, C, D, E);
impl_from_locator_for_tuple!(A, B, C, D, E, F);
impl_from_locator_for_tuple!(A, B, C, D, E, F, G);
impl_from_locator_for_tuple!(A, B, C, D, E, F, G, H);
impl_from_locator_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_from_locator_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_from_locator_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_from_locator_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

