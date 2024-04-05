use distcpu::prelude::*;


#[tokio::test]
async fn test_add() {
  let a: u8 = 10;
  let b: u8 = 2;

  let c = Add::new(a, b).resolve().await;

  assert_eq!(c, 12);
}

#[tokio::test]
async fn test_sub() {
  let a: u8 = 10;
  let b: u8 = 2;

  let c = Sub::new(a, b).resolve().await;

  assert_eq!(c, 8);
}

#[tokio::test]
async fn test_mul() {
  let a: u8 = 10;
  let b: u8 = 2;

  let c = Mul::new(a, b).resolve().await;

  assert_eq!(c, 20);
}

#[tokio::test]
async fn test_div() {
  let a: u8 = 10;
  let b: u8 = 2;

  let c = Div::new(a, b).resolve().await;

  assert_eq!(c, 5);
}

#[tokio::test]
async fn test_rem() {
  let a: u8 = 10;
  let b: u8 = 2;

  let c = Rem::new(a, b).resolve().await;

  assert_eq!(c, 0);
}

#[tokio::test]
async fn test_nested() {
  let a: u8 = 10;
  let b: u8 = 2;
  let c: u8 = 3;

  let d = Add::new(Mul::new(a, b), c).resolve().await;

  assert_eq!(d, 23);
}

#[tokio::test]
async fn test_nested_2() {
  let a: u8 = 10;
  let b: u8 = 2;
  let c: u8 = 3;

  let d = Add::new(Mul::new(a, b), Mul::new(b, c)).resolve().await;

  assert_eq!(d, 26);
}