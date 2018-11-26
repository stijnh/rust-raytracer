use math::{Vec3D, Ray};
use std::f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec3D,
    pub max: Vec3D,
}

impl AABB {
    #[inline(always)]
    pub fn new(a: Vec3D, b: Vec3D) -> Self {
        AABB {
            min: Vec3D::from_map(|i| min!(a[i], b[i])),
            max: Vec3D::from_map(|i| max!(a[i], b[i])),
        }
    }

    #[inline(always)]
    pub fn new_unchecked(min: Vec3D, max: Vec3D) -> Self {
        AABB { min, max }
    }

    #[inline(always)]
    pub fn empty() -> Self {
        AABB {
            min: Vec3D::fill(f32::INFINITY),
            max: Vec3D::fill(-f32::INFINITY),
        }
    }

    #[inline(always)]
    pub fn union(&self, other: &Self) -> Self {
        AABB {
            min: Vec3D::from_map(|i| min!(self.min[i], other.min[i])),
            max: Vec3D::from_map(|i| max!(self.max[i], other.max[i])),
        }
    }

    #[inline(always)]
    pub fn union_point(&self, point: Vec3D) -> Self {
        AABB {
            min: Vec3D::from_map(|i| min!(self.min[i], point[i])),
            max: Vec3D::from_map(|i| max!(self.max[i], point[i])),
        }
    }

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

}
