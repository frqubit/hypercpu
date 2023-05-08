use async_trait::async_trait;

pub mod convert;

pub mod prelude;

#[async_trait]
pub trait Moment: Send + Sync {
  type Value: Send + Sync;

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