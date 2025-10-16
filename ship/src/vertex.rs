//vertex.rs
// vertex.rs
use raylib::math::Vector3;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3,           // Posición original
    pub transformed_position: Vector3, // Posición transformada por el vertex shader
    pub color: Vector3,              // Color del vértice (opcional)
}

impl Vertex {
    pub fn new(position: Vector3) -> Self {
        Vertex {
            position,
            transformed_position: Vector3::new(0.0, 0.0, 0.0),
            color: Vector3::new(1.0, 1.0, 1.0), // Blanco por defecto
        }
    }
}

