use vec3::Vec3;

struct HitResult {
    pos: Vec3,
    norm: Vec3
}

trait Hit {
    pub fn hit(&self, ray: &Ray, max_t: f64, result: &mut HitResult) -> Option<f64>;
}

#[derive(Copy, Clone, Debug)]
struct Sphere {
    pos: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f64) {
        Sphere { pos, radius }
    }
}

impl Hit for Sphere {

}
