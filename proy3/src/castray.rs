use nalgebra_glm::Vec3;
use std::f32::INFINITY;
use crate::{Intersect, Light, Cube, Color, ray_intersect::RayIntersect, Texture};
use std::sync::Arc;

const ORIGIN_BIAS: f32 = 1e-4;

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    let (n_cosi, eta, n_normal) = if cosi < 0.0 {
        (-cosi, 1.0 / eta_t, -normal)
    } else {
        (cosi, eta_t, *normal)
    };
    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow(intersect: &Intersect, light: &Light, objects: &[Cube]) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = offset_origin(intersect, &light_dir);
    let light_distance = (light.position - shadow_ray_origin).magnitude();
    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            return 1.0;
        }
    }
    0.0
}

pub fn cast_ray(
    ray_origin: &Vec3, 
    ray_direction: &Vec3, 
    objects: &[Cube], 
    daylight: &Light, 
    other_lights: &[Light], 
    textures: &[Arc<Texture>],  // Added textures array here
    depth: u32
) -> Color {
    if depth > 3 {
        return Color::new(179, 179, 179);
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(&ray_origin, &ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        return calculate_background_color(daylight);
    }

    let calculate_light_intensity = |light: &Light| {
        let light_dir = (light.position - intersect.point).normalize();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);
        
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0);
        
        // Now passing the `textures` array to get the diffuse color from the texture
        let diffuse_color = intersect.material.get_diffuse_color(textures, intersect.uv.0, intersect.uv.1);
        
        let diffuse = ((light.color * 0.09) + diffuse_color) * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.spec);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

        diffuse + specular
    };

    let mut total_light = calculate_light_intensity(daylight);
    for light in other_lights {
        total_light = total_light + calculate_light_intensity(light);
    }

    let reflectivity = intersect.material.reflectivity;
    let transparency = intersect.material.transparency;

    let mut reflect_color = Color::black();
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = intersect.point + intersect.normal * 0.001;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, daylight, other_lights, textures, depth + 1);  // Added textures
    }

    let mut refract_color = Color::black();
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refraction_index).normalize();
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, daylight, other_lights, textures, depth + 1);  // Added textures
    }

    total_light * (1.0 - reflectivity - transparency) + (reflect_color * reflectivity) + (refract_color * transparency)
}

fn calculate_background_color(daylight: &Light) -> Color {
    let base_blue = Color::new(135, 206, 250);
    let intensity_factor = daylight.intensity;
    Color::new(
        ((base_blue.r as f32 * intensity_factor).min(255.0)) as i32,
        ((base_blue.g as f32 * intensity_factor).min(255.0)) as i32,
        ((base_blue.b as f32 * intensity_factor).min(255.0)) as i32,
    )
}





