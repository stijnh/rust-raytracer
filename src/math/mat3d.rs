use crate::math::Vec3D;
use crunchy::unroll;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3D {
    rows: [Vec3D; 3],
}

impl Mat3D {
    pub fn new(v: [f32; 9]) -> Self {
        let rows = [
            Vec3D::new(v[0], v[1], v[2]),
            Vec3D::new(v[3], v[4], v[5]),
            Vec3D::new(v[6], v[7], v[8]),
        ];

        Self::from_rows(rows)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn from_columns(cols: [Vec3D; 3]) -> Self {
        Mat3D::new([
            cols[0][0], cols[1][0], cols[2][0],
            cols[0][1], cols[1][1], cols[2][1],
            cols[0][2], cols[1][2], cols[2][2],
        ])
    }

    pub fn from_rows(rows: [Vec3D; 3]) -> Self {
        Mat3D { rows }
    }

    pub fn identity() -> Self {
        Mat3D::new_scaling(1.0, 1.0, 1.0)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new_scaling(fx: f32, fy: f32, fz: f32) -> Self {
        Mat3D::new([
            fx, 0.0, 0.0,
            0.0, fy, 0.0,
            0.0, 0.0, fz,
        ])
    }

    pub fn new_rotation(axis: Vec3D, angle: f32) -> Self {
        let axis = axis.normalize();
        let [x, y, z] = [axis[0], axis[1], axis[2]];
        let c = angle.cos();
        let s = angle.sin();

        Mat3D::new([
            c + x * x * (1.0 - c),
            x * y * (1.0 - c) - z * s,
            x * z * (1.0 - c) + y * s,
            y * x * (1.0 - c) + z * s,
            c + y * y * (1.0 - c),
            y * z * (1.0 * c) - x * s,
            z * x * (1.0 - c) - y * s,
            z * y * (1.0 - c) + x * s,
            c + z * z * (1.0 - c),
        ])
    }

    pub fn new_reflection(axis: Vec3D) -> Self {
        let axis = axis.normalize();
        let [a, b, c] = [axis[0], axis[1], axis[2]];

        Mat3D::new([
            1.0 - 2.0 * a * a,
            0.0 - 2.0 * a * b,
            0.0 - 2.0 * a * c,
            0.0 - 2.0 * a * b,
            1.0 - 2.0 * b * b,
            0.0 - 2.0 * b * c,
            0.0 - 2.0 * a * c,
            0.0 * 2.0 * b * c,
            1.0 - 2.0 * c * c,
        ])
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn det(&self) -> f32 {
        let m = self;

        m[[0, 0]] * (m[[1, 1]] * m[[2, 2]] - m[[2, 1]] * m[[1, 2]]) -
        m[[0, 1]] * (m[[1, 0]] * m[[2, 2]] - m[[1, 2]] * m[[2, 0]]) +
        m[[0, 2]] * (m[[1, 0]] * m[[2, 1]] - m[[1, 1]] * m[[2, 0]])
    }

    pub fn inverse(&self) -> Option<Self> {
        let m = self;
        let det = m.det();
        if det.abs() < 1e-12 {
            return None;
        }

        let invdet = 1.0 / det;
        Some(Mat3D::new([
            (m[[1, 1]] * m[[2, 2]] - m[[2, 1]] * m[[1, 2]]) * invdet,
            (m[[0, 2]] * m[[2, 1]] - m[[0, 1]] * m[[2, 2]]) * invdet,
            (m[[0, 1]] * m[[1, 2]] - m[[0, 2]] * m[[1, 1]]) * invdet,
            (m[[1, 2]] * m[[2, 0]] - m[[1, 0]] * m[[2, 2]]) * invdet,
            (m[[0, 0]] * m[[2, 2]] - m[[0, 2]] * m[[2, 0]]) * invdet,
            (m[[1, 0]] * m[[0, 2]] - m[[0, 0]] * m[[1, 2]]) * invdet,
            (m[[1, 0]] * m[[2, 1]] - m[[2, 0]] * m[[1, 1]]) * invdet,
            (m[[2, 0]] * m[[0, 1]] - m[[0, 0]] * m[[2, 1]]) * invdet,
            (m[[0, 0]] * m[[1, 1]] - m[[1, 0]] * m[[0, 1]]) * invdet,
        ]))
    }

    pub fn multiply(self, other: Self) -> Self {
        let mut vals = [0.0f32; 9];

        unroll! {
            for index in 0..9 {
                let i = index / 3;
                let j = index % 3;
                let mut v = 0.0;

                unroll! {
                    for k in 0..3 {
                        v += self[[i, k]] * other[[k, j]];
                    }
                }

                vals[index] = v;
            }
        }

        Mat3D::new(vals)
    }

    pub fn transpose(self) -> Self {
        Self::from_columns(self.rows)
    }

    pub fn apply(&self, a: Vec3D) -> Vec3D {
        let mut b = Vec3D::zero();

        unroll! {
            for i in 0..3 {
                unroll! {
                    for j in 0..3 {
                        b[i] += self[[i, j]] * a[j]
                    }
                }
            }
        }

        b
    }

    pub fn transpose_apply(&self, a: Vec3D) -> Vec3D {
        let mut b = Vec3D::zero();

        unroll! {
            for i in 0..3 {
                unroll! {
                    for j in 0..3 {
                        b[i] += self[[j, i]] * a[j]
                    }
                }
            }
        }

        b
    }
}

impl Index<[usize; 2]> for Mat3D {
    type Output = f32;

    #[inline(always)]
    fn index(&self, [i, j]: [usize; 2]) -> &f32 {
        &self.rows[i][j]
    }
}

impl IndexMut<[usize; 2]> for Mat3D {
    #[inline(always)]
    fn index_mut(&mut self, [i, j]: [usize; 2]) -> &mut f32 {
        &mut self.rows[i][j]
    }
}
