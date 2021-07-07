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
}
