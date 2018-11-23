use std::ops::{Neg, Add, Sub, Mul, Div, Index, IndexMut};
use num::{Zero, One, Num, Float};
use std::fmt;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Vec3<T> {
    pub data: [T; 3]
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { data: [x, y, z] }
    }

    pub fn iter(&self) -> ::std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn from_map<F>(fun: F) -> Self where F: Fn(usize) -> T {
        Self::new(fun(0), fun(1), fun(2))
    }

    pub fn map<F, S>(&self, fun: F) -> Vec3<S> where F: Fn(T) -> S {
        Vec3::new(fun(self[0]), fun(self[1]), fun(self[2]))
    }
}

impl<T: Clone> Vec3<T> {
    pub fn fill(x: T) -> Self {
        Self::new(x.clone(), x.clone(), x)
    }
}


impl<T: One + Zero> Vec3<T> {
    pub fn ones() -> Self {
        Self::from_map(|_| One::one())
    }

    pub fn zeros() -> Self {
        Self::from_map(|_| Zero::zero())
    }

    pub fn unit_x() -> Self {
        Self::new(One::one(), Zero::zero(), Zero::zero())
    }

    pub fn unit_y() -> Self {
        Self::new(Zero::zero(), One::one(), Zero::zero())
    }

    pub fn unit_z() -> Self {
        Self::new(Zero::zero(), Zero::zero(), One::one())
    }
}

impl <T: Num> Vec3<T> {
    pub fn cross(&self, that: Self) -> Self {
        Vec3::new(
             self[1] * that[2] - self[2] * that[1], 
             self[2] * that[0] - self[0] * that[2], 
             self[0] * that[1] - self[1] * that[0])

    }
}

impl<T: Num + Float> Vec3<T> {
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalize_safe(&self) -> Option<Vec3<T>> {
        let len = self.length();
        if len > Zero::zero() {
            Some(self / len)
        } else {
            None
        }
    }

    pub fn normalize(&self) -> Vec3<T> {
        self.normalize_safe().unwrap()
    }
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}


impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(v: (T, T, T)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(v: [T; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}

impl <T: Neg> Neg for Vec3<T> {
    type Output = Vec3<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl <T: Add> Add for Vec3<T> {
    type Output = Vec3<<T as Add>::Output>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_map(|i| self[i] + rhs[i])
    }
}

impl <T: Mul> Mul for Vec3<T> {
    type Output = Vec3<<T as Mul>::Output>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_map(|i| self[i] * rhs[i])
    }
}

impl <T: Mul> Mul<T> for Vec3<T> {
    type Output = Vec3<<T as Mul>::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::from_map(|i| self[i] * rhs)
    }
}

impl <T: Div> Div for Vec3<T> {
    type Output = Vec3<<T as Div>::Output>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_map(|i| self[i] / rhs[i])
    }
}

macro_rules! vec_mul {
    ($T:ident $($R:ident)*) => {
        impl Mul<Vec3<$T>> for $T {
            type Output = Vec3<$T>;

            fn mul(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3::from_map(|i| self * rhs[i])
            }
        }

        vec_mul!($($R),*);
    };
    () => {}
}

vec_mul!(f32 f64);
