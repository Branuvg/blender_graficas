// main.rs

mod framebuffer;
mod line;
mod triangle;

//use triangle::triangle;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

use crate::triangle::triangle;

fn transform(vertex: Vector2, translation: Vector2, scale: f32) -> Vector2 {
    let mut new_vertex = vertex;

    new_vertex.x *= scale;
    new_vertex.y *= scale;
    
    new_vertex.x += translation.x;
    new_vertex.y += translation.y;

    new_vertex
}

fn render(framebuffer: &mut Framebuffer, translation: Vector2, scale: f32){
    let start = Vector2::new(0.0, 0.0);
    let end = Vector2::new(300.0, 300.0);

    let tstart = transform(start, translation, scale);
    let tend = transform(end, translation, scale);

    line::line(framebuffer, tstart, tend);

    let v1 = Vector2::new(500.0, 500.0);
    let v2 = Vector2::new(600.0, 500.0);    
    let v3 = Vector2::new(550.0, 600.0);

    let tv1 = transform(v1, translation, scale);
    let tv2 = transform(v2, translation, scale);
    let tv3 = transform(v3, translation, scale);

    triangle(framebuffer, tv1, tv2, tv3);
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
    let mut translation = Vector2::new(0.0, 0.0); // traslación original
    let mut scale = 1.0; // escala original

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
            scale += 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            scale -= 0.1;
        }

        render(&mut framebuffer, translation, scale);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}