use nalgebra_glm::Vec3;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::material::Material;

/// Structure representing a cube in 3D space.
#[derive(Clone)]
pub struct Cube {
    pub min: Vec3,      // Minimum point of the cube (lower-left vertex)
    pub max: Vec3,      // Maximum point of the cube (upper-right vertex)
    pub material: Material, // Material of the cube
}

impl RayIntersect for Cube {
    /// Checks if a ray intersects with the cube and returns intersection details.
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let mut tmin = (self.min.x - ray_origin.x) / ray_direction.x;
        let mut tmax = (self.max.x - ray_origin.x) / ray_direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - ray_origin.y) / ray_direction.y;
        let mut tymax = (self.max.y - ray_origin.y) / ray_direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty(); // No intersection
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray_origin.z) / ray_direction.z;
        let mut tzmax = (self.max.z - ray_origin.z) / ray_direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty(); // No intersection
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        // If tmin is positive, there's an intersection in the direction of the ray
        if tmin > 0.0 {
            let point = ray_origin + ray_direction * tmin;
            let normal = self.calculate_normal(&point); // Calculate the normal at the intersection
            let distance = tmin;

            // Calculate UV coordinates
            let (u, v) = self.calculate_uv(&point);
            
            return Intersect::new(point, normal, distance, self.material.clone(), (u, v));
        }

        Intersect::empty() // No valid intersection
    }
}

impl Cube {
    /// Calculates the normal at the intersection point.
    fn calculate_normal(&self, point: &Vec3) -> Vec3 {
        let epsilon = 1e-4; // Small value for precision

        if (point.x - self.min.x).abs() < epsilon {
            return Vec3::new(-1.0, 0.0, 0.0); // Left face
        } else if (point.x - self.max.x).abs() < epsilon {
            return Vec3::new(1.0, 0.0, 0.0); // Right face
        } else if (point.y - self.min.y).abs() < epsilon {
            return Vec3::new(0.0, -1.0, 0.0); // Bottom face
        } else if (point.y - self.max.y).abs() < epsilon {
            return Vec3::new(0.0, 1.0, 0.0); // Top face
        } else if (point.z - self.min.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, -1.0); // Back face
        } else if (point.z - self.max.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, 1.0); // Front face
        }

        Vec3::new(0.0, 0.0, 0.0) // Default normal if no face matches
    }

    /// Calculates the UV texture coordinates at the intersection point.
    fn calculate_uv(&self, point: &Vec3) -> (f32, f32) {
        let epsilon = 1e-4;

        if (point.x - self.min.x).abs() < epsilon {
            // Left face (negative X axis)
            let u = (point.z - self.min.z) / (self.max.z - self.min.z);
            let v = (self.max.y - point.y) / (self.max.y - self.min.y); 
            return (u, v);
        } else if (point.x - self.max.x).abs() < epsilon {
            // Right face (positive X axis)
            let u = (point.z - self.min.z) / (self.max.z - self.min.z);
            let v = (self.max.y - point.y) / (self.max.y - self.min.y); 
            return (u, v);
        } else if (point.y - self.min.y).abs() < epsilon {
            // Bottom face (negative Y axis)
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.z - self.min.z) / (self.max.z - self.min.z);
            return (u, v);
        } else if (point.y - self.max.y).abs() < epsilon {
            // Top face (positive Y axis)
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.z - self.min.z) / (self.max.z - self.min.z);
            return (u, v);
        } else if (point.z - self.min.z).abs() < epsilon {
            // Back face (negative Z axis)
            let u = (self.max.x - point.x) / (self.max.x - self.min.x);
            let v = (self.max.y - point.y) / (self.max.y - self.min.y);
            return (u, v);
        } else {
            // Front face (positive Z axis)
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (self.max.y - point.y) / (self.max.y - self.min.y);
            return (u, v);
        }

        (0.0, 0.0) // Default UV if no face is matched
    }
}

