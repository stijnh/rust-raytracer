use super::Vec3D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub pos: Vec3D,
    pub dir: Vec3D,
    pub inv_dir: Vec3D,
}

impl Ray {
    pub fn new(pos: Vec3D, dir: Vec3D) -> Self {
        Self {
            pos,
            dir,
            inv_dir: 1.0 / dir,
        }
    }

    pub fn at(&self, t: f32) -> Vec3D {
        self.pos + Vec3D::fill(t) * self.dir
    }
}
