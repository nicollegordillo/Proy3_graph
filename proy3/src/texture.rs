extern crate image;

use image::{ImageReader, DynamicImage, GenericImageView};
use std::fmt;
use crate::color::Color;

#[derive(Clone)]
pub struct Texture {
    image: DynamicImage,
    pub width: usize,
    pub height: usize,
    color_array: Vec<Color>,
}

impl Texture {
    /// Creates a new Texture from the given file path.
    pub fn new(file_path: &str) -> Texture {
        let img = match ImageReader::open(file_path) {
            Ok(reader) => match reader.decode() {
                Ok(image) => image,
                Err(e) => {
                    println!("Error decoding image: {}", e);
                    return Texture::black(); // Use a black texture as a fallback
                }
            },
            Err(e) => {
                println!("Error opening image file: {}", e);
                return Texture::black(); // Use a black texture as a fallback
            }
        };

        let width = img.width() as usize;
        let height = img.height() as usize;

        let mut texture = Texture {
            image: img,
            width,
            height,
            color_array: vec![Color::black(); width * height],
        };

        texture.load_color_array();
        texture
    }

    /// Loads the color array from the image.
    /// Loads the color array from the image.
    fn load_color_array(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                // Get the pixel as Rgba<u8>
                let pixel = self.image.get_pixel(x as u32, y as u32);

                // Extract RGB channels
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                // Combine the channels into a color and store it
                let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                self.color_array[y * self.width + x] = Color::from_hex(color);
            }
        }
    }


    /// Returns the color at the specified pixel coordinates.
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            Color::from_hex(0xFF00FF) // Return magenta if out of bounds
        } else {
            self.color_array[y * self.width + x]
        }
    }

    /// Creates a black texture.
    pub fn black() -> Texture {
        let width = 1; // Width of 1 pixel
        let height = 1; // Height of 1 pixel

        let mut texture = Texture {
            image: DynamicImage::new_rgb8(width as u32, height as u32),
            width,
            height,
            color_array: vec![Color::new(0, 0, 0); width * height], // Black colors
        };

        texture.load_color_array(); // Load the black color
        texture
    }
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Texture")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
