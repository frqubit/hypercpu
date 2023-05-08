use std::error::Error;

use async_trait::async_trait;

pub mod convert;

pub mod prelude;

#[async_trait]
pub trait Moment: Send + Sync + Sized {
  type Value: Send + Sync + Sized;

  async fn resolve(self) -> Self::Value;
}

macro_rules! literal_moments {
  ($($ty:ty),*) => {
    $(
      #[async_trait]
      impl Moment for $ty {
        type Value = $ty;

        async fn resolve(self) -> Self::Value {
          self
        }
      }
    )*
  }
}

literal_moments![
  u8, u16, u32, u64, u128, usize,
  i8, i16, i32, i64, i128, isize,
  f32, f64,
  bool,
  char
];

#[async_trait]
impl<T, E> Moment for Result<T, E>
where
  T: Moment,
  E: Error + Send + Sync + 'static
{
  type Value = Result<T::Value, E>;

  async fn resolve(self) -> Self::Value {
    match self {
      Ok(value) => Ok(value.resolve().await),
      Err(error) => Err(error)
    }
  }
}

#[async_trait]
impl<T> Moment for Option<T>
where
  T: Moment
{
  type Value = Option<T::Value>;

  async fn resolve(self) -> Self::Value {
    match self {
      Some(value) => Some(value.resolve().await),
      None => None
    }
  }
}