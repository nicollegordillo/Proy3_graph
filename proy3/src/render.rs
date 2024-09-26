use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::ray_intersect::{Intersect, RayIntersect};
use nalgebra_glm::{Vec3, normalize};
use crate::object::Cube;

pub fn render_scene(ray_origin: Vec3, ray_direction: Vec3, framebuffer: &mut Framebuffer, objects: &[Cube], light: &Light) -> Option<Intersect> {
    let mut closest_intersect: Option<Intersect> = None;

    for object in objects {
        if let Some(intersect) = object.ray_intersect(&ray_origin, &ray_direction) {
            // Use as_ref() to avoid moving closest_intersect
            if closest_intersect.is_none() || intersect.distance < closest_intersect.as_ref().map_or(f32::MAX, |ci| ci.distance) {
                closest_intersect = Some(intersect);
            }
        }
    }

    closest_intersect
}

fn find_closest_intersect(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Box<dyn RayIntersect>]) -> Option<Intersect> {
    let mut closest_intersect: Option<Intersect> = None;

    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            // Use as_ref() to avoid moving closest_intersect
            if closest_intersect.is_none() || intersect.distance < closest_intersect.as_ref().map_or(f32::MAX, |ci| ci.distance) {
                closest_intersect = Some(intersect);
            }
        }
    }

    closest_intersect
}

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Box<dyn RayIntersect>], light: &Light, depth: u32) -> Color {
    if depth > 4 {
        return Color::new(0, 0, 0);
    }

    if let Some(closest_intersect) = find_closest_intersect(ray_origin, ray_direction, objects) {
        let mut color = closest_intersect.material.diffuse;

        let light_dir = (light.position - closest_intersect.point).normalize();
        let view_dir = (ray_origin - closest_intersect.point).normalize();
        let reflect_dir = reflect(&(-light_dir), &closest_intersect.normal);

        // Calculate diffuse and specular components
        let diffuse = light.intensity * f32::max(0.0, closest_intersect.normal.dot(&light_dir));
        let specular = light.intensity * f32::powf(f32::max(0.0, reflect_dir.dot(&view_dir)), closest_intersect.material.specular_exponent);

        // Apply texture if present
        if let Some(texture) = closest_intersect.material.texture {
            color = texture.get_color(closest_intersect.u, closest_intersect.v);
        }

        // Calculate final color with Phong model
        color = color * closest_intersect.material.albedo * diffuse + color * specular;

        // Shadow ray
        let shadow_ray_origin = closest_intersect.point + closest_intersect.normal * 0.001;
        let shadow_ray_dir = light_dir;
        if let Some(shadow_intersect) = find_closest_intersect(&shadow_ray_origin, &shadow_ray_dir, objects) {
            if shadow_intersect.distance < (light.position - closest_intersect.point).magnitude() {
                color = color * 0.3; // Darken the color if in shadow
            }
        }

        return color;
    }

    // Return background color
    Color::new(100, 100, 255) // Background sky color
}


fn reflect(direction: &Vec3, normal: &Vec3) -> Vec3 {
    direction - normal * 2.0 * direction.dot(normal)
}

