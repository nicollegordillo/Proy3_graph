use nalgebra_glm::Vec3;
use crate::{material::Material, ray_intersect::{Intersect, RayIntersect}};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material, // Ensure this is the same Material type
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = self.center - *ray_origin;
        let tca = oc.dot(ray_direction);
        let d2 = oc.dot(&oc) - tca * tca;
        let radius2 = self.radius * self.radius;

        if d2 > radius2 {
            return Intersect::empty(); // No intersection
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        // We need the closest positive t value
        let distance = if t0 > 0.0 {
            t0
        } else if t1 > 0.0 {
            t1
        } else {
            return Intersect::empty(); // Both t0 and t1 are negative
        };

        // Calculate the intersection point and normal
        let intersect_point = *ray_origin + distance * ray_direction;
        let normal = (intersect_point - self.center).normalize();

        Intersect::new(intersect_point, normal, distance, self.material) // Ensure this is the correct Material type
    }
}