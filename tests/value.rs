use distcpu::prelude::*;
use distcpu::cond::If;


#[tokio::test]
async fn operations() {
  let a = Value::new(1);

  let a = a + 1;
  assert_eq!(a.resolve().await, 2);
}

#[tokio::test]
async fn compound() {
  let a = Value::new(1);
  let a = a + 20;
  let a = a * 30;
  let a = a - 10;

  assert_eq!(a.resolve().await, 620);
}

#[tokio::test]
async fn compound_if() {
  let a = Value::new(1);

  let a = If::new(1 % 2 == 0, a.clone() + 10, a + 20);
  let a = a * 30;
  let a = a - 10;
  let a = a.resolve().await;

  assert_eq!(a.to_otherwise().await, Some(620));
}