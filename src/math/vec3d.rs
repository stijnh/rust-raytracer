use std::f32;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec3D {
    data: [f32; 3],
}

impl Vec3D {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3D { data: [x, y, z] }
    }

    #[inline(always)]
    pub fn from_array([x, y, z]: [f32; 3]) -> Self {
        Self::new(x, y, z)
    }

    #[inline(always)]
    pub fn into_array(self) -> [f32; 3] {
        self.data
    }

    #[inline(always)]
    pub fn fill(x: f32) -> Self {
        Self::new(x, x, x)
    }

    #[inline(always)]
    pub fn from_map<F>(mut f: F) -> Self
    where
        F: FnMut(usize) -> f32,
    {
        Vec3D::new(f(0), f(1), f(2))
    }

    #[inline(always)]
    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(f32) -> f32,
    {
        let [x, y, z] = self.data;
        Vec3D::new(f(x), f(y), f(z))
    }

    #[inline(always)]
    pub fn x_axis() -> Self {
        Vec3D::new(1.0, 0.0, 0.0)
    }

    #[inline(always)]
    pub fn y_axis() -> Self {
        Vec3D::new(0.0, 1.0, 0.0)
    }

    #[inline(always)]
    pub fn z_axis() -> Self {
        Vec3D::new(0.0, 0.0, 1.0)
    }

    #[inline(always)]
    pub fn nan() -> Self {
        Self::fill(f32::NAN)
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self::fill(0.0)
    }

    #[inline(always)]
    pub fn one() -> Self {
        Self::fill(1.0)
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self[0] == 0.0 && self[1] == 0.0 && self[2] == 0.0
    }

    #[inline(always)]
    pub fn is_nan(&self) -> bool {
        self[0].is_nan() || self[1].is_nan() || self[2].is_nan()
    }

    #[inline(always)]
    pub fn norm(&self) -> f32 {
        f32::sqrt(self.norm_squared())
    }

    #[inline(always)]
    pub fn norm_squared(self) -> f32 {
        Vec3D::dot(self, self)
    }

    #[inline(always)]
    pub fn normalize(self) -> Self {
        self.normalize_safe().unwrap_or_else(Vec3D::nan)
    }

    #[inline(always)]
    pub fn normalize_safe(self) -> Option<Self> {
        let len = self.norm();
        if len > 0.0 {
            Some(self / Vec3D::fill(len))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn dot(self, that: Self) -> f32 {
        self[0] * that[0] + self[1] * that[1] + self[2] * that[2]
    }

    #[inline(always)]
    pub fn cross(self, that: Self) -> Self {
        Self::new(
            self[1] * that[2] - self[2] * that[1],
            self[2] * that[0] - self[0] * that[2],
            self[0] * that[1] - self[1] * that[0],
        )
    }

    pub fn ortho_axes(&self) -> (Self, Self) {
        let a = *self;
        let b = if a[0].abs() > a[1].abs() {
            let inv_len = 1.0 / (a[0] * a[0] + a[2] * a[2]).sqrt();
            Self::new(-a[2] * inv_len, 0.0, a[0] * inv_len)
        } else {
            let inv_len = 1.0 / (a[1] * a[1] + a[2] * a[2]).sqrt();
            Self::new(0.0, a[2] * inv_len, -a[1] * inv_len)
        };

        let c = Self::cross(a, b);
        (b, c)
    }
}

impl std::fmt::Debug for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_tuple("")
            .field(&self[0])
            .field(&self[1])
            .field(&self[2])
            .finish()
    }
}

impl From<[f32; 3]> for Vec3D {
    fn from(data: [f32; 3]) -> Self {
        Vec3D::from_array(data)
    }
}

impl From<Vec3D> for [f32; 3] {
    fn from(v: Vec3D) -> [f32; 3] {
        v.into_array()
    }
}

impl Index<usize> for Vec3D {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vec3D {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Add<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn add(self, other: Self) -> Self::Output {
        Vec3D::from_map(move |i| self[i] + other[i])
    }
}

impl Sub<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn sub(self, other: Self) -> Self::Output {
        Vec3D::from_map(move |i| self[i] - other[i])
    }
}

impl Mul<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn mul(self, other: Self) -> Self::Output {
        Vec3D::from_map(move |i| self[i] * other[i])
    }
}

impl Div<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn div(self, other: Self) -> Self::Output {
        Vec3D::from_map(move |i| self[i] / other[i])
    }
}

impl Neg for Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Self::Output {
        Vec3D::from_map(move |i| -self[i])
    }
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, other: Vec3D) {
        *self = *self + other;
    }
}

impl SubAssign for Vec3D {
    fn sub_assign(&mut self, other: Vec3D) {
        *self = *self - other;
    }
}

impl MulAssign for Vec3D {
    fn mul_assign(&mut self, other: Vec3D) {
        *self = *self * other;
    }
}

impl DivAssign for Vec3D {
    fn div_assign(&mut self, other: Vec3D) {
        *self = *self / other;
    }
}

impl Mul<f32> for Vec3D {
    type Output = Vec3D;

    fn mul(self, other: f32) -> Self::Output {
        self * Vec3D::fill(other)
    }
}

impl Mul<Vec3D> for f32 {
    type Output = Vec3D;

    fn mul(self, other: Vec3D) -> Self::Output {
        Vec3D::fill(self) * other
    }
}

impl MulAssign<f32> for Vec3D {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * Vec3D::fill(other);
    }
}

impl Div<f32> for Vec3D {
    type Output = Vec3D;

    fn div(self, other: f32) -> Self::Output {
        self / Vec3D::fill(other)
    }
}

impl Div<Vec3D> for f32 {
    type Output = Vec3D;

    fn div(self, other: Vec3D) -> Self::Output {
        Vec3D::fill(self) / other
    }
}

impl DivAssign<f32> for Vec3D {
    fn div_assign(&mut self, other: f32) {
        *self = *self / Vec3D::fill(other);
    }
}
