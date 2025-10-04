use raylib::prelude::*;
use rand::Rng;

// Estructura para guardar la información de cada árbol
struct Tree {
    x: f32,
    z: f32,
    height: f32,
    leaf_size: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 700)
        .title("Diorama Minecraft Mejorado")
        .build();

    // Cámara estática
    let camera = Camera3D::perspective(
        Vector3::new(30.0, 30.0, 30.0),
        Vector3::new(10.0, 0.0, 10.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.set_target_fps(60);

    let terrain_size = 20;

    // Parámetros del río
    let river_start = 3;
    let river_width = 3;

    // Generar los árboles **solo una vez**
    let mut rng = rand::thread_rng();
    let mut trees: Vec<Tree> = Vec::new();
    for _ in 0..10 {
        trees.push(Tree {
            x: rng.gen_range(0..terrain_size) as f32,
            z: rng.gen_range(0..terrain_size) as f32,
            height: rng.gen_range(2..4) as f32,
            leaf_size: rng.gen_range(2..4) as f32,
        });
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::SKYBLUE);

        let mut d3 = d.begin_mode3D(&camera);

        // Terreno con colinas
        for x in 0..terrain_size {
            for z in 0..terrain_size {
                let height = ((x as f32 * 0.3).sin() + (z as f32 * 0.3).cos()) * 2.0 + 1.0;
                let height_int = height.round() as i32;

                // Bloques de tierra y césped
                for y in 0..height_int {
                    let color = if y == height_int - 1 { Color::GREEN } else { Color::BROWN };
                    d3.draw_cube(Vector3::new(x as f32, y as f32, z as f32), 1.0, 1.0, 1.0, color);
                }

                // Río que atraviesa todo el bosque
                if x >= river_start && x < river_start + river_width {
                    d3.draw_cube(Vector3::new(x as f32, 0.0, z as f32), 1.0, 0.5, 1.0, Color::BLUE.alpha(0.6));
                }
            }
        }

        // Dibujar árboles usando las posiciones fijas
        for tree in &trees {
            // Tronco
            d3.draw_cube(Vector3::new(tree.x, tree.height, tree.z), 1.0, tree.height, 1.0, Color::BROWN);

            // Hojas (varias capas para forma más natural)
            for y in 0..tree.leaf_size as i32 {
                let size = tree.leaf_size - y as f32 * 0.5;
                d3.draw_cube(Vector3::new(tree.x, tree.height + 1.0 + y as f32, tree.z), size, 1.0, size, Color::PINK);
            }
        }
    }
}
