use std::sync::Arc;

use geom::{Geometry, HitResult};
use math::{Ray, Vec3D, AABB};
use std::ops::Deref;

pub struct GeometryList<T> {
    list: Vec<T>,
}

impl<T: Geometry> GeometryList<T> {
    pub fn new() -> Self {
        Self::from_vec(vec![])
    }

    pub fn from_vec(list: Vec<T>) -> Self {
        GeometryList { 
            list
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.list
    }

    pub fn add(&mut self, obj: T) {
        self.list.push(obj)
    }
}

impl<T: Geometry> Geometry for GeometryList<T> {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut result: Option<HitResult> = None;

        for geom in self.list.iter() {
            if let Some(r) = geom.hit(ray, t_min, t_max) {
                t_max = r.t;
                result = Some(r);
            }
        }

        result
    }

    fn is_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for geom in self.list.iter() {
            if geom.is_hit(ray, t_min, t_max) {
                return true;
            }
        }

        false
    }

    fn bounding_box(&self) -> AABB {
        self.list
            .iter()
            .fold(AABB::empty(), |a, b| a.union(&b.bounding_box()))
    }
}

pub struct BoundingBox<T>(T, AABB);

impl<T: Geometry> BoundingBox<T> {
    pub fn new(obj: T) -> Self {
        let bbox = obj.bounding_box();
        let padding = (bbox.max - bbox.min).iter().sum::<f32>() * 0.001;
        let real_bbox = AABB::new(bbox.min - padding, bbox.max + padding);

        BoundingBox(obj, real_bbox)
    }
}

impl<T: Geometry> Geometry for BoundingBox<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        if let Some((t_in, t_out)) = self.1.intersect_ray(ray) {
            if t_in <= t_max && t_out >= t_min {
                return self.0.hit(ray, t_min, t_max);
            }
        }

        None
    }

    #[inline(always)]
    fn is_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        if let Some((t_in, t_out)) = self.1.intersect_ray(ray) {
            if t_in <= t_max && t_out >= t_min {
                if self.0.is_hit(ray, t_min, t_max) {
                    return true;
                }
            }
        }

        false
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        self.1
    }
}
