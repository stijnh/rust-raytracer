use geom::{Geometry, HitResult};
use math::{Dot, Ray, Vec3D, AABB};
use material::DEFAULT_MATERIAL;

#[derive(Debug, Constructor, PartialEq, Copy, Clone)]
pub struct Triangle {
    a: Vec3D,
    b: Vec3D,
    c: Vec3D,
}

impl Geometry for Triangle {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let v = self.a;
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let h = ray.dir.cross(e2);
        let a = e1.dot(h);

        let f = 1.0 / a;
        let s = ray.pos - v;
        let q = s.cross(e1);
        let u = f * s.dot(h);
        let v = f * ray.dir.dot(q);
        let t = f * e2.dot(q);

        if t > t_min && t < t_max && u >= 0.0 && v >= 0.0 && u + v < 1.0 {
            Some(HitResult {
                t,
                norm: e1.cross(e2),
                pos: ray.at(t),
                uv: (u, v),
                material: &DEFAULT_MATERIAL
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        let (a, b, c) = (self.a, self.b, self.c);
        let min = Vec3D::from_map(|i| min!(a[i], b[i], c[i]));
        let max = Vec3D::from_map(|i| max!(a[i], b[i], c[i]));
        AABB::new(min, max)
    }
}
