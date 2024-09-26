extern crate minifb;
extern crate nalgebra_glm as glm;

use color::Color;
use minifb::{Key, Window, WindowOptions};
use glm::Vec3;
use ray_intersect::{RayIntersect, Intersect};
use crate::{light::Light, camera::Camera,framebuffer::Framebuffer, material::Material, object::Sphere, render::render};

mod color;
mod framebuffer;
mod material;
mod object;
mod ray_intersect;
mod render;
mod camera;
mod light;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

     let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let light = Light::new(
        Vec3::new(10.0, 10.0, -10.0),
        Color::new(255, 255, 255),
        1.0,  // Full intensity
    );


    let red_material = Material {
        diffuse: Color::new(255, 0, 0),
    };

    let blue_material = Material {
        diffuse: Color::new(0, 0, 255),
    };

    let brown_material = Material {
        diffuse: Color::new(127, 79, 36),
    };

    let white_material = Material {
        diffuse: Color::new(255, 255, 255),
    };

    let black_material = Material {
        diffuse: Color::new(0, 0, 0),
    };

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Sphere {
            center: Vec3::new(-1.6, 1.5, -7.0),
            radius: 0.95,
            material: brown_material,
        }) as Box<dyn RayIntersect>,
        Box::new(Sphere {
            center: Vec3::new(1.6, 1.5, -7.0),
            radius: 0.95,
            material: brown_material,
        }) as Box<dyn RayIntersect>,
        Box::new(Sphere {
            center: Vec3::new(-1.5, 1.4, -6.0),
            radius: 0.5,
            material: white_material,
        }) as Box<dyn RayIntersect>,
        Box::new(Sphere {
            center: Vec3::new(1.5, 1.4, -6.0),
            radius: 0.5,
            material: white_material,
        }) as Box<dyn RayIntersect>,
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.5,
            material: brown_material,
        }) as Box<dyn RayIntersect>,
        Box::new(Sphere {
            center: Vec3::new(0.0, -0.5, -4.0),
            radius: 0.65,
            material: white_material,
        }) as Box<dyn RayIntersect>,
        Box::new(Sphere {
            center: Vec3::new(0.0, -0.1, -3.0),
            radius: 0.15,
            material: black_material,
        }) as Box<dyn RayIntersect>,
        Box::new(Sphere {
            center: Vec3::new(-0.25, 0.15, -2.0),
            radius: 0.08,
            material: black_material,
        }) as Box<dyn RayIntersect>,
        Box::new(Sphere {
            center: Vec3::new(0.25, 0.15, -2.0),
            radius: 0.08,
            material: black_material,
        }) as Box<dyn RayIntersect>,
    ];

    render(&mut framebuffer, &camera, &objects);

    let mut window = Window::new(
        "Raytraced Scene",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle input for camera movement
        if window.is_key_down(Key::W) {
            camera.orbit(0.0, 0.1); // Move camera up (pitch)
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, -0.1); // Move camera down (pitch)
        }
        if window.is_key_down(Key::A) {
            camera.orbit(-0.1, 0.0); // Rotate camera left (yaw)
        }
        if window.is_key_down(Key::D) {
            camera.orbit(0.1, 0.0); // Rotate camera right (yaw)
        }

        // Render the scene
        render(&mut framebuffer, &camera, &objects);

        window.update_with_buffer(framebuffer.get_buffer(), width, height).unwrap();
    }
}
