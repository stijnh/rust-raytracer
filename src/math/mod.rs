pub mod aabb;
pub mod quaternion;
pub mod ray;
pub mod vec3;

pub use self::aabb::AABB;
pub use self::quaternion::Quaternion;
pub use self::ray::Ray;
pub use self::vec3::Vec3;

pub type Vec3D = Vec3<f32>;

pub fn vec3d(x: f32, y: f32, z: f32) -> Vec3D {
    Vec3D::new(x, y, z)
}

pub trait Dot<RHS = Self> {
    type Output;
    fn dot(&self, rhs: RHS) -> Self::Output;
}
