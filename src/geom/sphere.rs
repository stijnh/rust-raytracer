use crate::geom::{Geometry, HitResult, Scale, Translate};
use crate::material::DEFAULT_MATERIAL;
use crate::math::*;
use std::f32::consts::PI;

#[derive(Debug, PartialEq)]
pub struct UnitSphere;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    obj: Translate<Scale<UnitSphere>>,
}

impl Sphere {
    pub fn new(center: Vec3D, radius: f32) -> Self {
        Self {
            obj: Translate::new(Scale::new(UnitSphere, radius), center),
        }
    }
}

#[inline(always)]
fn sphere_intersect(ray: &Ray) -> Option<(f32, f32)> {
    let a = -Vec3D::dot(ray.dir, ray.pos);
    let b = a * a - Vec3D::dot(ray.pos, ray.pos) + 1.0;

    if b >= 0.0 {
        let d = b.sqrt();
        Some((a - d, a + d))
    } else {
        None
    }
}

impl Geometry for UnitSphere {
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        let (t0, t1) = sphere_intersect(ray)?;

        let t = if t0 > t_max || t1 < 0.0 {
            return None;
        } else if t0 <= 0.0 {
            t0
        } else {
            t1
        };

        let pos = ray.at(t);
        let norm = pos;

        let u = norm[0].atan2(norm[1]) / PI * 0.5 + 0.5;
        let v = norm[2].acos() / PI;

        Some(HitResult {
            t,
            norm,
            pos,
            uv: [u, v],
            material: &DEFAULT_MATERIAL,
        })
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        if let Some((t0, t1)) = sphere_intersect(ray) {
            t0 <= t_max && t1 >= 0.0
        } else {
            false
        }
    }

    fn bounding_box(&self) -> AABB {
        AABB::from_points(-Vec3D::one(), Vec3D::one())
    }
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        self.obj.hit(ray, t_max)
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        self.obj.is_hit(ray, t_max)
    }

    fn bounding_box(&self) -> AABB {
        self.obj.bounding_box()
    }
}
