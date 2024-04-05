use distcpu::prelude::*;
use distcpu::cond::If;


#[tokio::test]
async fn is_true() {
  let c = true;
  let t = 1;
  let f = 2;

  let i = If::new(c, t, f);
  let r = i.resolve().await;

  assert_eq!(r.to_then().await, Some(1));
}

#[tokio::test]
async fn is_false() {
  let c = false;
  let t = 1;
  let f = 2;

  let i = If::new(c, t, f);
  let r = i.resolve().await;

  assert_eq!(r.to_otherwise().await, Some(2));
}

#[tokio::test]
async fn nested() {
  let a = 10;
  let b = 20;

  let c: i32 = If::new(a > b, a, b)
    .resolve()
    .await
    .to_otherwise()
    .await
    .unwrap();
  
  assert_eq!(c, 20);
}