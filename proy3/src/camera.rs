use std::f32::consts::PI;
use nalgebra_glm::Vec3;

pub struct Camera {
    pub eye: Vec3,    // Camera position
    pub center: Vec3, // Target the camera is looking at
    pub up: Vec3,     // Up direction of the camera
}

impl Camera {
    // Creates a new camera instance with given eye, center, and up vectors
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera { eye, center, up }
    }

    // Transforms a vector to camera's basis
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize(); // Forward direction
        let right = forward.cross(&self.up).normalize();    // Right direction
        let up = right.cross(&forward).normalize();         // Up direction

        // Transform vector to camera basis and normalize
        let rotated = vector.x * right + vector.y * up - vector.z * forward;
        rotated.normalize()
    }

    // Orbits the camera around the center point based on yaw and pitch angles
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();  // Distance from eye to center

        // Calculate current yaw and pitch angles
        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Apply new yaw and pitch adjustments
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        // Calculate new camera position (eye)
        self.eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos(),
        );
    }

    // Adjusts the camera's zoom by moving the eye closer or further from the center
    pub fn zoom(&mut self, zoom_factor: f32) {
        let direction = (self.center - self.eye).normalize(); // Direction from eye to center
        self.eye += direction * zoom_factor;                  // Move eye along the direction vector
    }
}
