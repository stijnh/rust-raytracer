use super::{Geometry, HitResult};
use crate::material::DEFAULT_MATERIAL;
use crate::math::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    pub a: Vec3D,
    pub b: Vec3D,
    pub c: Vec3D,
}

impl Triangle {
    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D) -> Self {
        Self { a, b, c }
    }
}

#[inline(always)]
pub fn moller_trumbore([a, b, c]: [Vec3D; 3], ray: &Ray) -> [f32; 3] {
    // Moller-Trumbore algorithm
    let ab = b - a;
    let ac = c - a;
    let pv = Vec3D::cross(ray.dir, ac);
    let det = Vec3D::dot(ab, pv);

    if det.abs() < 1e-12 {
        return [0.0, 0.0, 0.0];
    }

    let inv_det = 1.0 / det;
    let tv = ray.pos - a;
    let qv = Vec3D::cross(tv, ab);
    let t = Vec3D::dot(qv, ac) * inv_det;
    let u = Vec3D::dot(tv, pv) * inv_det;
    let v = Vec3D::dot(qv, ray.dir) * inv_det;

    [t, u, v]
}

impl Geometry for Triangle {
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult<'static>> {
        let [t, u, v] = moller_trumbore([self.a, self.b, self.c], ray);

        if t >= 0.0 && t <= t_max && u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            let norm = Vec3D::cross(self.b - self.a, self.c - self.a);

            Some(HitResult {
                t,
                norm,
                pos: ray.at(t),
                uv: [u, v],
                material: &DEFAULT_MATERIAL,
            })
        } else {
            None
        }
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        let [t, u, v] = moller_trumbore([self.a, self.b, self.c], ray);
        t >= 0.0 && t <= t_max && u >= 0.0 && v >= 0.0 && u + v <= 1.0
    }

    fn bounding_box(&self) -> AABB {
        AABB::from_point(self.a)
            .union_point(self.b)
            .union_point(self.c)
    }
}
