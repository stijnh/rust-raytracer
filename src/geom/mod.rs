use math::{Vec3D, Ray, AABB};
use std::ops::Deref;

mod common;
mod sphere;
mod transform;
mod triangle;
mod cuboid;

pub use self::common::{BoundingBox, GeometryList};
pub use self::transform::{Scale, Translate, Rotate, Transform};
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;
pub use self::cuboid::Cuboid;

pub struct HitResult {
    pub pos: Vec3D,
    pub norm: Vec3D,
    pub t: f32,
    pub uv: (f32, f32),
}

pub trait Geometry: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
    fn bounding_box(&self) -> AABB;
}

impl <T: Deref<Target=Geometry> + Send + Sync + ?Sized> Geometry for T {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        self.deref().hit(ray, t_min, t_max)
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        self.deref().bounding_box()
    }
}
