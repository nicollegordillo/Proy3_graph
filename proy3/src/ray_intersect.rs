use nalgebra_glm::Vec3;
use crate::{color::Color, material::Material}; // Import the Material type from the correct module

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub material: Material, // Ensure this matches the Material type
    pub is_intersecting: bool,
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material) -> Self {
        Intersect {
            point,
            normal,
            distance,
            material,
            is_intersecting: true,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            material: Material {
                diffuse: Color::new(0, 0, 0),
            },
            is_intersecting: false,
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}