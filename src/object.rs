use vec3::Vec3;
use world::Ray;
use std::mem::swap;
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        AABB {
            min: a.min(b),
            max: a.max(b),
        }
    }
}

pub struct HitResult {
    pub t: f64,
    pub normal: Vec3
}

pub trait Object: Sync + Send {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitResult>;
    fn bounding_box(&self) -> AABB;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pos: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f64) -> Self {
        Sphere { pos, radius }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitResult> {

        let offset = ray.pos - self.pos; // o - c
        let a = -ray.dir.dot(offset);    // -(l . (o - c))
        let b = a * a - offset.length_sqr() + self.radius * self.radius; // (l . (o - c))**2 - (o - c)**2 + r ** 2

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

        Some(HitResult{ t, normal })
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.pos + self.radius,
            self.pos - self.radius)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cuboid {
    min: Vec3,
    max: Vec3
}

impl Cuboid {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Cuboid {
            min: a.min(b),
            max: a.max(b),
        }
    }
}

impl Object for Cuboid {
    fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> Option<HitResult> {
        for i in 0..3 {
            if ray.dir[i].abs() > 0.001 {
                let mut tx_min = (self.min[i] - ray.pos[i]) / ray.dir[i];
                let mut tx_max = (self.max[i] - ray.pos[i]) / ray.dir[i];
                if tx_min > tx_max { swap(&mut tx_min, &mut tx_max); }

                t_min = t_min.max(tx_min);
                t_max = t_max.min(tx_max);
            } else if ray.pos[i] < self.min[i] || ray.pos[i] > self.max[i] {
                return None;
            }
        }


        if t_min < t_max {
            Some(HitResult{t: t_min, normal: Vec3::unit_x()})
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(self.min, self.max)
    }
}

pub struct ObjectList<T>(Vec<T>);

impl <T> ObjectList<T> {
    pub fn new(objs: Vec<T>) -> Self {
        ObjectList(objs)
    }
}

impl <T> Object for ObjectList<T> where T: Object {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitResult> {
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
        let mut result  = self.0[0].bounding_box();

        for obj in self.0.iter() {
            let bb = obj.bounding_box();
            result = AABB::new(result.min.min(bb.min), result.max.max(bb.max));
        }

        result
    }
}

pub struct BoundingBox<T>(T, AABB);

impl <T: Object> BoundingBox<T> {
    pub fn new(obj: T) -> Self {
        let bb = obj.bounding_box();
        BoundingBox(obj, bb)
    }
}

impl <T: Object> Object for BoundingBox<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let b = &self.1;
        let tx0 = (b.min.x - ray.pos.x) / ray.dir.x;
        let tx1 = (b.max.x - ray.pos.x) / ray.dir.x;
        let ty0 = (b.min.y - ray.pos.y) / ray.dir.y;
        let ty1 = (b.max.y - ray.pos.y) / ray.dir.y;
        let tz0 = (b.min.z - ray.pos.z) / ray.dir.z;
        let tz1 = (b.max.z - ray.pos.z) / ray.dir.z;

        let t0 = t_min.max(tx0.min(tx1)).max(ty0.min(ty1)).max(tz0.min(tz1));
        let t1 = t_max.min(tx0.max(tx1)).min(ty0.max(ty1)).min(tz0.max(tz1));

        if t0 < t1 {
            let dt = (t1 - t0) * 0.01;

            self.0.hit(ray, t0 - dt, t1 + dt)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.1
    }
}

impl <T: Object + ?Sized> Object for Box<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        self.deref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> AABB {
        self.deref().bounding_box()
    }
}

#[derive(Debug, Copy, Clone, Constructor)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

impl Object for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
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
                normal: e1.cross(e2)
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        let lbnd = self.a.min(self.b).min(self.c);
        let ubnd = self.a.max(self.b).max(self.c);
        AABB::new(lbnd, ubnd)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FastTriangle {
    a: Vec3,
    a2b: Vec3,
    a2c: Vec3,
}

impl FastTriangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        let a2b = b - a;
        let a2c = c - a;

        Self {a, a2b, a2c}
    }
}

impl Object for FastTriangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
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
            Some(HitResult{t, normal}) 
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        let a = self.a;
        let b = a + self.a2b;
        let c = a + self.a2c;
        AABB::new(a.min(b).min(c), a.max(b).max(c))
    }
}


/*
enum ObjectTree<T> {
    Leaf(Vec<T>),
    Split {
        axis: u8,
        lower: Box<ObjectTree<T>>,
        lower_max: f64,
        upper: Box<ObjectTree<T>>,
        upper_min: f64,
    }
}

impl <T> Object for ObjectTree<T> {
    fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> Option<f64> {
        match self {
            Leaf(list) => list.hit(ray, t_min, t_max),
            node @ Split{ .. } => {


            }
        }
    }

    fn bounding_box(&self) -> AABB {
        match self {
            Leaf(list) => list.bounding_box(),
            node @ Split{ .. } => {
                let a = node.lower.bounding_box();
                let b = node.upper.bounding_box();

                AABB::new(a.min.min(b.min), b.max.max(b.max))
            }
        }
    }
}
*/

