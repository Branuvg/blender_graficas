// triangle.rs
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::line::line;
use crate::Vector3;
use crate::light::Light;

fn barycentric_coordinates(p_x: f32, p_y: f32, a: &Vertex, b: &Vertex, c: &Vertex)  -> (f32, f32, f32) {
    let a_x = a.transformed_position.x;   
    let b_x = b.transformed_position.x;
    let c_x = c.transformed_position.x;
    let a_y = a.transformed_position.y;
    let b_y = b.transformed_position.y;
    let c_y = c.transformed_position.y;

    let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

    if area.abs() < 1e-10  {
        return (-1.0, -1.0, -1.0);
    }
    
    let w = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
    let v = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
    let u = 1.0 - w - v;

    (w, v, u)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, light: &Light) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let a_x = v1.transformed_position.x;
    let b_x = v2.transformed_position.x;
    let c_x = v3.transformed_position.x;
    let a_y = v1.transformed_position.y;
    let b_y = v2.transformed_position.y;
    let c_y = v3.transformed_position.y;

/*     let color_a = Vector3::new(1.0, 0.0, 0.0);
    let color_b = Vector3::new(0.0, 1.0, 0.0);
    let color_c = Vector3::new(0.0, 0.0, 1.0); */

    let min_x = a_x.min(b_x).min(c_x).floor() as i32;
    let min_y = a_y.min(b_y).min(c_y).floor() as i32;

    let max_x = a_x.max(b_x).max(c_x).ceil() as i32;
    let max_y = a_y.max(b_y).max(c_y).ceil() as i32;

    let light = light.position.normalized();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (w, v, u) = barycentric_coordinates(x  as f32, y as f32, v1, v2, v3);

            let normal = v1.transformed_normal;
            let depth = v1.transformed_position.z * w + v2.transformed_position.z * v + v3.transformed_position.z * u;
            let color = Vector3::new(1.0, 0.5, 0.5);
            //let color = color_a * w + color_b * v + color_c * u;

            let intensity = normal.dot(light).max(0.0);

            let final_color = color * intensity;

            if w >= 0.0 && v >= 0.0 && u >= 0.0 {
                fragments.push(Fragment::new(
                    x as f32,
                    y as f32,
                    final_color,
                    depth,
                ));
            }
        }
    }

    fragments
}