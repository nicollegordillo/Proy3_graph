use crate::{color::Color, framebuffer::Framebuffer, ray_intersect::{Intersect, RayIntersect}};
use nalgebra_glm::{Vec3, normalize};

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            // Convert Color to u32 and set the pixel color
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