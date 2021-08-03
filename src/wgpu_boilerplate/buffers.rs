use cgmath::SquareMatrix;

use super::camera::Camera;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// All vertices will have pos. and color
pub struct Vertex {
    // x, y, z of 3d spce
    pub position: [f32; 3],
    // rgb
    // color: [f32; 3],
}

// ccw: top, bot left, bot right
pub const VERTICES_PENTAGON: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
    }, // E
];

// pub const INDICES_PENTAGON: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];

// ccw: top, bot left, bot right
pub const VERTICES_HEXAGON: &[Vertex] = &[
    Vertex {
        position: [-0.25, -0.433, 0.0],
    }, // A = 0
    Vertex {
        position: [0.25, -0.433, 0.0],
    }, // B = 1
    Vertex {
        position: [0.5, 0.0, 0.0],
    }, // C = 2
    Vertex {
        position: [0.25, 0.433, 0.0],
    }, // D = 3
    Vertex {
        position: [-0.25, 0.433, 0.0],
    }, // E = 4
    Vertex {
        position: [-0.5, 0.0, 0.0],
    }, // F = 5
    Vertex {
        position: [0.0, 0.0, 0.0],
    }, // G = 6
];

// pub const INDICES_HEXAGON: &[u16] = &[
//     0, 1, 6, // abg
//     1, 2, 6, // bcg
//     2, 3, 6, // cdg
//     3, 4, 6, // edg
//     4, 5, 6, // efg
//     5, 0, 6, // fag
//     0,
// ];

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
                // wgpu::VertexAttribute {
                //     offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                //     shader_location: 1,
                //     format: wgpu::VertexFormat::Float32x2,
                // },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    view: [[f32; 4]; 4],
    model: [[f32; 4]; 4]
}

impl Uniforms {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            // view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
            view: cgmath::Matrix4::identity().into(),
            // model: cgmath::Matrix4::ortho(0.0, , bottom, top, near, far)
            model: cgmath::ortho(0.0, x, 0.0, y, -1.0, 1.0).into()
        }
    }
    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view = camera.build_view_projection_matrix().into();
    }
}

impl Default for Uniforms {
    /// Probably should not call this
    fn default() -> Self {
        Self {
            // view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
            view: cgmath::Matrix4::identity().into(),
            // model: cgmath::Matrix4::ortho(0.0, , bottom, top, near, far)
            model: cgmath::ortho(0.0, 800.0, 0.0, 600.0, -1.0, 1.0).into()
        }
    }
}

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
}

// NEW!
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 5,
                },

                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },

                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },

                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                }
            ],
        }
    }
}

// NEW!
impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)).into(),
        }
    }
}
