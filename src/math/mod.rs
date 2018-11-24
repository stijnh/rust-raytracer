pub mod ray;
pub mod vec3;

pub use self::ray::Ray;
pub use self::vec3::Vec3;

trait Dot<RHS=Self> {
    type Output;
    fn dot(&self, rhs: &RHS) -> Self::Output;
}

type Vec3D = Vec3<f32>;
