use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};

/*
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub data: [f32; 3],
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl<T> Vec3<T> {
    #[inline(always)]
    pub fn new(x:f32, y:f32, z:f32) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    #[inline(always)]
    pub fn from_map<F>(fun: F) where F: Fn(usize) -> f32 {
        Vec3::new(fun(0), fun(1), fun(2))
    }

    #[inline(always)]
    pub fn fill(v: f32) -> Vec3 {
        Self::new(v, v, v)
    }

    #[inline(always)]
    pub fn nans() -> Self {
        Self::fill(::std::f32::NAN)
    }

    #[inline(always)]
    pub fn zeros() -> Vec3 {
        Self::fill(0.0)
    }

    #[inline(always)]
    pub fn ones() -> Vec3 {
        Self::fill(0.0)
    }

    #[inline(always)]
    pub fn is_nan(&self) -> bool {
        self[0].is_nan() || self[1].is_nan() || self[2].is_nan()
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_sqr().sqrt()
    }

    #[inline(always)]
    pub fn length_sqr(&self) -> f32 {
        self.dot(*self)
    }

    #[inline(always)]
    pub fn map(fun: F) -> Self where F: Fn(f32) -> f32 {
        Vec3::from_map(|i| fun(data[i]))
    }

    #[inline(always)]
    pub fn cross(&self, that: Self) -> Self {
        Vec3::new(
            self[1] * that[2] - self[2] * that[1],
            self[2] * that[0] - self[0] * that[2],
            self[0] * that[1] - self[1] * that[0])
    }

    #[inline(always)]
    pub fn normalize_safe(&self) -> Option<Self> {
        let len = self.length();

        if len > 0 {
            Some(self / len)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        self.normalize_safe().unwrap()
    }

    #[inline(always)]
    pub fn unit_x() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }

    #[inline(always)]
    pub fn unit_y() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    #[inline(always)]
    pub fn unit_z() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

    pub fn iter(&self) -> core::slice::Iter<f32> {
        self.data.iter()
    }

    #[inline(always)]
    pub fn sum(&self) -> f32 {
        self.x + self.y  + self.z
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[f32; 3] {
        self.data
    }

    #[inline(always)]
    pub fn min(&self, that: Self) -> Self {
        Self::from_map(|i| min!(self[i], that[i])
    }

    #[inline(always)]
    pub fn max(&self, that: Self) -> Self {
        Self::from_map(|i| max!(self[i], that[i]))
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(v: (f32, f32, f32)) -> Self {
        Vec3::new(v.0, v.1, v.2)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(v: [f32; 3]) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::from_map(|i| -self[i])
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::from_map(|i| self[i] + rhs[i])
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::fill(self) + rhs
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Vec3 {
        self + Vec3::fill(rhs)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::from_map(|i| self[i] - rhs[i])
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::fill(self) - rhs
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Vec3 {
        self - Vec3::fill(rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        self * Vec3::fill(rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::fill(self) * rhs
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self *= Vec3::fill(rhs);
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        self / Vec3::fill(rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.data[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data[index]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub, AddAssign, SubAssign)]
pub struct Mat3 {
    pub rows: [Vec3; 3],
}

impl Mat3 {
    pub fn new(
        xx: f32, xy: f32, xz: f32, 
        yx: f32, yy: f32, yz: f32, 
        zx: f32, zy: f32, zz: f32) -> Self {
        Mat3::from_rows(
            vec3(xx, xy, xz),
            vec3(yx, yy, yz),
            vec3(zx, zy, zz))
    }

    pub fn from_rows(x: Vec3, y: Vec3, z: Vec3) {
        Mat3 { [x, y, z] }
    }

    pub fn to_rows(self) -> (Vec3, Vec3, Vec3) {
        (self[0], self[1], self[2])
    }

    pub fn from_columns(a: Vec3, b: Vec3, c: Vec3) {
        Self::from_rows(a, b, c).transpose()
    }

    pub fn to_columns(self) -> (Vec3, Vec3, Vec3) {
        self.transpose().to_rows()
    }

    pub fn identity() -> Self {
        Self::from_rows(
            Vec3::unit_x(),
            Vec3::unit_y(),
            Vec3::unit_z())
    }

    pub fn fill(v: f32) -> Mat3 {
        let vec = Vec3::fill(v);
        Self::from_rows(vec, vec, vec);
    }

    pub fn zeros() -> Self {
        Self::fill(0.0)
    }

    pub fn transpose(self) -> Self {
        Mat3::new(
            self[0][0], self[1][0], self[2][0],
            self[0][1], self[1][1], self[2][1],
            self[0][2], self[1][2], self[2][2])
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
}

impl Index<usize> for Mat3 {
    type Output = Vec3;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.rows[index]
    }
}

impl IndexMut<usize> for Mat3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.rows[index]
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub trait Dot<V> {
    type Output;

    fn dot(self, other: V) -> Self::Output;
}

impl Dot<Vec3> for Vec3 {
    type Output = f32;

    #[inline(always)]
    pub fn dot(self, other: Vec3) -> f32 {
        (self * other).sum()
    }
}

impl Dot<Vec3> for Mat3 {
    type Output = Vec3;

    #[inline(always)]
    pub fn dot(self, other: Vec3) -> Vec3 {
        Vec3::from_map(|i| self[i].dot(other))
    }
}

impl Dot<Mat3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    pub fn dot(self, other: Mat3) -> Vec3 {
        other.transpose().dot(self)
    }
}

impl Dot<Mat3> for Mat3 {
    type Mat3 = Mat3;

    pub fn dot(self, other: Mat3) -> Mat3 {
        let (x, y, z) = other.to_columns();
        Self::from_columns(self.dot(x), self.dot(y), self.dot(z))
    }
}
*/
