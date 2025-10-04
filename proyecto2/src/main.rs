use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, normalize};
use std::f32::consts::PI;

mod color;
mod framebuffer;
mod camera;
mod ray;
mod cube;
mod material;
mod intersect;
mod light;

use color::Color;
use framebuffer::Framebuffer;
use camera::Camera;
use ray::Ray;
use cube::Cube;
use material::Material;
use intersect::Intersect;
use light::Light;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    
    let mut window = Window::new(
        "Minecraft Diorama - Raytracer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS

    // Configurar la cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 5.0, 15.0),  // eye (posición)
        Vec3::new(0.0, 2.0, 0.0),    // center (hacia dónde mira)
        Vec3::new(0.0, 1.0, 0.0),    // up
        45.0 * PI / 180.0,            // fov en radianes
        ASPECT_RATIO,
    );

    // Crear la escena con cubos
    let cubes = create_scene();
    
    // Crear luz
    let light = Light::new(
        Vec3::new(10.0, 15.0, 10.0),
        Color::new(1.0, 1.0, 1.0),
        1.5,
    );

    println!("Controles:");
    println!("  A/D - Rotar cámara");
    println!("  W/S - Zoom in/out");
    println!("  Q/E - Subir/bajar cámara");
    println!("  ESC - Salir");
    println!("\nRendering...");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Controles de cámara
        if window.is_key_down(Key::A) {
            camera.orbit(0.05);
        }
        if window.is_key_down(Key::D) {
            camera.orbit(-0.05);
        }
        if window.is_key_down(Key::W) {
            camera.zoom(-0.2);
        }
        if window.is_key_down(Key::S) {
            camera.zoom(0.2);
        }
        if window.is_key_down(Key::Q) {
            camera.change_height(0.2);
        }
        if window.is_key_down(Key::E) {
            camera.change_height(-0.2);
        }

        // Renderizar
        render(&mut framebuffer, &camera, &cubes, &light);

        // Actualizar ventana
        window
            .update_with_buffer(framebuffer.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}

fn render(framebuffer: &mut Framebuffer, camera: &Camera, cubes: &[Cube], light: &Light) {
    let width = framebuffer.width;
    let height = framebuffer.height;

    for y in 0..height {
        for x in 0..width {
            // Convertir coordenadas de pantalla a espacio NDC
            let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
            let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;

            // Calcular la dirección del rayo
            let screen_pos = Vec3::new(screen_x, screen_y, -1.0);
            let ray_direction = normalize(&camera.basis_change(&screen_pos));
            let ray = Ray::new(camera.eye, ray_direction);

            // Hacer raycast
            let pixel_color = cast_ray(&ray, cubes, light, 0);
            framebuffer.point(x, y, pixel_color);
        }
    }
}

fn cast_ray(ray: &Ray, cubes: &[Cube], light: &Light, depth: u32) -> Color {
    if depth > 3 {
        return Color::new(0.5, 0.7, 1.0); // Color de cielo
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    // Encontrar la intersección más cercana
    for cube in cubes {
        let i = cube.ray_intersect(ray);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        // No hay intersección, retornar color de cielo
        return Color::new(0.5, 0.7, 1.0);
    }

    // Calcular iluminación
    let light_dir = normalize(&(light.position - intersect.point));
    let view_dir = normalize(&(ray.origin - intersect.point));
    let reflect_dir = reflect(&-light_dir, &intersect.normal);

    // Componente difusa
    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0);
    let diffuse = intersect.material.diffuse * intersect.material.albedo[0] * diffuse_intensity;

    // Componente especular
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = Color::white() * intersect.material.albedo[1] * specular_intensity;

    // Color final
    let mut color = (diffuse + specular) * light.intensity;

    // Reflexión
    if intersect.material.reflectivity > 0.0 {
        let reflect_dir = reflect(&ray.direction, &intersect.normal);
        let reflect_origin = offset_origin(&intersect.point, &intersect.normal);
        let reflect_ray = Ray::new(reflect_origin, reflect_dir);
        let reflect_color = cast_ray(&reflect_ray, cubes, light, depth + 1);
        color = color * (1.0 - intersect.material.reflectivity) + reflect_color * intersect.material.reflectivity;
    }

    // Refracción (para agua)
    if intersect.material.transparency > 0.0 {
        let refract_dir = refract(&ray.direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = offset_origin(&intersect.point, &-intersect.normal);
        let refract_ray = Ray::new(refract_origin, refract_dir);
        let refract_color = cast_ray(&refract_ray, cubes, light, depth + 1);
        color = color * (1.0 - intersect.material.transparency) + refract_color * intersect.material.transparency;
    }

    color
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - normal * 2.0 * incident.dot(normal)
}

fn refract(incident: &Vec3, normal: &Vec3, refractive_index: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    let (etai, etat, n);

    if cosi < 0.0 {
        // Ray is inside the object
        etai = refractive_index;
        etat = 1.0;
        n = -normal;
    } else {
        // Ray is outside
        etai = 1.0;
        etat = refractive_index;
        n = *normal;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        reflect(incident, &n)
    } else {
        incident * eta + n * (eta * cosi - k.sqrt())
    }
}

fn offset_origin(point: &Vec3, normal: &Vec3) -> Vec3 {
    let offset = normal * 0.001;
    point + offset
}

fn create_scene() -> Vec<Cube> {
    let mut cubes = Vec::new();

    // Piso de césped (base)
    for x in -5..6 {
        for z in -5..6 {
            cubes.push(Cube::new(
                Vec3::new(x as f32, 0.0, z as f32),
                1.0,
                Material::grass(),
            ));
        }
    }

    // Tronco del árbol de cerezo
    for y in 1..5 {
        cubes.push(Cube::new(
            Vec3::new(0.0, y as f32, 0.0),
            1.0,
            Material::cherry_wood(),
        ));
    }

    // Hojas del árbol de cerezo (copa)
    for x in -2..3 {
        for z in -2..3 {
            for y in 4..7 {
                if !(x == 0 && z == 0 && y < 6) {  // No poner hojas en el centro del tronco
                    cubes.push(Cube::new(
                        Vec3::new(x as f32, y as f32, z as f32),
                        1.0,
                        Material::cherry_leaves(),
                    ));
                }
            }
        }
    }

    // Río de agua
    for x in -5..6 {
        for z in 2..5 {
            cubes.push(Cube::new(
                Vec3::new(x as f32, 0.0, z as f32),
                1.0,
                Material::water(),
            ));
        }
    }

    // Pequeña colina de tierra
    for y in 1..3 {
        for x in 3..5 {
            cubes.push(Cube::new(
                Vec3::new(x as f32, y as f32, -3.0),
                1.0,
                Material::dirt(),
            ));
        }
    }

    // Piedras decorativas
    cubes.push(Cube::new(Vec3::new(-3.0, 1.0, -3.0), 1.0, Material::stone()));
    cubes.push(Cube::new(Vec3::new(-4.0, 1.0, -2.0), 1.0, Material::stone()));

    cubes
}