use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Default, Debug, PartialEq, Neg, Add, Sub, AddAssign, SubAssign)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn fill(v: f64) -> Vec3 {
        Self::new(v, v, v)
    }

    pub fn nan() -> Self {
        Self::fill(::std::f64::NAN)
    }

    pub fn zero() -> Vec3 {
        Self::fill(0.0)
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn dot(&self, that: Self) -> f64 {
        (*self * that).sum()
    }

    pub fn cross(&self, that: Self) -> Self {
        Vec3::new(
            self.y * that.z - self.z * that.y,
            self.z * that.x - self.x * that.z,
            self.x * that.y - self.y * that.x)
    }

    pub fn normalize_safe(&self) -> Option<Self> {
        let v = self.normalize();
        if !v.is_nan() {
            Some(v)
        } else {
            None
        }
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn angle_to(self, that: Self) -> f64 {
        match (self.normalize_safe(), that.normalize_safe()) {
            (Some(a), Some(b)) => {
                a.dot(b).acos()
            },
            _ => 0.0
        }
    }

    pub fn unit_x() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    pub fn unit_z() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

    pub fn sum(&self) -> f64 {
        self.x + self.y  + self.z
    }

    pub fn as_slice(&self) -> &[f64; 3] {
        unsafe {
            (&self.x as *const f64 as *const [f64; 3]).as_ref().unwrap()
        }
    }

    pub fn min(&self, that: Self) -> Self {
        Self::new(self.x.min(that.x), self.y.min(that.y), self.z.min(that.z))
    }

    pub fn max(&self, that: Self) -> Self {
        Self::new(self.x.max(that.x), self.y.max(that.y), self.z.max(that.z))
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(v: (f64, f64, f64)) -> Self {
        Vec3::new(v.0, v.1, v.2)
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(v: [f64; 3]) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::fill(self) + rhs
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        self + Vec3::fill(rhs)
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::fill(self) - rhs
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Vec3 {
        self - Vec3::fill(rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        self * Vec3::fill(rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::fill(self) * rhs
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= Vec3::fill(rhs);
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        self / Vec3::fill(rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
