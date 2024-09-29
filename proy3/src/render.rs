use crate::color::Color;
use crate::light::Light;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::object::Cube;
use nalgebra_glm::Vec3;

pub fn render_scene(ray_origin: Vec3, ray_direction: Vec3, objects: &[Cube], light: &Light) -> Color {
    if let Some(closest_intersect) = find_closest_intersect(&ray_origin, &ray_direction, objects) {
        let material = &closest_intersect.material;
        let light_dir = (light.position - closest_intersect.point).normalize();
        let view_dir = (ray_origin - closest_intersect.point).normalize();
        let reflect_dir = reflect(&(-light_dir), &closest_intersect.normal);

        // Phong shading
        let diffuse = light.intensity * closest_intersect.normal.dot(&light_dir).max(0.0);
        let specular = light.intensity * reflect_dir.dot(&view_dir).max(0.0).powf(material.specular_exponent);

        // Apply color
        let color = material.diffuse * material.albedo * diffuse + material.diffuse * specular;

        // Shadow check
        if is_in_shadow(&closest_intersect, light, objects) {
            //println!("Object is in shadow");
            return color * 0.3; // Darken color in shadow
        }

        color
    } else {
        //println!("No intersection found");
        Color::new(100, 100, 255) // Background color
    }
}

fn find_closest_intersect(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube]) -> Option<Intersect> {
    objects.iter()
        .filter_map(|object| {
            let intersect = object.ray_intersect(ray_origin, ray_direction);
            if intersect.is_some() {
                //println!("Intersection found: {:?}", intersect);
            }
            intersect
        })
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
}

fn reflect(direction: &Vec3, normal: &Vec3) -> Vec3 {
    direction - normal * 2.0 * direction.dot(normal)
}

fn is_in_shadow(intersect: &Intersect, light: &Light, objects: &[Cube]) -> bool {
    let shadow_origin = intersect.point + intersect.normal * 0.001;
    let light_dir = (light.position - intersect.point).normalize();

    objects.iter()
        .filter_map(|object| object.ray_intersect(&shadow_origin, &light_dir))
        .any(|shadow_intersect| shadow_intersect.distance < (light.position - intersect.point).magnitude())
}
