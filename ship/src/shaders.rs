// shaders.rs
use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::matrix::multiply_matrix_vector4;
use crate::fragment::Fragment;
use crate::framebuffer::Framebuffer; // Importamos Framebuffer desde su módulo
use crate::triangle; // Importamos la función triangle
use crate::light::Light; // Importamos Light desde su módulo

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Convert vertex position to homogeneous coordinates (Vec4) by adding a w-component of 1.0
    let mut position_vec4 = Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    // Modificar la posición si estamos renderizando anillos o luna
    match uniforms.render_type {
        1 => { // rings
            // Generar posición para anillos - solo coordenadas X y Z, Y es cercano a 0
            let angle = (vertex.position.x + vertex.position.z) * 3.0; // Usar coordenadas para generar ángulo
            let radius = 1.5 + (vertex.position.y * 0.1); // Variar radio basado en Y
            position_vec4.x = radius * angle.cos();
            position_vec4.z = radius * angle.sin();
            position_vec4.y = vertex.position.y * 0.1; // Hacer anillo delgado
        }
        2 => { // moon
            // Calcular posición orbital de la luna
            let moon_orbit_time = uniforms.time * 0.5; // Luna orbita más lento
            let moon_distance = 3.0; // Distancia de la luna
            let moon_x = moon_distance * moon_orbit_time.cos();
            let moon_z = moon_distance * moon_orbit_time.sin();
            let moon_y = (moon_orbit_time * 2.0).sin() * 0.5; // Movimiento vertical
            
            // Posición base de la luna
            let moon_base = Vector3::new(moon_x, moon_y, moon_z);
            
            // Añadir posición relativa del vértice
            position_vec4.x = moon_base.x + vertex.position.x * 0.3; // Luna más pequeña
            position_vec4.y = moon_base.y + vertex.position.y * 0.3;
            position_vec4.z = moon_base.z + vertex.position.z * 0.3;
        }
        _ => {} // Planet - usar posición original
    }

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

// Función auxiliar para calcular ruido simple
fn noise(pos: &Vector3) -> f32 {
    let x = pos.x as i32;
    let y = pos.y as i32;
    let z = pos.z as i32;
    
    let n = (x.wrapping_add(y.wrapping_mul(57)).wrapping_add(z.wrapping_mul(113))) as f32;
    ((n * n * 41597.5453).sin() * 43758.5453) % 1.0
}

// Función para generar ruido fractal (más suave)
fn fractal_noise(pos: &Vector3, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        value += noise(&Vector3::new(pos.x * frequency, pos.y * frequency, pos.z * frequency)) * amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    value
}

// Función para simular iluminación basada en el normal
fn simulate_lighting(normal: &Vector3, light_dir: &Vector3) -> f32 {
    let mut light_dir_normalized = *light_dir;
    let length = (light_dir_normalized.x * light_dir_normalized.x + 
                 light_dir_normalized.y * light_dir_normalized.y + 
                 light_dir_normalized.z * light_dir_normalized.z).sqrt();
    
    if length > 0.0 {
        light_dir_normalized.x /= length;
        light_dir_normalized.y /= length;
        light_dir_normalized.z /= length;
    }
    
    let intensity = normal.x * light_dir_normalized.x + 
                   normal.y * light_dir_normalized.y + 
                   normal.z * light_dir_normalized.z;
    
    intensity.max(0.0).min(1.0) * 0.8 + 0.2 // Agrega algo de luz ambiente
}

// Función para aplicar rotación al planeta
fn rotate_planet_position(pos: &Vector3, time: f32, rotation_speed: f32) -> Vector3 {
    let angle = time * rotation_speed;
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    // Rotación alrededor del eje Y (rotación axial)
    Vector3::new(
        pos.x * cos_a - pos.z * sin_a,
        pos.y,
        pos.x * sin_a + pos.z * cos_a
    )
}

// Función para generar patrones de terreno rocoso realista
fn rocky_planet_color(pos: &Vector3, time: f32) -> Vector3 {
    // Aplicar rotación al planeta (rotación sobre su eje)
    let rotated_pos = rotate_planet_position(pos, time, 0.3); // Rotación lenta
    
    // Calcular coordenadas esféricas
    let x = rotated_pos.x;
    let y = rotated_pos.y;
    let z = rotated_pos.z;
    
    // Ruido para crear patrones de terreno
    let base_noise = fractal_noise(&rotated_pos, 4);
    let detail_noise = fractal_noise(&Vector3::new(rotated_pos.x * 8.0, rotated_pos.y * 8.0, rotated_pos.z * 8.0), 2);
    
    // Colores base para planeta rocoso (como Marte o la Luna)
    let base_color = Vector3::new(0.6, 0.4, 0.2); // Marrón rojizo
    let mountain_color = Vector3::new(0.5, 0.3, 0.15); // Zonas más altas
    let valley_color = Vector3::new(0.4, 0.25, 0.1); // Zonas bajas
    
    // Determinar elevación basada en el ruido
    let elevation = (base_noise + detail_noise * 0.3) * 0.5 + 0.5;
    
    // Mezclar colores según la elevación
    let mut color = if elevation > 0.6 {
        // Montañas
        mountain_color
    } else if elevation < 0.4 {
        // Valles
        valley_color
    } else {
        // Terreno plano
        base_color
    };
    
    // Añadir patrones de cráteres
    let crater_noise = fractal_noise(&Vector3::new(rotated_pos.x * 4.0, rotated_pos.y * 4.0, rotated_pos.z * 4.0), 2);
    let crater_factor = (crater_noise * 10.0 + time * 0.1).sin().abs() * 0.2;
    color = color * (1.0 - crater_factor) + Vector3::new(0.3, 0.2, 0.1) * crater_factor;
    
    // Simular iluminación simple
    let light_dir = Vector3::new(1.0, 1.0, 1.0); // Luz del sol
    let lighting = simulate_lighting(&Vector3::new(x, y, z), &light_dir);
    
    color * lighting
}

// Función para generar patrones de planeta gaseoso realista
fn gaseous_planet_color(pos: &Vector3, time: f32) -> Vector3 {
    // Aplicar rotación al planeta (rotación rápida)
    let rotated_pos = rotate_planet_position(pos, time, 0.8); // Rotación rápida como Júpiter
    
    // Coordenadas esféricas para bandas
    let lat = rotated_pos.z.atan2((rotated_pos.x * rotated_pos.x + rotated_pos.y * rotated_pos.y).sqrt());
    let _lon = rotated_pos.y.atan2(rotated_pos.x);
    
    // Ruido para crear bandas horizontales
    let _band_noise = fractal_noise(&Vector3::new(lat * 6.0, 0.0, time * 0.1), 2);
    
    // Colores base para planeta gaseoso (como Júpiter o Saturno)
    let base_color = Vector3::new(0.8, 0.7, 0.6); // Crema
    let band_color1 = Vector3::new(0.9, 0.6, 0.3); // Naranja
    let band_color2 = Vector3::new(0.6, 0.5, 0.7); // Púrpura claro
    let storm_color = Vector3::new(0.8, 0.3, 0.2); // Gran mancha roja
    
    // Efectos de bandas horizontales
    let band_factor = (lat * 10.0 + time * 0.1).sin().abs();
    let mut color = base_color * (1.0 - band_factor * 0.4) + 
                   band_color1 * band_factor * 0.3 + 
                   band_color2 * band_factor * 0.3;
    
    // Añadir efectos de tormenta
    let storm_x = (rotated_pos.x + 0.3).abs() < 0.15;
    let storm_y = (rotated_pos.y - 0.2).abs() < 0.1;
    if storm_x && storm_y {
        color = color * 0.6 + storm_color * 0.4;
    }
    
    // Añadir nubes con ruido
    let cloud_noise = fractal_noise(&Vector3::new(_lon * 12.0, lat * 8.0, time * 0.05), 4);
    let cloud_factor = cloud_noise.max(0.0) * 0.4;
    color = color + Vector3::new(0.9, 0.9, 0.9) * cloud_factor;
    
    // Simular iluminación
    let light_dir = Vector3::new(1.0, 1.0, 1.0);
    let lighting = simulate_lighting(&Vector3::new(rotated_pos.x, rotated_pos.y, rotated_pos.z), &light_dir);
    
    color * lighting
}

// Función para generar un planeta personalizado (azul marino con nubes)
fn custom_planet_color(pos: &Vector3, time: f32) -> Vector3 {
    // Aplicar rotación al planeta (rotación como la Tierra)
    let rotated_pos = rotate_planet_position(pos, time, 0.5); // Rotación moderada
    
    // Coordenadas para patrones
    let lat = rotated_pos.z.atan2((rotated_pos.x * rotated_pos.x + rotated_pos.y * rotated_pos.y).sqrt());
    let _lon = rotated_pos.y.atan2(rotated_pos.x);
    
    // Ruido para tierra y agua
    let terrain_noise = fractal_noise(&rotated_pos, 3);
    
    // Colores base
    let water_color = Vector3::new(0.1, 0.3, 0.6); // Agua profunda
    let shallow_water_color = Vector3::new(0.2, 0.5, 0.8); // Agua poco profunda
    let land_color = Vector3::new(0.2, 0.6, 0.2); // Tierra
    let forest_color = Vector3::new(0.15, 0.5, 0.15); // Bosques
    let snow_color = Vector3::new(0.9, 0.95, 1.0); // Nieve
    
    // Determinar tipo de superficie
    let terrain_factor = terrain_noise * 0.5 + 0.5;
    let mut color = if terrain_factor < 0.3 {
        // Agua profunda
        water_color
    } else if terrain_factor < 0.4 {
        // Agua poco profunda
        shallow_water_color
    } else if terrain_factor < 0.8 {
        // Tierra
        if terrain_factor > 0.7 {
            forest_color
        } else {
            land_color
        }
    } else {
        // Nieve en polos
        if lat.abs() > 1.0 {
            snow_color
        } else {
            land_color
        }
    };
    
    // Añadir nubes
    let cloud_noise = fractal_noise(&Vector3::new(_lon * 10.0, lat * 8.0, time * 0.03), 3);
    let cloud_coverage = 0.3;
    let cloud_factor = (cloud_noise * 0.5 + 0.5).min(1.0) * cloud_coverage;
    
    if cloud_factor > 0.1 {
        color = color * (1.0 - cloud_factor) + Vector3::new(0.95, 0.95, 0.95) * cloud_factor;
    }
    
    // Simular iluminación
    let light_dir = Vector3::new(1.0, 1.0, 1.0);
    let lighting = simulate_lighting(&Vector3::new(rotated_pos.x, rotated_pos.y, rotated_pos.z), &light_dir);
    
    color * lighting
}

// Función para generar un planeta extra - Anillos (como Saturno)
fn ringed_planet_color(pos: &Vector3, time: f32) -> Vector3 {
    // Aplicar rotación al planeta
    let rotated_pos = rotate_planet_position(pos, time, 0.4); // Rotación moderada
    
    // Coordenadas esféricas
    let lat = rotated_pos.z.atan2((rotated_pos.x * rotated_pos.x + rotated_pos.y * rotated_pos.y).sqrt());
    let _lon = rotated_pos.y.atan2(rotated_pos.x);
    
    // Ruido para el planeta
    let _planet_noise = fractal_noise(&rotated_pos, 3);
    
    // Colores base para planeta con anillos
    let base_color = Vector3::new(0.7, 0.6, 0.5); // Color arena
    let band_color = Vector3::new(0.8, 0.7, 0.4); // Bandas
    
    // Color del planeta
    let planet_factor = (lat * 6.0 + time * 0.05).sin().abs() * 0.3;
    let planet_color = base_color * (1.0 - planet_factor) + band_color * planet_factor;
    
    // Simular iluminación
    let light_dir = Vector3::new(1.0, 1.0, 1.0);
    let lighting = simulate_lighting(&Vector3::new(rotated_pos.x, rotated_pos.y, rotated_pos.z), &light_dir);
    
    planet_color * lighting
}

// Función para generar un planeta extra - Lava activa
fn lava_planet_color(pos: &Vector3, time: f32) -> Vector3 {
    // Aplicar rotación al planeta (rotación lenta)
    let rotated_pos = rotate_planet_position(pos, time, 0.2); // Rotación lenta
    
    // Ruido para patrones de lava
    let _base_noise = fractal_noise(&rotated_pos, 4);
    let lava_noise = fractal_noise(&Vector3::new(rotated_pos.x * 6.0, rotated_pos.y * 6.0, rotated_pos.z * 6.0 + time), 3);
    
    // Colores base
    let base_color = Vector3::new(0.2, 0.1, 0.05); // Roca negra
    let cool_lava = Vector3::new(0.6, 0.2, 0.05); // Lava fría
    let hot_lava = Vector3::new(0.9, 0.3, 0.1); // Lava caliente
    let very_hot = Vector3::new(1.0, 0.8, 0.2); // Lava muy caliente
    
    // Determinar actividad de lava
    let lava_activity = lava_noise * 0.5 + 0.5;
    let mut color = if lava_activity > 0.8 {
        very_hot
    } else if lava_activity > 0.6 {
        hot_lava
    } else if lava_activity > 0.4 {
        cool_lava
    } else {
        base_color
    };
    
    // Añadir efectos de erupción
    let eruption_noise = fractal_noise(&Vector3::new(rotated_pos.x * 3.0, rotated_pos.y * 3.0, rotated_pos.z * 3.0 + time * 2.0), 2);
    if eruption_noise > 0.7 {
        color = color * 0.5 + Vector3::new(1.0, 0.9, 0.4) * 0.5; // Destellos
    }
    
    // Simular iluminación (la lava emite luz)
    let light_dir = Vector3::new(1.0, 1.0, 1.0);
    let lighting = simulate_lighting(&Vector3::new(rotated_pos.x, rotated_pos.y, rotated_pos.z), &light_dir);
    
    // La lava también emite luz
    let self_illumination = if color.x > 0.7 { 0.3 } else { 0.0 };
    color * lighting + Vector3::new(self_illumination, self_illumination * 0.5, 0.0)
}

// Función para renderizar anillos
pub fn render_rings(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light) {
    // Vertex Shader Stage para anillos
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    let mut ring_uniforms = uniforms.clone();
    ring_uniforms.render_type = 1; // rings
    
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, &ring_uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle::triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage para anillos
    for fragment in fragments {      
        let ring_color = Vector3::new(0.8, 0.7, 0.6); // Color de anillos
        let ring_normal = Vector3::new(0.0, 1.0, 0.0); // Normal apuntando hacia arriba
        let light_dir = Vector3::new(1.0, 1.0, 1.0);
        let lighting = simulate_lighting(&ring_normal, &light_dir);
        
        let final_color = ring_color * lighting;
        
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color,
            fragment.depth,
        );
    }
}

// Función para renderizar luna
pub fn render_moon(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light) {
    // Vertex Shader Stage para luna
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    let mut moon_uniforms = uniforms.clone();
    moon_uniforms.render_type = 2; // moon
    
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, &moon_uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle::triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage para luna
    for fragment in fragments {      
        let moon_color = Vector3::new(0.7, 0.7, 0.7); // Color de luna
        let moon_normal = Vector3::new(fragment.world_position.x, fragment.world_position.y, fragment.world_position.z);
        let light_dir = Vector3::new(1.0, 1.0, 1.0);
        let lighting = simulate_lighting(&moon_normal, &light_dir);
        
        let final_color = moon_color * lighting;
        
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color,
            fragment.depth,
        );
    }
}

// receives fragment -> returns Vector3 color
pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    let planet_type = uniforms.planet_type;
    
    let color = match planet_type {
        0 => rocky_planet_color(&pos, time), // Planeta rocoso realista
        1 => gaseous_planet_color(&pos, time), // Gigante gaseoso con bandas
        2 => custom_planet_color(&pos, time), // Planeta con agua y tierra
        3 => ringed_planet_color(&pos, time), // Planeta con anillos (Saturno-like)
        4 => lava_planet_color(&pos, time), // Planeta de lava
        _ => Vector3::new(0.5, 0.5, 0.5), // Color por defecto
    };
    
    // Aseguramos que los valores estén entre 0 y 1
    Vector3::new(
        color.x.max(0.0).min(1.0),
        color.y.max(0.0).min(1.0),
        color.z.max(0.0).min(1.0),
    )
}

pub fn set_planet_type(_planet_type: i32) {
    // Esta función podría ser usada si necesitas manejar el estado globalmente
    // Por ahora no es necesaria ya que el tipo se pasa en los uniforms
}