// main.rs

mod framebuffer;
mod line;
mod triangle;
mod obj;
mod matrix;

use triangle::triangle;
use obj::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
//use std::f32::consts::PI;
use matrix::{create_model_matrix, multiply_matrix_vector4, create_view_matrix};

fn transform(vertex: Vector3, translation: Vector3, scale: f32, rotation: Vector3) -> Vector3 {
    let model : Matrix = create_model_matrix(translation, scale, rotation);
    
    let view : Matrix = create_view_matrix(
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.1),
    );
    
    let vertex4 = Vector4::new(vertex.x, vertex.y, vertex.z, 1.0);
    
    let world_transform = multiply_matrix_vector4(&model, &vertex4);
    
    let view_transform = multiply_matrix_vector4(&view, &world_transform);
    
    let transformed_vertex4 = view_transform;
    
    let transformed_vertex3 = Vector3::new(transformed_vertex4.x / transformed_vertex4.w, transformed_vertex4.y / transformed_vertex4.w, transformed_vertex4.z / transformed_vertex4.w);
    
    transformed_vertex3
}

fn render(framebuffer: &mut Framebuffer, translation: Vector3, scale: f32, rotation: Vector3, vertex_array: &[Vector3]){
    // recorrer el array y transformar
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = transform(vertex.clone(), translation, scale, rotation);
        transformed_vertices.push(transformed);
    }

    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangle(framebuffer, transformed_vertices[i], transformed_vertices[i + 1], transformed_vertices[i + 2]);
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
    let mut translation = Vector3::new(800.0, 600.0, 0.0); // traslación original
    let mut scale = 100.0; // escala original
    let mut rotation = Vector3::new(0.0, 0.0, 0.0); // rotación original

    let obj = Obj::load("./models/cube.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    framebuffer.set_background_color(Color::new(25, 25, 75, 255));

    while !window.window_should_close() {
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));
        
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            translation.x += 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            translation.x -= 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            translation.y -= 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            translation.y += 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            scale *= 1.1;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            scale *= 0.9;
        }
        if window.is_key_down(KeyboardKey::KEY_Q) {
            rotation.x += 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            rotation.x -= 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            rotation.y += 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_R) {
            rotation.y -= 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_T) {
            rotation.z += 10.0;
        }
        if window.is_key_down(KeyboardKey::KEY_Y) {
            rotation.z -= 10.0;
        }

        render(&mut framebuffer, translation, scale, rotation, &vertex_array);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}