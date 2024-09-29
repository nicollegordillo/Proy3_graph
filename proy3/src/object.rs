use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use nalgebra_glm::{Vec3, vec3};

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let half_size = self.size / 2.0;
        let min = self.center - vec3(half_size, half_size, half_size);
        let max = self.center + vec3(half_size, half_size, half_size);

        let epsilon = 1e-6;
        let inv_dir = Vec3::new(1.0 / ray_direction.x, 1.0 / ray_direction.y, 1.0 / ray_direction.z);


        // Calculate the potential intersections
        let t_min = (min - ray_origin).component_mul(&inv_dir);
        let t_max = (max - ray_origin).component_mul(&inv_dir);

        // Find the largest t_min and smallest t_max
        let t_near = t_min.x.max(t_min.y).max(t_min.z);
        let t_far = t_max.x.min(t_max.y).min(t_max.z);

        if t_near > t_far || t_far < 0.0 {
            return None; // No intersection
        }

        // Determine hit point and normal
        let hit_point = ray_origin + ray_direction * t_near;

        // Calculate the normal
        let normal = calculate_normal(&hit_point, &min, &max);
        //println!("Hit normal: {:?}", normal);

        // Debug output
        //println!("Hit point: {:?}", hit_point);
        //println!("Normal: {:?}", normal);

        Some(Intersect {
            distance: t_near,
            point: hit_point,
            normal,
            material: self.material.clone(),
            u: 0.0,
            v: 0.0,
        })
    }
}


fn calculate_normal(hit_point: &Vec3, min: &Vec3, max: &Vec3) -> Vec3 {
    let center = (min + max) * 0.5;
    let diff = hit_point - center;

    if diff.x.abs() > diff.y.abs() && diff.x.abs() > diff.z.abs() {
        if diff.x > 0.0 {
            vec3(1.0, 0.0, 0.0)
        } else {
            vec3(-1.0, 0.0, 0.0)
        }
    } else if diff.y.abs() > diff.x.abs() && diff.y.abs() > diff.z.abs() {
        if diff.y > 0.0 {
            vec3(0.0, 1.0, 0.0)
        } else {
            vec3(0.0, -1.0, 0.0)
        }
    } else {
        if diff.z > 0.0 {
            vec3(0.0, 0.0, 1.0)
        } else {
            vec3(0.0, 0.0, -1.0)
        }
    }
}
