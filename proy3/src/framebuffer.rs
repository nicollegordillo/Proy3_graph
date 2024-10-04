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


    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.color_to_u32(&color);
        }
    }

    pub fn point_vertex(&mut self, vertex: Vector3<f32>, color: Color) {
        let x = vertex.x.round() as usize;
        let y = vertex.y.round() as usize;
        self.point(x, y, color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.u32_to_color(self.buffer[y * self.width + x])
    }

    fn color_to_u32(&self, color: &Color) -> u32 {
        (255u32 << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
    }

    fn u32_to_color(&self, color: u32) -> Color {
        Color {
            r: ((color >> 16) & 0xFF) as u8,
            g: ((color >> 8) & 0xFF) as u8,
            b: (color & 0xFF) as u8,
        }
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}

