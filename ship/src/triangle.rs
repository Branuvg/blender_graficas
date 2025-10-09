// triangle.rs
use raylib::math::{Vector2, Vector3};
use crate::{framebuffer::Framebuffer, line::line};

pub fn triangle(
    framebuffer: &mut Framebuffer,
    a: Vector3,
    b: Vector3,
    c: Vector3
) {
    let a = Vector2::new(a.x, a.y);
    let b = Vector2::new(b.x, b.y);
    let c = Vector2::new(c.x, c.y);
    
    line(framebuffer, a, b);
    line(framebuffer, b, c);
    line(framebuffer, c, a);
}