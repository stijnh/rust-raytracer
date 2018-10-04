use vec3::Vec3;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Ray {
    pos: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(pos: Vec3, dir: Vec3) -> Self {
        Ray { pos, dir: dir.normalize() }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        *self.pos + *self.dir * t
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Camera {
    pos: Vec3,
    dir: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vec3::zero(),
            dir: Vec3::unit_z(),
            horizontal: Vec3::unit_x(),
            vertical: Vec3::unit_y(),
        }
    }

    pub fn position(self, pos: Vec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn look_towards(self, dir: Vec3, up: Vec3) -> Self {
        let dir = dir.normalize();
        let horz = up.cross(dir).normalize();
        let vert = dir.cross(horz).normalize();

        self.dir = dir;
        self.horizontal = horz * self.horziontal.length();
        self.vertical = vert * self.vertical.length();
        self
    }

    pub fn look_at(self, lookat: Vec3, up: Vec3) -> Self {
        self.look_towards(self.pos - lookat, up)
    }

    pub fn perspective(self, fov: f64, near: f64, width: f64, height: f64) -> Self {
        d
    }

    pub fn ray_at(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.pos, self.dir + u * self.horizontal + v * self.vertical)
    }
}
