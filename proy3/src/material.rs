use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::color::Color;
use crate::texture::Texture;

// Initializing static textures
static TEXTURES: Lazy<Vec<Arc<Texture>>> = Lazy::new(|| {
    vec![
        Arc::new(Texture::new("./src/imagenes/ice.webp")),
        Arc::new(Texture::new("./src/imagenes/Birch.webp")),
        Arc::new(Texture::new("./src/imagenes/Flower.webp")),
        Arc::new(Texture::new("./src/imagenes/Furnace_front.png")),
        Arc::new(Texture::new("./src/imagenes/Snow_top.webp")),
        Arc::new(Texture::new("./src/imagenes/Furnace_side.webp")),
        Arc::new(Texture::new("./src/imagenes/Furnace_top.webp")),
    ]
});

#[derive(Debug, Clone)]
pub enum TextureType {
    Ice,
    Birch,
    Flower,
    Ffront,
    Snow_top,
    Fside,
    Ftop,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub spec: f32,
    pub albedo: [f32; 2],
    pub reflectivity: f32,
    pub transparency: f32,
    pub refraction_index: f32,
    pub has_texture: bool,
    pub texture_index: usize, // Stores the texture index
}

impl Material {
    // Create a new material without texture
    pub fn new(
        diffuse: Color,
        spec: f32,
        albedo: [f32; 2],
        reflectivity: f32,
        transparency: f32,
        refraction_index: f32,
    ) -> Self {
        Self {
            diffuse,
            spec,
            albedo,
            reflectivity,
            transparency,
            refraction_index,
            has_texture: false,
            texture_index: 0, // Default to 0, no texture initially
        }
    }

    // Create a new material with a specific texture
    pub fn new_with_texture(
        spec: f32,
        albedo: [f32; 2],
        reflectivity: f32,
        transparency: f32,
        refraction_index: f32,
        texture_type: TextureType, // Only one texture type
    ) -> Self {
        let texture_index = match texture_type {
            TextureType::Ice => 0,
            TextureType::Birch => 1,
            TextureType::Flower => 2,
            TextureType::Ffront => 3,
            TextureType::Snow_top => 4,
            TextureType::Fside => 5,
            TextureType::Ftop => 6,
        };

        Self {
            diffuse: Color::new(0, 0, 0),
            spec,
            albedo,
            reflectivity,
            transparency,
            refraction_index,
            has_texture: true,
            texture_index, // Store the texture index
        }
    }

    // Get the color of the active texture
    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if self.has_texture {
            let texture = &TEXTURES[self.texture_index]; // Use texture by index
            let x = (u * (texture.width as f32 - 1.0)) as usize;
            let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
            return texture.get_color(x, y);
        }
        self.diffuse // Fallback to diffuse color if no texture
    }

    pub fn black() -> Self {
        Self {
            diffuse: Color::new(0, 0, 0),
            spec: 0.0,
            albedo: [0.0, 0.0],
            reflectivity: 0.0,
            transparency: 0.0,
            refraction_index: 0.0,
            has_texture: false,
            texture_index: 0, // Default to 0, no texture
        }
    }
}

