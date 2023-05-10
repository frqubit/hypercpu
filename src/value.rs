use crate::Moment;
use crate::ops::*;
use async_trait::async_trait;
use std::ops;


/// A `Moment`-exclusive operable value.
/// 
/// Performing operations on this type will return
/// `Moment`s that perform the operation on the
/// value instead of the standard compiler
/// behaviors.
#[derive(Clone)]
pub struct Value<T>(T);

impl<T> Value<T> {
  /// Create a new `Value` from a value.
  pub fn new(value: T) -> Self {
    Self(value)
  }

  /// Get the inner value.
  pub fn into_inner(self) -> T {
    self.0
  }
}

#[async_trait]
impl<T> Moment for Value<T>
where
  T: Moment
{
  type Value = Self;

  async fn resolve(&self) -> Self::Value {
    self.clone()
  }
}

impl<L, R, O> ops::Add<R> for Value<L>
where
  L: Moment,
  R: Moment,
  O: Moment,
  L::Value: ops::Add<R::Value, Output = O>
{
  type Output = Add<L, R, O>;

  fn add(self, rhs: R) -> Self::Output {
    Add::new(self.into_inner(), rhs)
  }
}

impl<L, R, O> ops::Sub<R> for Value<L>
where
  L: Moment,
  R: Moment,
  O: Moment,
  L::Value: ops::Sub<R::Value, Output = O>
{
  type Output = Sub<L, R, O>;

  fn sub(self, rhs: R) -> Self::Output {
    Sub::new(self.into_inner(), rhs)
  }
}

impl<L, R, O> ops::Mul<R> for Value<L>
where
  L: Moment,
  R: Moment,
  O: Moment,
  L::Value: ops::Mul<R::Value, Output = O>
{
  type Output = Mul<L, R, O>;

  fn mul(self, rhs: R) -> Self::Output {
    Mul::new(self.into_inner(), rhs)
  }
}

impl<L, R, O> ops::Div<R> for Value<L>
where
  L: Moment,
  R: Moment,
  O: Moment,
  L::Value: ops::Div<R::Value, Output = O>
{
  type Output = Div<L, R, O>;

  fn div(self, rhs: R) -> Self::Output {
    Div::new(self.into_inner(), rhs)
  }
}

impl<L, R, O> ops::Rem<R> for Value<L>
where
  L: Moment,
  R: Moment,
  O: Moment,
  L::Value: ops::Rem<R::Value, Output = O>
{
  type Output = Rem<L, R, O>;

  fn rem(self, rhs: R) -> Self::Output {
    Rem::new(self.into_inner(), rhs)
  }
}