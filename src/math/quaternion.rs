use std::ops::{Index, IndexMut, Mul};
use num::{Zero, One, Float};

use math::{Vec3, Dot};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Quaternion<T> {
    data: [T; 4]
}

impl<T: Float> Quaternion<T> {
    pub fn new() -> Self {
        let zero = Zero::zero();
        let one = One::one();

        Self { data: [one, zero, zero, zero] }
    }

    pub fn from_rotation(axis: Vec3<T>, angle: T) -> Self {
        let two: T = T::one() + T::one();

        let factor = (angle / two).sin();
        let (x, y, z) = axis.map(|v| v * factor).into_tuple();

        let w = (angle / two).cos();
        let norm = (x*x + y*y + z*z + w*w).sqrt();

        Quaternion { data: [w / norm, x / norm, y / norm, z / norm] }
    }

    #[inline(always)]
    pub fn multiply(&self, other: &Self) -> Self {
        let [q1w, q1x, q1y, q1z] = self.data;
        let [q2w, q2x, q2y, q2z] = other.data;

        let x =  q1x * q2w + q1y * q2z - q1z * q2y + q1w * q2x;
        let y = -q1x * q2z + q1y * q2w + q1z * q2x + q1w * q2y;
        let z =  q1x * q2y - q1y * q2x + q1z * q2w + q1w * q2z;
        let w = -q1x * q2x - q1y * q2y - q1z * q2z + q1w * q2w;

        Quaternion { data: [w, x, y, z] }
    }

    #[inline(always)]
    pub fn apply(&self, v: Vec3<T>) -> Vec3<T> {
        let one = T::one();
        let this = self.data;

        let num12 = this[1] + this[1];
        let num2 = this[2] + this[2];
        let num = this[3] + this[3];

        let num11 = this[0] * num12;
        let num10 = this[0] * num2;
        let num9 = this[0] * num;

        let num8 = this[1] * num12;
        let num7 = this[1] * num2;
        let num6 = this[1] * num;

        let num5 = this[2] * num2;
        let num4 = this[2] * num;
        let num3 = this[3] * num;

        let num15 = ((v[0] * ((one - num5) - num3)) + (v[1] * (num7 - num9))) + (v[2] * (num6 + num10));
        let num14 = ((v[0] * (num7 + num9)) + (v[1] * ((one - num8) - num3))) + (v[2] * (num4 - num11));
        let num13 = ((v[0] * (num6 - num10)) + (v[1] * (num4 + num11))) + (v[2] * ((one - num8) - num5));

        Vec3::new(num15, num14, num13)
    }

    #[inline(always)]
    pub fn inverse(&self) -> Self {
        let [w, x, y, z] = self.data;
        Quaternion { data: [w, -x, -y, -z] }
    }

    #[inline(always)]
    pub fn inverse_apply(&self, v: Vec3<T>) -> Vec3<T> {
        self.inverse().apply(v)
    }
}
