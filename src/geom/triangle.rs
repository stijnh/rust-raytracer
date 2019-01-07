use crate::math::*;
use crate::material::DEFAULT_MATERIAL;
use super::{Geometry, HitResult};


pub struct Triangle {
    a: Vec3D,
    b: Vec3D,
    c: Vec3D,
}

impl Triangle {
    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D) -> Self {
        Self { a, b, c }
    }
}

impl Geometry for Triangle {
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        // Moller-Trumbore algorithm
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let pv = Vec3D::cross(ray.dir, ac);
        let det = Vec3D::dot(ab, pv);

        if det.abs() < 1e-12 {
            return None;
        }

        let invDet = 1.0 / det;
        let tv = ray.pos - self.a;
        let qv = Vec3D::cross(tv, ab);
        let t = Vec3D::dot(qv, ac) * invDet;

        if t < 0.0 || t > t_max {
            return None;
        }

        let u = Vec3D::dot(tv, pv) * invDet;
        let v = Vec3D::dot(qv, ray.dir) * invDet;

        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            Some(HitResult {
                t,
                norm: Vec3D::cross(ab, ac),
                pos: ray.at(t),
                uv: [u, v],
                material: &DEFAULT_MATERIAL,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        AABB::from_point(self.a)
            .union_point(self.b)
            .union_point(self.c)
    }
}
