// shaders.rs
use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::matrix::multiply_matrix_vector4;
use crate::fragment::Fragment;

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);
    let transformed_normal_vec4 = multiply_matrix_vector4(model_matrix, &normal_vec4);
    
    let mut transformed_normal = Vector3::new(
        transformed_normal_vec4.x,
        transformed_normal_vec4.y,
        transformed_normal_vec4.z,
    );
    
    transformed_normal.normalize();
    transformed_normal
}

// Función de ruido para crear arcos de fuego más pronunciados con movimiento rápido
fn fire_arc_noise(x: f32, y: f32, z: f32, time: f32) -> f32 {
    // Movimiento más rápido en las funciones de ruido
    let n1 = (x * 2.0 + time * 1.5).sin() * (y * 1.5 + time * 1.2).cos() + 
             (y * 2.0 + time * 1.4).sin() * (z * 1.8 + time * 1.1).cos() +
             (z * 2.0 + time * 1.3).sin() * (x * 1.6 + time * 1.3).cos();
    
    // Función gaussiana para crear "arcos" más definidos
    let gaussian = (-((x*x + y*y + z*z) * 0.5)).exp();
    
    // Combinar para crear estructuras lineales con mayor contraste
    let arc_structure = (n1 * 2.0).sin() * gaussian * 0.7 + 0.3;
    
    arc_structure
}

// Función de ruido fractal para mayor detalle con movimiento rápido
fn fractal_fire_noise(x: f32, y: f32, z: f32, time: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    
    for i in 0..3 {
        // Movimiento más rápido en cada octava
        let adjusted_time = time * 2.0 + (i as f32) * 50.0;
        let noise_val = fire_arc_noise(x * frequency, y * frequency, z * frequency, adjusted_time);
        value += noise_val * amplitude;
        amplitude *= 0.6;
        frequency *= 2.0;
    }
    
    value / 2.8 // Normalizar
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let _position_vec4 = Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let time = uniforms.time;
    
    // Movimiento más rápido en la distorsión del vertex
    let noise_amount = fractal_fire_noise(
        vertex.position.x * 1.5, 
        vertex.position.y * 1.5, 
        vertex.position.z * 1.5, 
        time * 2.0  // Movimiento más rápido aquí también
    ) * 0.08; // Distorsión moderada para efecto de arcos
    
    // Aplicar distorsión en dirección normal para mantener forma esférica
    let distorted_pos = Vector3::new(
        vertex.position.x + vertex.normal.x * noise_amount,
        vertex.position.y + vertex.normal.y * noise_amount,
        vertex.position.z + vertex.normal.z * noise_amount,
    );

    let distorted_vec4 = Vector4::new(
        distorted_pos.x,
        distorted_pos.y,
        distorted_pos.z,
        1.0
    );

    // Apply Model transformation con posición distorsionada
    let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &distorted_vec4);

    // Apply View transformation (camera)
    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);

    // Apply Projection transformation (perspective)
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

    // Perform perspective division to get NDC (Normalized Device Coordinates)
    let ndc = if clip_position.w != 0.0 {
        Vector3::new(
            clip_position.x / clip_position.w,
            clip_position.y / clip_position.w,
            clip_position.z / clip_position.w,
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };
    
    // Apply Viewport transformation to get screen coordinates
    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);
    
    let transformed_position = Vector3::new(
        screen_position.x,
        screen_position.y,
        screen_position.z,
    );
    
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Calcular la distancia desde el centro para efectos radiales
    let distance_from_center = pos.length();
    
    // Normalizar la posición para efectos direccionales
    let length = pos.length();
    let normalized_pos = if length > 0.0 {
        Vector3::new(pos.x / length, pos.y / length, pos.z / length)
    } else {
        Vector3::new(0.0, 0.0, 1.0)
    };
    
    // Efecto de "arcos de fuego" - estructuras lineales prominentes con movimiento rápido
    let fire_arcs = fractal_fire_noise(normalized_pos.x, normalized_pos.y, normalized_pos.z, time * 2.0); // Movimiento 2x más rápido
    
    // Efecto de pulsación solar global con movimiento rápido
    let solar_pulse = (time * 2.0).sin().abs() * 0.3 + 0.7; // 4x más rápido que antes
    
    // Efecto radial para mantener forma esférica (reducir el negro)
    let sphere_effect = 1.0 - (distance_from_center - 0.9).abs().min(0.1) * 5.0; // Menos oscuro
    let edge_glow = (distance_from_center - 0.85).max(0.0) * 2.0; // Brillo en los bordes
    
    // Gradiente de temperatura de estrella (más caliente en el centro)
    let temperature = if distance_from_center < 0.8 {
        // Interior rojo intenso
        Vector3::new(1.0, 0.4, 0.2)
    } else if distance_from_center < 0.9 {
        // Superficie naranja
        Vector3::new(1.0, 0.6, 0.3)
    } else {
        // Coroa amarilla
        Vector3::new(1.0, 0.8, 0.4)
    };
    
    // Intensidad de los arcos de fuego (menos picos pero más altos)
    let arc_intensity = fire_arcs * 2.0; // Mayor intensidad para arcos prominentes
    let arc_bright = (arc_intensity - 0.6).max(0.0) * 4.0; // Solo los picos más altos brillan mucho
    
    // Combinar efectos
    let base_intensity = sphere_effect * solar_pulse + edge_glow * 0.5;
    let total_intensity = base_intensity + arc_bright * 0.8;
    
    // Color final con arcos de fuego prominentes
    let final_color = temperature * total_intensity + 
                     Vector3::new(1.0, 0.95, 0.7) * arc_bright * 0.6; // Luz blanca/amarilla muy brillante para los arcos
    
    // Asegurar valores máximos y mantener realismo, pero reducir el negro
    Vector3::new(
        (final_color.x).min(1.0).max(0.1),  // Mínimo 0.1 para evitar negro
        (final_color.y * 0.95).min(1.0).max(0.1),  // Mínimo 0.1 para evitar negro
        (final_color.z * 0.8).min(1.0).max(0.1),  // Mínimo 0.1 para evitar negro
    )
}