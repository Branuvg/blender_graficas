#![allow(dead_code)]

use raylib::prelude::*;

pub fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        // This function manually multiplies a 4x4 matrix with a 4D vector (in homogeneous coordinates)
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

/// Creates a 4x4 matrix from 16 float values, specified in traditional row-major order.
pub fn new_matrix4(
    // Row 0
    r0c0: f32, r0c1: f32, r0c2: f32, r0c3: f32,
    // Row 1
    r1c0: f32, r1c1: f32, r1c2: f32, r1c3: f32,
    // Row 2
    r2c0: f32, r2c1: f32, r2c2: f32, r2c3: f32,
    // Row 3
    r3c0: f32, r3c1: f32, r3c2: f32, r3c3: f32,
) -> Matrix {
    // Raylib's Matrix is column-major, so we transpose the row-major input.
    Matrix {
        m0: r0c0, m1: r1c0, m2: r2c0, m3: r3c0, // Column 0
        m4: r0c1, m5: r1c1, m6: r2c1, m7: r3c1, // Column 1
        m8: r0c2, m9: r1c2, m10: r2c2, m11: r3c2, // Column 2
        m12: r0c3, m13: r1c3, m14: r2c3, m15: r3c3, // Column 3
    }
}

/// Creates a 4x4 transformation matrix from a 3x3 matrix, specified in row-major order.
pub fn new_matrix3(
    // Row 0
    r0c0: f32, r0c1: f32, r0c2: f32,
    // Row 1
    r1c0: f32, r1c1: f32, r1c2: f32,
    // Row 2
    r2c0: f32, r2c1: f32, r2c2: f32,
) -> Matrix {
    new_matrix4(
        r0c0, r0c1, r0c2, 0.0,
        r1c0, r1c1, r1c2, 0.0,
        r2c0, r2c1, r2c2, 0.0,
        0.0,  0.0,  0.0,  1.0,
    )
}

/// Creates a model matrix combining translation, scale, and rotation
pub fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    // Rotation around the X-axis
    let rotation_matrix_x = new_matrix4(
        1.0, 0.0,    0.0,    0.0,
        0.0, cos_x,  -sin_x, 0.0,
        0.0, sin_x,  cos_x,  0.0,
        0.0, 0.0,    0.0,    1.0
    );

    // Rotation around the Y-axis
    let rotation_matrix_y = new_matrix4(
        cos_y,  0.0, sin_y, 0.0,
        0.0,    1.0, 0.0,   0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0,    0.0, 0.0,   1.0
    );

    // Rotation around the Z-axis
    let rotation_matrix_z = new_matrix4(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z,  0.0, 0.0,
        0.0,   0.0,    1.0, 0.0,
        0.0,   0.0,    0.0, 1.0
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    // Scaling matrix
    let scale_matrix = new_matrix4(
        scale, 0.0,   0.0,   0.0,
        0.0,   scale, 0.0,   0.0,
        0.0,   0.0,   scale, 0.0,
        0.0,   0.0,   0.0,   1.0
    );

    // Translation matrix
    let translation_matrix = new_matrix4(
        1.0, 0.0, 0.0, translation.x,
        0.0, 1.0, 0.0, translation.y,
        0.0, 0.0, 1.0, translation.z,
        0.0, 0.0, 0.0, 1.0
    );

    scale_matrix * translation_matrix * rotation_matrix 
}