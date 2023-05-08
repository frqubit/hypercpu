use hypercpu::prelude::*;
use hypercpu::cond::If;


#[tokio::test]
async fn is_true() {
  let c = true;
  let t = 1;
  let f = 2;

  let i = If::new(c, t, f);
  let r = i.resolve().await;

  assert_eq!(r.expect_left("Was not true"), 1);
}

#[tokio::test]
async fn is_false() {
  let c = false;
  let t = 1;
  let f = 2;

  let i = If::new(c, t, f);
  let r = i.resolve().await;

  assert_eq!(r.expect_right("Was not false"), 2);
}