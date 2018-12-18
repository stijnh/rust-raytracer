use math::{Ray, Vec3D, AABB};
use std::ops::Deref;

mod bvh;
mod common;
mod cuboid;
mod sphere;
mod transform;
mod triangle;

pub use scene::{HitResult, Geometry};
pub use self::bvh::AABBTree;
pub use self::common::{BoundingBox, GeometryList};
pub use self::cuboid::Cuboid;
pub use self::sphere::Sphere;
pub use self::transform::{Rotate, Scale, Transform, Translate};
pub use self::triangle::Triangle;


