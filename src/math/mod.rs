mod aabb;
mod mat3d;
mod quaternion;
mod ray;
mod vec3d;

pub use self::aabb::AABB;
pub use self::mat3d::Mat3D;
pub use self::quaternion::Quaternion;
pub use self::ray::Ray;
pub use self::vec3d::Vec3D;

pub fn vec3d(x: f32, y: f32, z: f32) -> Vec3D {
    Vec3D::new(x, y, z)
}
