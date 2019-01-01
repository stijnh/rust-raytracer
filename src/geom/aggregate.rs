use super::{Geometry, HitResult};
use crate::material::Material;
use crate::math::*;
use std::sync::Arc;

pub struct GeometryList<T>(Vec<T>);

impl<T: Geometry> GeometryList<T> {
    pub fn new() -> Self {
        Self::from_vec(vec![])
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        GeometryList(v)
    }

    pub fn push(&mut self, obj: T) {
        self.0.push(obj)
    }
}

impl<T: Geometry> Geometry for GeometryList<T> {
    fn hit(&self, ray: &Ray, mut t_max: f32) -> Option<HitResult> {
        let mut result = None;

        for geom in &self.0 {
            if let Some(r) = geom.hit(ray, t_max) {
                t_max = r.t;
                result = Some(r)
            }
        }

        result
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        for geom in &self.0 {
            if geom.is_hit(ray, t_max) {
                return true;
            }
        }

        false
    }

    fn bounding_box(&self) -> AABB {
        let mut bbox = AABB::new();

        for geom in &self.0 {
            bbox = AABB::union(bbox, geom.bounding_box());
        }

        bbox
    }
}

pub struct BoundingBox<T>(T, AABB);

impl<T: Geometry> BoundingBox<T> {
    pub fn new(obj: T) -> Self {
        let bbox = obj.bounding_box();
        let delta = bbox.max - bbox.min;
        let pad = Vec3D::fill(delta[0] + delta[1] * delta[2]) * 0.001;
        let real_bbox = AABB::from_points(bbox.min - pad, bbox.max + pad);

        BoundingBox(obj, real_bbox)
    }
}

impl<T: Geometry> Geometry for BoundingBox<T> {
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        if let Some((t_in, t_out)) = self.1.intersect_ray(ray) {
            if t_in <= t_max && t_out >= 0.0 {
                return self.0.hit(ray, t_max);
            }
        }

        None
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        if let Some((t_in, t_out)) = self.1.intersect_ray(ray) {
            if t_in <= t_max && t_out >= 0.0 {
                return self.0.is_hit(ray, t_max);
            }
        }

        false
    }

    fn bounding_box(&self) -> AABB {
        self.1
    }
}

pub struct Object {
    pub geometry: Arc<dyn Geometry>,
    pub material: Arc<Material>,
}

impl Object {
    pub fn new<G, M>(geometry: G, material: M) -> Self
    where
        M: Into<Arc<Material>>,
        G: Into<Arc<dyn Geometry>>,
    {
        Object {
            geometry: geometry.into(),
            material: material.into(),
        }
    }
}

impl Geometry for Object {
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        if let Some(mut h) = self.geometry.hit(ray, t_max) {
            h.material = &self.material;
            Some(h)
        } else {
            None
        }
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        self.geometry.is_hit(ray, t_max)
    }

    fn bounding_box(&self) -> AABB {
        self.geometry.bounding_box()
    }
}
