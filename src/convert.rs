use crate::Moment;
use async_trait::async_trait;

use std::marker::PhantomData;

pub struct Convert<I: Moment, O: Moment>(pub I, PhantomData<O>);

impl<I, O> Convert<I, O>
where
  I: Moment,
  O: Moment
{
  pub fn new(input: I) -> Self {
    Self(input, PhantomData)
  }
}

#[async_trait]
impl<I, O> Moment for Convert<I, O>
where
  I: Moment,
  O: Moment,
  I::Value: Into<O>
{
  type Value = O;

  async fn resolve(self) -> Self::Value {
    self.0.resolve().await.into()
  }
}