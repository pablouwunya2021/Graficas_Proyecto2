use nalgebra_glm::{Vec3, look_at, perspective};

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    
    // Para rotación orbital
    pub orbit_radius: f32,
    pub orbit_angle: f32,
    pub orbit_height: f32,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, fov: f32, aspect: f32) -> Self {
        let orbit_radius = (eye - center).magnitude();
        let orbit_angle = (eye.x - center.x).atan2(eye.z - center.z);
        let orbit_height = eye.y;
        
        Camera {
            eye,
            center,
            up,
            fov,
            aspect,
            near: 0.1,
            far: 1000.0,
            orbit_radius,
            orbit_angle,
            orbit_height,
        }
    }

    pub fn get_view_matrix(&self) -> nalgebra_glm::Mat4 {
        look_at(&self.eye, &self.center, &self.up)
    }

    pub fn get_projection_matrix(&self) -> nalgebra_glm::Mat4 {
        perspective(self.aspect, self.fov, self.near, self.far)
    }

    // Rotar la cámara alrededor del centro
    pub fn orbit(&mut self, delta_angle: f32) {
        self.orbit_angle += delta_angle;
        self.update_eye_position();
    }

    // Acercar/alejar la cámara (zoom)
    pub fn zoom(&mut self, delta: f32) {
        self.orbit_radius = (self.orbit_radius + delta).max(2.0).min(50.0);
        self.update_eye_position();
    }

    // Cambiar altura de la órbita
    pub fn change_height(&mut self, delta: f32) {
        self.orbit_height = (self.orbit_height + delta).max(0.5).min(30.0);
        self.update_eye_position();
    }

    fn update_eye_position(&mut self) {
        self.eye = Vec3::new(
            self.center.x + self.orbit_radius * self.orbit_angle.sin(),
            self.orbit_height,
            self.center.z + self.orbit_radius * self.orbit_angle.cos(),
        );
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated = 
            right * vector.x +
            up * vector.y +
            (-forward) * vector.z;

        rotated
    }
}