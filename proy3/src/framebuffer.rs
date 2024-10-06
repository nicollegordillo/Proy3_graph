use nalgebra::Vector3;
use crate::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: Color::new(179, 179, 179),
            current_color: Color::new(255, 255, 255),
        }
    }

    pub fn clear(&mut self) {
        let color_u32 = self.color_to_u32(&self.background_color);
        self.buffer.fill(color_u32);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    fn color_to_u32(&self, color: &Color) -> u32 {
        (255u32 << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
    }

}

