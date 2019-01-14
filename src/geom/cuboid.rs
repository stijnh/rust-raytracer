use crate::geom::{Geometry, HitResult};
use crate::material::DEFAULT_MATERIAL;
use crate::math::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitCuboid;

fn ray_box_intersection(ray: &Ray, min: Vec3D, max: Vec3D) -> (f32, f32) {
    let a = (min - ray.pos) * ray.inv_dir;
    let b = (max - ray.pos) * ray.inv_dir;

    let t0 = max!(min!(a[0], b[0]), min!(a[1], b[1]), min!(a[2], b[2]));
    let t1 = min!(max!(a[0], b[0]), max!(a[1], b[1]), max!(a[2], b[2]));

    (t0, t1)
}

impl Geometry for UnitCuboid {
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        let (t0, t1) = ray_box_intersection(ray, -Vec3D::one(), Vec3D::one());

        let t = if t0 > t1 || t0 > t_max || t1 < 0.0 {
            return None;
        } else if t0 > 0.0 {
            t0
        } else {
            t1
        };

        let p = ray.at(t);
        let p_abs = p.map(f32::abs);

        let (n, u, v) = if p_abs[0] > p_abs[1] && p_abs[0] > p_abs[2] {
            (Vec3D::new(p[0], 0.0, 0.0), p[1], p[2])
        } else if p_abs[1] > p_abs[2] {
            (Vec3D::new(0.0, p[1], 0.0), p[0], p[2])
        } else {
            (Vec3D::new(0.0, 0.0, p[2]), p[0], p[1])
        };

        Some(HitResult {
            pos: p,
            norm: n,
            uv: [u, v],
            t,
            material: &DEFAULT_MATERIAL,
        })
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        let (t0, t1) = ray_box_intersection(ray, -Vec3D::one(), Vec3D::one());

        t0 <= t1 && t0 <= t_max && t1 >= 0.0
    }

    fn bounding_box(&self) -> AABB {
        AABB::from_min_max(-Vec3D::one(), Vec3D::one())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cuboid {
    center: Vec3D,
    extent: Vec3D,
    inv_extent: Vec3D,
}

impl Cuboid {
    pub fn new(a: Vec3D, b: Vec3D) -> Self {
        let center = (a + b) * 0.5;
        let extent = (a - center).map(f32::abs);

        Cuboid {
            center,
            extent,
            inv_extent: 1.0 / extent,
        }
    }
}

impl Geometry for Cuboid {
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        let p = (ray.pos - self.center) * self.inv_extent;
        let d = ray.dir * self.inv_extent;
        let new_ray = Ray::new(p, d);

        if let Some(mut h) = UnitCuboid.hit(&new_ray, t_max) {
            h.pos = h.pos * self.extent + self.center;
            Some(h)
        } else {
            None
        }
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        let (t0, t1) =
            ray_box_intersection(ray, self.center - self.extent, self.center + self.extent);

        t0 <= t1 && t0 <= t_max && t1 >= 0.0
    }

    fn bounding_box(&self) -> AABB {
        AABB::from_min_max(self.center - self.extent, self.center + self.extent)
    }
}
