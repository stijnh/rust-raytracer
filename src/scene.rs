use crate::geom::Geometry;
use crate::light::Light;
use crate::math::*;
use crate::texture::{Color, Texture};
use std::f32::consts::PI;
use std::sync::Arc;

pub struct Scene {
    pub root: Arc<dyn Geometry>,
    pub skybox: Arc<dyn Texture>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
}

impl Scene {
    pub fn calculate_background(&self, ray: &Ray) -> Color {
        let [x, y, z] = ray.dir.into_array();
        let u = f32::atan2(x, y) / PI * 0.5 + 0.5;
        let v = f32::acos(z) / PI;
        self.skybox.color_at(u, v)
    }
}

#[derive(Debug, PartialEq)]
pub struct Camera {
    width: usize,
    height: usize,
    pos: Vec3D,
    dir: Vec3D,
    horizontal: Vec3D,
    vertical: Vec3D,
}

impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        let camera = Camera {
            width,
            height,
            pos: Vec3D::zero(),
            dir: Vec3D::z_axis(),
            horizontal: Vec3D::x_axis(),
            vertical: Vec3D::y_axis(),
        };

        camera
            .look_towards(Vec3D::z_axis(), Vec3D::y_axis())
            .perspective(100.0)
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
        self.horizontal = horz * self.horizontal.norm();
        self.vertical = vert * self.vertical.norm();
        self
    }

    pub fn look_at(self, lookat: Vec3D, up: Vec3D) -> Self {
        let dir = lookat - self.pos;
        self.look_towards(dir, up)
    }

    pub fn perspective(mut self, fov: f32) -> Self {
        let fac = (fov / 2.0).to_radians().tan();
        let aspect = (self.height as f32) / (self.width as f32);

        self.horizontal *= fac / self.horizontal.norm();
        self.vertical *= fac / self.vertical.norm() * aspect;
        self
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn generate_ray(&self, x: f32, y: f32) -> Ray {
        let u = 2.0 * (x / self.width as f32) - 1.0;
        let v = 2.0 * (y / self.height as f32) - 1.0;
        let dir = self.dir + u * self.horizontal + v * self.vertical;

        Ray::new(self.pos, dir.normalize())
    }
}
