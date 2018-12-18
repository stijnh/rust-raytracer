use math::Dot;
use num::{Float, Num, One, Zero};
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Vec3<T> {
    pub data: [T; 3],
}

impl<T> Vec3<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vec3 { data: [x, y, z] }
    }

    #[inline(always)]
    pub fn from_tuple((x, y, z): (T, T, T)) -> Self {
        Vec3::new(x, y, z)
    }

    #[inline(always)]
    pub fn from_array([x, y, z]: [T; 3]) -> Self {
        Vec3::new(x, y, z)
    }

    #[inline(always)]
    pub fn into_tuple(mut self) -> (T, T, T) {
        unsafe {
            let x = ::std::mem::replace(&mut self[0], ::std::mem::zeroed());
            let y = ::std::mem::replace(&mut self[1], ::std::mem::zeroed());
            let z = ::std::mem::replace(&mut self[2], ::std::mem::zeroed());
            (x, y, z)
        }
    }

    #[inline(always)]
    pub fn into_array(self) -> [T; 3] {
        self.data
    }

    #[inline(always)]
    pub fn iter(&self) -> ::std::slice::Iter<T> {
        self.data.iter()
    }

    #[inline(always)]
    pub fn from_map<F>(mut fun: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        Self::new(fun(0), fun(1), fun(2))
    }
}

impl<T: Clone> Vec3<T> {
    #[inline(always)]
    pub fn fill(x: T) -> Self {
        Self::new(x.clone(), x.clone(), x)
    }

    #[inline(always)]
    pub fn map<F, S>(&self, mut fun: F) -> Vec3<S>
    where
        F: FnMut(T) -> S,
    {
        Vec3::new(
            fun(self[0].clone()),
            fun(self[1].clone()),
            fun(self[2].clone()),
        )
    }

    #[inline(always)]
    pub fn all<F>(&self, mut fun: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        let b = self.map(fun);
        b[0] && b[1] && b[2]
    }

    #[inline(always)]
    pub fn any<F>(&self, mut fun: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        !self.all(|v| !fun(v))
    }
}

impl<T: One + Zero> Vec3<T> {
    #[inline(always)]
    pub fn one() -> Self {
        Self::from_map(|_| One::one())
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self::from_map(|_| Zero::zero())
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self[0].is_zero() && self[1].is_zero() && self[2].is_zero()
    }

    #[inline(always)]
    pub fn unit_x() -> Self {
        Self::new(One::one(), Zero::zero(), Zero::zero())
    }

    #[inline(always)]
    pub fn unit_y() -> Self {
        Self::new(Zero::zero(), One::one(), Zero::zero())
    }

    #[inline(always)]
    pub fn unit_z() -> Self {
        Self::new(Zero::zero(), Zero::zero(), One::one())
    }
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T>> Vec3<T> {
    #[inline(always)]
    pub fn cross(&self, that: Self) -> Self {
        Vec3::new(
            self[1] * that[2] - self[2] * that[1],
            self[2] * that[0] - self[0] * that[2],
            self[0] * that[1] - self[1] * that[0],
        )
    }
}

impl<T: Num + Float> Vec3<T> {
    #[inline(always)]
    pub fn length_squared(&self) -> T {
        self.dot(*self)
    }

    #[inline(always)]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn normalize_safe(&self) -> Option<Vec3<T>> {
        let len = self.length();
        if len > Zero::zero() {
            Some(self.map(|v| v / len))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec3<T> {
        self.normalize_safe().unwrap()
    }

    #[inline(always)]
    pub fn nan(&self) -> Self {
        Self::fill(T::nan())
    }

    #[inline(always)]
    pub fn is_nan(&self) -> bool {
        self.any(|v| v.is_nan())
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Dot for Vec3<T> {
    type Output = <T as Add>::Output;

    #[inline(always)]
    fn dot(&self, rhs: Self) -> Self::Output {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    #[inline(always)]
    fn from(v: (T, T, T)) -> Self {
        Self::from_tuple(v)
    }
}

impl<T> From<[T; 3]> for Vec3<T> {
    #[inline(always)]
    fn from(v: [T; 3]) -> Self {
        Self::from_array(v)
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}

impl<T: Neg + Copy> Neg for Vec3<T> {
    type Output = Vec3<<T as Neg>::Output>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl<T: Add + Copy> Add for Vec3<T> {
    type Output = Vec3<<T as Add>::Output>;

    #[inline(always)]
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_map(|i| self[i] + rhs[i])
    }
}

impl<T: Sub + Copy> Sub for Vec3<T> {
    type Output = Vec3<<T as Sub>::Output>;

    #[inline(always)]
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_map(|i| self[i] - rhs[i])
    }
}

impl<T: Mul + Copy> Mul for Vec3<T> {
    type Output = Vec3<<T as Mul>::Output>;

    #[inline(always)]
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_map(|i| self[i] * rhs[i])
    }
}

impl<T: Div + Copy> Div for Vec3<T> {
    type Output = Vec3<<T as Div>::Output>;

    #[inline(always)]
    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_map(|i| self[i] / rhs[i])
    }
}

impl<T: Copy, S> AddAssign<S> for Vec3<T>
where
    Vec3<T>: Add<S, Output = Vec3<T>>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: S) {
        *self = *self + rhs;
    }
}

impl<T: Copy, S> SubAssign<S> for Vec3<T>
where
    Vec3<T>: Sub<S, Output = Vec3<T>>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: S) {
        *self = *self - rhs;
    }
}

impl<T: Copy, S> MulAssign<S> for Vec3<T>
where
    Vec3<T>: Mul<S, Output = Vec3<T>>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: S) {
        *self = *self * rhs;
    }
}

impl<T: Copy, S> DivAssign<S> for Vec3<T>
where
    Vec3<T>: Div<S, Output = Vec3<T>>,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: S) {
        *self = *self / rhs;
    }
}

macro_rules! vec_scalar {
    ($T:ident $($R:ident)*) => {
        vec_scalar!($($R),*);

        impl Add<$T> for Vec3<$T> {
            type Output = Vec3<$T>;

            #[inline(always)]
            fn add(self, rhs: $T) -> Self::Output {
                self + Vec3::fill(rhs)
            }
        }

        impl Add<Vec3<$T>> for $T {
            type Output = Vec3<$T>;

            #[inline(always)]
            fn add(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3::fill(self) + rhs
            }
        }

        impl Sub<$T> for Vec3<$T> {
            type Output = Vec3<$T>;

            #[inline(always)]
            fn sub(self, rhs: $T) -> Self::Output {
                self - Vec3::fill(rhs)
            }
        }

        impl Sub<Vec3<$T>> for $T {
            type Output = Vec3<$T>;

            #[inline(always)]
            fn sub(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3::fill(self) - rhs
            }
        }

        impl Mul<$T> for Vec3<$T> {
            type Output = Vec3<$T>;

            #[inline(always)]
            fn mul(self, rhs: $T) -> Self::Output {
                self * Vec3::fill(rhs)
            }
        }

        impl Mul<Vec3<$T>> for $T {
            type Output = Vec3<$T>;

            #[inline(always)]
            fn mul(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3::fill(self) * rhs
            }
        }

        impl Div<$T> for Vec3<$T> {
            type Output = Vec3<$T>;

            #[inline(always)]
            fn div(self, rhs: $T) -> Self::Output {
                self / Vec3::fill(rhs)
            }
        }

        impl Div<Vec3<$T>> for $T {
            type Output = Vec3<$T>;

            #[inline(always)]
            fn div(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3::fill(self) / rhs
            }
        }
    };
    () => {}
}

vec_scalar!(f32 f64);
