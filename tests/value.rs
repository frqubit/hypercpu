use hypercpu::prelude::*;


#[tokio::test]
async fn operations() {
  let a = Value::new(1);

  let a = a + 1;
  assert_eq!(a.resolve().await, 2);
}