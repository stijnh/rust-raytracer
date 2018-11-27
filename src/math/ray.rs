use math::Vec3D;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub pos: Vec3D,
    pub dir: Vec3D,
}

impl Ray {
    pub fn new(pos: Vec3D, dir: Vec3D) -> Self {
        Ray {
            pos,
            dir: dir.normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Vec3D {
        self.pos + self.dir * t
    }
}
