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

pub fn cartesian_to_polar(xyz: Vec3D) -> (f32, f32) {
    let (x, y, z) = xyz.into_tuple();
    let r = (x * x + y * y + z * z).sqrt();

    if r > 0.0 {
        let azi = (z / r).acos();
        let inc = y.atan2(x);
        (inc, azi)
    } else {
        (0.0, 0.0)
    }
}
