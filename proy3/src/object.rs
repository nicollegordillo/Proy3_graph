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

        // Calculate t_min and t_max for ray-box intersection
        let t_min = (min - ray_origin).component_div(ray_direction); // Adjusted for potential glm usage
        let t_max = (max - ray_origin).component_div(ray_direction); // Adjusted for potential glm usage

        // Find the maximum of t_min and the minimum of t_max
        let t_near = t_min[0].max(t_min[1]).max(t_min[2]);
        let t_far = t_max[0].min(t_max[1]).min(t_max[2]);

        if t_near > t_far || t_far < 0.0 {
            return None; // No intersection
        }

        // Calculate hit point
        let hit_point = ray_origin + ray_direction * t_near;

        // Determine the normal based on the face hit
        let normal = if t_near == t_min[0] {
            vec3(-1.0, 0.0, 0.0) // hit the left face
        } else if t_near == t_min[1] {
            vec3(0.0, -1.0, 0.0) // hit the bottom face
        } else if t_near == t_min[2] {
            vec3(0.0, 0.0, -1.0) // hit the back face
        } else if t_near == t_max[0] {
            vec3(1.0, 0.0, 0.0) // hit the right face
        } else if t_near == t_max[1] {
            vec3(0.0, 1.0, 0.0) // hit the top face
        } else {
            vec3(0.0, 0.0, 1.0) // hit the front face
        };

        // Calculate UV coordinates
        let (u, v) = calculate_cube_uv(&normal);

        Some(Intersect {
            distance: t_near,
            point: hit_point,
            normal,
            material: self.material.clone(),
            u,
            v,
        })
    }
}

// Helper function to calculate UV coordinates for cube faces
fn calculate_cube_uv(normal: &Vec3) -> (f32, f32) {
    let (u, v) = if normal.x.abs() > normal.y.abs() && normal.x.abs() > normal.z.abs() {
        // x is the dominant axis
        let u = (normal.z / normal.x + 1.0) * 0.5;
        let v = (normal.y / normal.x + 1.0) * 0.5;
        (u, v)
    } else if normal.y.abs() > normal.z.abs() {
        // y is the dominant axis
        let u = (normal.x / normal.y + 1.0) * 0.5;
        let v = (normal.z / normal.y + 1.0) * 0.5;
        (u, v)
    } else {
        // z is the dominant axis
        let u = (normal.x / normal.z + 1.0) * 0.5;
        let v = (normal.y / normal.z + 1.0) * 0.5;
        (u, v)
    };

    (u, v)
}

