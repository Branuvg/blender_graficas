//vertex.rs
use raylib::math::Vector3;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vector3,           // Posición original del vértice
    pub transformed_position: Vector3, // Posición después de transformaciones
    pub color: Vector3,              // Color del vértice (opcional)
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            position: Vector3::new(x, y, z),
            transformed_position: Vector3::new(x, y, z),
            color: Vector3::new(1.0, 1.0, 1.0), // Blanco por defecto
        }
    }
}

// Uniforms para pasar datos al vertex shader
pub struct Uniforms {
    pub model: Matrix,
    pub view: Matrix,
    pub projection: Matrix,
    pub viewport: Matrix,
}