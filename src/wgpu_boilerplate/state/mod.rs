mod adapter;
mod buffer;
mod device_queue;
pub mod render;
mod render_pipeline;
mod shader;
mod surface;
mod swap_chain;

mod challenges;

use super::buffers;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipelines: Vec<wgpu::RenderPipeline>,
    selected_rd_pipeline_idx: usize,
    vertex_buffers: [wgpu::Buffer; 2],
    index_buffers: [wgpu::Buffer; 2],
    selected_buffer_idx: usize,
    num_indices: [u32; 2],
}

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        // First create the wgpu instance, choosing the primary backend
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let surface = Self::create_surface(&instance, window);

        let adapter = Self::create_adapter(&instance, &surface).await;

        let (device, queue) = Self::create_device_queue(&adapter).await;

        let sc_desc = Self::create_swap_chain_descriptor(&surface, &size, &adapter);

        let swap_chain = Self::create_swap_chain(&sc_desc, &surface, &device);

        let shader = Self::create_shader(&device);

        let render_pipeline_layout = Self::create_render_pipeline_layout(&device);

        let render_pipeline_1 =
            Self::create_render_pipeline(&render_pipeline_layout, &sc_desc, &device, &shader);

        let render_pipeline_2 =
            Self::create_render_pipeline_2(&render_pipeline_layout, &sc_desc, &device, &shader);

        let render_pipelines = vec![render_pipeline_1, render_pipeline_2];

        let selected_rd_pipeline_idx = 0;

        let vertex_buffer_pentagon = Self::create_buffer(
            &device,
            Some("Pentagon Vertex Buffer"),
            bytemuck::cast_slice(buffers::VERTICES_PENTAGON),
            wgpu::BufferUsage::VERTEX,
        );
        let index_buffer_pentagon = Self::create_buffer(
            &device,
            Some("Pentagon Index Buffer"),
            bytemuck::cast_slice(buffers::INDICES_PENTAGON),
            wgpu::BufferUsage::INDEX,
        );

        let vertex_buffer_hexagon = Self::create_buffer(
            &device,
            Some("Pentagon Vertex Buffer"),
            bytemuck::cast_slice(buffers::VERTICES_HEXAGON),
            wgpu::BufferUsage::VERTEX,
        );
        let index_buffer_hexagon = Self::create_buffer(
            &device,
            Some("Pentagon Index Buffer"),
            bytemuck::cast_slice(buffers::VERTICES_HEXAGON),
            wgpu::BufferUsage::INDEX,
        );

        let num_indices_pentagon = buffers::INDICES_PENTAGON.len() as u32;
        let num_indices_hexagon = buffers::INDICES_HEXAGON.len() as u32;

        let num_indices = [num_indices_pentagon, num_indices_hexagon];

        let vertex_buffers = [vertex_buffer_pentagon, vertex_buffer_hexagon];
        let index_buffers = [index_buffer_pentagon, index_buffer_hexagon];

        let selected_buffer_idx = 0;

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipelines,
            selected_rd_pipeline_idx,
            vertex_buffers,
            index_buffers,
            selected_buffer_idx,
            num_indices,
        }
    }
}
