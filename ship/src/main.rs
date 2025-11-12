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
use matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix};
use vertex::Vertex;
use camera::Camera;
use shaders::{vertex_shader, fragment_shader};
use light::Light;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32, // elapsed time in seconds
    pub dt: f32, // delta time in seconds
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light) {
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
        
        let final_color = fragment_shader(&fragment, uniforms);
        
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color, //poner fragment.color si no se quiere nada de shading
            fragment.depth,
        );
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

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Proyecto 3 - Graficas")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    
    // Inicializar cámara
    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 50.0), // eye
        Vector3::new(0.0, 0.0, 0.0), // target
        Vector3::new(0.0, 1.0, 0.0), // up
    );

    // Light
    let light = Light::new(Vector3::new(0.0, 0.0, 0.0));

    let obj = Obj::load("./models/sphere.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    framebuffer.set_background_color(Color::new(25, 25, 75, 255));

    // Crear el sol y Venus
    let sun = CelestialBody {
        name: "Sun".to_string(),
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 10.0,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 0.0,  // No orbit for the sun
        orbit_speed: 0.0,
        rotation_speed: 0.5, // Rotates on its axis
        color: Color::new(255, 255, 0, 255), // Yellow for sun
    };

    let venus = CelestialBody {
        name: "Venus".to_string(),
        translation: Vector3::new(0.0, 0.0, 0.0), // This will be updated based on orbit
        scale: 3.0, // Smaller than sun
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 25.0, // Distance from sun
        orbit_speed: 0.3, // Orbital speed
        rotation_speed: 0.7, // Rotation speed on its axis
        color: Color::new(255, 165, 0, 255), // Orange for Venus
    };

    let celestial_bodies = vec![sun, venus];

    let mut time = 0.0;

    while !window.window_should_close() {
        let dt = window.get_frame_time();
        time += dt;
        
        camera.process_input(&window);
        
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));

        // Render each celestial body
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

            render(&mut framebuffer, &uniforms, &vertex_array, &light);
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}