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

// Función para calcular si un punto está en sombra
fn is_in_shadow(x: f32, y: f32, z: f32, trees: &Vec<Tree>, rocks: &Vec<Rock>, light_dir: Vector3) -> bool {
    let shadow_steps = 15;
    let step_size = 0.6;
    
    for i in 1..shadow_steps {
        let test_pos = Vector3::new(
            x - light_dir.x * i as f32 * step_size,
            y - light_dir.y * i as f32 * step_size,
            z - light_dir.z * i as f32 * step_size,
        );
        
        // Verificar colisión con árboles
        for tree in trees {
            let dx = test_pos.x - tree.x;
            let dz = test_pos.z - tree.z;
            let dist = (dx * dx + dz * dz).sqrt();
            
            // Sombra del tronco
            if dist < 0.5 && test_pos.y >= 0.0 && test_pos.y <= tree.height {
                return true;
            }
            
            // Sombra de las hojas
            for layer in 0..tree.leaf_layers {
                let layer_y = tree.height + layer as f32 * 0.8;
                let size = (tree.leaf_layers - layer) as f32 * 0.8 + 1.5;
                if dist < size / 2.0 && (test_pos.y - layer_y).abs() < 0.6 {
                    return true;
                }
            }
        }
        
        // Verificar colisión con rocas
        for rock in rocks {
            let dx = test_pos.x - rock.x;
            let dz = test_pos.z - rock.z;
            let dist = (dx * dx + dz * dz).sqrt();
            
            let rock_height = ((rock.x * 0.3).sin() + (rock.z * 0.3).cos()) * 2.5
                + ((rock.x * 0.5).sin() * (rock.z * 0.5).cos()) * 1.5
                + 2.0;
            
            if dist < rock.size / 2.0 && (test_pos.y - rock_height).abs() < rock.size {
                return true;
            }
        }
        
        // Si el rayo sale muy lejos, no hay sombra
        if test_pos.y > 60.0 {
            break;
        }
    }
    
    false
}

// Función para aplicar sombra a un color
fn apply_shadow(base_color: Color, in_shadow: bool) -> Color {
    if in_shadow {
        Color::new(
            (base_color.r as f32 * 0.5) as u8,
            (base_color.g as f32 * 0.5) as u8,
            (base_color.b as f32 * 0.5) as u8,
            base_color.a,
        )
    } else {
        base_color
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1400, 900)
        .title("Diorama Minecraft - Sunset con Sombras")
        .build();

    // Cámara con mejor ángulo
    let mut camera = Camera3D::perspective(
        Vector3::new(50.0, 45.0, 50.0),
        Vector3::new(25.0, 0.0, 25.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.set_target_fps(60);

    let terrain_size = 50;
    let river_center_x = 20;
    let river_width = 4;

    // Generar árboles con más variedad
    let mut rng = rand::thread_rng();
    let mut trees: Vec<Tree> = Vec::new();
    
    for _ in 0..35 {
        let x = rng.gen_range(0..terrain_size) as f32;
        let z = rng.gen_range(0..terrain_size) as f32;
        
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

    // Shader mejorado con sunset
    let mut shader = rl.load_shader_from_memory(
        &thread,
        Some(r#"
        #version 330
        in vec3 vertexPosition;
        in vec2 vertexTexCoord;
        in vec3 vertexNormal;
        in vec4 vertexColor;
        
        uniform mat4 mvp;
        uniform mat4 matModel;
        uniform mat4 matNormal;
        
        out vec3 fragPosition;
        out vec2 fragTexCoord;
        out vec4 fragColor;
        out vec3 fragNormal;
        
        void main() {
            fragPosition = vec3(matModel * vec4(vertexPosition, 1.0));
            fragTexCoord = vertexTexCoord;
            fragColor = vertexColor;
            fragNormal = normalize(vec3(matNormal * vec4(vertexNormal, 1.0)));
            gl_Position = mvp * vec4(vertexPosition, 1.0);
        }
        "#),
        Some(r#"
        #version 330
        in vec3 fragPosition;
        in vec2 fragTexCoord;
        in vec4 fragColor;
        in vec3 fragNormal;
        
        uniform vec3 viewPos;
        uniform vec3 lightPos;
        uniform vec3 sunsetColor;
        uniform vec4 colDiffuse;
        
        out vec4 finalColor;
        
        void main() {
            // Sunset ambient con colores cálidos
            float ambientStrength = 0.45;
            vec3 ambient = ambientStrength * sunsetColor;
            
            // Diffuse con luz del atardecer
            vec3 norm = normalize(fragNormal);
            vec3 lightDir = normalize(lightPos - fragPosition);
            float diff = max(dot(norm, lightDir), 0.0);
            vec3 diffuse = diff * sunsetColor * 1.3;
            
            // Specular con tonos cálidos del sunset
            float specularStrength = 0.6;
            vec3 viewDir = normalize(viewPos - fragPosition);
            vec3 reflectDir = reflect(-lightDir, norm);
            float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);
            vec3 specular = specularStrength * spec * vec3(1.0, 0.85, 0.7);
            
            // Ambient Occlusion
            float ao = 1.0 - (fragPosition.y * 0.006);
            ao = clamp(ao, 0.65, 1.0);
            
            // Global Illumination con colores del atardecer
            vec3 skyColor = sunsetColor * 0.9;
            vec3 groundColor = vec3(0.35, 0.25, 0.18);
            float upDot = dot(norm, vec3(0.0, 1.0, 0.0));
            vec3 gi = mix(groundColor, skyColor, upDot * 0.5 + 0.5) * 0.35;
            
            // Rim lighting para el efecto sunset
            float rimStrength = 1.0 - max(dot(viewDir, norm), 0.0);
            rimStrength = pow(rimStrength, 2.5);
            vec3 rimLight = rimStrength * sunsetColor * 0.6;
            
            vec3 result = ((ambient + diffuse + specular + gi + rimLight) * fragColor.rgb * colDiffuse.rgb * ao);
            finalColor = vec4(result, fragColor.a * colDiffuse.a);
        }
        "#)
    );

    // Obtener ubicaciones de uniforms
    let view_pos_loc = shader.get_shader_location("viewPos");
    let light_pos_loc = shader.get_shader_location("lightPos");
    let sunset_color_loc = shader.get_shader_location("sunsetColor");

    while !rl.window_should_close() {
        // Rotación suave de cámara
        camera_angle += 0.15;
        let radius = 70.0;
        camera.position.x = (terrain_size as f32 / 2.0) + radius * camera_angle.to_radians().cos();
        camera.position.z = (terrain_size as f32 / 2.0) + radius * camera_angle.to_radians().sin();
        camera.position.y = 50.0;
        camera.target = Vector3::new(terrain_size as f32 / 2.0, 0.0, terrain_size as f32 / 2.0);

        // Sol en ángulo de 40 grados (sunset)
        let sun_angle = 40.0f32.to_radians();
        let sun_azimuth = 220.0f32.to_radians(); // Dirección del sol
        
        let light_pos = Vector3::new(
            25.0 + 80.0 * sun_azimuth.cos() * sun_angle.cos(),
            80.0 * sun_angle.sin(),
            25.0 + 80.0 * sun_azimuth.sin() * sun_angle.cos(),
        );

        // Dirección de la luz (para calcular sombras)
        let light_dir = Vector3::new(
            sun_azimuth.cos() * sun_angle.cos(),
            sun_angle.sin(),
            sun_azimuth.sin() * sun_angle.cos(),
        ).normalized();

        // Colores del atardecer
        let sunset_color = Vector3::new(1.0, 0.65, 0.35);

        // Actualizar uniforms del shader
        shader.set_shader_value(view_pos_loc, camera.position);
        shader.set_shader_value(light_pos_loc, light_pos);
        shader.set_shader_value(sunset_color_loc, sunset_color);

        let mut d = rl.begin_drawing(&thread);
        
        // Cielo del atardecer con degradado
        d.clear_background(Color::new(255, 140, 80, 255));
        d.draw_rectangle_gradient_v(0, 0, 1400, 450, 
            Color::new(255, 130, 70, 255),
            Color::new(255, 190, 130, 255)
        );

        let mut d3 = d.begin_mode3D(&camera);
        
        // Activar shader para efectos de sunset
        let mut d_shader = d3.begin_shader_mode(&mut shader);

        // Terreno con sombras
        for x in 0..terrain_size {
            for z in 0..terrain_size {
                let height = ((x as f32 * 0.2).sin() + (z as f32 * 0.2).cos()) * 2.5
                    + ((x as f32 * 0.5).sin() * (z as f32 * 0.5).cos()) * 1.5
                    + 2.0;
                let height_int = height.round() as i32;

                for y in 0..height_int {
                    let base_color = if y == height_int - 1 {
                        Color::new(34, 139, 34, 255)
                    } else if y >= height_int - 2 {
                        Color::new(101, 67, 33, 255)
                    } else {
                        Color::new(70, 70, 70, 255)
                    };
                    
                    // Calcular sombra para este bloque
                    let in_shadow = is_in_shadow(x as f32, y as f32, z as f32, &trees, &rocks, light_dir);
                    let color = apply_shadow(base_color, in_shadow);
                    
                    d_shader.draw_cube(Vector3::new(x as f32, y as f32, z as f32), 1.0, 1.0, 1.0, color);
                }

                // Río con reflexión del sunset
                let dist_to_river = (x as i32 - river_center_x).abs();
                if dist_to_river < river_width {
                    let water_depth = if dist_to_river == 0 { 1.5 } else { 1.0 };
                    
                    // Agua con tonos del atardecer
                    d_shader.draw_cube(
                        Vector3::new(x as f32, -water_depth / 2.0, z as f32),
                        1.0,
                        water_depth,
                        1.0,
                        Color::new(60, 130, 210, 140),
                    );
                    
                    // Arena
                    d_shader.draw_cube(
                        Vector3::new(x as f32, -water_depth - 0.3, z as f32),
                        1.0,
                        0.5,
                        1.0,
                        Color::new(238, 214, 175, 255),
                    );
                }
            }
        }

        // Dibujar árboles con sombras
        for tree in &trees {
            let (trunk_color, leaf_color) = match tree.tree_type {
                TreeType::Oak => (Color::new(101, 67, 33, 255), Color::new(34, 139, 34, 255)),
                TreeType::Cherry => (Color::new(139, 90, 43, 255), Color::new(255, 182, 193, 255)),
                TreeType::Birch => (Color::new(245, 245, 220, 255), Color::new(50, 205, 50, 255)),
            };

            // Tronco
            for i in 0..(tree.height as i32) {
                let in_shadow = is_in_shadow(tree.x, i as f32, tree.z, &trees, &rocks, light_dir);
                let color = apply_shadow(trunk_color, in_shadow);
                
                d_shader.draw_cube(
                    Vector3::new(tree.x, i as f32, tree.z),
                    0.8,
                    1.0,
                    0.8,
                    color,
                );
            }

            // Copa del árbol
            for y in 0..tree.leaf_layers {
                let size = (tree.leaf_layers - y) as f32 * 0.8 + 1.5;
                let layer_y = tree.height + y as f32 * 0.8;
                
                let in_shadow = is_in_shadow(tree.x, layer_y, tree.z, &trees, &rocks, light_dir);
                let color = apply_shadow(leaf_color, in_shadow);
                
                d_shader.draw_cube(
                    Vector3::new(tree.x, layer_y, tree.z),
                    size,
                    1.0,
                    size,
                    color,
                );
            }

            // Punta
            d_shader.draw_cube(
                Vector3::new(tree.x, tree.height + tree.leaf_layers as f32 * 0.8, tree.z),
                0.8,
                0.8,
                0.8,
                leaf_color,
            );
        }

        // Dibujar rocas con sombras
        for rock in &rocks {
            let height = ((rock.x * 0.3).sin() + (rock.z * 0.3).cos()) * 2.5
                + ((rock.x * 0.5).sin() * (rock.z * 0.5).cos()) * 1.5
                + 2.0;
            
            let in_shadow = is_in_shadow(rock.x, height, rock.z, &trees, &rocks, light_dir);
            let color = apply_shadow(Color::new(128, 128, 128, 255), in_shadow);
            
            d_shader.draw_cube(
                Vector3::new(rock.x, height, rock.z),
                rock.size,
                rock.size * 0.8,
                rock.size,
                color,
            );
        }

        // Dibujar flores con sombras
        for flower in &flowers {
            let height = ((flower.x * 0.3).sin() + (flower.z * 0.3).cos()) * 2.5
                + ((flower.x * 0.5).sin() * (flower.z * 0.5).cos()) * 1.5
                + 2.0;
            
            let in_shadow = is_in_shadow(flower.x, height + 0.5, flower.z, &trees, &rocks, light_dir);
            
            // Tallo
            let stem_color = apply_shadow(Color::GREEN, in_shadow);
            d_shader.draw_cube(
                Vector3::new(flower.x, height + 0.3, flower.z),
                0.1,
                0.6,
                0.1,
                stem_color,
            );
            
            // Flor
            let flower_color = apply_shadow(flower.color, in_shadow);
            d_shader.draw_cube(
                Vector3::new(flower.x, height + 0.7, flower.z),
                0.3,
                0.2,
                0.3,
                flower_color,
            );
        }

        drop(d_shader);
        drop(d3);

        // HUD con información
        d.draw_text("Diorama Minecraft - Sunset con Sombras Ray-Traced", 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("Árboles: {} | Flores: {} | Rocas: {}", trees.len(), flowers.len(), rocks.len()), 10, 35, 16, Color::WHITE);
        d.draw_text("Luz Solar: 40° | Dirección: Suroeste", 10, 60, 16, Color::new(255, 200, 100, 255));
        d.draw_text("Efectos: Sunset + Ray-Traced Shadows + Rim Light", 10, 85, 16, Color::YELLOW);
        d.draw_text("ESC para salir", 10, 110, 16, Color::WHITE);
    }
}