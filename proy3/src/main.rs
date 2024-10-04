use material::{Material, TextureType};
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::{f32::consts::PI, time::Duration};
use rayon::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Arc;


mod framebuffer;
use framebuffer::Framebuffer;

mod color;
use color::Color;

mod ray_intersect;
use ray_intersect::Intersect;

mod material;

mod camera;
use camera::Camera;

mod object;
use object::Cube;

mod light;
use light::Light;

mod castray;
use castray::cast_ray;

mod texture;
use texture::Texture;

mod render;
use render::render;



fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height);

    let frame_delay = Duration::from_millis(0);

   
    let mut window = Window::new(
        "Raytracing",
        width,
        height,
        WindowOptions::default(),
    ).unwrap();

    window.set_position(100, 100);
    window.update();

    framebuffer.set_background_color(Color::new(179, 179, 179));

    
    let ice = Material::new_with_texture(
        30.0,
        [0.6, 0.2],
        0.0,
        0.0,
        0.0,
        TextureType::Ice
    );

    let birch = Material::new_with_texture(
        50.0,
        [0.6, 0.5],
        0.0,
        0.0,
        0.0,
        TextureType::Birch
    );

    let flower = Material::new_with_texture(
        40.0,
        [0.6, 0.3],
        0.0,
        0.0,
        0.0,
        TextureType::Flower
    );

    let magma = Material::new_with_texture(
        100.0,
        [0.7, 0.7],
        0.1,
        0.0,
        0.0,
        TextureType::Snow_top
    );

    let ffront = Material::new_with_texture(
        100.0,
        [0.7, 0.9],
        0.1,
        0.2,
        0.3,
        TextureType::Ffront
    );

    let fside = Material::new_with_texture(
        100.0,
        [0.7, 0.9],
        0.1,
        0.2,
        0.3,
        TextureType::Fside
    );

    let ftop = Material::new_with_texture(
        100.0,
        [0.7, 0.9],
        0.1,
        0.2,
        0.3,
        TextureType::Ftop
    );

    let sand = Material::new_with_texture(
        30.0,
        [0.8, 0.1],
        0.0,
        0.0,
        0.0,
        TextureType::Snow_top
    );
    let cube_size = 0.5;  // Tamaño del cubo
    let mut objects = Vec::new();
    let lights = vec![
      Light::new(Vec3::new(1.2*cube_size , -0.9, 4.4* cube_size), Color::new(220, 91, 2), 0.5), // Adjust Y to be higher*/
    ];
    

    for i in 0..4 { // Número de cubos en la dirección x (4 cubos)
        for j in 0..3 { // Número de cubos en la dirección z (3 cubos)
            objects.push(Cube {
                min: Vec3::new((i) as f32 * cube_size, -1.5, j as f32 * cube_size), // Vértice inferior izquierdo
                max: Vec3::new((i) as f32 * cube_size + cube_size, -1.0, j as f32 * cube_size + cube_size), // Vértice superior derecho
                material: sand.clone(),
            });
        }
    }
    for i in 0..2 { // Número de cubos en la dirección x (2 cubos)
        for j in 0..3 { // Número de cubos en la dirección z (3 cubos)
            objects.push(Cube {
                min: Vec3::new((i + 2) as f32 * cube_size, -1.5, (j + 3) as f32 * cube_size), // Vértice inferior izquierdo
                max: Vec3::new((i + 2) as f32 * cube_size + cube_size, -1.0, (j + 3) as f32 * cube_size + cube_size), // Vértice superior derecho
                material: sand.clone(),
            });
        }
    }
    for i in 0..2 { // Número de cubos en la dirección x (2 cubos)
        for j in 0..3 { // Número de cubos en la dirección z (3 cubos)
            objects.push(Cube {
                min: Vec3::new((i) as f32 * cube_size, -1.5, (j + 3) as f32 * cube_size), // Vértice inferior izquierdo
                max: Vec3::new((i) as f32 * cube_size + cube_size, -1.0, (j + 3) as f32 * cube_size + cube_size), // Vértice superior derecho
                material: ice.clone(),
            });
        }
    }
    for k in 0..3 { // Pile of 3 cubes
        objects.push(Cube {
            min: Vec3::new(2.0 * cube_size, -1.0 + (k as f32 * cube_size), 1.0 * cube_size), // Vértice inferior izquierdo
            max: Vec3::new(2.0 * cube_size + cube_size, -0.5 + (k as f32 * cube_size), 1.0 * cube_size + cube_size), // Vértice superior derecho
            material: birch.clone(), // or any material you want
        });
    }
    for i in 0..3 { // Número de cubos en la dirección x (4 cubos)
        for j in 0..3 { // Número de cubos en la dirección z (3 cubos)
            objects.push(Cube {
                min: Vec3::new((i+1) as f32 * cube_size, 0.5, j as f32 * cube_size), // Vértice inferior izquierdo
                max: Vec3::new((i+1) as f32 * cube_size + cube_size, 1.0, j as f32 * cube_size + cube_size), // Vértice superior derecho
                material: flower.clone(),
            });
        }
    }
    for j in 0..3 { // Adjusted range to only include the middle cube in z (1 cube)
        objects.push(Cube {
            min: Vec3::new(2.0*cube_size, 1.0, j as f32 * cube_size), // Vértice inferior izquierdo
            max: Vec3::new(3.0*cube_size, 1.5, j as f32 * cube_size + cube_size), // Vértice superior derecho
            material: flower.clone(),
        });
    }
    objects.push(Cube {
        min: Vec3::new(1.0*cube_size, 1.0, 1.0 * cube_size), // Vértice inferior izquierdo
        max: Vec3::new(2.0*cube_size, 1.5, 1.0 * cube_size + cube_size), // Vértice superior derecho
        material: flower.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(3.0*cube_size, 1.0, 1.0 * cube_size), // Vértice inferior izquierdo
        max: Vec3::new(4.0*cube_size, 1.5, 1.0 * cube_size + cube_size), // Vértice superior derecho
        material: flower.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(3.0*cube_size, -1.0, 5.0 * cube_size), // Vértice inferior izquierdo
        max: Vec3::new(3.0*cube_size, -0.5, 5.0 * cube_size + cube_size), // Vértice superior derecho
        material: ffront.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(3.0*cube_size, -1.0, 5.0 * cube_size), // Vértice inferior izquierdo
        max: Vec3::new(4.0*cube_size, -0.5, 5.0 * cube_size + cube_size), // Vértice superior derecho
        material: ftop.clone(),
    });
    

    

    let mut camera = Camera::new(
        Vec3::new(2.0,2.0,7.0),
        Vec3::new(1.5,1.0,0.0),
        Vec3::new(0.0,1.0,0.0), 
    );

    let mut daylight = Light::new(
        Vec3::new(4.0, 4.0, 7.0),
        Color::new(229, 156, 19),
        0.2, // Initial intensity
    );
    
    

    let rotaton_speed = PI/50.0;
    let zoom_speed = 0.1; 

    // Bucle principal
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::W) {
            camera.orbit(0.0, -rotaton_speed);
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, rotaton_speed);
        }
        if window.is_key_down(Key::A) {
            camera.orbit(rotaton_speed, 0.0);
        }
        if window.is_key_down(Key::D) {
            camera.orbit(-rotaton_speed, 0.0);
        }

        if window.is_key_down(Key::Up) {
            camera.zoom(zoom_speed); 
        }
        if window.is_key_down(Key::Down) {
            camera.zoom(-zoom_speed);  
        }

        // Adjust light intensity with key presses
        if window.is_key_down(Key::Right) {
            daylight.set_intensity((daylight.intensity + 0.1).min(2.0)); // Increase intensity
        }
        if window.is_key_down(Key::Left) {
            daylight.set_intensity((daylight.intensity - 0.1).max(0.0)); // Decrease intensity
        }


        framebuffer.clear();

        render(
            &mut framebuffer,
            &objects,
            &mut camera,
            &lights,
            &daylight
        );

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

