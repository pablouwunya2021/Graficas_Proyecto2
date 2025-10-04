use crate::color::Color;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,           // Color base (albedo)
    pub specular: f32,            // Intensidad especular (0.0 - 1.0)
    pub albedo: [f32; 2],         // [difuso, especular]
    pub refractive_index: f32,    // Índice de refracción (1.0 = aire, 1.33 = agua, 1.5 = vidrio)
    pub transparency: f32,        // Transparencia (0.0 = opaco, 1.0 = transparente)
    pub reflectivity: f32,        // Reflectividad (0.0 = no refleja, 1.0 = espejo)
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 2],
        refractive_index: f32,
        transparency: f32,
        reflectivity: f32,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            transparency,
            reflectivity,
        }
    }

    // Material de césped (grass)
    pub fn grass() -> Self {
        Material::new(
            Color::new(0.4, 0.8, 0.2),  // Verde césped
            10.0,
            [0.9, 0.1],
            1.0,
            0.0,    // Opaco
            0.0,    // No refleja
        )
    }

    // Material de tierra (dirt)
    pub fn dirt() -> Self {
        Material::new(
            Color::new(0.55, 0.4, 0.25),  // Café tierra
            5.0,
            [0.95, 0.05],
            1.0,
            0.0,    // Opaco
            0.0,    // No refleja
        )
    }

    // Material de madera de cerezo (cherry wood)
    pub fn cherry_wood() -> Self {
        Material::new(
            Color::new(0.8, 0.5, 0.5),  // Marrón rosado
            15.0,
            [0.85, 0.15],
            1.0,
            0.0,    // Opaco
            0.0,    // No refleja
        )
    }

    // Material de hojas de cerezo (cherry leaves)
    pub fn cherry_leaves() -> Self {
        Material::new(
            Color::new(1.0, 0.7, 0.8),  // Rosa
            8.0,
            [0.8, 0.2],
            1.0,
            0.3,    // Semi-transparente
            0.0,    // No refleja
        )
    }

    // Material de agua (water) - ¡CON REFRACCIÓN Y REFLEXIÓN!
    pub fn water() -> Self {
        Material::new(
            Color::new(0.3, 0.5, 0.8),  // Azul agua
            50.0,
            [0.3, 0.7],
            1.33,   // Índice de refracción del agua
            0.9,    // Muy transparente
            0.4,    // Refleja moderadamente
        )
    }

    // Material de piedra (stone)
    pub fn stone() -> Self {
        Material::new(
            Color::new(0.5, 0.5, 0.5),  // Gris piedra
            20.0,
            [0.8, 0.2],
            1.0,
            0.0,    // Opaco
            0.1,    // Refleja un poco
        )
    }

    pub fn black() -> Self {
        Material::new(
            Color::black(),
            0.0,
            [0.0, 0.0],
            1.0,
            0.0,
            0.0,
        )
    }
}