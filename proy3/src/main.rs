mod camera;
mod color;
mod framebuffer;
mod light;
mod material;
mod object;
mod ray_intersect;
mod render;

use minifb::{Key, Window, WindowOptions};
use std::vec::Vec;
use framebuffer::Framebuffer;
use light::Light;
use nalgebra_glm::Vec3;
use object::Cube;
use material::Material;
use render::render_scene;
use color::Color;
use camera::Camera; // Import the Camera module

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Set up the camera
    let camera_eye = Vec3::new(0.0, 0.0, 0.0);
    let camera_center = Vec3::new(0.0, 0.0, -1.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let mut camera = Camera::new(camera_eye, camera_center, camera_up);

    // Define light
    let light = Light {
        position: Vec3::new(10.0, 10.0, -10.0),
        color: Color::new(255, 255, 255),
        intensity: 1.0,
    };

    // Define materials
    let material1 = Material {
        diffuse: Color::new(255, 0, 0),
        albedo: 0.9,
        specular_exponent: 50.0,
        refraction_index: None,
        texture: None,
    };

    let material2 = Material {
        diffuse: Color::new(0, 255, 0),
        albedo: 0.9,
        specular_exponent: 50.0,
        refraction_index: Some(1.5),
        texture: None,
    };

    let cube1 = Cube {
        center: Vec3::new(0.0, 0.0, -5.0),
        size: 2.0,
        material: material1,
    };

    let cube2 = Cube {
        center: Vec3::new(2.0, 0.0, -7.0),
        size: 3.0,
        material: material2,
    };

    let objects: Vec<Cube> = vec![cube1, cube2]; // Pass cubes to your rendering logic


    // Render the scene
    for y in 0..height {
        for x in 0..width {
            // Calculate normalized device coordinates (u, v)
            let u = (x as f32 + 0.5) / width as f32 * 2.0 - 1.0; // Normalize to [-1, 1]
            let v = 1.0 - (y as f32 + 0.5) / height as f32 * 2.0; // Normalize to [-1, 1]

            // Generate ray from camera
            let ray_direction = camera.generate_ray(u, v);
            let ray_origin = camera.eye;

            // Render the scene
            render_scene(ray_origin, ray_direction, &mut framebuffer, &objects, &light);
        }
    }

    // Create the window for displaying the framebuffer
    let mut window = Window::new("Ray Tracing - Press ESC to exit", width, height, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer = framebuffer.to_u32();
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

