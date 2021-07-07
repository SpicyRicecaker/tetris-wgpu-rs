#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// All vertices will have pos. and color
pub struct Vertex {
    // x, y, z of 3d spce
    position: [f32; 3],
    // rgb
    color: [f32; 3],
}

// ccw: top, bot left, bot right
pub const VERTICES_PENTAGON: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];

pub const INDICES_PENTAGON: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];

// ccw: top, bot left, bot right
pub const VERTICES_HEXAGON: &[Vertex] = &[
    Vertex {
        position: [-0.25, -0.433, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // A = 0
    Vertex {
        position: [0.25, -0.433, 0.0],
        color: [1.0, 0.647, 0.0],
    }, // B = 1
    Vertex {
        position: [0.5, 0.0, 0.0],
        color: [1.0, 1.0, 0.0],
    }, // C = 2
    Vertex {
        position: [0.25, 0.433, 0.0],
        color: [0.0, 0.25, 0.0],
    }, // D = 3
    Vertex {
        position: [-0.25, 0.433, 0.0],
        color: [0.0, 0.0, 1.0],
    }, // E = 4
    Vertex {
        position: [-0.5, 0.0, 0.0],
        color: [0.294, 0.0, 0.251],
    }, // F = 5
    Vertex {
        position: [0.0, 0.0, 0.0],
        color: [0.934, 0.251, 0.934],
    }, // G = 6
];

pub const INDICES_HEXAGON: &[u16] = &[
    0, 1, 6, // abg
    1, 2, 6, // bcg
    2, 3, 6, // cdg
    3, 4, 6, // edg
    4, 5, 6, // efg
    5, 0, 6, // fag
    0
];

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // How wide each vertex is, to skip over
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // When to change vertices
            step_mode: wgpu::InputStepMode::Vertex,
            // Describe the individual parts of the vertex, which in this case maps to the triangle rust struct equivalent
            attributes: &[
                wgpu::VertexAttribute {
                    // Offset between each field
                    offset: 0,
                    // Location to store the attribute at, field #, => layout(location=0)
                    shader_location: 0,
                    // [f32; 3]
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
