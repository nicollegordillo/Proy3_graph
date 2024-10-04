use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::material::Material;

/// Structure representing the result of a ray-object intersection.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Intersect {
    pub point: Vec3,          // Intersection point
    pub normal: Vec3,         // Normal at the intersection
    pub distance: f32,        // Distance from the ray origin to the intersection
    pub is_intersecting: bool, // Flag indicating if an intersection occurred
    pub material: Material,    // Material of the intersected object
    pub uv: (f32, f32),       // Texture coordinates
}

impl Intersect {
    /// Constructs a new `Intersect` instance for a successful intersection.
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material, uv: (f32, f32)) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
            uv,
        }
    }

    /// Creates an empty `Intersect` instance, indicating no intersection.
    pub fn empty() -> Self {
        Intersect {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            distance: 0.0,
            is_intersecting: false,
            material: Material::new(
                Color::new(0, 0, 0),
                0.0,
                [0.0, 0.0],
                0.0,
                0.0,
                0.0,
            ),
            uv: (0.0, 0.0),
        }
    }
}

/// Trait for ray intersection functionality.
pub trait RayIntersect {
    /// Computes the intersection of a ray with the implementing object.
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}
