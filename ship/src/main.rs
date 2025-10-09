// main.rs

mod framebuffer;
mod line;
mod triangle;
mod obj;

use triangle::triangle;
use obj::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

fn transform(vertex: Vector3, translation: Vector2, scale: f32, rotation: Vector3,) -> Vector3 {
    let mut new_vertex = vertex;
    //pre calcular seno y coseno para rotación
    let (sin_x, cos_x) = (rotation.x * PI / 180.0).sin_cos();
    let (sin_y, cos_y) = (rotation.y * PI / 180.0).sin_cos();
    let (sin_z, cos_z) = (rotation.z * PI / 180.0).sin_cos();

    //rotación
    // en X
    let rotated_y = new_vertex.y * cos_x - new_vertex.z * sin_x;
    let rotated_z = new_vertex.y * sin_x + new_vertex.z * cos_x;
    new_vertex.y = rotated_y;
    new_vertex.z = rotated_z;

    // en Y
    let rotated_x = new_vertex.x * cos_y + new_vertex.z * sin_y;
    let rotated_z = -new_vertex.x * sin_y + new_vertex.z * cos_y;
    new_vertex.x = rotated_x;
    new_vertex.z = rotated_z;

    // en Z
    let rotated_x = new_vertex.x * cos_z - new_vertex.y * sin_z;
    let rotated_y = new_vertex.x * sin_z + new_vertex.y * cos_z;

    new_vertex.x = rotated_x;
    new_vertex.y = rotated_y;
    
    //escala
    new_vertex.x *= scale;
    new_vertex.y *= scale;
    
    //traslación
    new_vertex.x += translation.x;
    new_vertex.y += translation.y;

    new_vertex
}

fn render(framebuffer: &mut Framebuffer, translation: Vector2, scale: f32, rotation: Vector3, vertex_array: &[Vector3]){
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
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut translation = Vector2::new(800.0, 600.0); // traslación original
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