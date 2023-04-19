#![allow(clippy::type_complexity)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
};
use crate::{AsyncInvoke, FromLocator, Invoke, LocatorError};

/// A wrapper that stores the services from a locator.
pub enum Provider {
    Single(Box<dyn Fn() -> Box<dyn Any + Send + Sync> + Send + Sync>),
    Factory(Box<dyn Fn(&Locator) -> Box<dyn Any + Send + Sync> + Send + Sync>),
}

/// A service locator.
#[derive(Default)]
pub struct Locator(HashMap<TypeId, Provider>);


impl Locator {
    /// Inserts a provider without checking the types.
    #[inline]
    pub fn unchecked_insert(&mut self, id: TypeId, provider: Provider) -> Option<Provider> {
        self.0.insert(id, provider)
    }

    /// Gets a provider for the given type without checking if the types matches.
    #[inline]
    pub fn unchecked_get(&self, id: &TypeId) -> Option<&Provider> {
        self.0.get(id)
    }
}

impl Locator {
    /// Creates a new `Locator`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Inserts a value of type `T` into the `Locator`.
    pub fn insert<T>(&mut self, value: T) -> Option<Provider>
    where
        T: Send + Sync + Clone + 'static,
    {
        let provider = Provider::Single(Box::new(move || Box::new(value.clone())));
        self.unchecked_insert(TypeId::of::<T>(), provider)
    }

    /// Inserts a value of type `T` into the `Locator` using a factory function that takes a `Locator` as input.
    pub fn insert_with<F, T>(&mut self, factory: F) -> Option<Provider>
    where
        F: Fn(&Self) -> T + 'static + Send + Sync,
        T: Send + Sync + 'static,
    {
        let provider = Provider::Factory(Box::new(move |locator| {
            let value = factory(locator);
            Box::new(value)
        }));

        self.unchecked_insert(TypeId::of::<T>(), provider)
    }

    /// Returns a value of type `T` from the `Locator` if it exists.
    pub fn get<T>(&self) -> Option<T>
    where
        T: Send + Sync + 'static,
    {
        let provider = self.unchecked_get(&TypeId::of::<T>())?;

        match provider {
            Provider::Single(f) => {
                let value = f();
                value.downcast::<T>().map(|x| *x).ok()
            }
            Provider::Factory(f) => {
                let value = f(self);
                value.downcast::<T>().map(|x| *x).ok()
            }
        }
    }

    /// Returns a boolean indicating whether a value of type `T` exists in the `Locator`.
    pub fn contains<T>(&self) -> bool
    where
        T: Send + Sync + 'static,
    {
        self.0.contains_key(&TypeId::of::<T>())
    }

    /// Removes a value of type `T` from the `Locator` if it exists.
    pub fn remove<T>(&mut self) -> Option<Provider>
    where
        T: Send + Sync + 'static,
    {
        self.0.remove(&TypeId::of::<T>())
    }

    /// Returns the number of services in the locator.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the locator is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Adds the providers from other locator.
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    /// Invoke the given function injecting the dependencies from this locator.
    pub fn invoke<F, Args>(&self, f: F) -> Result<F::Output, LocatorError>
    where
        F: Invoke<Args>,
        Args: FromLocator,
    {
        let args = Args::from_locator(self)?;
        Ok(Invoke::call(f, args))
    }

    /// Invoke the given async function injecting the dependencies from this locator.
    pub async fn invoke_async<F, Fut, Args>(&self, f: F) -> Result<Fut::Output, LocatorError>
    where
        F: AsyncInvoke<Args, Fut = Fut>,
        Fut: Future,
        Args: FromLocator,
    {
        let args = Args::from_locator(self)?;
        Ok(AsyncInvoke::call(f, args).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct MyStruct {
        val: i32,
    }

    #[test]
    fn test_insert_single() {
        let mut locator = Locator::new();

        assert!(locator.insert(MyStruct { val: 42 }).is_none());
        assert_eq!(locator.get::<MyStruct>().unwrap().val, 42);
    }

    #[test]
    fn test_insert_with_factory() {
        let mut locator = Locator::new();

        locator.insert_with::<_, MyStruct>(|_| MyStruct { val: 42 });
        assert_eq!(locator.get::<MyStruct>().unwrap().val, 42);
    }

    #[test]
    fn test_contains() {
        let mut locator = Locator::new();

        assert!(!locator.contains::<MyStruct>());
        locator.insert(MyStruct { val: 42 });
        assert!(locator.contains::<MyStruct>());
    }

    #[test]
    fn test_remove() {
        let mut locator = Locator::new();

        locator.insert(MyStruct { val: 42 });

        assert!(locator.contains::<MyStruct>());
        assert!(locator.remove::<MyStruct>().is_some());
        assert!(!locator.contains::<MyStruct>());
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut locator = Locator::new();

        assert!(locator.is_empty());
        locator.insert(MyStruct { val: 42 });

        assert_eq!(locator.len(), 1);
        assert!(!locator.is_empty());
    }

    #[test]
    fn test_extend() {
        let mut locator1 = Locator::new();
        let mut locator2 = Locator::new();

        locator1.insert(MyStruct { val: 42 });
        locator2.insert_with::<_, MyStruct>(|_| MyStruct { val: 10 });
        locator1.extend(locator2);

        assert_eq!(locator1.get::<MyStruct>().unwrap().val, 10);
    }

    #[test]
    fn test_invoke() {
        let mut locator = Locator::new();

        locator.insert(MyStruct { val: 42 });
        let result = locator.invoke(|my_struct: MyStruct| my_struct.val).unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_invoke_async() {
        let mut locator = Locator::new();

        locator.insert(MyStruct { val: 42 });

        let result = locator
            .invoke_async(|my_struct: MyStruct| async move { my_struct.val })
            .await
            .unwrap();

        assert_eq!(result, 42);
    }
}
