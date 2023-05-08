use hypercpu::prelude::*;


#[tokio::test]
async fn basic() {
  let a = 1u8;
  let b: u16 = a.to_convert().resolve().await;

  assert_eq!(a, b as u8);
}

#[tokio::test]
async fn fallible() {
  let a: u64 = 1;
  let b: u32 = a.to_try_convert().resolve().await.unwrap();

  assert_eq!(a, b as u64);
}

#[tokio::test]
#[should_panic]
async fn fallible_panic() {
  let a: i64 = -1;
  let _: u32 = a.to_try_convert().resolve().await.unwrap();
}