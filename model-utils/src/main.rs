use clap::Parser;

const COLOR_TABLE: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    texture_coords: [f32; 2],
}

#[derive(Debug)]
struct Polygon {
    sides: u32,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {}

fn main() {
    let rect = Polygon::new(6);
    println!("{:?}", rect);
}

impl Polygon {
    // Create a new polygon with the given number of sides. The polygon will be
    // centered at the origin and have a radius of 1.0. Triangles are created
    // from the origin and the two points on the unit circle that are the same
    // distance from the origin as the given number of sides.
    fn new(sides: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        // Create the vertices.
        vertices.push(Vertex {
            position: [0.0, 0.0, 0.0],
            texture_coords: [0.5, 0.5],
        });
        for i in 0..sides {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (sides as f32);
            vertices.push(Vertex {
                position: [angle.cos(), angle.sin(), 0.0],
                texture_coords: [(angle.cos() / 2.0) + 0.5, (angle.sin() / 2.0) + 0.5],
            });
        }
        // Create the indices.
        for i in 1..sides {
            indices.push(0);
            indices.push(i as u16);
            indices.push(i as u16 + 1);
        }
        indices.push(0);
        indices.push(sides as u16);
        indices.push(1);
        Self {
            sides,
            vertices,
            indices,
        }
    }
}
