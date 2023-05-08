use hypercpu::prelude::*;


#[tokio::test]
async fn numbers() {
  let a = 1u8;

  let b: u16 = Convert::new(a).resolve().await;
  let c: u32 = Convert::new(b).resolve().await;
  let d: u64 = Convert::new(c).resolve().await;

  assert_eq!(a, b as u8);
  assert_eq!(a, c as u8);
  assert_eq!(a, d as u8);

  let a = 1i8;

  let b: i16 = Convert::new(a).resolve().await;
  let c: i32 = Convert::new(b).resolve().await;
  let d: i64 = Convert::new(c).resolve().await;

  assert_eq!(a, b as i8);
  assert_eq!(a, c as i8);
  assert_eq!(a, d as i8);

  let a = 1f32;

  let b: f64 = Convert::new(a).resolve().await;

  assert_eq!(a, b as f32);
}