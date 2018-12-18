use util::{Color, COLOR_BLACK, COLOR_WHITE};
use image;


pub trait Texture: Send + Sync {
    fn color_at(&self, u: f32, v: f32) -> Color;
}

impl Texture for Color {
    fn color_at(&self, u: f32, v: f32) -> Color {
        *self
    }
}

pub struct CheckerTexture {
    pub repeat: i32,
    pub even: Color,
    pub odd: Color,
}

impl CheckerTexture {
    pub fn new(repeat: i32, even: Color, odd: Color) -> Self {
        Self { repeat, even, odd }
    }

    pub fn with_black_white(repeat: i32) -> Self {
        Self::new(repeat, COLOR_BLACK, COLOR_WHITE)
    }
}

impl Texture for CheckerTexture {
    fn color_at(&self, u: f32, v: f32) -> Color {
        let (u, v) = (u * self.repeat as f32, v * self.repeat as f32);
        let x = iff!(u > 0.0, u % 1.0, 1.0 + u % 1.0);
        let y = iff!(v > 0.0, v % 1.0, 1.0 + v % 1.0);

        iff!((x < 0.5) ^ (y < 0.5), self.even, self.odd)
    }
}

pub struct UVTexture;

impl Texture for UVTexture {
    fn color_at(&self, u: f32, v: f32) -> Color {
        let x = iff!(u > 0.0, u % 1.0, 1.0 + u % 1.0);
        let y = iff!(v > 0.0, v % 1.0, 1.0 + v % 1.0);

        Color::new(x, y, 0.0)
    }
}

pub struct ImageTexture {
    pub image: image::RgbImage,
}

impl ImageTexture {
    pub fn new(image: image::RgbImage) -> Self {
        Self { image }
    }

    pub fn from_filename(filename: &str) -> Option<Self> {
        match image::open(filename).ok() {
            Some(img) => Some(Self::new(img.to_rgb())),
            None => None,
        }
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }
}

impl Texture for ImageTexture {
    fn color_at(&self, u: f32, v: f32) -> Color {
        let (w, h) = self.image.dimensions();
        let x = iff!(u >= 0.0, u % 1.0, 1.0 + u % 1.0) * (w as f32);
        let y = iff!(v >= 0.0, v % 1.0, 1.0 + v % 1.0) * (h as f32);

        let i = min!(x as u32, w - 1);
        let j = min!(y as u32, h - 1);

        let [r, g, b] = self.image.get_pixel(i, j).data;
        Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }
}
