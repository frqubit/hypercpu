use crate::Moment;
use async_trait::async_trait;
use either::Either;

/// A conditional calculation.
/// 
/// This struct allows you to branch calculations based
/// on a condition. Both sides are still optimized, but
/// only one side will be fully resolved.
pub struct If<C: Moment<Value = bool>, T: Moment, F: Moment> {
  condition: C,
  then: T,
  otherwise: F,
}

impl<C: Moment<Value = bool>, T: Moment, F: Moment> If<C, T, F> {
  /// Create a new `If` struct.
  /// 
  /// This will branch the calculation based on the
  /// condition. If the condition is true, the `then`
  /// calculation will be resolved. Otherwise, the
  /// `otherwise` calculation will be resolved.
  pub fn new(condition: C, then: T, otherwise: F) -> Self {
    Self {
      condition,
      then,
      otherwise,
    }
  }
}

#[async_trait]
impl<C, T, F> Moment for If<C, T, F>
where
  C: Moment<Value = bool>,
  T: Moment,
  F: Moment
{
  type Value = Either<T, F>;

  async fn resolve(self) -> Self::Value {
    if self.condition.resolve().await {
      Either::Left(self.then)
    } else {
      Either::Right(self.otherwise)
    }
  }
}

#[async_trait]
impl<L, R> Moment for Either<L, R>
where
  L: Moment,
  R: Moment
{
  type Value = Either<L::Value, R::Value>;

  async fn resolve(self) -> Self::Value {
    match self {
      Either::Left(l) => Either::Left(l.resolve().await),
      Either::Right(r) => Either::Right(r.resolve().await),
    }
  }
}