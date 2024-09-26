use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn set_current_color(&mut self, color: u32) {
        // Implementation is now part of point()
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}