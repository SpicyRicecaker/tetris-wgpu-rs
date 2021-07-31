use crate::wgpu_boilerplate::buffers::Vertex;

use super::State;

use wgpu::util::DeviceExt;

impl State {
    pub fn create_buffer(
        device: &wgpu::Device,
        label: Option<&str>,
        contents: &[u8],
        usage: wgpu::BufferUsage,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents,
            usage,
        })
    }
    pub fn update_buffer(
        &mut self,
        slice: &[Vertex]
    ) {
        let new_buffer = Self::create_buffer(
            &self.device,
            Some("Player vertex buffer"),
            bytemuck::cast_slice(slice),
            wgpu::BufferUsage::VERTEX,
        );
        self.vertex_buffers[0] = new_buffer;
        self.num_vertices[0] = slice.len() as u32;
    }
}
