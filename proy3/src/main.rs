extern crate minifb;
extern crate nalgebra_glm as glm;

use color::Color;
use minifb::{Key, Window, WindowOptions};
use glm::Vec3;
use ray_intersect::{RayIntersect, Intersect};
use crate::{framebuffer::Framebuffer, material::Material, object::Sphere, render::render};
use camera::Camera;

mod color;
mod framebuffer;
mod material;
mod object;
mod ray_intersect;
mod render;
mod camera;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Posiciones iniciales de la cámara
    let eye = Vec3::new(0.0, 2.0, -5.0); // Posición inicial de la cámara
    let center = Vec3::new(0.0, 0.0, 0.0); // Centro de la escena
    let up = Vec3::new(0.0, 1.0, 0.0); // Vector hacia arriba

    // Creamos la cámara
    let mut camera = Camera::new(eye, center, up);

    let red_material = Material {
        diffuse: Color::new(255, 0, 0),
    };

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Sphere {
            center: Vec3::new(-1.6, 1.5, -7.0),
            radius: 0.95,
            material: red_material,
        }),
        // Otros objetos de la escena...
    ];

    let mut angle = 0.0;
    let radius = 5.0;

    let mut window = Window::new(
        "Raytraced Scene",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Bucle de renderizado
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Girar la cámara alrededor del centro de la escena
        angle += 0.02; // Controlar la velocidad de rotación
        camera.orbit(angle, radius);

        // Generar rayos y renderizar la escena
        render(&mut framebuffer, &objects);

        // Actualizar la ventana con el buffer
        window.update_with_buffer(framebuffer.get_buffer(), width, height).unwrap();
    }
}

