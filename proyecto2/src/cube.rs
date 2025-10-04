use nalgebra_glm::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::intersect::Intersect;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Material) -> Self {
        Cube {
            center,
            size,
            material,
        }
    }

    // Intersección rayo-cubo usando el método de slabs
    pub fn ray_intersect(&self, ray: &Ray) -> Intersect {
        let half_size = self.size / 2.0;
        let min = self.center - Vec3::new(half_size, half_size, half_size);
        let max = self.center + Vec3::new(half_size, half_size, half_size);

        let mut tmin = (min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (max.x - ray.origin.x) / ray.direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (max.y - ray.origin.y) / ray.direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty();
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (max.z - ray.origin.z) / ray.direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty();
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        if tmin < 0.0 {
            return Intersect::empty();
        }

        let point = ray.at(tmin);
        let normal = self.get_normal(&point);

        Intersect::new(point, normal, tmin, self.material)
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        let half_size = self.size / 2.0;
        let local = point - self.center;
        let epsilon = 0.001;

        if (local.x - half_size).abs() < epsilon {
            return Vec3::new(1.0, 0.0, 0.0);
        }
        if (local.x + half_size).abs() < epsilon {
            return Vec3::new(-1.0, 0.0, 0.0);
        }
        if (local.y - half_size).abs() < epsilon {
            return Vec3::new(0.0, 1.0, 0.0);
        }
        if (local.y + half_size).abs() < epsilon {
            return Vec3::new(0.0, -1.0, 0.0);
        }
        if (local.z - half_size).abs() < epsilon {
            return Vec3::new(0.0, 0.0, 1.0);
        }
        if (local.z + half_size).abs() < epsilon {
            return Vec3::new(0.0, 0.0, -1.0);
        }

        Vec3::new(0.0, 1.0, 0.0)
    }
}