use super::{Geometry, HitResult};
use crate::math::*;
use crunchy::unroll;

#[derive(PartialEq, Debug)]
pub struct Translate<T> {
    obj: T,
    offset: Vec3D,
}

#[derive(PartialEq, Debug)]
pub struct Scale<T> {
    obj: T,
    scale: f32,
}

#[derive(PartialEq, Debug)]
pub struct Rotate<T> {
    obj: T,
    mat: Mat3D,
}

#[derive(PartialEq, Debug)]
pub struct Transform<T> {
    obj: Translate<Scale<Rotate<T>>>,
}

impl<T: Geometry> Translate<T> {
    pub fn new(obj: T, offset: Vec3D) -> Self {
        Translate { obj, offset }
    }

    pub fn translate(self, offset: Vec3D) -> Self {
        Self::new(self.obj, self.offset + offset)
    }
}

impl<T: Geometry> Geometry for Translate<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        let new_ray = Ray::new(ray.pos - self.offset, ray.dir);

        if let Some(mut h) = self.obj.hit(&new_ray, t_max) {
            h.pos += self.offset;
            Some(h)
        } else {
            None
        }
    }

    #[inline(always)]
    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        let new_ray = Ray::new(ray.pos - self.offset, ray.dir);
        self.obj.is_hit(&new_ray, t_max)
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        let bbox = self.obj.bounding_box();
        AABB::from_min_max(bbox.min + self.offset, bbox.max + self.offset)
    }
}

impl<T: Geometry> Scale<T> {
    pub fn new(obj: T, scale: f32) -> Self {
        assert!(scale > 0.0);
        Self { obj, scale }
    }

    pub fn scale(self, factor: f32) -> Self {
        Self::new(self.obj, self.scale * factor)
    }
}

impl<T: Geometry> Geometry for Scale<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        let (scale, inv_scale) = (self.scale, 1.0 / self.scale);
        let new_ray = Ray::new(ray.pos * inv_scale, ray.dir);

        if let Some(mut h) = self.obj.hit(&new_ray, t_max * inv_scale) {
            h.t *= scale;
            h.pos *= scale;
            Some(h)
        } else {
            None
        }
    }

    #[inline(always)]
    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        let inv_scale = 1.0 / self.scale;
        let new_ray = Ray::new(ray.pos * inv_scale, ray.dir);
        self.obj.is_hit(&new_ray, t_max * inv_scale)
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        let scale = self.scale;
        let bbox = self.obj.bounding_box();
        AABB::from_min_max(bbox.min * scale, bbox.max * scale)
    }
}

impl<T: Geometry> Rotate<T> {
    pub fn new(obj: T) -> Self {
        Self {
            obj,
            mat: Mat3D::identity(),
        }
    }

    pub fn rotate(mut self, axis: Vec3D, angle: f32) -> Self {
        let m = Mat3D::new_rotation(axis, angle);
        self.mat = Mat3D::multiply(m, self.mat);
        self
    }

    pub fn rotate_x(self, angle: f32) -> Self {
        self.rotate(Vec3D::x_axis(), angle)
    }

    pub fn rotate_y(self, angle: f32) -> Self {
        self.rotate(Vec3D::y_axis(), angle)
    }

    pub fn rotate_z(self, angle: f32) -> Self {
        self.rotate(Vec3D::z_axis(), angle)
    }

    pub fn reflect(mut self, axis: Vec3D) -> Self {
        let m = Mat3D::new_reflection(axis);
        self.mat = Mat3D::multiply(m, self.mat);
        self
    }

    pub fn reflect_x(self) -> Self {
        self.reflect(Vec3D::x_axis())
    }

    pub fn reflect_y(self) -> Self {
        self.reflect(Vec3D::y_axis())
    }

    pub fn reflect_z(self) -> Self {
        self.reflect(Vec3D::z_axis())
    }
}

impl<T: Geometry> Geometry for Rotate<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        let p = self.mat.transpose_apply(ray.pos);
        let d = self.mat.transpose_apply(ray.dir);
        let new_ray = Ray::new(p, d);

        if let Some(mut result) = self.obj.hit(&new_ray, t_max) {
            result.pos = self.mat.apply(result.pos);
            result.norm = self.mat.apply(result.norm);
            Some(result)
        } else {
            None
        }
    }

    #[inline(always)]
    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        let p = self.mat.transpose_apply(ray.pos);
        let d = self.mat.transpose_apply(ray.dir);
        let new_ray = Ray::new(p, d);

        self.obj.is_hit(&new_ray, t_max)
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        let old_bbox = self.obj.bounding_box();
        let corners = [old_bbox.min, old_bbox.max];
        let mut new_bbox = AABB::new();

        unroll! {
            for i in 0..8 {
                let p = Vec3D::new(
                    corners[(i / 1) % 2][0],
                    corners[(i / 2) % 2][1],
                    corners[(i / 4) % 2][2],
                );

                let q = self.mat.apply(p);
                new_bbox = new_bbox.union_point(q);
            }
        }

        new_bbox
    }
}

impl<T: Geometry> Transform<T> {
    pub fn new(obj: T) -> Self {
        let obj = Rotate::new(obj);
        let obj = Scale::new(obj, 1.0);
        let obj = Translate::new(obj, Vec3D::zero());

        Self { obj }
    }

    pub fn translate(mut self, offset: Vec3D) -> Self {
        self.obj = self.obj.translate(offset);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.obj.obj = self.obj.obj.scale(scale);
        self.obj.offset *= scale;
        self
    }

    pub fn rotate(mut self, axis: Vec3D, angle: f32) -> Self {
        let offset = self.obj.obj.obj.mat.transpose_apply(self.obj.offset);
        self.obj.obj.obj = self.obj.obj.obj.rotate(axis, angle);
        self.obj.offset = self.obj.obj.obj.mat.apply(offset);
        self
    }

    pub fn rotate_x(self, angle: f32) -> Self {
        self.rotate(Vec3D::x_axis(), angle)
    }

    pub fn rotate_y(self, angle: f32) -> Self {
        self.rotate(Vec3D::y_axis(), angle)
    }

    pub fn rotate_z(self, angle: f32) -> Self {
        self.rotate(Vec3D::z_axis(), angle)
    }

    pub fn reflect(mut self, axis: Vec3D) -> Self {
        let offset = self.obj.obj.obj.mat.transpose_apply(self.obj.offset);
        self.obj.obj.obj = self.obj.obj.obj.reflect(axis);
        self.obj.offset = self.obj.obj.obj.mat.apply(offset);
        self
    }

    pub fn reflect_x(self) -> Self {
        self.reflect(Vec3D::x_axis())
    }

    pub fn reflect_y(self) -> Self {
        self.reflect(Vec3D::y_axis())
    }

    pub fn reflect_z(self) -> Self {
        self.reflect(Vec3D::z_axis())
    }
}

impl<T: Geometry> Geometry for Transform<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        self.obj.hit(ray, t_max)
    }

    #[inline(always)]
    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        self.obj.is_hit(ray, t_max)
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        self.obj.bounding_box()
    }
}
