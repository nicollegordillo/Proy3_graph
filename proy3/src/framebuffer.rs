use image::{ImageBuffer, RgbImage};
use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::new(0, 0, 0); width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[y * self.width + x] = color;
    }

    pub fn save(&self, filename: &str) -> Result<(), image::ImageError> {
        let img: RgbImage = ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            let color = self.buffer[y as usize * self.width + x as usize];
            image::Rgb([color.r, color.g, color.b])
        });
        img.save(filename)
    }

    pub fn to_u32(&self) -> Vec<u32> {
        self.buffer.iter().map(|color| {
            // Pack the RGB values into a single u32 (ARGB format)
            (0xff << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32) // Ensure casting to u32
        }).collect()
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|color| color.to_u32()).collect()
    }
}