use geom::{Geometry, HitResult};
use math::{Dot, Ray, Vec3D, AABB};
use material::DEFAULT_MATERIAL;
use std::f32::consts::PI;

#[derive(Debug, PartialEq, Constructor)]
pub struct Sphere {
    center: Vec3D,
    radius: f32,
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let offset = ray.pos - self.center;
        let a = -ray.dir.dot(offset);
        let b = a * a - offset.length_squared() + self.radius * self.radius;

        if b < 0.0 {
            return None;
        }

        let t0 = a - b.sqrt();
        let t1 = a + b.sqrt();

        let t = if t0 < t1 && t0 > t_min && t0 < t_max {
            t0
        } else if t1 > t_min && t1 < t_max {
            t1
        } else {
            return None;
        };

        let pos = ray.at(t);
        let norm = (offset + ray.dir * t).normalize();

        let u = norm[0].atan2(norm[1]) / PI * 0.5 + 0.5;
        let v = norm[2].acos() / PI;


        Some(HitResult {
            t,
            norm,
            pos,
            uv: (u, v),
            material: &DEFAULT_MATERIAL,
        })
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.center + Vec3D::fill(self.radius),
            self.center - Vec3D::fill(self.radius),
        )
    }
}
