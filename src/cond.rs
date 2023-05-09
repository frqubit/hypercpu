use crate::Moment;
use async_trait::async_trait;

/// A conditional calculation.
/// 
/// This struct allows you to branch calculations based
/// on a condition. Both sides are still optimized, but
/// only one side will be fully resolved.
pub struct If<C: Moment<Value = bool>, T: Moment, F: Moment> {
  pub(crate) condition: C,
  pub(crate) then: T,
  pub(crate) otherwise: F,
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
  type Value = If<C, T::Value, F::Value>;

  async fn resolve(self) -> Self::Value {
    If::new(
      self.condition,
      self.then.resolve().await,
      self.otherwise.resolve().await,
    )
  }
}