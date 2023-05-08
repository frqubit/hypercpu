#![warn(missing_docs)]

//! # HyperCPU
//! 
//! HyperCPU is a library for creating and running
//! any kind of calculation across instances. It
//! allows you to share your calculation logic
//! across threads, processes, and even machines.
//!
//! At the core of HyperCPU is the [`Moment`] trait.
//! A `Moment` is a unit of work that can be
//! resolved into a value. The `resolve` method
//! is asynchronous, so values do not need to be
//! immediately available.
//!
//! Thanks to the logic behind a `Moment`, calculations
//! can be stacked without resolution. This allows
//! machines to resolve values in parallel, even
//! when they depend on each other.

use std::error::Error;

use async_trait::async_trait;

/// Provides types and traits for converting between `Moment`s.
/// 
/// This module is almost always required when using HyperCPU.
/// It provides the [`Convert`] and [`TryConvert`] traits,
/// which allow you to convert between `Moment`s of different
/// value types.
pub mod convert;

/// Provides types and traits for various operations on `Moment`s.
/// 
/// This module is almost always required when using HyperCPU.
/// It provides the standard arithmetic and logical operators
/// for `Moment`s.
pub mod ops;

/// Provides the HyperCPU prelude.
pub mod prelude;

/// A unit of work that can be resolved into a value.
/// 
/// Any type that implements `Moment` can be stacked with
/// other `Moment`s to create a calculation. The calculation
/// can be resolved in parallel, even when the `Moment`s
/// depend on the output of each other.
#[async_trait]
pub trait Moment: Send + Sync + Sized {
  /// The type of value that this `Moment` resolves to.
  /// 
  /// It must be thread friendly and sized.
  type Value: Send + Sync + Sized;

  /// Resolve this `Moment` into its value.
  /// 
  /// This will resolve any dependencies of this `Moment`
  /// and then resolve this `Moment` into its value. If
  /// this `Moment` has no dependencies, it will resolve
  /// immediately.
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