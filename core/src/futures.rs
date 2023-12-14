use std::pin::Pin;

use futures::Future;

pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;