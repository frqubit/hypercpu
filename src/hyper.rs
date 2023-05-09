use crate::Moment;
use crate::convert::Convert;
use crate::ops::*;
use crate::cond::If;
use std::ops;

/****************************************
* Convert
****************************************/
impl<I, L, R, O> ops::Add<R> for Convert<I, L>
where
  I: Moment,
  L: Moment + ops::Add<R::Value, Output = O>,
  R: Moment,
  O: Moment,
  I::Value: Into<L>,
{
  type Output = Add<Convert<I, L>, R, O>;

  fn add(self, rhs: R) -> Self::Output {
    Add::new(self, rhs)
  }
}

impl<I, L, R, O> ops::Sub<R> for Convert<I, L>
where
  I: Moment,
  L: Moment + ops::Sub<R::Value, Output = O>,
  R: Moment,
  O: Moment,
  I::Value: Into<L>,
{
  type Output = Sub<Convert<I, L>, R, O>;

  fn sub(self, rhs: R) -> Self::Output {
    Sub::new(self, rhs)
  }
}

impl<I, L, R, O> ops::Mul<R> for Convert<I, L>
where
  I: Moment,
  L: Moment + ops::Mul<R::Value, Output = O>,
  R: Moment,
  O: Moment,
  I::Value: Into<L>,
{
  type Output = Mul<Convert<I, L>, R, O>;

  fn mul(self, rhs: R) -> Self::Output {
    Mul::new(self, rhs)
  }
}

impl<I, L, R, O> ops::Div<R> for Convert<I, L>
where
  I: Moment,
  L: Moment + ops::Div<R::Value, Output = O>,
  R: Moment,
  O: Moment,
  I::Value: Into<L>,
{
  type Output = Div<Convert<I, L>, R, O>;

  fn div(self, rhs: R) -> Self::Output {
    Div::new(self, rhs)
  }
}

impl<I, L, R, O> ops::Rem<R> for Convert<I, L>
where
  I: Moment,
  L: Moment + ops::Rem<R::Value, Output = O>,
  R: Moment,
  O: Moment,
  I::Value: Into<L>,
{
  type Output = Rem<Convert<I, L>, R, O>;

  fn rem(self, rhs: R) -> Self::Output {
    Rem::new(self, rhs)
  }
}

/****************************************
* Math Operators
****************************************/
macro_rules! math_operator {
  ($t:ident) => {
    impl<L0, R0, L1, R1, O> ops::Add<R1> for $t<L0, R0, L1>
    where
      L0: Moment,
      R0: Moment,
      L1: Moment + ops::Add<R1::Value, Output = O>,
      R1: Moment,
      O: Moment,
      L0::Value: ops::$t<R0::Value, Output = L1>,
    {
      type Output = Add<$t<L0, R0, L1>, R1, O>;

      fn add(self, rhs: R1) -> Self::Output {
        Add::new(self, rhs)
      }
    }

    impl<L0, R0, L1, R1, O> ops::Sub<R1> for $t<L0, R0, L1>
    where
      L0: Moment,
      R0: Moment,
      L1: Moment + ops::Sub<R1::Value, Output = O>,
      R1: Moment,
      O: Moment,
      L0::Value: ops::$t<R0::Value, Output = L1>,
    {
      type Output = Sub<$t<L0, R0, L1>, R1, O>;

      fn sub(self, rhs: R1) -> Self::Output {
        Sub::new(self, rhs)
      }
    }

    impl<L0, R0, L1, R1, O> ops::Mul<R1> for $t<L0, R0, L1>
    where
      L0: Moment,
      R0: Moment,
      L1: Moment + ops::Mul<R1::Value, Output = O>,
      R1: Moment,
      O: Moment,
      L0::Value: ops::$t<R0::Value, Output = L1>,
    {
      type Output = Mul<$t<L0, R0, L1>, R1, O>;

      fn mul(self, rhs: R1) -> Self::Output {
        Mul::new(self, rhs)
      }
    }

    impl<L0, R0, L1, R1, O> ops::Div<R1> for $t<L0, R0, L1>
    where
      L0: Moment,
      R0: Moment,
      L1: Moment + ops::Div<R1::Value, Output = O>,
      R1: Moment,
      O: Moment,
      L0::Value: ops::$t<R0::Value, Output = L1>,
    {
      type Output = Div<$t<L0, R0, L1>, R1, O>;

      fn div(self, rhs: R1) -> Self::Output {
        Div::new(self, rhs)
      }
    }

    impl<L0, R0, L1, R1, O> ops::Rem<R1> for $t<L0, R0, L1>
    where
      L0: Moment,
      R0: Moment,
      L1: Moment + ops::Rem<R1::Value, Output = O>,
      R1: Moment,
      O: Moment,
      L0::Value: ops::$t<R0::Value, Output = L1>,
    {
      type Output = Rem<$t<L0, R0, L1>, R1, O>;

      fn rem(self, rhs: R1) -> Self::Output {
        Rem::new(self, rhs)
      }
    }
  }
}

math_operator!(Add);
math_operator!(Sub);
math_operator!(Mul);
math_operator!(Div);
math_operator!(Rem);

/****************************************
* If
****************************************/
impl<C, T, F, R, TO, FO> ops::Add<R> for If<C, T, F>
where
  C: Moment<Value = bool>,
  T: Moment,
  F: Moment,
  R: Moment,
  TO: Moment,
  FO: Moment,
  T::Value: ops::Add<R::Value, Output = TO>,
  F::Value: ops::Add<R::Value, Output = FO>
{
  type Output = If<C, Add<T, R, TO>, Add<F, R, FO>>;

  fn add(self, rhs: R) -> Self::Output {
    If::new(
      self.condition,
      Add::new(self.then, rhs),
      Add::new(self.otherwise, rhs)
    )
  }
}