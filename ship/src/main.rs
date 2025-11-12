mod framebuffer;
mod triangle;
mod obj;
mod matrix;
mod fragment;
mod vertex;
mod camera;
mod shaders;
mod light;

use triangle::triangle;
use obj::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
use matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix, multiply_matrix_vector4};
use vertex::Vertex;
use camera::Camera;
use shaders::{vertex_shader, fragment_shader, mercury_fragment_shader, sun_fragment_shader, earth_fragment_shader, mars_fragment_shader, uranus_fragment_shader, nave_fragment_shader};
use light::Light;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32, // elapsed time in seconds
    pub dt: f32, // delta time in seconds
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light, planet_type: &str) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage
    for fragment in fragments {      
        let final_color = match planet_type {
            "Sun" => sun_fragment_shader(&fragment, uniforms),
            "Mercury" => mercury_fragment_shader(&fragment, uniforms),
            "Earth" => earth_fragment_shader(&fragment, uniforms),
            "Mars" => mars_fragment_shader(&fragment, uniforms),
            "Uranus" => uranus_fragment_shader(&fragment, uniforms),
            "Nave" => nave_fragment_shader(&fragment, uniforms),
            _ => fragment_shader(&fragment, uniforms), // Default to simple shader
        };
        
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color, //poner fragment.color si no se quiere nada de shading 
            fragment.depth,
        );
    }
}

// Función para dibujar una órbita circular en 3D
fn draw_orbit_3d(framebuffer: &mut Framebuffer, orbit_radius: f32, orbit_color: Color, view_matrix: &Matrix, projection_matrix: &Matrix, viewport_matrix: &Matrix) {
    let segments = 128; // Aumentamos el número de segmentos para una línea más suave
    let angle_increment = 2.0 * PI / segments as f32;
    
    // Crear un vértice temporal para transformar puntos
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut first_point = true;
    
    // Guardar el primer punto para cerrar el círculo
    let mut first_x = 0;
    let mut first_y = 0;
    
    for i in 0..segments {
        let angle = i as f32 * angle_increment;
        
        // Punto en el círculo (en el plano XZ, Y=0)
        let x = angle.cos() * orbit_radius;
        let y = 0.0; // En el plano XZ
        let z = angle.sin() * orbit_radius;
        
        // Transformar el punto a coordenadas de pantalla
        let position_vec4 = Vector4::new(x, y, z, 1.0);
        
        // Aplicar transformaciones
        let view_position = multiply_matrix_vector4(view_matrix, &position_vec4);
        let clip_position = multiply_matrix_vector4(projection_matrix, &view_position);
        
        // Perspectiva division
        let ndc = if clip_position.w != 0.0 {
            Vector3::new(
                clip_position.x / clip_position.w,
                clip_position.y / clip_position.w,
                clip_position.z / clip_position.w,
            )
        } else {
            Vector3::new(clip_position.x, clip_position.y, clip_position.z)
        };
        
        // Aplicar matriz de viewport
        let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
        let screen_position = multiply_matrix_vector4(viewport_matrix, &ndc_vec4);
        
        let screen_x = screen_position.x as i32;
        let screen_y = screen_position.y as i32;
        
        // Guardar el primer punto
        if i == 0 {
            first_x = screen_x;
            first_y = screen_y;
        }
        
        // Dibujar línea desde el punto anterior al actual
        if !first_point {
            // Dibujar la línea con una profundidad mayor (más lejos) que los planetas
            framebuffer.draw_line_with_depth(prev_x, prev_y, screen_x, screen_y, orbit_color, 1000.0);
        } else {
            first_point = false;
        }
        
        prev_x = screen_x;
        prev_y = screen_y;
    }
    
    // Cerrar el círculo conectando el último punto con el primero
    if segments > 0 {
        framebuffer.draw_line_with_depth(prev_x, prev_y, first_x, first_y, orbit_color, 1000.0);
    }
}

#[derive(Clone)]
struct CelestialBody {
    name: String,
    translation: Vector3,
    scale: f32,
    rotation: Vector3,
    orbit_radius: f32,
    orbit_speed: f32,
    rotation_speed: f32,
    color: Color,
}

// Función para verificar colisión entre dos esferas
fn check_collision(pos1: Vector3, radius1: f32, pos2: Vector3, radius2: f32) -> bool {
    let distance = ((pos1.x - pos2.x).powi(2) + (pos1.y - pos2.y).powi(2) + (pos1.z - pos2.z).powi(2)).sqrt();
    distance < (radius1 + radius2)
}

// Función para evitar colisiones
fn avoid_collision(camera_pos: Vector3, target_pos: Vector3, celestial_bodies: &[CelestialBody], time: f32) -> (Vector3, Vector3) {
    let mut new_camera_pos = camera_pos;
    let mut new_target_pos = target_pos;
    
    // Verificar colisiones con cada cuerpo celeste
    for body in celestial_bodies {
        let body_pos = if body.name != "Sun" {
            // Calcular posición actual del planeta en su órbita
            let x = (time * body.orbit_speed).cos() * body.orbit_radius;
            let z = (time * body.orbit_speed).sin() * body.orbit_radius;
            Vector3::new(x, 0.0, z)
        } else {
            body.translation // Posición del sol
        };
        
        // Calcular radios efectivos (considerando el tamaño del cuerpo)
        let camera_radius = 2.0; // Radio de colisión de la cámara
        let body_radius = body.scale * 0.8; // Radio de colisión del cuerpo celeste
        
        // Verificar si hay colisión con la cámara
        if check_collision(new_camera_pos, camera_radius, body_pos, body_radius) {
            // Calcular vector de separación
            let diff_x = new_camera_pos.x - body_pos.x;
            let diff_y = new_camera_pos.y - body_pos.y;
            let diff_z = new_camera_pos.z - body_pos.z;
            let distance = (diff_x.powi(2) + diff_y.powi(2) + diff_z.powi(2)).sqrt();
            
            if distance > 0.0 {
                // Normalizar el vector de separación
                let norm_x = diff_x / distance;
                let norm_y = diff_y / distance;
                let norm_z = diff_z / distance;
                
                // Calcular nueva posición para evitar la colisión
                let min_distance = body_radius + camera_radius;
                new_camera_pos.x = body_pos.x + norm_x * min_distance;
                new_camera_pos.y = body_pos.y + norm_y * min_distance;
                new_camera_pos.z = body_pos.z + norm_z * min_distance;
            }
        }
        
        // Verificar si hay colisión con el punto de mira
        if check_collision(new_target_pos, camera_radius, body_pos, body_radius) {
            // Calcular vector de separación
            let diff_x = new_target_pos.x - body_pos.x;
            let diff_y = new_target_pos.y - body_pos.y;
            let diff_z = new_target_pos.z - body_pos.z;
            let distance = (diff_x.powi(2) + diff_y.powi(2) + diff_z.powi(2)).sqrt();
            
            if distance > 0.0 {
                // Normalizar el vector de separación
                let norm_x = diff_x / distance;
                let norm_y = diff_y / distance;
                let norm_z = diff_z / distance;
                
                // Calcular nueva posición para evitar la colisión
                let min_distance = body_radius + camera_radius;
                new_target_pos.x = body_pos.x + norm_x * min_distance;
                new_target_pos.y = body_pos.y + norm_y * min_distance;
                new_target_pos.z = body_pos.z + norm_z * min_distance;
            }
        }
    }
    
    (new_camera_pos, new_target_pos)
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Proyecto 3 - Graficas")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    
    // Posición inicial de la cámara
    let initial_camera_pos = Vector3::new(0.0, 20.0, 75.0);
    let initial_camera_target = Vector3::new(0.0, 0.0, 0.0);
    let initial_camera_up = Vector3::new(0.0, 1.0, 0.0);
    
    // Inicializar cámara
    let mut camera = Camera::new(
        initial_camera_pos,
        initial_camera_target,
        initial_camera_up,
    );

    // Light
    let light = Light::new(Vector3::new(0.0, 0.0, 0.0)); // fix light

    let obj = Obj::load("./models/sphere.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    // Cargar la nave espacial
    let nave_obj = Obj::load("./models/nave.obj").expect("Failed to load nave.obj");
    let nave_vertex_array = nave_obj.get_vertex_array();

    framebuffer.set_background_color(Color::new(25, 25, 75, 255));

    let sun = CelestialBody {
        name: "Sun".to_string(),
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 15.0,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 0.0,  // No orbit for the sun
        orbit_speed: 0.0,
        rotation_speed: 0.5, // Rotates on its axis
        color: Color::new(255, 255, 0, 255), // Yellow for sun
    };

    let mercury = CelestialBody {
        name: "Mercury".to_string(),
        translation: Vector3::new(0.0, 0.0, 0.0), // This will be updated based on orbit
        scale: 2.0, 
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 15.0, // Distance from sun
        orbit_speed: 0.8, // Orbital speed
        rotation_speed: 2.0, // Rotation speed on its axis
        color: Color::new(169, 169, 169, 255), // Gray for Mercury
    };

    let earth = CelestialBody {
        name: "Earth".to_string(),
        translation: Vector3::new(0.0, 0.0, 0.0), // This will be updated based on orbit
        scale: 3.0, 
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 25.0, // Distance from sun
        orbit_speed: 0.5, // Orbital speed
        rotation_speed: 1.5, // Rotation speed on its axis
        color: Color::new(0, 100, 200, 255), // Blue for Earth
    };

    let mars = CelestialBody {
        name: "Mars".to_string(),
        translation: Vector3::new(0.0, 0.0, 0.0), // This will be updated based on orbit
        scale: 2.5, 
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 35.0, // Distance from sun
        orbit_speed: 0.3, // Orbital speed
        rotation_speed: 1.2, // Rotation speed on its axis
        color: Color::new(205, 92, 92, 255), // Red for Mars
    };

    let uranus = CelestialBody {
        name: "Uranus".to_string(),
        translation: Vector3::new(0.0, 0.0, 0.0), // This will be updated based on orbit
        scale: 5.0, 
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 45.0, // Distance from sun
        orbit_speed: 0.1, // Orbital speed
        rotation_speed: 0.8, // Rotation speed on its axis
        color: Color::new(173, 216, 230, 255), // Light blue for Uranus
    };

    let celestial_bodies = vec![sun, mercury.clone(), earth.clone(), mars.clone(), uranus.clone()];

    let mut time = 0.0;

    while !window.window_should_close() {
        let dt = window.get_frame_time();
        time += dt;
        
        // Verificar teclas para teletransportación
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            // Vista 1: Estado inicial de la cámara
            camera = Camera::new(
                initial_camera_pos,
                initial_camera_target,
                initial_camera_up,
            );
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            // Vista 2: Sistema solar desde arriba
            camera = Camera::new(
                Vector3::new(0.0, 100.0, 0.0), // eye
                Vector3::new(0.0, 0.0, 0.0), // target
                Vector3::new(0.0, 0.0, -1.0), // up
            );
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            // Vista 3: Cercana a la Tierra 
            let camera_distance = earth.orbit_radius + 20.0; // Distancia desde el sol
            let camera_x = (time * earth.orbit_speed).cos() * camera_distance;
            let camera_z = (time * earth.orbit_speed).sin() * camera_distance;
            
            camera = Camera::new(
                Vector3::new(camera_x, 20.0, camera_z), // eye
                Vector3::new(0.0, -15.0, 0.0), // target
                Vector3::new(0.0, 1.0, 0.0), // up
            );
        }
        if window.is_key_pressed(KeyboardKey::KEY_FOUR) {
            // Vista 4: Cercana a Marte 
            let camera_distance = mars.orbit_radius + 20.0; // Distancia desde el sol
            let camera_x = (time * mars.orbit_speed).cos() * camera_distance;
            let camera_z = (time * mars.orbit_speed).sin() * camera_distance;
            
            camera = Camera::new(
                Vector3::new(camera_x, 15.0, camera_z), // eye
                Vector3::new(0.0, -10.0, 0.0), // target
                Vector3::new(0.0, 1.0, 0.0), // up
            );
        }
        if window.is_key_pressed(KeyboardKey::KEY_FIVE) {
            // Vista 5: Cercana a Urano
            let camera_distance = uranus.orbit_radius + 20.0; // Distancia desde el sol
            let camera_x = (time * uranus.orbit_speed).cos() * camera_distance;
            let camera_z = (time * uranus.orbit_speed).sin() * camera_distance;
            
            camera = Camera::new(
                Vector3::new(camera_x, 10.0, camera_z), // eye
                Vector3::new(0.0, -5.0, 0.0), // target
                Vector3::new(0.0, 1.0, 0.0), // up
            );
        }
        
        // Procesar entrada de cámara con movimiento 3D
        camera.process_input(&window);
        
        // Verificar colisiones y ajustar la posición de la cámara si es necesario
        let (adjusted_eye, adjusted_target) = avoid_collision(camera.eye, camera.target, &celestial_bodies, time);
        camera.eye = adjusted_eye;
        camera.target = adjusted_target;
        
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));

        // Render each celestial body FIRST
        for mut body in celestial_bodies.clone() {
            // Update orbital position for planets (not for the sun)
            if body.name != "Sun" {
                body.translation.x = (time * body.orbit_speed).cos() * body.orbit_radius;
                body.translation.z = (time * body.orbit_speed).sin() * body.orbit_radius;
            }
            
            // Update rotation for all bodies
            body.rotation.y += dt * body.rotation_speed;
            
            // Set color for the body
            framebuffer.set_current_color(body.color);
            
            // Crear matrices de transformación para este cuerpo celeste
            let model_matrix = create_model_matrix(
                body.translation, 
                body.scale, 
                body.rotation
            );
            let view_matrix = camera.get_view_matrix();
            let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
            let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

            // Crear uniforms
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                dt,
            };

            render(&mut framebuffer, &uniforms, &vertex_array, &light, &body.name);
        }

        // Crear matrices de transformación comunes
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // Dibujar las órbitas de los planetas en blanco AFTER rendering the planets
        for body in &celestial_bodies {
            if body.name != "Sun" {
                let orbit_color = Color::new(255, 255, 255, 50); // Blanco con menor transparencia (más discreto)
                draw_orbit_3d(&mut framebuffer, body.orbit_radius, orbit_color, &view_matrix, &projection_matrix, &viewport_matrix);
            }
        }

        // Renderizar la nave espacial en su órbita angulada
        {
            // Calcular posición de la nave en su órbita
            let nave_orbit_radius = 30.0; // Radio de la órbita de la nave
            let nave_orbit_speed = 0.4; // Velocidad de la nave
            let nave_angle = time * nave_orbit_speed;
            
            // Posición de la nave en su órbita angulada
            let nave_x = nave_angle.cos() * nave_orbit_radius;
            let nave_y = (nave_angle * 0.5).sin() * 10.0; // Movimiento vertical para crear órbita angulada
            let nave_z = nave_angle.sin() * nave_orbit_radius;
            
            // Calcular rotación de la nave para que apunte en la dirección de movimiento
            let rotation_y = nave_angle + PI / 2.0; // Ajustar para que apunte en la dirección correcta
            let rotation_x = (nave_angle * 0.5).cos() * 0.2; // Pequeña rotación en X para seguir la órbita
            
            // Crear matriz de modelo para la nave
            let nave_model_matrix = create_model_matrix(
                Vector3::new(nave_x, nave_y, nave_z),
                0.3, // Escala de la nave
                Vector3::new(rotation_x, rotation_y, 0.0) // Rotación de la nave
            );
            
            // Crear uniforms para la nave
            let nave_uniforms = Uniforms {
                model_matrix: nave_model_matrix,
                view_matrix: camera.get_view_matrix(),
                projection_matrix: create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0),
                viewport_matrix: create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32),
                time,
                dt,
            };
            
            // Renderizar la nave con su shader específico
            render(&mut framebuffer, &nave_uniforms, &nave_vertex_array, &light, "Nave");
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}