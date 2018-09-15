extern crate log;
extern crate math_basics;

use math_basics::Zero;
use math_basics::One;
use std::convert::From;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;

// N.B. These collections are intentionally non-generic
//
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct Vector2<T> {
  x: T,
  y: T,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct Vector3<T> {
  x: T,
  y: T,
  z: T,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct Vector4<T> {
  x: T,
  y: T,
  z: T,
  w: T,
}

impl<T> Vector2<T> {
  pub fn new(x: T, y: T) -> Vector2<T> {
    Vector2 { x: x, y: y }
  }

  pub fn xy(x: T, y: T) -> Vector2<T> {
    Vector2::new(x, y)
  }

  pub fn x(&self) -> &T {
    &self.x
  }

  pub fn y(&self) -> &T {
    &self.y
  }
}

impl<T> Vector3<T> {
  pub fn new(x: T, y: T, z: T) -> Vector3<T> {
    Vector3 { x: x, y: y, z: z }
  }

  pub fn xyz(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::new(x, y, z)
  }

  pub fn x(&self) -> &T {
    &self.x
  }

  pub fn y(&self) -> &T {
    &self.y
  }

  pub fn z(&self) -> &T {
    &self.z
  }
}

impl<T> Vector4<T> {
  pub fn new(x: T, y: T, z: T, w: T) -> Vector4<T> {
    Vector4 {
      x: x,
      y: y,
      z: z,
      w: w,
    }
  }

  pub fn xyzw(x: T, y: T, z: T, w: T) -> Vector4<T> {
    Vector4::new(x, y, z, w)
  }

  pub fn x(&self) -> &T {
    &self.x
  }

  pub fn y(&self) -> &T {
    &self.y
  }

  pub fn z(&self) -> &T {
    &self.z
  }

  pub fn w(&self) -> &T {
    &self.w
  }
}

impl<T: Copy> From<(T, T)> for Vector2<T> {
  fn from(tuple: (T, T)) -> Vector2<T> {
    Vector2 {
      x: tuple.0,
      y: tuple.1,
    }
  }
}

impl<T: Copy> From<(T, T, T)> for Vector3<T> {
  fn from(tuple: (T, T, T)) -> Vector3<T> {
    Vector3 {
      x: tuple.0,
      y: tuple.1,
      z: tuple.2,
    }
  }
}

impl<T: Copy> From<(T, T, T, T)> for Vector4<T> {
  fn from(tuple: (T, T, T, T)) -> Vector4<T> {
    Vector4 {
      x: tuple.0,
      y: tuple.1,
      z: tuple.2,
      w: tuple.3,
    }
  }
}

impl<T: Copy> From<[T; 2]> for Vector2<T> {
  fn from(arr: [T; 2]) -> Vector2<T> {
    Vector2 {
      x: arr[0],
      y: arr[1],
    }
  }
}

impl<T: Copy> From<[T; 3]> for Vector3<T> {
  fn from(arr: [T; 3]) -> Vector3<T> {
    Vector3 {
      x: arr[0],
      y: arr[1],
      z: arr[2],
    }
  }
}

impl<T: Copy> From<[T; 4]> for Vector4<T> {
  fn from(arr: [T; 4]) -> Vector4<T> {
    Vector4 {
      x: arr[0],
      y: arr[1],
      z: arr[2],
      w: arr[3],
    }
  }
}

impl<T: Copy> Vector2<T> {
  pub fn tuple(&self) -> (T, T) {
    (self.x, self.y)
  }

  pub fn arr(&self) -> [T; 2] {
    [self.x, self.y]
  }

  pub fn with_z(&self, z: T) -> Vector3<T> {
    Vector3 {
      x: self.x,
      y: self.y,
      z: z,
    }
  }

  pub fn with_z_w(&self, z: T, w: T) -> Vector4<T> {
    Vector4 {
      x: self.x,
      y: self.y,
      z: z,
      w: w,
    }
  }
}

impl<T: Copy> Vector3<T> {
  pub fn tuple(&self) -> (T, T, T) {
    (self.x, self.y, self.z)
  }

  pub fn arr(&self) -> [T; 3] {
    [self.x, self.y, self.z]
  }

  pub fn xy(&self) -> Vector2<T> {
    Vector2 {
      x: self.x,
      y: self.y,
    }
  }

  pub fn yx(&self) -> Vector2<T> {
    Vector2 {
      x: self.x,
      y: self.y,
    }
  }

  pub fn xz(&self) -> Vector2<T> {
    Vector2 {
      x: self.x,
      y: self.z,
    }
  }

  pub fn zx(&self) -> Vector2<T> {
    Vector2 {
      x: self.z,
      y: self.x,
    }
  }

  pub fn yz(&self) -> Vector2<T> {
    Vector2 {
      x: self.y,
      y: self.z,
    }
  }

  pub fn zy(&self) -> Vector2<T> {
    Vector2 {
      x: self.z,
      y: self.y,
    }
  }

  pub fn with_w(&self, w: T) -> Vector4<T> {
    Vector4 {
      x: self.x,
      y: self.y,
      z: self.z,
      w: w,
    }
  }
}

impl<T: Copy> Vector4<T> {
  pub fn tuple(&self) -> (T, T, T, T) {
    (self.x, self.y, self.z, self.w)
  }

  pub fn arr(&self) -> [T; 4] {
    [self.x, self.y, self.z, self.w]
  }

  pub fn xy(&self) -> Vector2<T> {
    Vector2 {
      x: self.x,
      y: self.y,
    }
  }

  pub fn yx(&self) -> Vector2<T> {
    Vector2 {
      x: self.y,
      y: self.x,
    }
  }

  pub fn xz(&self) -> Vector2<T> {
    Vector2 {
      x: self.x,
      y: self.z,
    }
  }

  pub fn zx(&self) -> Vector2<T> {
    Vector2 {
      x: self.z,
      y: self.x,
    }
  }

  pub fn xw(&self) -> Vector2<T> {
    Vector2 {
      x: self.x,
      y: self.w,
    }
  }

  pub fn wx(&self) -> Vector2<T> {
    Vector2 {
      x: self.w,
      y: self.x,
    }
  }

  pub fn yz(&self) -> Vector2<T> {
    Vector2 {
      x: self.y,
      y: self.z,
    }
  }

  pub fn zy(&self) -> Vector2<T> {
    Vector2 {
      x: self.z,
      y: self.y,
    }
  }

  pub fn yw(&self) -> Vector2<T> {
    Vector2 {
      x: self.y,
      y: self.w,
    }
  }

  pub fn wy(&self) -> Vector2<T> {
    Vector2 {
      x: self.w,
      y: self.y,
    }
  }

  pub fn zw(&self) -> Vector2<T> {
    Vector2 {
      x: self.z,
      y: self.w,
    }
  }

  pub fn wz(&self) -> Vector2<T> {
    Vector2 {
      x: self.w,
      y: self.z,
    }
  }

  pub fn xyz(&self) -> Vector3<T> {
    Vector3 {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }

  pub fn xzy(&self) -> Vector3<T> {
    Vector3 {
      x: self.x,
      y: self.z,
      z: self.y,
    }
  }

  pub fn yxz(&self) -> Vector3<T> {
    Vector3 {
      x: self.y,
      y: self.x,
      z: self.z,
    }
  }

  pub fn yzx(&self) -> Vector3<T> {
    Vector3 {
      x: self.y,
      y: self.z,
      z: self.x,
    }
  }

  pub fn zxy(&self) -> Vector3<T> {
    Vector3 {
      x: self.z,
      y: self.x,
      z: self.y,
    }
  }

  pub fn zyx(&self) -> Vector3<T> {
    Vector3 {
      x: self.z,
      y: self.y,
      z: self.x,
    }
  }

  pub fn xyw(&self) -> Vector3<T> {
    Vector3 {
      x: self.x,
      y: self.y,
      z: self.w,
    }
  }
  pub fn xwy(&self) -> Vector3<T> {
    Vector3 {
      x: self.x,
      y: self.w,
      z: self.y,
    }
  }

  pub fn yxw(&self) -> Vector3<T> {
    Vector3 {
      x: self.y,
      y: self.x,
      z: self.w,
    }
  }

  pub fn ywx(&self) -> Vector3<T> {
    Vector3 {
      x: self.y,
      y: self.w,
      z: self.x,
    }
  }

  pub fn wxy(&self) -> Vector3<T> {
    Vector3 {
      x: self.w,
      y: self.x,
      z: self.y,
    }
  }

  pub fn wyx(&self) -> Vector3<T> {
    Vector3 {
      x: self.w,
      y: self.y,
      z: self.x,
    }
  }

  pub fn xzw(&self) -> Vector3<T> {
    Vector3 {
      x: self.x,
      y: self.z,
      z: self.w,
    }
  }
  pub fn xwz(&self) -> Vector3<T> {
    Vector3 {
      x: self.x,
      y: self.w,
      z: self.z,
    }
  }
  pub fn zxw(&self) -> Vector3<T> {
    Vector3 {
      x: self.z,
      y: self.x,
      z: self.w,
    }
  }
  pub fn zwx(&self) -> Vector3<T> {
    Vector3 {
      x: self.z,
      y: self.w,
      z: self.x,
    }
  }
  pub fn wxz(&self) -> Vector3<T> {
    Vector3 {
      x: self.w,
      y: self.x,
      z: self.z,
    }
  }
  pub fn wzx(&self) -> Vector3<T> {
    Vector3 {
      x: self.x,
      y: self.z,
      z: self.x,
    }
  }

  pub fn yzw(&self) -> Vector3<T> {
    Vector3 {
      x: self.y,
      y: self.z,
      z: self.w,
    }
  }

  pub fn ywz(&self) -> Vector3<T> {
    Vector3 {
      x: self.y,
      y: self.w,
      z: self.z,
    }
  }

  pub fn zyw(&self) -> Vector3<T> {
    Vector3 {
      x: self.z,
      y: self.y,
      z: self.w,
    }
  }

  pub fn zwy(&self) -> Vector3<T> {
    Vector3 {
      x: self.z,
      y: self.w,
      z: self.y,
    }
  }

  pub fn wyz(&self) -> Vector3<T> {
    Vector3 {
      x: self.w,
      y: self.y,
      z: self.z,
    }
  }

  pub fn wzy(&self) -> Vector3<T> {
    Vector3 {
      x: self.w,
      y: self.z,
      z: self.y,
    }
  }
}

impl<T: One + Zero> Vector2<T> {
  pub fn unit_x() -> Vector2<T> {
    Vector2 {
      x: T::one(),
      y: T::zero(),
    }
  }

  pub fn unit_y() -> Vector2<T> {
    Vector2 {
      x: T::zero(),
      y: T::one(),
    }
  }
}

impl<T: One + Zero> Vector3<T> {
  pub fn unit_x() -> Vector3<T> {
    Vector3 {
      x: T::one(),
      y: T::zero(),
      z: T::zero(),
    }
  }

  pub fn unit_y() -> Vector3<T> {
    Vector3 {
      x: T::zero(),
      y: T::one(),
      z: T::zero(),
    }
  }

  pub fn unit_z() -> Vector3<T> {
    Vector3 {
      x: T::zero(),
      y: T::zero(),
      z: T::one(),
    }
  }
}

impl<T: One + Zero> Vector4<T> {
  pub fn unit_x() -> Vector4<T> {
    Vector4 {
      x: T::one(),
      y: T::zero(),
      z: T::zero(),
      w: T::zero(),
    }
  }

  pub fn unit_y() -> Vector4<T> {
    Vector4 {
      x: T::zero(),
      y: T::one(),
      z: T::zero(),
      w: T::zero(),
    }
  }

  pub fn unit_z() -> Vector4<T> {
    Vector4 {
      x: T::zero(),
      y: T::zero(),
      z: T::one(),
      w: T::zero(),
    }
  }

  pub fn unit_w() -> Vector4<T> {
    Vector4 {
      x: T::zero(),
      y: T::zero(),
      z: T::zero(),
      w: T::one(),
    }
  }
}

impl<T: Copy + MulAssign<T>> Vector2<T> {
  pub fn scale(&mut self, rhs: T) {
    self.x *= rhs;
    self.y *= rhs;
  }
}

impl<T: Copy + MulAssign<T>> Vector3<T> {
  pub fn scale(&mut self, rhs: T) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
  }
}

impl<T: Copy + MulAssign<T>> Vector4<T> {
  pub fn scale(&mut self, rhs: T) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
    self.w *= rhs;
  }
}

impl<T: Copy + Mul<T, Output = T>> Vector2<T> {
  pub fn scaled(&self, rhs: T) -> Vector2<T> {
    Vector2 {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

impl<T: Copy + Mul<T, Output = T>> Vector3<T> {
  pub fn scaled(&self, rhs: T) -> Vector3<T> {
    Vector3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl<T: Copy + Mul<T, Output = T>> Vector4<T> {
  pub fn scaled(&self, rhs: T) -> Vector4<T> {
    Vector4 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      w: self.w * rhs,
    }
  }
}

impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> Vector2<T> {
  pub fn len_sq(&self) -> T {
    (self.x * self.x + self.y * self.y)
  }
}

impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> Vector3<T> {
  pub fn len_sq(&self) -> T {
    (self.x * self.x + self.y * self.y + self.z * self.z)
  }

  pub fn dot(&self, other: &Vector3<T>) -> T {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> Vector4<T> {
  pub fn len_sq(&self) -> T {
    (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
  }
}

impl<T: Copy + Mul<T, Output = T> + Sub<T, Output = T>> Vector3<T> {
  pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
    Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }
}

impl Vector2<f32> {
  pub fn len(&self) -> f32 {
    self.len_sq().sqrt()
  }

  pub fn magnitude(&self) -> f32 {
    self.len()
  }

  pub fn norm(&self) -> f32 {
    self.len()
  }

  pub fn normalize(&mut self) {
    let len = self.len();

    if len == 0f32 {
      return;
    }

    self.x /= len;
    self.y /= len;
  }

  pub fn normalized(&mut self) -> Vector2<f32> {
    let len = self.len();

    if len == 0f32 {
      return Vector2::zero();
    }
    Vector2 {
      x: self.x / len,
      y: self.y / len,
    }
  }
}

impl Vector3<f32> {
  pub fn len(&self) -> f32 {
    self.len_sq().sqrt()
  }

  pub fn magnitude(&self) -> f32 {
    self.len()
  }

  pub fn norm(&self) -> f32 {
    self.len()
  }

  pub fn normalize(&mut self) {
    let len = self.len();

    if len == 0f32 {
      return;
    }

    self.x /= len;
    self.y /= len;
    self.z /= len;
  }

  pub fn normalized(&mut self) -> Vector3<f32> {
    let len = self.len();

    if len == 0f32 {
      return Vector3::zero();
    }
    Vector3 {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
    }
  }
}

impl Vector4<f32> {
  pub fn len(&self) -> f32 {
    self.len_sq().sqrt()
  }

  pub fn magnitude(&self) -> f32 {
    self.len()
  }

  pub fn norm(&self) -> f32 {
    self.len()
  }

  pub fn normalize(&mut self) {
    let len = self.len();

    if len == 0f32 {
      return;
    }

    self.x /= len;
    self.y /= len;
    self.z /= len;
    self.w /= len;
  }

  pub fn normalized(&mut self) -> Vector4<f32> {
    let len = self.len();

    if len == 0f32 {
      return Vector4::zero();
    }
    Vector4 {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
      w: self.w / len,
    }
  }
}

impl Vector2<f64> {
  pub fn len(&self) -> f64 {
    self.len_sq().sqrt()
  }

  pub fn magnitude(&self) -> f64 {
    self.len()
  }

  pub fn norm(&self) -> f64 {
    self.len()
  }

  pub fn normalize(&mut self) {
    let len = self.len();

    if len == 0f64 {
      return;
    }

    self.x /= len;
    self.y /= len;
  }

  pub fn normalized(&mut self) -> Vector2<f64> {
    let len = self.len();

    if len == 0f64 {
      return Vector2::zero();
    }
    Vector2 {
      x: self.x / len,
      y: self.y / len,
    }
  }
}

impl Vector3<f64> {
  pub fn len(&self) -> f64 {
    self.len_sq().sqrt()
  }

  pub fn magnitude(&self) -> f64 {
    self.len()
  }

  pub fn norm(&self) -> f64 {
    self.len()
  }

  pub fn normalize(&mut self) {
    let len = self.len();

    if len == 0f64 {
      return;
    }

    self.x /= len;
    self.y /= len;
    self.z /= len;
  }

  pub fn normalized(&mut self) -> Vector3<f64> {
    let len = self.len();

    if len == 0f64 {
      return Vector3::zero();
    }
    Vector3 {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
    }
  }
}

impl Vector4<f64> {
  pub fn len(&self) -> f64 {
    self.len_sq().sqrt()
  }

  pub fn magnitude(&self) -> f64 {
    self.len()
  }

  pub fn norm(&self) -> f64 {
    self.len()
  }

  pub fn normalize(&mut self) {
    let len = self.len();

    if len == 0f64 {
      return;
    }

    self.x /= len;
    self.y /= len;
    self.z /= len;
    self.w /= len;
  }

  pub fn normalized(&mut self) -> Vector4<f64> {
    let len = self.len();

    if len == 0f64 {
      return Vector4::zero();
    }
    Vector4 {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
      w: self.w / len,
    }
  }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
  type Output = Vector2<T>;
  fn add(self, other: Vector2<T>) -> Vector2<T> {
    Vector2 {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl<T: Add<Output = T>> Add for Vector3<T> {
  type Output = Vector3<T>;
  fn add(self, other: Vector3<T>) -> Vector3<T> {
    Vector3 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl<T: Add<Output = T>> Add for Vector4<T> {
  type Output = Vector4<T>;
  fn add(self, other: Vector4<T>) -> Vector4<T> {
    Vector4 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
      w: self.w + other.w,
    }
  }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
  fn add_assign(&mut self, other: Vector2<T>) {
    self.x.add_assign(other.x);
    self.y.add_assign(other.y);
  }
}

impl<T: AddAssign> AddAssign for Vector3<T> {
  fn add_assign(&mut self, other: Vector3<T>) {
    self.x.add_assign(other.x);
    self.y.add_assign(other.y);
    self.z.add_assign(other.z);
  }
}

impl<T: AddAssign> AddAssign for Vector4<T> {
  fn add_assign(&mut self, other: Vector4<T>) {
    self.x.add_assign(other.x);
    self.y.add_assign(other.y);
    self.z.add_assign(other.z);
    self.w.add_assign(other.w);
  }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
  type Output = Vector2<T>;
  fn sub(self, other: Vector2<T>) -> Vector2<T> {
    Vector2 {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
  type Output = Vector3<T>;
  fn sub(self, other: Vector3<T>) -> Vector3<T> {
    Vector3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl<T: Sub<Output = T>> Sub for Vector4<T> {
  type Output = Vector4<T>;
  fn sub(self, other: Vector4<T>) -> Vector4<T> {
    Vector4 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
      w: self.w - other.w,
    }
  }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
  fn sub_assign(&mut self, other: Vector2<T>) {
    self.x.sub_assign(other.x);
    self.y.sub_assign(other.y);
  }
}

impl<T: SubAssign> SubAssign for Vector3<T> {
  fn sub_assign(&mut self, other: Vector3<T>) {
    self.x.sub_assign(other.x);
    self.y.sub_assign(other.y);
    self.z.sub_assign(other.z);
  }
}

impl<T: SubAssign> SubAssign for Vector4<T> {
  fn sub_assign(&mut self, other: Vector4<T>) {
    self.x.sub_assign(other.x);
    self.y.sub_assign(other.y);
    self.z.sub_assign(other.z);
    self.w.sub_assign(other.w);
  }
}

impl<U: Copy, T: Div<U, Output = T>> Div<U> for Vector2<T> {
  type Output = Vector2<T>;
  fn div(self, rhs: U) -> Vector2<T> {
    Vector2 {
      x: self.x / rhs,
      y: self.y / rhs,
    }
  }
}

impl<U: Copy, T: Div<U, Output = T>> Div<U> for Vector3<T> {
  type Output = Vector3<T>;
  fn div(self, rhs: U) -> Vector3<T> {
    Vector3 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

impl<U: Copy, T: Div<U, Output = T>> Div<U> for Vector4<T> {
  type Output = Vector4<T>;
  fn div(self, rhs: U) -> Vector4<T> {
    Vector4 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
      w: self.w / rhs,
    }
  }
}

impl<U: Copy, T: DivAssign<U>> DivAssign<U> for Vector2<T> {
  fn div_assign(&mut self, rhs: U) {
    self.x.div_assign(rhs);
    self.y.div_assign(rhs);
  }
}

impl<U: Copy, T: DivAssign<U>> DivAssign<U> for Vector3<T> {
  fn div_assign(&mut self, rhs: U) {
    self.x.div_assign(rhs);
    self.y.div_assign(rhs);
    self.z.div_assign(rhs);
  }
}

impl<U: Copy, T: DivAssign<U>> DivAssign<U> for Vector4<T> {
  fn div_assign(&mut self, rhs: U) {
    self.x.div_assign(rhs);
    self.y.div_assign(rhs);
    self.z.div_assign(rhs);
    self.w.div_assign(rhs);
  }
}

impl<U: Copy, T: Mul<U, Output = T>> Mul<U> for Vector2<T> {
  type Output = Vector2<T>;

  fn mul(self, rhs: U) -> Vector2<T> {
    Vector2 {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

impl<U: Copy, T: Mul<U, Output = T>> Mul<U> for Vector3<T> {
  type Output = Vector3<T>;

  fn mul(self, rhs: U) -> Vector3<T> {
    Vector3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl<U: Copy, T: Mul<U, Output = T>> Mul<U> for Vector4<T> {
  type Output = Vector4<T>;

  fn mul(self, rhs: U) -> Vector4<T> {
    Vector4 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      w: self.w * rhs,
    }
  }
}

impl<U: Copy, T: MulAssign<U>> MulAssign<U> for Vector2<T> {
  fn mul_assign(&mut self, rhs: U) {
    self.x.mul_assign(rhs);
    self.y.mul_assign(rhs);
  }
}

impl<U: Copy, T: MulAssign<U>> MulAssign<U> for Vector3<T> {
  fn mul_assign(&mut self, rhs: U) {
    self.x.mul_assign(rhs);
    self.y.mul_assign(rhs);
    self.z.mul_assign(rhs);
  }
}

impl<U: Copy, T: MulAssign<U>> MulAssign<U> for Vector4<T> {
  fn mul_assign(&mut self, rhs: U) {
    self.x.mul_assign(rhs);
    self.y.mul_assign(rhs);
    self.z.mul_assign(rhs);
    self.w.mul_assign(rhs);
  }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
  type Output = Vector2<T>;

  fn neg(self) -> Vector2<T> {
    Vector2 {
      x: self.x.neg(),
      y: self.y.neg(),
    }
  }
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
  type Output = Vector3<T>;

  fn neg(self) -> Vector3<T> {
    Vector3 {
      x: self.x.neg(),
      y: self.y.neg(),
      z: self.z.neg(),
    }
  }
}

impl<T: Neg<Output = T>> Neg for Vector4<T> {
  type Output = Vector4<T>;

  fn neg(self) -> Vector4<T> {
    Vector4 {
      x: self.x.neg(),
      y: self.y.neg(),
      z: self.z.neg(),
      w: self.w.neg(),
    }
  }
}

impl<T: Zero> Zero for Vector2<T> {
  fn zero() -> Vector2<T> {
    Vector2 {
      x: T::zero(),
      y: T::zero(),
    }
  }
}

impl<T: Zero> Zero for Vector3<T> {
  fn zero() -> Vector3<T> {
    Vector3 {
      x: T::zero(),
      y: T::zero(),
      z: T::zero(),
    }
  }
}

impl<T: Zero> Zero for Vector4<T> {
  fn zero() -> Vector4<T> {
    Vector4 {
      x: T::zero(),
      y: T::zero(),
      z: T::zero(),
      w: T::zero(),
    }
  }
}

#[cfg(test)]
mod tests {
  use math_basics::Zero;
  use super::*;

  #[test]
  fn test_did_derive() {
    let zero = Vector3::<f32>::zero();
    let op1 = -zero;
    let op2 = op1 / 5.0;
    let op3 = op2 * 5.0;
    let op4 = op3 + op2;
    let _op5 = op4 - op3;
  }
}
