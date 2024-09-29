#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: (self.r as f32 * rhs).min(255.0) as u8,
            g: (self.g as f32 * rhs).min(255.0) as u8,
            b: (self.b as f32 * rhs).min(255.0) as u8,
        }
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: (self.r as u16 + rhs.r as u16).min(255) as u8,
            g: (self.g as u16 + rhs.g as u16).min(255) as u8,
            b: (self.b as u16 + rhs.b as u16).min(255) as u8,
        }
    }
}