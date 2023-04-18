use crate::{Locator, LocatorError, Provider};
use std::any::TypeId;

/// A locator that may fail to resolve a service.
pub trait TryLocator: sealed::Sealed {
    /// Attempts to insert a service that may fail to resolve.
    fn try_insert_with<F, T>(&mut self, factory: F) -> Option<Provider>
    where
        F: Fn(&Self) -> Result<T, LocatorError> + 'static,
        T: Send + Sync + 'static;

    /// Returns a service inserted by `try_insert_with` or fail if cannot be resolved.
    fn try_get<T>(&self) -> Result<T, LocatorError>
    where
        T: Send + Sync + 'static;
}

impl TryLocator for Locator {
    fn try_insert_with<F, T>(&mut self, factory: F) -> Option<Provider>
    where
        F: Fn(&Self) -> Result<T, LocatorError> + 'static,
        T: Send + Sync + 'static,
    {
        let provider = Provider::Factory(Box::new(move |locator| {
            let value = factory(locator);
            Box::new(value)
        }));

        self.unchecked_insert(TypeId::of::<Result<T, LocatorError>>(), provider)
    }

    fn try_get<T>(&self) -> Result<T, LocatorError>
    where
        T: Send + Sync + 'static,
    {
        let provider = self
            .unchecked_get(&TypeId::of::<Result<T, LocatorError>>())
            .ok_or(LocatorError::NotFound {
                expected: std::any::type_name::<T>(),
            })?;

        match provider {
            Provider::Single(f) => {
                let value = f();
                value
                    .downcast::<Result<T, LocatorError>>()
                    .map(|x| *x)
                    .map_err(|_| LocatorError::NotFound {
                        expected: std::any::type_name::<T>(),
                    })
                    .and_then(std::convert::identity)
            }
            Provider::Factory(f) => {
                let value = f(&self);
                value
                    .downcast::<Result<T, LocatorError>>()
                    .map(|x| *x)
                    .map_err(|_| LocatorError::NotFound {
                        expected: std::any::type_name::<T>(),
                    })
                    .and_then(std::convert::identity)
            }
        }
    }
}

impl sealed::Sealed for Locator {}

pub(crate) mod sealed {
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use crate::{try_locator::TryLocator, Locator, LocatorError};

    #[derive(Debug)]
    struct ServiceA;

    #[derive(Debug)]
    struct ServiceB;

    #[test]
    fn test_try_insert_and_try_get() {
        let mut locator = Locator::new();

        // Insert a service that may fail to resolve.
        locator.try_insert_with::<_, ServiceA>(|_| Ok(ServiceA));

        // Try to get the service.
        let service_a = locator.try_get::<ServiceA>();

        // Ensure the service is returned successfully.
        assert!(service_a.is_ok());

        // Insert another service that may fail to resolve.
        locator.try_insert_with::<_, ServiceB>(|_| Err(LocatorError::not_found::<ServiceB>()));

        // Try to get the service.
        let service_b = locator.try_get::<ServiceB>();

        // Ensure the service cannot be resolved.
        assert!(service_b.is_err());
        assert!(matches!(
            service_b.unwrap_err(),
            LocatorError::NotFound { .. }
        ));
    }
}
