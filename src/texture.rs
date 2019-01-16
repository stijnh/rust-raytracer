use crate::math::Vec3D;

pub type Color = Vec3D;
pub const COLOR_WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const COLOR_BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const COLOR_RED: Color = Color::new(1.0, 0.0, 0.0);
pub const COLOR_GREEN: Color = Color::new(0.0, 1.0, 0.0);
pub const COLOR_BLUE: Color = Color::new(0.0, 0.0, 1.0);

pub trait Texture: Send + Sync + 'static {
    fn color_at(&self, u: f32, v: f32) -> Color;
}

impl Texture for Color {
    fn color_at(&self, _: f32, _: f32) -> Color {
        *self
    }
}

pub struct UVTexture;

impl Texture for UVTexture {
    fn color_at(&self, u: f32, v: f32) -> Color {
        Color::new(u, v, 0.0)
    }
}

pub struct Checkerboard(i32);

impl Checkerboard {
    pub fn new(repeats: i32) -> Self {
        Checkerboard(repeats)
    }
}

impl Texture for Checkerboard {
    fn color_at(&self, u: f32, v: f32) -> Color {
        let i = (u * self.0 as f32) as i32;
        let j = (v * self.0 as f32) as i32;

        if (i % 2 == 0) ^ (j % 2 == 0) {
            COLOR_BLACK
        } else {
            COLOR_WHITE
        }
    }
}

pub struct Image(image::RgbImage);

impl Image {
    pub fn new<T: image::ConvertBuffer<image::RgbImage>>(img: &T) -> Self {
        Image(img.convert())
    }

    pub fn open(filename: &str) -> Result<Self, image::ImageError> {
        let img = image::open(filename)?;
        Ok(Image(img.to_rgb()))
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }
}

impl Texture for Image {
    fn color_at(&self, u: f32, v: f32) -> Color {
        let (w, h) = self.0.dimensions();
        let x = (u * w as f32).min(w as f32 - 1.0).max(0.0) as u32;
        let y = (v * h as f32).min(h as f32 - 1.0).max(0.0) as u32;
        let p = self.0.get_pixel(x, y);

        Color::new(
            p[0] as f32 / 255.0,
            p[1] as f32 / 255.0,
            p[2] as f32 / 255.0,
        )
    }
}
