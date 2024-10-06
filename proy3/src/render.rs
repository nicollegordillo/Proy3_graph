use crate::Framebuffer;
use crate::Cube;
use crate::Camera;
use crate::Light;
use std::{f32::consts::PI};
use rayon::prelude::*;
use crate::cast_ray;
use nalgebra_glm::Vec3;
use std::sync::Arc;
use crate::Texture;

/// Renders the scene to the framebuffer.
pub fn render(
    framebuffer: &mut Framebuffer, 
    objects: &[Cube], 
    camera: &Camera, 
    lights: &[Light], 
    daylight: &Light, 
    textures: &[Arc<Texture>]  // Added textures argument
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0; // Field of view
    let perspective_scale = (fov / 2.0).tan(); // Perspective scale based on FOV

    framebuffer.buffer.par_chunks_mut(framebuffer.width as usize).enumerate().for_each(|(y, row)| {
        let screen_y = -(2.0 * y as f32) / height + 1.0; // Transform to normalized device coordinates
        let screen_y = screen_y * perspective_scale; // Scale for perspective

        row.iter_mut().enumerate().for_each(|(x, pixel)| {
            let screen_x = (2.0 * x as f32) / width - 1.0; // Transform to normalized device coordinates
            let screen_x = screen_x * aspect_ratio * perspective_scale; // Scale for aspect ratio

            // Construct the ray direction in camera space
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();

            // Rotate the ray direction based on the camera's orientation
            let rotated_direction = camera.basis_change(&ray_direction);

            // Cast the ray from the camera's position in the direction of the rotated ray
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, daylight, lights, textures, 0);  // Pass textures here
            *pixel = pixel_color.to_u32(); // Store the color in the framebuffer
        });
    });
}


