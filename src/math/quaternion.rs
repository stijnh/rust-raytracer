use super::Vec3D;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Quaternion {
    data: [f32; 4],
}

impl Quaternion {
    pub fn new() -> Self {
        Self {
            data: [1.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn from_rotation(axis: Vec3D, angle: f32) -> Self {
        let factor = (angle / 2.0).sin();
        let [x, y, z]: [f32; 3] = axis.map(|v| v * factor).into();

        let w = (angle / 2.0).cos();
        let norm = (x * x + y * y + z * z + w * w).sqrt();

        Quaternion {
            data: [w / norm, x / norm, y / norm, z / norm],
        }
    }

    #[inline(always)]
    pub fn multiply(&self, other: &Self) -> Self {
        let [q1w, q1x, q1y, q1z] = self.data;
        let [q2w, q2x, q2y, q2z] = other.data;

        let x = q1x * q2w + q1y * q2z - q1z * q2y + q1w * q2x;
        let y = -q1x * q2z + q1y * q2w + q1z * q2x + q1w * q2y;
        let z = q1x * q2y - q1y * q2x + q1z * q2w + q1w * q2z;
        let w = -q1x * q2x - q1y * q2y - q1z * q2z + q1w * q2w;

        Quaternion { data: [w, x, y, z] }
    }

    #[inline(always)]
    pub fn apply(&self, v: Vec3D) -> Vec3D {
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

        let num15 =
            ((v[0] * ((1.0 - num5) - num3)) + (v[1] * (num7 - num9))) + (v[2] * (num6 + num10));
        let num14 =
            ((v[0] * (num7 + num9)) + (v[1] * ((1.0 - num8) - num3))) + (v[2] * (num4 - num11));
        let num13 =
            ((v[0] * (num6 - num10)) + (v[1] * (num4 + num11))) + (v[2] * ((1.0 - num8) - num5));

        Vec3D::new(num15, num14, num13)
    }

    #[inline(always)]
    pub fn inverse(&self) -> Self {
        let [w, x, y, z] = self.data;
        Quaternion {
            data: [w, -x, -y, -z],
        }
    }

    #[inline(always)]
    pub fn inverse_apply(&self, v: Vec3D) -> Vec3D {
        self.inverse().apply(v)
    }
}
