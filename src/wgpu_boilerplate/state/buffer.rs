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
    pub fn update_vertex_buffer(
        &mut self,
        slice: &[Vertex]
    ) {
        let vertex_buffer = Self::create_buffer(
            &self.device,
            Some("vertex buffer"),
            bytemuck::cast_slice(slice),
            wgpu::BufferUsage::VERTEX,
        );
        self.vertex_buffer = vertex_buffer;
        self.num_vertices = slice.len() as u32;
    }
    pub fn update_index_buffer(
        &mut self,
        slice: &[u16]
    ) {
        let index_buffer = Self::create_buffer(
            &self.device,
            Some("index buffer"),
            bytemuck::cast_slice(slice),
            wgpu::BufferUsage::INDEX,
        );
        self.index_buffer = index_buffer;
        self.num_indices = slice.len() as u32;
    }
}
