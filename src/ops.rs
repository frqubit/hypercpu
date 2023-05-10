use crate::Moment;
use async_trait::async_trait;
use std::marker::PhantomData;

/// Add two [`Moment`]s together.
#[derive(Clone)]
pub struct Add<A: Moment, B: Moment, O: Moment>(pub A, pub B, PhantomData<O>);
impl<A, B, O> Add<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Add<B::Value, Output = O>
{
  /// Create a new `Add` struct.
  pub fn new(a: A, b: B) -> Self {
    Self(a, b, PhantomData)
  }
}

#[async_trait]
impl<A, B, O> Moment for Add<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Add<B::Value, Output = O>
{
  type Value = O;

  async fn resolve(&self) -> Self::Value {
    self.0.resolve().await + self.1.resolve().await
  }
}

/// Subtract one [`Moment`] from another.
#[derive(Clone)]
pub struct Sub<A: Moment, B: Moment, O: Moment>(pub A, pub B, PhantomData<O>);
impl<A, B, O> Sub<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Sub<B::Value, Output = O>
{
  /// Create a new `Sub` struct.
  pub fn new(a: A, b: B) -> Self {
    Self(a, b, PhantomData)
  }
}

#[async_trait]
impl<A, B, O> Moment for Sub<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Sub<B::Value, Output = O>
{
  type Value = O;

  async fn resolve(&self) -> Self::Value {
    self.0.resolve().await - self.1.resolve().await
  }
}

/// Multiply two [`Moment`]s together.
#[derive(Clone)]
pub struct Mul<A: Moment, B: Moment, O: Moment>(pub A, pub B, PhantomData<O>);
impl<A, B, O> Mul<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Mul<B::Value, Output = O>
{
  /// Create a new `Mul` struct.
  pub fn new(a: A, b: B) -> Self {
    Self(a, b, PhantomData)
  }
}

#[async_trait]
impl<A, B, O> Moment for Mul<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Mul<B::Value, Output = O>
{
  type Value = O;

  async fn resolve(&self) -> Self::Value {
    self.0.resolve().await * self.1.resolve().await
  }
}

/// Divide one [`Moment`] by another.
#[derive(Clone)]
pub struct Div<A: Moment, B: Moment, O: Moment>(pub A, pub B, PhantomData<O>);
impl<A, B, O> Div<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Div<B::Value, Output = O>
{
  /// Create a new `Div` struct.
  pub fn new(a: A, b: B) -> Self {
    Self(a, b, PhantomData)
  }
}

#[async_trait]
impl<A, B, O> Moment for Div<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Div<B::Value, Output = O>
{
  type Value = O;

  async fn resolve(&self) -> Self::Value {
    self.0.resolve().await / self.1.resolve().await
  }
}

/// Get the remainder of dividing one [`Moment`] by another.
#[derive(Clone)]
pub struct Rem<A: Moment, B: Moment, O: Moment>(pub A, pub B, PhantomData<O>);
impl<A, B, O> Rem<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Rem<B::Value, Output = O>
{
  /// Create a new `Rem` struct.
  pub fn new(a: A, b: B) -> Self {
    Self(a, b, PhantomData)
  }
}

#[async_trait]
impl<A, B, O> Moment for Rem<A, B, O>
where
  A: Moment,
  B: Moment,
  O: Moment,
  A::Value: std::ops::Rem<B::Value, Output = O>
{
  type Value = O;

  async fn resolve(&self) -> Self::Value {
    self.0.resolve().await % self.1.resolve().await
  }
}