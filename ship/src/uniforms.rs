// uniforms.rs
use raylib::prelude::*;

pub struct Uniforms {
    pub model: Matrix,
    pub view: Matrix,
    pub projection: Matrix,
    pub viewport: Matrix,
}