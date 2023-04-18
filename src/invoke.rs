use std::future::Future;

/// Represents a function that can be invoke using a service locator.
pub trait Invoke<Args> {
    /// The result of the function.
    type Output;

    /// Invokes the given function with its arguments.
    fn call(self, args: Args) -> Self::Output;
}

macro_rules! impl_invoke {
    ($($ty:ident),*) => {
        impl<Func, Out, $($ty),*> Invoke<($($ty,)*)> for Func
            where Func: FnOnce($($ty),*) -> Out,
        {
            type Output = Out;

            #[inline]
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            fn call(self, ($($ty,)*): (($($ty,)*))) -> Self::Output {
                (self)( $($ty),*)
            }
        }
    };
}

impl_invoke!(A);
impl_invoke!(A, B);
impl_invoke!(A, B, C);
impl_invoke!(A, B, C, D);
impl_invoke!(A, B, C, D, E);
impl_invoke!(A, B, C, D, E, F);
impl_invoke!(A, B, C, D, E, F, G);
impl_invoke!(A, B, C, D, E, F, G, H);
impl_invoke!(A, B, C, D, E, F, G, H, I);
impl_invoke!(A, B, C, D, E, F, G, H, I, J);
impl_invoke!(A, B, C, D, E, F, G, H, I, J, K);
impl_invoke!(A, B, C, D, E, F, G, H, I, J, K, L);

/// Represents an async function that can be invoke using a service locator.
pub trait AsyncInvoke<Args> {
    /// The resulting future.
    type Fut: Future;

    /// Invokes the given function with its arguments.
    fn call(self, args: Args) -> Self::Fut;
}

macro_rules! impl_async_invoke {
    ($($ty:ident),*) => {
        impl<Func, Fut, $($ty),*> AsyncInvoke<($($ty,)*)> for Func
            where Func: FnOnce($($ty),*) -> Fut,
            Fut: Future
        {
            type Fut = Fut;

            #[inline]
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            fn call(self, ($($ty,)*): (($($ty,)*))) -> Self::Fut {
                (self)( $($ty),*)
            }
        }
    };
}

impl_async_invoke!(A);
impl_async_invoke!(A, B);
impl_async_invoke!(A, B, C);
impl_async_invoke!(A, B, C, D);
impl_async_invoke!(A, B, C, D, E);
impl_async_invoke!(A, B, C, D, E, F);
impl_async_invoke!(A, B, C, D, E, F, G);
impl_async_invoke!(A, B, C, D, E, F, G, H);
impl_async_invoke!(A, B, C, D, E, F, G, H, I);
impl_async_invoke!(A, B, C, D, E, F, G, H, I, J);
impl_async_invoke!(A, B, C, D, E, F, G, H, I, J, K);
impl_async_invoke!(A, B, C, D, E, F, G, H, I, J, K, L);
