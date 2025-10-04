use nalgebra_glm::Vec3;
use crate::material::Material;

pub struct Intersect {
    pub is_intersecting: bool,
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material) -> Self {
        Intersect {
            is_intersecting: true,
            distance,
            point,
            normal,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            is_intersecting: false,
            distance: f32::INFINITY,
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            material: Material::black(),
        }
    }
}