use crate::Moment;
use async_trait::async_trait;

use std::{marker::PhantomData, error::Error};

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

pub trait AsConvert<O: Moment>: Moment {
  fn to_convert(self) -> Convert<Self, O> {
    Convert::new(self)
  }
}

impl<I, O> AsConvert<O> for I
where
  I: Moment,
  O: Moment,
  I::Value: Into<O>
{}


pub struct TryConvert<
  I: Moment,
  O: Moment
>(pub I, PhantomData<O>);

impl<I, O> TryConvert<I, O>
where
  I: Moment,
  O: Moment
{
  pub fn new(input: I) -> Self {
    Self(input, PhantomData)
  }
}

#[async_trait]
impl<I, O> Moment for TryConvert<I, O>
where
  I: Moment,
  O: Moment,
  I::Value: TryInto<O>,
  <I::Value as TryInto<O>>::Error: Error + Send + Sync + 'static 
{
  type Value = Result<O, <I::Value as TryInto<O>>::Error>;

  async fn resolve(self) -> Self::Value {
    self.0.resolve().await.try_into()
  }
}

pub trait AsTryConvert<O: Moment>: Moment {
  fn to_try_convert(self) -> TryConvert<Self, O> {
    TryConvert::new(self)
  }
}

impl<I, O> AsTryConvert<O> for I
where
  I: Moment,
  O: Moment,
  I::Value: TryInto<O>
{}