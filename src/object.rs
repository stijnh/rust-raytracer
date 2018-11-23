use world::Ray;
use std::mem::swap;
use std::ops::Deref;
use util::{vec3d, Vec3D, Mat3D, Dot};

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Vec3D,
    pub max: Vec3D,
}

impl AABB {
    pub fn new(a: Vec3D, b: Vec3D) -> Self {
        AABB {
            min: Vec3D::from_map(|i| min!(a[i], b[i])),
            max: Vec3D::from_map(|i| max!(a[i], b[i])),
        }
    }

    pub fn degenerate() -> Self {
        AABB {
            min: Vec3D::from_scalar(::std::f32::INFINITY),
            max: Vec3D::from_scalar(-::std::f32::INFINITY),
        }
    }

    #[inline(always)]
    pub fn intersect_ray(&self, ray: &Ray) -> Option<(f32, f32)> {
        let tx0 = (self.min[0] - ray.pos[0]) / ray.dir[0];
        let tx1 = (self.max[0] - ray.pos[0]) / ray.dir[0];
        let ty0 = (self.min[1] - ray.pos[1]) / ray.dir[1];
        let ty1 = (self.max[1] - ray.pos[1]) / ray.dir[1];
        let tz0 = (self.min[2] - ray.pos[2]) / ray.dir[2];
        let tz1 = (self.max[2] - ray.pos[2]) / ray.dir[2];

        let t0 = max!(min!(tx0, tx1), min!(ty0, ty1), min!(tz0, tz1));
        let t1 = min!(max!(tx0, tx1), max!(ty0, ty1), max!(tz0, tz1));

        if t0 < t1 {
            Some((t0, t1))
        } else {
            None
        }
    }

    pub fn union_point(&self, point: Vec3D) -> Self {
        AABB {
            min: Vec3D::from_map(|i| min!(self.min[i], point[i])),
            max: Vec3D::from_map(|i| max!(self.max[i], point[i])),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        self.union_point(other.min).union_point(other.max)
    }
}

pub struct HitResult {
    pub t: f32,
    pub n: Vec3D,
    pub p: Vec3D,
    pub uv: (f32, f32),
}

pub trait Object: Sync + Send {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitResult>;
    fn bounding_box(&self) -> AABB;
}

pub struct Transform<T> {
    obj: T,
    pos: Vec3D,
    rot: Mat3D,
    rot_inv: Mat3D,
    scale: f32,
    scale_inv: f32,
}

impl <T> Transform<T> {
    pub fn new(obj: T) -> Self {
        Transform {
            obj,
            pos: Vec3D::zero(),
            rot: Mat3D::one(),
            rot_inv: Mat3D::one(),
            scale: 1.0,
            scale_inv: 1.0
        }
    }

    pub fn translate(mut self, p: Vec3D) -> Self {
        self.pos += p;
        self
    }

    pub fn scale(mut self, factor: f32) -> Self {
        assert!(factor > 0.0);
        self.scale *= factor;
        self.scale_inv = 1.0 / self.scale;
        self
    }

    pub fn rotate(mut self, axis: Vec3D, angle: f32) -> Self {
        let u = axis.normalize();
        let (ux, uy, uz) = (axis[0], axis[1], axis[2]);

        let c = angle.cos();
        let s = angle.sin();

        let m = Mat3D::from(
            c + ux * ux * (1.0 - c),
            ux * uy * (1.0 - c) - uz * s,
            ux * uz * (1.0 - c) + uy * s,

            uy * ux * (1.0 - c) + uz * s,
            c + uy * uy * (1.0 - c),
            uy * uz * (1.0 - c) - ux * s,

            uz * ux * (1.0 - c) - uy * s,
            uz * uy * (1.0 - c) + ux * s,
            c + uz * uz * (1.0 - c),
        );

        self.pos = m.dot(self.pos);
        self.rot = m.dot(self.rot);
        self.rot_inv = self.rot.transpose();
        self
    }

    pub fn rotate_x(mut self, angle: f32) -> Self {
        self.rotate(Vec3D::from(1.0, 0.0, 0.0), angle)
    }

    pub fn rotate_y(mut self, angle: f32) -> Self {
        self.rotate(Vec3D::from(0.0, 1.0, 0.0), angle)
    }

    pub fn rotate_z(mut self, angle: f32) -> Self {
        self.rotate(Vec3D::from(0.0, 0.0, 1.0), angle)
    }
}

impl <T: Object> Object for Transform<T> {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitResult> {
        let p = self.rot_inv.dot((ray.pos - self.pos) * self.scale_inv);
        let d = self.rot_inv.dot(ray.dir);
        let r = Ray::new(p, d);

        self.obj.hit(&r, min_t, max_t).map(|r| HitResult {
            t: r.t * self.scale,
            p: self.rot.dot(r.p) * self.scale + self.pos,
            n: self.rot.dot(r.n),
            uv: r.uv,
        })
    }


    fn bounding_box(&self) -> AABB {
        let b = self.obj.bounding_box();
        let (min, max) = (b.min, b.max);
        let mut bbox = AABB::degenerate();

        for i in 0..8 {
            let p = vec3d(
                iff!(i % 2 < 1, min[0], max[0]),
                iff!(i % 4 < 2, min[1], max[1]),
                iff!(i     < 4, min[2], max[2]),
            );

            let q = self.rot.dot(p) * self.scale + self.pos;
            bbox = bbox.union_point(q);
        }

        bbox
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pos: Vec3D,
    radius: f32,
}

impl Sphere {
    pub fn new(pos: Vec3D, radius: f32) -> Self {
        Sphere { pos, radius }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitResult> {

        let offset = ray.pos - self.pos; // o - c
        let a = -ray.dir.dot(offset);    // -(l . (o - c))
        let b = a * a - offset.sqrlen() + self.radius * self.radius; // (l . (o - c))**2 - (o - c)**2 + r ** 2

        if b < 0.0 {
            return None
        }

        let t0 = a - b.sqrt();
        let t1 = a + b.sqrt();

        let t = if t0 < t1 && t0 > min_t && t0 < max_t {
            t0
        } else if t1 > min_t && t1 < max_t {
            t1
        } else {
            return None
        };

        let normal = offset + ray.dir * t;

        Some(HitResult {
            t, 
            n: normal,
            p: ray.pos + ray.dir * t,
            uv: (0.0, 0.0),
        })
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.pos + Vec3D::from_scalar(self.radius),
            self.pos - Vec3D::from_scalar(self.radius))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cuboid {
    min: Vec3D,
    max: Vec3D
}

impl Cuboid {
    pub fn new(a: Vec3D, b: Vec3D) -> Self {
        let bbox = AABB::new(a, b);

        Cuboid {
            min: bbox.min,
            max: bbox.max,
        }
    }
}

impl Object for Cuboid {
    fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut t = t_min;
        let mut normal = Vec3D::zero();

        for i in 0..3 {
            if ray.dir[i].abs() > 0.001 {
                let mut tx_min = (self.min[i] - ray.pos[i]) / ray.dir[i];
                let mut tx_max = (self.max[i] - ray.pos[i]) / ray.dir[i];
                let mut n = Vec3D::from_map(|j| iff!(i == j, -1.0, 0.0));

                if tx_min > tx_max { 
                    swap(&mut tx_min, &mut tx_max); 
                    n = -n;
                }

                if tx_min > t {
                    t = tx_min;
                    normal = n;
                }

                if tx_max < t_max {
                    t_max = tx_max;
                }

            } else if ray.pos[i] < self.min[i] || ray.pos[i] > self.max[i] {
                return None;
            }
        }


        if t < t_max {
            Some(HitResult{
                t, 
                n: normal,
                p: ray.pos + ray.dir * t,
                uv: (0.0, 0.0),
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(self.min, self.max)
    }
}

pub struct ObjectList<T>(Box<[T]>);

impl <T> ObjectList<T> {
    pub fn new(objs: Vec<T>) -> Self {
        ObjectList(objs.into_boxed_slice())
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0.into_vec()
    }
}

impl <T> Object for ObjectList<T> where T: Object {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut result = None;

        for obj in self.0.iter() {
            if let Some(r) = obj.hit(ray, t_min, t_max) {
                t_max = r.t;
                result = Some(r);
            }
        }

        result
    }

    fn bounding_box(&self) -> AABB {
        self.0.iter()
            .fold(AABB::degenerate(), |result, obj| {
                result.union(&obj.bounding_box())
            })
    }
}

pub struct BoundingBox<T>(T, AABB);

impl <T: Object> BoundingBox<T> {
    pub fn new(obj: T) -> Self {
        let bb = obj.bounding_box();
        let diff = (bb.max - bb.min).iter().sum::<f32>() * 1e-5;
        let bb = AABB::new(
            bb.min - Vec3D::from_scalar(diff),
            bb.max + Vec3D::from_scalar(diff),
        );

        BoundingBox(obj, bb)
    }
}

impl <T: Object> Object for BoundingBox<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        if let Some((t_in, t_out)) = self.1.intersect_ray(ray) {
            if t_in <= t_max && t_out >= t_min {
                let t0 = max!(t_in, t_min);
                let t1 = min!(t_out, t_max);

                return self.0.hit(ray, t0, t1);
            }
        }

        None
    }

    fn bounding_box(&self) -> AABB {
        self.1
    }
}

impl <T: Object + ?Sized> Object for Box<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        self.deref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> AABB {
        self.deref().bounding_box()
    }
}

#[derive(Debug, Copy, Clone, Constructor)]
pub struct Triangle {
    a: Vec3D,
    b: Vec3D,
    c: Vec3D,
}

impl Object for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let v = self.a;
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let h = ray.dir.cross(e2);
        let a = e1.dot(h);

        if a.abs() < 0.001 {
            return None
        }

        let f = 1.0 / a;
        let s = ray.pos - v;
        let q = s.cross(e1);
        let u = f * s.dot(h);
        let v = f * ray.dir.dot(q);
        let t = f * e2.dot(q);

        if t > t_min && t < t_max && u >= 0.0 && v >= 0.0 && u + v < 1.0 {
            Some(HitResult {
                t,
                n: e1.cross(e2),
                p: ray.pos + t * ray.dir,
                uv: (u, v),
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

#[derive(Debug, Copy, Clone)]
pub struct FastTriangle {
    a: Vec3D,
    a2b: Vec3D,
    a2c: Vec3D,
}

impl FastTriangle {
    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D) -> Self {
        let a2b = b - a;
        let a2c = c - a;

        Self {a, a2b, a2c}
    }
}

impl Object for FastTriangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let O = ray.pos;
        let D = ray.dir;
        let V = self.a;
        let E1 = self.a2b;
        let E2 = self.a2c;

        let T = O - V;
        let P = D.cross(E2);
        let Q = T.cross(E1);

        let f = P.dot(E1);
        let t = Q.dot(E2) / f;

        if t < t_min || t > t_max { return None }

        let u = P.dot(T) / f;
        let v = Q.dot(D) / f;

        if u > 0.0 && v > 0.0 && v + u < 1.0 {
            let normal = E1.cross(E2);
            Some(HitResult{
                t, 
                n: normal,
                p: O + D * t,
                uv: (u, v),
            }) 
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        let a = self.a;
        let b = a + self.a2b;
        let c = a + self.a2c;
        Triangle { a, b, c }.bounding_box()
    }
}

