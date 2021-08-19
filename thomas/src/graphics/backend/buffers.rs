use cgmath::SquareMatrix;

use super::Camera;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// All vertices will have pos. and color
pub struct Vertex {
    // x, y, z of 3d spce
    pub position: [f32; 3],
    // rgb
    pub color: [f32; 4],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // How wide each vertex is, to skip over
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // When to change vertices
            step_mode: wgpu::VertexStepMode::Vertex,
            // Describe the individual parts of the vertex, which in this case maps to the triangle rust struct equivalent,
            attributes: &[
                // The first field is position, 3 f32s, x, y, z
                wgpu::VertexAttribute {
                    // Offset between each field
                    offset: 0,
                    // Location to store the attribute at, field #, => layout(location=0)
                    shader_location: 0,
                    // [f32; 3]
                    format: wgpu::VertexFormat::Float32x3,
                },
                // The second field is color, 4 f32s, r, g, b, a
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    view: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            view: cgmath::Matrix4::identity().into(),
            model: cgmath::ortho(0.0, x, y, 0.0, -1.0, 1.0).into(),
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
            view: cgmath::Matrix4::identity().into(),
            model: cgmath::ortho(0.0, 800.0, 0.0, 600.0, -1.0, 1.0).into(),
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
            step_mode: wgpu::VertexStepMode::Instance,
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
                },
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

#[cfg(test)]
mod test {
    use cgmath::Vector4;

    use super::Uniforms;
    #[test]
    fn make_sure_ortho_works() {
        let uniforms = Uniforms::new(800.0, 600.0);
        let model = cgmath::Matrix4::from(uniforms.model);

        let src: Vector4<f32> = cgmath::vec4(200.0, 600.0, 0.0, 1.0);

        let res = model * src;

        assert_eq!(res, cgmath::vec4(-0.5, -1.0, 0.0, 1.0));
    }
}
