use raylib::prelude::*;
use rand::Rng;

// Estructura para guardar la información de cada árbol
struct Tree {
    x: f32,
    z: f32,
    height: f32,
    leaf_layers: i32,
    tree_type: TreeType,
}

#[derive(Clone, Copy)]
enum TreeType {
    Oak,
    Cherry,
    Birch,
}

// Estructura para rocas decorativas
struct Rock {
    x: f32,
    z: f32,
    size: f32,
}

// Estructura para flores
struct Flower {
    x: f32,
    z: f32,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1400, 900)
        .title("Diorama Minecraft - Mundo Expandido")
        .build();

    // Cámara con mejor ángulo
    let mut camera = Camera3D::perspective(
        Vector3::new(50.0, 45.0, 50.0),
        Vector3::new(25.0, 0.0, 25.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.set_target_fps(60);

    let terrain_size = 50; // Mapa mucho más grande

    // Parámetros del río mejorado
    let river_center_x = 20;
    let river_width = 4;

    // Generar árboles con más variedad
    let mut rng = rand::thread_rng();
    let mut trees: Vec<Tree> = Vec::new();
    
    for _ in 0..35 { // Más árboles
        let x = rng.gen_range(0..terrain_size) as f32;
        let z = rng.gen_range(0..terrain_size) as f32;
        
        // Evitar poner árboles en el río
        if (x as i32) < river_center_x - river_width || (x as i32) > river_center_x + river_width {
            let tree_type = match rng.gen_range(0..3) {
                0 => TreeType::Oak,
                1 => TreeType::Cherry,
                _ => TreeType::Birch,
            };
            
            trees.push(Tree {
                x,
                z,
                height: rng.gen_range(3..6) as f32,
                leaf_layers: rng.gen_range(3..5),
                tree_type,
            });
        }
    }

    // Generar rocas decorativas
    let mut rocks: Vec<Rock> = Vec::new();
    for _ in 0..20 {
        rocks.push(Rock {
            x: rng.gen_range(0..terrain_size) as f32,
            z: rng.gen_range(0..terrain_size) as f32,
            size: rng.gen_range(5..12) as f32 / 10.0,
        });
    }

    // Generar flores
    let mut flowers: Vec<Flower> = Vec::new();
    for _ in 0..50 {
        let flower_colors = [
            Color::RED,
            Color::YELLOW,
            Color::PURPLE,
            Color::ORANGE,
            Color::MAGENTA,
        ];
        
        flowers.push(Flower {
            x: rng.gen_range(0..terrain_size) as f32 + 0.3,
            z: rng.gen_range(0..terrain_size) as f32 + 0.3,
            color: flower_colors[rng.gen_range(0..flower_colors.len())],
        });
    }

    let mut camera_angle = 0.0f32;

    while !rl.window_should_close() {
        // Rotación suave de cámara
        camera_angle += 0.2;
        let radius = 70.0;
        camera.position.x = (terrain_size as f32 / 2.0) + radius * camera_angle.to_radians().cos();
        camera.position.z = (terrain_size as f32 / 2.0) + radius * camera_angle.to_radians().sin();
        camera.position.y = 50.0;
        camera.target = Vector3::new(terrain_size as f32 / 2.0, 0.0, terrain_size as f32 / 2.0);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(135, 206, 235, 255)); // Cielo más realista

        let mut d3 = d.begin_mode3D(&camera);

        // Terreno con colinas más naturales usando noise
        for x in 0..terrain_size {
            for z in 0..terrain_size {
                // Función de altura más compleja para terreno natural
                let height = ((x as f32 * 0.2).sin() + (z as f32 * 0.2).cos()) * 2.5
                    + ((x as f32 * 0.5).sin() * (z as f32 * 0.5).cos()) * 1.5
                    + 2.0;
                let height_int = height.round() as i32;

                // Bloques de tierra y césped con más capas
                for y in 0..height_int {
                    let color = if y == height_int - 1 {
                        Color::new(34, 139, 34, 255) // Verde césped
                    } else if y >= height_int - 2 {
                        Color::new(101, 67, 33, 255) // Marrón tierra
                    } else {
                        Color::new(70, 70, 70, 255) // Piedra
                    };
                    
                    d3.draw_cube(Vector3::new(x as f32, y as f32, z as f32), 1.0, 1.0, 1.0, color);
                    d3.draw_cube_wires(Vector3::new(x as f32, y as f32, z as f32), 1.0, 1.0, 1.0, Color::new(0, 0, 0, 30));
                }

                // Río con orillas y profundidad
                let dist_to_river = (x as i32 - river_center_x).abs();
                if dist_to_river < river_width {
                    // Agua con diferentes profundidades
                    let water_depth = if dist_to_river == 0 { 1.5 } else { 1.0 };
                    d3.draw_cube(
                        Vector3::new(x as f32, -water_depth / 2.0, z as f32),
                        1.0,
                        water_depth,
                        1.0,
                        Color::new(30, 144, 255, 180),
                    );
                    
                    // Arena en el fondo del río
                    d3.draw_cube(
                        Vector3::new(x as f32, -water_depth - 0.3, z as f32),
                        1.0,
                        0.5,
                        1.0,
                        Color::new(238, 214, 175, 255),
                    );
                }
            }
        }

        // Dibujar árboles mejorados
        for tree in &trees {
            let (trunk_color, leaf_color) = match tree.tree_type {
                TreeType::Oak => (Color::new(101, 67, 33, 255), Color::new(34, 139, 34, 255)),
                TreeType::Cherry => (Color::new(139, 90, 43, 255), Color::new(255, 182, 193, 255)),
                TreeType::Birch => (Color::new(245, 245, 220, 255), Color::new(50, 205, 50, 255)),
            };

            // Tronco con textura
            for i in 0..(tree.height as i32) {
                d3.draw_cube(
                    Vector3::new(tree.x, i as f32, tree.z),
                    0.8,
                    1.0,
                    0.8,
                    trunk_color,
                );
                d3.draw_cube_wires(
                    Vector3::new(tree.x, i as f32, tree.z),
                    0.8,
                    1.0,
                    0.8,
                    Color::new(0, 0, 0, 50),
                );
            }

            // Copa del árbol con múltiples capas piramidales
            for y in 0..tree.leaf_layers {
                let size = (tree.leaf_layers - y) as f32 * 0.8 + 1.5;
                let layer_y = tree.height + y as f32 * 0.8;
                
                // Capa principal
                d3.draw_cube(
                    Vector3::new(tree.x, layer_y, tree.z),
                    size,
                    1.0,
                    size,
                    leaf_color,
                );
                
                // Detalles adicionales en las hojas
                if y < tree.leaf_layers - 1 {
                    d3.draw_cube_wires(
                        Vector3::new(tree.x, layer_y, tree.z),
                        size,
                        1.0,
                        size,
                        Color::new(0, 100, 0, 80),
                    );
                }
            }

            // Punta del árbol
            d3.draw_cube(
                Vector3::new(tree.x, tree.height + tree.leaf_layers as f32 * 0.8, tree.z),
                0.8,
                0.8,
                0.8,
                leaf_color,
            );
        }

        // Dibujar rocas
        for rock in &rocks {
            let height = ((rock.x * 0.3).sin() + (rock.z * 0.3).cos()) * 2.5
                + ((rock.x * 0.5).sin() * (rock.z * 0.5).cos()) * 1.5
                + 2.0;
            
            d3.draw_cube(
                Vector3::new(rock.x, height, rock.z),
                rock.size,
                rock.size * 0.8,
                rock.size,
                Color::new(128, 128, 128, 255),
            );
        }

        // Dibujar flores
        for flower in &flowers {
            let height = ((flower.x * 0.3).sin() + (flower.z * 0.3).cos()) * 2.5
                + ((flower.x * 0.5).sin() * (flower.z * 0.5).cos()) * 1.5
                + 2.0;
            
            // Tallo
            d3.draw_cube(
                Vector3::new(flower.x, height + 0.3, flower.z),
                0.1,
                0.6,
                0.1,
                Color::GREEN,
            );
            
            // Flor
            d3.draw_cube(
                Vector3::new(flower.x, height + 0.7, flower.z),
                0.3,
                0.2,
                0.3,
                flower.color,
            );
        }

        drop(d3);

        // HUD con información
        d.draw_text("Diorama Minecraft - Mundo Expandido", 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("Árboles: {} | Flores: {} | Rocas: {}", trees.len(), flowers.len(), rocks.len()), 10, 35, 16, Color::WHITE);
        d.draw_text("ESC para salir", 10, 60, 16, Color::WHITE);
    }
}