use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub, AddAssign, SubAssign)]
pub struct Mat3 {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
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
        Mat3 { x, y, z }
    }

    pub fn to_rows(self) -> (Vec3, Vec3, Vec3) {
        (self.x, self.y, self.z)
    }

    pub fn from_columns(a: Vec3, b: Vec3, c: Vec3) {
        Self::from_rows(a, b, c).transpose()
    }

    pub fn to_columns(self) -> (Vec3, Vec3, Vec3) {
        self.transpose().to_rows()
    }

    pub fn identity() -> Self {
        Self::new(
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
            self.x.x, self.y.x, self.z.x,
            self.x.y, self.y.y, self.z.y,
            self.x.z, self.y.z, self.z.z)
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn mult(&self, &other: &Self) -> Self {
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


#[derive(Copy, Clone, Default, Debug, PartialEq, Neg, Add, Sub, AddAssign, SubAssign)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    pub fn new(x:f32, y:f32, z:f32) -> Vec3 {
        Vec3 {x, y, z}
    }

    #[inline(always)]
    pub fn fill(v: f32) -> Vec3 {
        Self::new(v, v, v)
    }

    #[inline(always)]
    pub fn nan() -> Self {
        Self::fill(::std::f32::NAN)
    }

    #[inline(always)]
    pub fn zero() -> Vec3 {
        Self::fill(0.0)
    }

    #[inline(always)]
    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
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
    pub fn cross(&self, that: Self) -> Self {
        Vec3::new(
            self.y * that.z - self.z * that.y,
            self.z * that.x - self.x * that.z,
            self.x * that.y - self.y * that.x)
    }

    #[inline(always)]
    pub fn normalize_safe(&self) -> Option<Self> {
        let v = self.normalize();
        if !v.is_nan() {
            Some(v)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    #[inline(always)]
    pub fn angle_to(self, that: Self) -> f32 {
        match (self.normalize_safe(), that.normalize_safe()) {
            (Some(a), Some(b)) => {
                a.dot(b).acos()
            },
            _ => 0.0
        }
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

    #[inline(always)]
    pub fn sum(&self) -> f32 {
        self.x + self.y  + self.z
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[f32; 3] {
        unsafe {
            (&self.x as *const f32 as *const [f32; 3]).as_ref().unwrap()
        }
    }

    #[inline(always)]
    pub fn min(&self, that: Self) -> Self {
        Self::new(self.x.min(that.x), self.y.min(that.y), self.z.min(that.z))
    }

    #[inline(always)]
    pub fn max(&self, that: Self) -> Self {
        Self::new(self.x.max(that.x), self.y.max(that.y), self.z.max(that.z))
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
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

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
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
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds, length is 3 and index is {}", index)
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds, length is 3 and index is {}", index)
        }
    }
}

pub trait Dot<V> {
    type Output;
    fn dot(self, other: V) -> Self::Output;
}

impl Dot<Vec3> for Vec3 {
    type Output = f32;

    pub fn dot(self, other: Vec3) -> f32 {
        (self * other).sum()
    }
}

impl Dot<Vec3> for Mat3 {
    type Output = Vec3;

    pub fn dot(self, other: Vec3) -> Vec3 {
        vec3(self.x.dot(other), self.y.dot(other), self.z.dot(other))
    }
}

impl Dot<Mat3> for Vec3 {
    type Output = Vec3;

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
