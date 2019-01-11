mod aggregate;
mod bvh;
mod cuboid;
mod sphere;
mod transform;
mod triangle;
mod mesh;

pub use self::aggregate::{BoundingBox, GeometryList};
pub use self::bvh::AABBTree;
pub use self::cuboid::{Cuboid, UnitCuboid};
pub use self::sphere::{Sphere, UnitSphere};
pub use self::transform::{Rotate, Scale, Transform, Translate};
pub use self::triangle::Triangle;
pub use self::mesh::Mesh;
use crate::material::Material;
use crate::math::*;
use std::ops::Deref;

pub struct HitResult<'a> {
    pub pos: Vec3D,
    pub norm: Vec3D,
    pub t: f32,
    pub uv: [f32; 2],
    pub material: &'a Material,
}

pub trait Geometry: Send + Sync {
    fn bounding_box(&self) -> AABB;
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult<'_>>;
    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        self.hit(ray, t_max).is_some()
    }
}

impl<'a, T> Geometry for T
where
    T: Deref + Send + Sync,
    <T as Deref>::Target: Geometry + 'a,
{
    fn bounding_box(&self) -> AABB {
        self.deref().bounding_box()
    }

    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult<'_>> {
        self.deref().hit(ray, t_max)
    }

    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        self.deref().is_hit(ray, t_max)
    }
}
