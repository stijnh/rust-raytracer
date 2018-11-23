use util::Vec3D;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub pos: Vec3D,
    pub dir: Vec3D,
}

impl Ray {
    pub fn new(pos: Vec3D, dir: Vec3D) -> Self {
        Ray { pos, dir: dir.normalize() }
    }

    pub fn at(&self, t: f32) -> Vec3D {
        self.pos + self.dir * t
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Camera {
    pos: Vec3D,
    dir: Vec3D,
    horizontal: Vec3D,
    vertical: Vec3D,
    width: f32,
    height: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vec3D::zero(),
            dir: Vec3D::from(0.0, 0.0, 1.0),
            horizontal: Vec3D::from(1.0, 0.0, 0.0),
            vertical: Vec3D::from(0.0, 1.0, 0.0),
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn position(mut self, pos: Vec3D) -> Self {
        self.pos = pos;
        self
    }

    pub fn look_towards(mut self, dir: Vec3D, up: Vec3D) -> Self {
        let dir = dir.normalize();
        let horz = up.cross(dir).normalize();
        let vert = dir.cross(horz).normalize();

        self.dir = dir;
        self.horizontal = horz * self.horizontal.length();
        self.vertical = vert * self.vertical.length();
        self
    }

    pub fn look_at(self, lookat: Vec3D, up: Vec3D) -> Self {
        self.look_towards(lookat - self.pos, up)
    }

    pub fn perspective(mut self, fov: f32, width: f32, height: f32) -> Self {
        let fac = (fov / 2.0).to_radians().tan();

        self.width = width;
        self.height = height;
        self.horizontal *= fac / self.horizontal.length();
        self.vertical *= fac / self.vertical.length() * (height / width);
        self
    }

    pub fn ray_at(&self, x: f32, y: f32) -> Ray {
        let u = 2.0 * (x / self.width) - 1.0;
        let v = 2.0 * (y / self.height) - 1.0;

        Ray::new(self.pos, self.dir + u * self.horizontal + v * self.vertical)
    }
}
