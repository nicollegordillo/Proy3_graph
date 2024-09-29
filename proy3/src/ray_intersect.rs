use crate::material::Material;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone)]
pub struct Intersect {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}