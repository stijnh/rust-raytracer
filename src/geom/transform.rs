use geom::{Geometry, HitResult};
use math::{Quaternion, Ray, Vec3D, AABB};

#[derive(PartialEq, Debug)]
pub struct Translate<T> {
    geom: T,
    pos: Vec3D,
}

#[derive(PartialEq, Debug)]
pub struct Scale<T> {
    geom: T,
    scale: f32,
}

#[derive(PartialEq, Debug)]
pub struct Rotate<T> {
    geom: T,
    rotate: Quaternion<f32>,
}

#[derive(PartialEq, Debug)]
pub struct Transform<T> {
    geom: Translate<Scale<Rotate<T>>>,
}

impl<T: Geometry> Translate<T> {
    pub fn new(geom: T, pos: Vec3D) -> Self {
        Self { geom, pos }
    }

    pub fn translate(mut self, delta: Vec3D) -> Self {
        self.pos += delta;
        self
    }
}

impl<T: Geometry> Geometry for Translate<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let new_ray = Ray::new(ray.pos - self.pos, ray.dir);

        if let Some(mut result) = self.geom.hit(&new_ray, t_min, t_max) {
            result.pos += self.pos;
            Some(result)
        } else {
            None
        }
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        let bbox = self.geom.bounding_box();

        AABB::new_unchecked(bbox.min + self.pos, bbox.max + self.pos)
    }
}

impl<T: Geometry> Scale<T> {
    pub fn new(geom: T, scale: f32) -> Self {
        Self { geom, scale }
    }

    pub fn scale(mut self, factor: f32) -> Self {
        assert!(factor > 0.0);
        self.scale *= factor;
        self
    }
}

impl<T: Geometry> Geometry for Scale<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let new_ray = Ray::new(ray.pos / self.scale, ray.dir);
        let (t0, t1) = (t_min / self.scale, t_max / self.scale);

        if let Some(mut result) = self.geom.hit(&new_ray, t0, t1) {
            result.t *= self.scale;
            result.pos *= self.scale;
            Some(result)
        } else {
            None
        }
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        let bbox = self.geom.bounding_box();

        AABB::new_unchecked(bbox.min * self.scale, bbox.max * self.scale)
    }
}

impl<T: Geometry> Rotate<T> {
    pub fn new(geom: T) -> Self {
        Self {
            geom,
            rotate: Quaternion::new(),
        }
    }

    pub fn rotate(mut self, axis: Vec3D, angle: f32) -> Self {
        let q = Quaternion::from_rotation(axis, angle);
        self.rotate = self.rotate.multiply(&q);
        self
    }

    pub fn rotate_x(self, angle: f32) -> Self {
        self.rotate(Vec3D::unit_x(), angle)
    }

    pub fn rotate_y(self, angle: f32) -> Self {
        self.rotate(Vec3D::unit_y(), angle)
    }

    pub fn rotate_z(self, angle: f32) -> Self {
        self.rotate(Vec3D::unit_z(), angle)
    }
}

impl<T: Geometry> Geometry for Rotate<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let p = self.rotate.inverse_apply(ray.pos);
        let d = self.rotate.inverse_apply(ray.dir);
        let new_ray = Ray::new(p, d);

        if let Some(mut result) = self.geom.hit(&new_ray, t_min, t_max) {
            result.pos = self.rotate.apply(result.pos);
            result.norm = self.rotate.apply(result.norm);
            Some(result)
        } else {
            None
        }
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        let old_bbox = self.geom.bounding_box();
        let corners = [old_bbox.min, old_bbox.max];
        let mut bbox = AABB::empty();

        unroll! {
            for i in 0..8 {
                let p = Vec3D::new(
                    corners[(i / 1) % 2][0],
                    corners[(i / 2) % 2][1],
                    corners[(i / 4) % 2][2],
                );

                let q = self.rotate.apply(p);
                bbox = bbox.union_point(q);
            }
        }

        bbox
    }
}

impl<T: Geometry> Transform<T> {
    pub fn new(geom: T) -> Self {
        let a = Rotate::new(geom);
        let b = Scale::new(a, 1.0);
        let c = Translate::new(b, Vec3D::zero());
        Self { geom: c }
    }

    pub fn translate(mut self, delta: Vec3D) -> Self {
        self.geom = self.geom.translate(delta);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.geom.geom = self.geom.geom.scale(scale);
        self.geom.pos *= scale;
        self
    }

    pub fn rotate(mut self, axis: Vec3D, angle: f32) -> Self {
        let rot = Quaternion::from_rotation(axis, angle);

        self.geom.geom.geom = self.geom.geom.geom.rotate(axis, angle);
        self.geom.pos = rot.apply(self.geom.pos);
        self
    }

    pub fn rotate_x(self, angle: f32) -> Self {
        self.rotate(Vec3D::unit_x(), angle)
    }

    pub fn rotate_y(self, angle: f32) -> Self {
        self.rotate(Vec3D::unit_y(), angle)
    }

    pub fn rotate_z(self, angle: f32) -> Self {
        self.rotate(Vec3D::unit_z(), angle)
    }
}

impl<T: Geometry> Geometry for Transform<T> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        self.geom.hit(ray, t_min, t_max)
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        self.geom.bounding_box()
    }
}
