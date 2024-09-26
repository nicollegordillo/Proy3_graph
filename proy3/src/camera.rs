extern crate nalgebra_glm as glm;
use glm::{Vec3, normalize};

pub struct Camera {
    pub eye: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub yaw: f32, // Added for yaw
    pub pitch: f32, // Added for pitch
}

impl Camera {
    // Create a new camera with position, look-at target, and up vector
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Camera {
        let forward = normalize(&(center - eye));
        let right = normalize(&glm::cross(&forward, &up));
        let up = glm::cross(&right, &forward);

        let fov = glm::half_pi::<f32>();

        Camera {
            eye,
            forward,
            right,
            up,
            fov,
            yaw: 0.0, // Initialize yaw
            pitch: 0.0, // Initialize pitch
        }
    }

    // Generate a ray for a pixel, given normalized coordinates (u, v)
    pub fn generate_ray(&self, u: f32, v: f32) -> Vec3 {
        // Adjust the field of view by scaling right and up vectors
        let aspect_ratio = 1.0; // Assuming square aspect ratio
        let scale = (self.fov * 0.5).tan();

        // Calculate ray direction in camera space
        let ray_direction = self.forward
            + self.right * (u * scale * aspect_ratio)
            + self.up * (v * scale);

        // Return normalized ray direction
        normalize(&ray_direction)
    }

    // Optional: Orbit camera around the scene (for animation)
     pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;

        // Clamp pitch to avoid gimbal lock
        if self.pitch > std::f32::consts::FRAC_PI_2 - 0.1 {
            self.pitch = std::f32::consts::FRAC_PI_2 - 0.1;
        } else if self.pitch < -std::f32::consts::FRAC_PI_2 + 0.1 {
            self.pitch = -std::f32::consts::FRAC_PI_2 + 0.1;
        }

        // Calculate new camera direction
        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();

        let new_forward = Vec3::new(
            cos_pitch * sin_yaw,
            sin_pitch,
            cos_pitch * cos_yaw
        );

        self.forward = normalize(&new_forward);
        self.right = normalize(&glm::cross(&self.forward, &self.up));
        self.up = glm::cross(&self.right, &self.forward);

        // Update camera position
        self.eye = self.forward * 10.0; // Assume some distance from the origin
    }
}
