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

fn render(framebuffer: &mut Framebuffer){
    //line::line(framebuffer, Vector2::new(10.0, 10.0), Vector2::new(300.0, 300.0));

    let v1 = Vector2::new(500.0, 500.0);
    let v2 = Vector2::new(600.0, 500.0);
    let v3 = Vector2::new(550.0, 600.0);

    triangle(framebuffer, v1, v2, v3);
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
    
    framebuffer.set_background_color(Color::new(25, 25, 75, 255));

    while !window.window_should_close() {
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));
        
        render(&mut framebuffer);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}