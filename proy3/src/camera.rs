extern crate nalgebra_glm as glm;
use glm::{Vec3, Mat4};

// Estructura para la c�mara
pub struct Camera {
    pub eye: Vec3,    // Posici�n de la c�mara
    pub center: Vec3, // Punto al que mira
    pub up: Vec3,     // Vector hacia arriba
}

impl Camera {
    // Constructor
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Camera {
        Camera { eye, center, up }
    }

    // Generar el rayo desde la c�mara
    pub fn generate_ray(&self, u: f32, v: f32) -> Vec3 {
        let forward = glm::normalize(&(self.center - self.eye));
        let right = glm::normalize(&glm::cross(&self.up, &forward));
        let up = glm::cross(&forward, &right);

        // Calculamos la direcci�n del rayo bas�ndonos en los valores u y v que van de 0 a 1
        let direction = forward + u * right + v * up;
        glm::normalize(&direction)
    }

    // Obtener la matriz de vista (necesaria para transformar la escena desde la perspectiva de la c�mara)
    pub fn get_view_matrix(&self) -> Mat4 {
        glm::look_at(&self.eye, &self.center, &self.up)
    }

    // M�todo para orbitar alrededor del centro
    pub fn orbit(&mut self, angle: f32, radius: f32) {
        // Usamos la f�rmula param�trica para obtener la posici�n en un c�rculo
        self.eye.x = self.center.x + radius * angle.cos();
        self.eye.z = self.center.z + radius * angle.sin();
        // self.eye.y se mantiene sin cambios para mantener la altura constante
    }
}