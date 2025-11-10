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

// Implementación de una función de ruido tipo Perlin simplificada
fn perlin_noise_3d(x: f32, y: f32, z: f32, time: f32) -> f32 {
    // Función de ruido pseudoaleatorio más suave que la trigonométrica
    let x = x + time * 0.3;
    let y = y + time * 0.2;
    let z = z + time * 0.1;
    
    // Combinación de funciones trigonométricas con pesos diferentes
    let n = (x.sin() * 7.3 + y.cos() * 5.1 + z.sin() * 3.7 + 
             x.cos() * y.sin() * 2.3 + y.cos() * z.sin() * 1.9 + 
             x.sin() * z.cos() * 1.1).sin();
    
    // Normalizar al rango [0, 1]
    (n + 1.0) * 0.5
}

// Función de ruido fractal (multi-octave) tipo Perlin
fn fractal_noise(x: f32, y: f32, z: f32, time: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    let mut total_amplitude = 0.0;
    
    // 4 octavas para efecto más detallado
    for i in 0..4 {
        let noise_val = perlin_noise_3d(x * frequency, y * frequency, z * frequency, time + (i as f32) * 100.0);
        value += noise_val * amplitude;
        total_amplitude += amplitude;
        
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    value / total_amplitude
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position_vec4 = Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    // Calcular ruido para distorsión del vertex
    let time = uniforms.time;
    let noise_amount = fractal_noise(
        vertex.position.x * 2.0, 
        vertex.position.y * 2.0, 
        vertex.position.z * 2.0, 
        time
    ) * 0.1; // Pequeña distorsión para efecto de turbulencia
    
    // Aplicar distorsión basada en ruido
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
    
    // Create a new Vertex with the transformed position
    Vertex {
        position: vertex.position, // Mantener la posición original para cálculos
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
    
    // Calcular ruido fractal para efectos solares
    let turbulence = fractal_noise(pos.x, pos.y, pos.z, time);
    let turbulence_detail = fractal_noise(pos.x * 4.0, pos.y * 4.0, pos.z * 4.0, time * 2.0) * 0.3;
    
    // Efecto de pulsación cíclica para simular actividad solar
    let solar_activity = (time * 0.8).sin().abs() * 0.3 + 0.7;
    
    // Calcular distancia desde el centro para efectos de capas
    let distance_from_center = pos.length();
    
    // Gradientes de temperatura de estrella realista
    // De rojo intenso (núcleo) a amarillo/blanco (superficie)
    let temperature_factor = if distance_from_center < 0.8 {
        // Interior más rojo
        let t = distance_from_center / 0.8;
        Vector3::new(1.0, 0.2 + 0.3 * t, 0.1 + 0.2 * t)
    } else {
        // Superficie más amarilla
        let t = (distance_from_center - 0.8) / 0.2;
        let t_clamped = t.min(1.0);
        Vector3::new(1.0, 0.5 + 0.5 * (1.0 - t_clamped), 0.2 + 0.3 * (1.0 - t_clamped))
    };
    
    // Efecto de emisión variable (simular picos de energía)
    let emission_pulse = (time * 2.5 + pos.x * 10.0 + pos.y * 8.0).sin().abs() * 0.4;
    let emission_spikes = fractal_noise(pos.x * 8.0, pos.y * 8.0, pos.z * 8.0, time * 3.0) * 0.6;
    
    // Combinar todos los efectos
    let final_intensity = (turbulence * 1.2 + turbulence_detail + solar_activity * 0.8 + 
                          emission_pulse + emission_spikes * 0.5) * 0.7;
    
    // Color final combinando temperatura y efectos
    let final_color = temperature_factor * final_intensity;
    
    // Asegurar que los valores estén en el rango [0, 1] y añadir brillo adicional
    Vector3::new(
        (final_color.x * 1.2).min(1.0),
        (final_color.y * 1.1).min(1.0),
        final_color.z.min(1.0),
    )
}