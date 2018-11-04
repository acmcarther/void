#[macro_use]
extern crate log;

pub mod vector;

pub trait Zero {
  fn zero() -> Self;
}

#[macro_export]
macro_rules! impl_zero {
  ($t:ty => $v:expr) => {
impl $crate::Zero for $t {
  fn zero() -> $t {
    $v
  }
}
  }
}

impl_zero!(f32 => 0f32);
impl_zero!(f64 => 0f64);
impl_zero!(i8 => 0i8);
impl_zero!(i16 => 0i16);
impl_zero!(i32 => 0i32);
impl_zero!(i64 => 0i64);
impl_zero!(u8 => 0u8);
impl_zero!(u16 => 0u16);
impl_zero!(u32 => 0u32);
impl_zero!(u64 => 0u64);
impl_zero!(usize => 0usize);
impl_zero!(isize => 0isize);

pub trait One {
  fn one() -> Self;
}

#[macro_export]
macro_rules! impl_one {
  ($t:ty => $v:expr) => {
impl $crate::One for $t {
  fn one() -> $t {
    $v
  }
}
  }
}

impl_one!(f32 => 1f32);
impl_one!(f64 => 1f64);
impl_one!(i8 => 1i8);
impl_one!(i16 => 1i16);
impl_one!(i32 => 1i32);
impl_one!(i64 => 1i64);
impl_one!(u8 => 1u8);
impl_one!(u16 => 1u16);
impl_one!(u32 => 1u32);
impl_one!(u64 => 1u64);
impl_one!(usize => 1usize);
impl_one!(isize => 1isize);
