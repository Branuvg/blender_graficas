// main.rs

mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);


    while !window.window_should_close() {
        framebuffer.set_background_color(Color::new(255, 255, 255, 255));
        framebuffer.set_pixel(10, 10);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}