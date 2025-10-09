// triangle.rs
use raylib::math::Vector2;
use crate::{framebuffer::Framebuffer, line::line};

pub fn triangle(
    framebuffer: &mut Framebuffer,
    a: Vector2,
    b: Vector2,
    c: Vector2
) {
    line(framebuffer, a, b);
    line(framebuffer, b, c);
    line(framebuffer, c, a);
}