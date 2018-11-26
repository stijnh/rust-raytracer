use math::{Vec3D, AABB, Ray};
use geom::{Geometry, HitResult};

pub struct Cuboid {
    min: Vec3D,
    max: Vec3D,
}

impl Cuboid {
    pub fn new(a: Vec3D, b: Vec3D) -> Self {
        Cuboid {
            min: Vec3D::from_map(|i| min!(a[i], b[i])),
            max: Vec3D::from_map(|i| max!(a[i], b[i])),
        }
    }

    pub fn from_center(center: Vec3D, sides: Vec3D) -> Self {
        Cuboid::new(center - sides / 2.0, center + sides / 2.0)
    }
}

impl Geometry for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let a = (self.min - ray.pos) / ray.dir;
        let b = (self.max - ray.pos) / ray.dir;

        let t0 = max!(min!(a[0], b[0]), min!(a[1], b[1]), min!(a[2], b[2]));
        let t1 = min!(max!(a[0], b[0]), max!(a[1], b[1]), max!(a[2], b[2]));
        let t = iff!(t0 > t_min, t0, t1);

        if t0 > t1 || t0 > t_max || t1 < t_min {
            return None;
        }

        let p = ray.at(t);
        let normal = if false { Vec3D::zero() }
            else if t == a[0] { -Vec3D::unit_x() }
            else if t == a[1] { -Vec3D::unit_y() }
            else if t == a[2] { -Vec3D::unit_z() }
            else if t == b[0] { Vec3D::unit_x() }
            else if t == b[1] { Vec3D::unit_y() }
            else if t == b[2] { Vec3D::unit_z() }
            else { unreachable!() };

        Some(HitResult {
            pos: p,
            norm: normal,
            t: t,
            uv: (0.0, 0.0),
        })
    }

    fn bounding_box(&self) -> AABB {
        AABB::new_unchecked(self.min, self.max)
    }
}
