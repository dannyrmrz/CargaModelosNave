use crate::fragment::Fragment;
use crate::vertex::Vertex;
use nalgebra_glm::Vec2;

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Get transformed positions
    let p0 = v1.transformed_position;
    let p1 = v2.transformed_position;
    let p2 = v3.transformed_position;

    // Find bounding box of the triangle
    let min_x = p0.x.min(p1.x.min(p2.x)) as i32;
    let max_x = p0.x.max(p1.x.max(p2.x)) as i32;
    let min_y = p0.y.min(p1.y.min(p2.y)) as i32;
    let max_y = p0.y.max(p1.y.max(p2.y)) as i32;

    // Rasterize triangle
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = Vec2::new(x as f32, y as f32);
            
            // Calculate barycentric coordinates
            let (b0, b1, b2) = barycentric(
                Vec2::new(p0.x, p0.y),
                Vec2::new(p1.x, p1.y),
                Vec2::new(p2.x, p2.y),
                px
            );

            // Check if point is inside the triangle
            if b0 >= 0.0 && b1 >= 0.0 && b2 >= 0.0 {
                // Interpolate depth (z coordinate)
                let z = p0.z * b0 + p1.z * b1 + p2.z * b2;
                
                // Use black color for solid fill
                let color = crate::color::Color::black();
                
                fragments.push(Fragment::new(x as f32, y as f32, color, z));
            }
        }
    }

    fragments
}

// Helper function to calculate barycentric coordinates
fn barycentric(p0: Vec2, p1: Vec2, p2: Vec2, p: Vec2) -> (f32, f32, f32) {
    let v0 = p2 - p0;
    let v1 = p1 - p0;
    let v2 = p - p0;

    let dot00 = v0.x * v0.x + v0.y * v0.y;
    let dot01 = v0.x * v1.x + v0.y * v1.y;
    let dot02 = v0.x * v2.x + v0.y * v2.y;
    let dot11 = v1.x * v1.x + v1.y * v1.y;
    let dot12 = v1.x * v2.x + v1.y * v2.y;

    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    (1.0 - u - v, v, u)
}

