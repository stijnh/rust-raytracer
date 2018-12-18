use material::Material;
use math::{Vec3D, AABB, Ray};
use std::ops::Deref;
use std::sync::Arc;

pub struct HitResult<'a> {
    pub pos: Vec3D,
    pub norm: Vec3D,
    pub t: f32,
    pub uv: (f32, f32),
    pub material: &'a (Material + 'a),
}

pub trait Geometry: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult<'_>>;
    fn bounding_box(&self) -> AABB;

    fn is_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        self.hit(ray, t_min, t_max).is_some()
    }
}

impl<T: Deref<Target = Geometry> + Send + Sync + ?Sized> Geometry for T {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        self.deref().hit(ray, t_min, t_max)
    }

    #[inline(always)]
    fn is_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        self.deref().is_hit(ray, t_min, t_max)
    }

    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        self.deref().bounding_box()
    }
}

pub struct Object {
    pub geometry: Arc<dyn Geometry>,
    pub material: Arc<dyn Material>,
}

impl Object {
    pub fn new<G, M>(geometry: G, material: M) -> Self where
            G: Into<Arc<dyn Geometry>>, 
            M: Into<Arc<dyn Material>> {
        Object {
            geometry: geometry.into(),
            material: material.into(),
        }
    }
}

impl Geometry for Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        self.geometry.hit(ray, t_min, t_max).map(|mut h| {
            h.material = &*self.material;
            h
        })
    }

    fn is_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        self.geometry.is_hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> AABB {
        self.geometry.bounding_box()
    }
}
