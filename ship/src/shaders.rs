use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::matrix::multiply_matrix_vector4;
use crate::fragment::Fragment;

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    // Convierte el normal a coordenadas homogéneas (añade coordenada w = 0.0)
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);

    let transformed_normal_vec4 = multiply_matrix_vector4(model_matrix, &normal_vec4);

    // Convierte de vuelta a Vector3 y normaliza
    let mut transformed_normal = Vector3::new(
        transformed_normal_vec4.x,
        transformed_normal_vec4.y,
        transformed_normal_vec4.z,
    );
    
    transformed_normal.normalize();
    transformed_normal
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Convert vertex position to homogeneous coordinates (Vec4) by adding a w-component of 1.0
    let position_vec4 = Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    // Apply Model transformation
    let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

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
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
    }
}

// Función de ruido pseudoaleatorio para efectos solares
fn solar_noise(x: f32, y: f32, z: f32, time: f32) -> f32 {
    // Combinación de funciones trigonométricas para simular ruido
    let n1 = (x * 3.0 + time * 0.7).sin() * (y * 2.0 + time * 0.5).cos() * (z * 4.0 + time * 0.3).sin();
    let n2 = (x * 6.0 + time * 1.2).cos() * (y * 3.0 + time * 0.8).sin() * (z * 2.0 + time * 1.1).cos();
    let n3 = (x * 12.0 + time * 2.0).sin() * (y * 8.0 + time * 1.5).cos() * (z * 6.0 + time * 0.9).sin();
    
    // Combinar diferentes frecuencias para efecto más complejo
    (n1 * 0.5 + n2 * 0.3 + n3 * 0.2).abs()
}

// Shader simple para cualquier objeto que no tenga un shader específico
pub fn fragment_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    // Color gris simple para ahorrar recursos
    fragment.color
}

// Shader específico para el sol con efectos complejos
pub fn sun_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Calcular ruido en múltiples escalas para efecto de turbulencia solar
    let turbulence = solar_noise(pos.x, pos.y, pos.z, time) * 0.6 +
                    solar_noise(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0, time + 100.0) * 0.3 +
                    solar_noise(pos.x * 4.0, pos.y * 4.0, pos.z * 4.0, time + 200.0) * 0.1;
    
    // Efecto de pulsación cíclica
    let pulsation = (time * 1.0).sin().abs() * 0.2 + 0.8;
    
    // Efecto basado en la distancia desde el centro para simular capas
    let distance_from_center = pos.length();
    
    // Color base de la estrella (tonos cálidos)
    let core_color = Vector3::new(1.0, 0.3, 0.1);      // Rojo intenso central
    let surface_color = Vector3::new(1.0, 0.6, 0.2);   // Naranja superficial
    let corona_color = Vector3::new(1.0, 0.9, 0.4);    // Amarillo de la corona
    
    // Determinar zona de la estrella basada en la distancia
    let zone_factor = if distance_from_center < 0.7 {
        0.0  // núcleo
    } else if distance_from_center < 0.9 {
        (distance_from_center - 0.7) / 0.2  // superficie
    } else {
        (distance_from_center - 0.9) / 0.1  // corona
    }.min(1.0);
    
    // Mezclar colores según la zona
    let base_color = if zone_factor < 0.5 {
        let t = zone_factor * 2.0;
        core_color * (1.0 - t) + surface_color * t
    } else {
        let t = (zone_factor - 0.5) * 2.0;
        surface_color * (1.0 - t) + corona_color * t
    };
    
    // Aplicar efectos de turbulencia y pulsación
    let intensity = (turbulence * 1.5 + pulsation) * 0.8;
    
    // Efecto de "llamaradas" solares aleatorias
    let solar_flare_noise = solar_noise(pos.x * 0.5, pos.y * 0.5, pos.z * 0.5, time * 2.0);
    let flare_effect = (solar_flare_noise * 2.0 + (time * 3.0).sin().abs() * 0.5).min(1.0);
    
    // Combinar todo para el color final
    let final_color = base_color * intensity * (1.0 - flare_effect * 0.3) + 
                     Vector3::new(1.0, 1.0, 0.8) * flare_effect * 0.7;
    
    // Asegurar que los valores estén en el rango [0, 1]
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Nuevo shader específico para Mercurio
pub fn mercury_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Simular cráteres y superficie rocosa de Mercurio
    let crater_pattern = (pos.x * 8.0).sin() * (pos.y * 8.0).cos() * (pos.z * 8.0).sin();
    let crater_depth = (crater_pattern * 0.5 + 0.5).powf(3.0); // Cráteres más pronunciados
    
    // Textura rocosa con variaciones
    let surface_noise = (pos.x * 15.0 + pos.z * 10.0).sin() * (pos.y * 12.0).cos();
    let rocky_pattern = (surface_noise * 0.3 + 0.7).abs();
    
    // Colores base de Mercurio (tonos grises rocosos)
    let dark_surface = Vector3::new(0.3, 0.3, 0.35);   // Gris oscuro de llanuras
    let light_surface = Vector3::new(0.55, 0.5, 0.48); // Gris claro de zonas elevadas
    let crater_color = Vector3::new(0.2, 0.2, 0.22);   // Gris muy oscuro de cráteres
    
    // Mezclar colores según profundidad de cráteres
    let base_color = if crater_depth < 0.3 {
        // Dentro de un cráter
        crater_color * (1.0 - crater_depth * 2.0) + dark_surface * crater_depth * 2.0
    } else {
        // Superficie normal
        dark_surface * (1.0 - crater_depth) + light_surface * crater_depth
    };
    
    // Aplicar textura rocosa
    let textured_color = base_color * rocky_pattern;
    
    // Efecto sutil de reflejo solar en zonas expuestas (Mercurio está muy cerca del sol)
    let sun_exposure = (pos.y + 1.0) * 0.5; // Zonas superiores más iluminadas
    let sun_reflection = Vector3::new(0.7, 0.65, 0.6) * sun_exposure * 0.15;
    
    let final_color = textured_color + sun_reflection;
    
    // Asegurar que los valores estén en el rango [0, 1]
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}