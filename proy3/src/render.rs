use crate::{camera::Camera, color::Color, framebuffer::Framebuffer, ray_intersect::{Intersect, RayIntersect}};
use nalgebra_glm::Vec3;

pub fn render(framebuffer: &mut Framebuffer, camera: &Camera, objects: &[Box<dyn RayIntersect>]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Normalized device coordinates (NDC) from -1 to 1
            let u = (2.0 * (x as f32 + 0.5) / width - 1.0);
            let v = (1.0 - 2.0 * (y as f32 + 0.5) / height);

            // Use the camera to generate the ray direction for this pixel
            let ray_direction = camera.generate_ray(u, v);

            // The ray originates from the camera's position
            let ray_origin = camera.eye;

            // Trace the ray and get the pixel color
            let pixel_color = cast_ray(&ray_origin, &ray_direction, objects);

            // Convert the Color to u32 and set the pixel color
            framebuffer.point(x, y, pixel_color.to_u32());
        }
    }
}

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Box<dyn RayIntersect>]) -> Color {
    let mut closest_intersect = Intersect::empty();
    let mut min_distance = f32::INFINITY;

    for object in objects.iter() {
        let intersect = object.ray_intersect(ray_origin, ray_direction);
        if intersect.is_intersecting && intersect.distance < min_distance {
            min_distance = intersect.distance;
            closest_intersect = intersect;
        }
    }

    if closest_intersect.is_intersecting {
        closest_intersect.material.diffuse
    } else {
        Color::new(0, 0, 0) // Background color (black)
    }
}
