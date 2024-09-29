use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: f32,
    pub specular_exponent: f32,
    pub refraction_index: Option<f32>,
    pub texture: Option<Texture>,
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Texture {
    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let x = (u * self.width as f32) as usize;
        let y = ((1.0 - v) * self.height as f32) as usize;
        self.data[y * self.width + x]
    }
}