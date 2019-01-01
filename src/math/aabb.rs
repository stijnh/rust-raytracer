use super::{Ray, Vec3D};
use crunchy::unroll;
use std::mem::swap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    pub min: Vec3D,
    pub max: Vec3D,
}

impl AABB {
    pub fn new() -> Self {
        AABB {
            min: Vec3D::fill(std::f32::INFINITY),
            max: Vec3D::fill(std::f32::NEG_INFINITY),
        }
    }

    pub fn from_point(p: Vec3D) -> Self {
        AABB { min: p, max: p }
    }

    pub fn from_points(p: Vec3D, q: Vec3D) -> Self {
        Self::from_point(p).union_point(q)
    }

    pub fn from_min_max(min: Vec3D, max: Vec3D) -> Self {
        Self { min, max }
    }

    pub fn union(self, other: Self) -> Self {
        AABB {
            min: Vec3D::from_map(move |i| min!(self.min[i], other.min[i])),
            max: Vec3D::from_map(move |i| max!(self.max[i], other.max[i])),
        }
    }

    pub fn union_point(self, p: Vec3D) -> Self {
        self.union(Self::from_point(p))
    }

    pub fn surface_area(&self) -> f32 {
        let d = self.max - self.min;
        2.0 * (d[0] * d[1] + d[1] * d[2] + d[2] * d[0])
    }

    #[inline(always)]
    pub fn intersect_ray(&self, ray: &Ray) -> Option<(f32, f32)> {
        let a = (self.min - ray.pos) / ray.dir;
        let b = (self.max - ray.pos) / ray.dir;

        let t0 = max!(min!(a[0], b[0]), min!(a[1], b[1]), min!(a[2], b[2]));
        let t1 = min!(max!(a[0], b[0]), max!(a[1], b[1]), max!(a[2], b[2]));

        if t0 < t1 {
            Some((t0, t1))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn fast_intersect_ray(
        &self,
        ray_pos: Vec3D,
        inv_ray_dir: Vec3D,
        neg_ray_dir: [bool; 3],
    ) -> Option<(f32, f32)> {
        let mut a = (self.min - ray_pos) * inv_ray_dir;
        let mut b = (self.max - ray_pos) * inv_ray_dir;

        unroll! {
            for i in 0..3 {
                if neg_ray_dir[i] {
                    swap(&mut a[i], &mut b[i])
                }
            }
        };

        let t0 = max!(a[0], a[1], a[2]);
        let t1 = min!(b[0], b[1], b[2]);

        if t0 < t1 {
            Some((t0, t1))
        } else {
            None
        }
    }
}
