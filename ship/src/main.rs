// main.rs
mod framebuffer;
mod line;
mod triangle;
mod obj;
mod matrix;
mod fragment;
mod vertex;
mod uniforms;
mod camera;

use triangle::triangle;
use obj::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
use matrix::{create_model_matrix, multiply_matrix_vector4, create_projection_matrix, create_viewport_matrix};
use fragment::Fragment;
use vertex::Vertex;
use uniforms::Uniforms;
use camera::Camera;

// Vertex Shader: Transforma vértices usando matrices
fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let mut result = vertex.clone();
    
    // Convertir a Vector4 para transformaciones
    let vertex4 = Vector4::new(
        vertex.position.x, 
        vertex.position.y, 
        vertex.position.z, 
        1.0
    );
    
    // Aplicar transformaciones: Model -> View -> Projection -> Viewport
    let world_transform = multiply_matrix_vector4(&uniforms.model, &vertex4);
    let view_transform = multiply_matrix_vector4(&uniforms.view, &world_transform);
    let projection_transform = multiply_matrix_vector4(&uniforms.projection, &view_transform);
    let viewport_transform = multiply_matrix_vector4(&uniforms.viewport, &projection_transform);
    
    // División perspectiva
    let transformed_vertex3 = Vector3::new(
        viewport_transform.x / viewport_transform.w,
        viewport_transform.y / viewport_transform.w,
        viewport_transform.z / viewport_transform.w
    );
    
    result.transformed_position = transformed_vertex3;
    result
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
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
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        // Verificar que el fragment esté dentro de los límites del framebuffer
        if fragment.position.x >= 0.0 && fragment.position.x < framebuffer.width as f32 &&
           fragment.position.y >= 0.0 && fragment.position.y < framebuffer.height as f32 {
            
            framebuffer.set_pixel(
                fragment.position.x as i32,
                fragment.position.y as i32,
            );
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Ship")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    
    // Inicializar cámara
    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0), // eye
        Vector3::new(0.0, 0.0, 0.0), // target
        Vector3::new(0.0, 1.0, 0.0), // up
    );

    // Parámetros de transformación del modelo
    let mut translation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 1.0;
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);

    let obj = Obj::load("./models/cube.obj").expect("Failed to load obj");
    
    // Convertir Vector3 a Vertex
    let vertex_array: Vec<Vertex> = obj.get_vertex_array()
        .iter()
        .map(|v| Vertex::new(*v))
        .collect();

    framebuffer.set_background_color(Color::new(25, 25, 75, 255));

    while !window.window_should_close() {
        // Procesar entrada de cámara
        camera.process_input(&window);
        
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));
        
        // Controles de transformación del modelo
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            translation.x += 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            translation.x -= 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            translation.y += 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            translation.y -= 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            scale *= 1.1;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            scale *= 0.9;
        }
        if window.is_key_down(KeyboardKey::KEY_Q) {
            rotation.z += 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            rotation.z -= 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            rotation.y += 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_R) {
            rotation.y -= 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_T) {
            rotation.x += 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_Y) {
            rotation.x -= 0.1;
        }

        // Crear matrices de transformación
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // Crear uniforms
        let uniforms = Uniforms {
            model: model_matrix,
            view: view_matrix,
            projection: projection_matrix,
            viewport: viewport_matrix,
        };

        render(&mut framebuffer, &uniforms, &vertex_array);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}