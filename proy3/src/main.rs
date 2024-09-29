mod camera;
mod color;
mod framebuffer;
mod light;
mod material;
mod object;
mod ray_intersect;
mod render;

use framebuffer::Framebuffer;
use light::Light;
use nalgebra_glm::Vec3;
use object::Cube;
use material::Material;
use render::render_scene;
use color::Color;
use camera::Camera;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Camera setup
    let camera_eye = Vec3::new(0.0, 0.0, -5.0);
    let camera_center = Vec3::new(0.0, 0.0, -1.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let mut camera = Camera::new(camera_eye, camera_center, camera_up);

    // Light setup
    let light = Light {
        position: Vec3::new(0.0, 0.0, -4.0),
        color: Color::new(255, 255, 255),
        intensity: 1.0,
    };

    let green_material = Material {
        diffuse: Color::new(0, 255, 0),
        albedo: 0.9,
        specular_exponent: 50.0,
        refraction_index: Some(1.5),
        texture: None,
    };

    // Objects (cubes)
    let objects = vec![
        Cube {
            center: Vec3::new(0.0, 0.0, 0.0),
            size: 2.0,
            material: Material {
                diffuse: Color::new(255, 0, 0),
                albedo: 0.9,
                specular_exponent: 50.0,
                refraction_index: None,
                texture: None,
            },
        },
        Cube {
            center: Vec3::new(-2.0, 0.0, 0.0),
            size: 2.0,
            material: Material {
                diffuse: Color::new(255, 100, 250),
                albedo: 0.8,
                specular_exponent: 50.0,
                refraction_index: None,
                texture: None,
            },
        },
        Cube {
            center: Vec3::new(2.0, 0.0, 0.0),
            size: 2.0,
            material: green_material,
        },
    ];

    /*for object in &objects {
        object.print_corner_coordinates();
    }*/

    // Create the window
    let mut window = Window::new("Ray Tracing - Press ESC to exit", width, height, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    // Camera movement variables
    let move_speed = 0.5;
    let rotate_speed = 1.5;


    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle camera movement input
        if window.is_key_down(Key::W) {
            camera.eye += camera.forward * move_speed; // Move forward
        }
        if window.is_key_down(Key::S) {
            camera.eye -= camera.forward * move_speed; // Move backward
        }
        if window.is_key_down(Key::A) {
            camera.eye -= camera.right * move_speed; // Strafe left
        }
        if window.is_key_down(Key::D) {
            camera.eye += camera.right * move_speed; // Strafe right
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, rotate_speed); // Look up (pitch)
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, -rotate_speed); // Look down (pitch)
        }
        if window.is_key_down(Key::Left) {
            camera.orbit(-rotate_speed, 0.0); // Look left (yaw)
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(rotate_speed, 0.0); // Look right (yaw)
        }

        // Rendering loop
        for y in 0..height {
            for x in 0..width {
                let u = (x as f32 + 0.5) / width as f32 * 2.0 - 1.0;
                let v = 1.0 - (y as f32 + 0.5) / height as f32 * 2.0;

                let ray_direction = camera.generate_ray(u, v);
                let ray_origin = camera.eye;

                let color = render_scene(ray_origin, ray_direction, &objects, &light);
                framebuffer.set_pixel(x, y, color);
            }
        }

        // Update the window with the framebuffer content
        window.update_with_buffer(&framebuffer.to_u32_buffer(), width, height).unwrap();
    }
}
