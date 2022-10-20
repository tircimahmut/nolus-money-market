use finance::currency::{AnyVisitor, Currency, SingleVisitor};
use serde::{de::DeserializeOwned, Serialize};

pub mod lease;
pub mod lpn;
pub mod native;
pub mod payment;

struct SingleVisitorAdapter<V>(V);

impl<V> From<V> for SingleVisitorAdapter<V> {
    fn from(any_visitor: V) -> Self {
        Self(any_visitor)
    }
}

impl<C, V> SingleVisitor<C> for SingleVisitorAdapter<V>
where
    C: 'static + Currency + Serialize + DeserializeOwned,
    V: AnyVisitor,
{
    type Output = V::Output;
    type Error = V::Error;

    fn on(self) -> Result<Self::Output, Self::Error> {
        self.0.on::<C>()
    }
}
