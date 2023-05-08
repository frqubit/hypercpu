use crate::Moment;
use async_trait::async_trait;

use std::{marker::PhantomData, error::Error};

/// Convert between `Moment`s of different value types.
/// 
/// This struct is created by the `to_convert` method on
/// [`Moment`]s. See its documentation for more information.
pub struct Convert<I: Moment, O: Moment>(pub I, PhantomData<O>);

impl<I, O> Convert<I, O>
where
  I: Moment,
  O: Moment,
  I::Value: Into<O>
{
  /// Create a [`Convert`] struct from an input [`Moment`].
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

/// Create a [`Convert`] struct from a [`Moment`].
/// 
/// This trait is automatically implemented for all
/// combinations of [`Moment`]s that can be converted
/// with the [`Into`] trait.
/// 
/// A [`Convert`] struct can be resolved into a value
/// of the output type. This is useful when you need
/// to convert between [`Moment`]s with different
/// value types, such as when the bitsize of an
/// integer needs to be changed.
pub trait AsConvert<O: Moment>: Moment {
  /// Create a [`Convert`] struct from this [`Moment`].
  fn to_convert(self) -> Convert<Self, O> {
    Convert(self, PhantomData)
  }
}

impl<I, O> AsConvert<O> for I
where
  I: Moment,
  O: Moment,
  I::Value: Into<O>
{}


/// Try to convert between `Moment`s of different value types.
/// 
/// This struct is created by the `to_try_convert` method on
/// [`Moment`]s. See the `AsConvert` trait for more information.
pub struct TryConvert<
  I: Moment,
  O: Moment
>(pub I, PhantomData<O>);

impl<I, O> TryConvert<I, O>
where
  I: Moment,
  O: Moment,
  I::Value: TryInto<O>,
  <I::Value as TryInto<O>>::Error: Error + Send + Sync + 'static 
{
  /// Create a [`TryConvert`] struct from an input [`Moment`].
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

/// Create a [`TryConvert`] struct from a [`Moment`].
/// 
/// This trait is automatically implemented for all
/// combinations of [`Moment`]s that can be converted
/// with the [`TryInto`] trait.
/// 
/// See the [`AsConvert`] trait for more information.
pub trait AsTryConvert<O: Moment>: Moment {
  /// Create a [`TryConvert`] struct from this [`Moment`].
  fn to_try_convert(self) -> TryConvert<Self, O> {
    TryConvert(self, PhantomData)
  }
}

impl<I, O> AsTryConvert<O> for I
where
  I: Moment,
  O: Moment,
  I::Value: TryInto<O>
{}