mod adapter;
mod buffer;
mod device_queue;
pub mod render;
mod render_pipeline;
mod shader;
mod surface;
mod swap_chain;
mod texture;

mod challenges;

use super::buffers;
use super::camera::Camera;
use buffers::Uniforms;
use wgpu::util::DeviceExt;
use super::camera::camera_controller::CameraController;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    camera: Camera,
    camera_controller: CameraController,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    render_pipelines: Vec<wgpu::RenderPipeline>,
    selected_rd_pipeline_idx: usize,
    vertex_buffers: [wgpu::Buffer; 2],
    index_buffers: [wgpu::Buffer; 2],
    selected_buffer_idx: usize,
    num_indices: [u32; 2],
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
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

        let diffuse_bytes = include_bytes!("..\\..\\..\\assets\\memories.png");
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "memories.png").unwrap();

        // bindgroup = resources, & how shader can access them
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        // binding index, matches shading index (e.g. layout(set = 0, binding = 1))
                        binding: 0,
                        // Only visible to fragment shader
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        // ty = type of binding
                        ty: wgpu::BindingType::Texture {
                            // Sampling returns floats
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        // Only visible to fragment shader
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            filtering: true,
                            comparison: false,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // Bind group is a more specific bind group layout, which allows for hotswapping (so long as bind group layouts are shared)
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let camera = Camera::new(sc_desc.width as f32, sc_desc.height as f32);

        let camera_controller: CameraController = CameraController::new(0.2);

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        // recall, bindgroup are resources that the gpu can access through specified shaders
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    // layout
                    binding: 0,
                    // visible only to vertex stage shaders
                    visibility: wgpu::ShaderStage::VERTEX,
                    // ty = type of binding
                    ty: wgpu::BindingType::Buffer {
                        // uniform value buffer
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        // create uniform bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let shader = Self::create_shader(&device);

        let render_pipeline_layout = Self::create_render_pipeline_layout(
            &device,
            &texture_bind_group_layout,
            &uniform_bind_group_layout,
        );

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
            Some("Hexagon Vertex Buffer"),
            bytemuck::cast_slice(buffers::VERTICES_HEXAGON),
            wgpu::BufferUsage::VERTEX,
        );
        let index_buffer_hexagon = Self::create_buffer(
            &device,
            Some("Hexagon Index Buffer"),
            bytemuck::cast_slice(buffers::INDICES_HEXAGON),
            wgpu::BufferUsage::INDEX,
        );

        let num_indices_pentagon = buffers::INDICES_PENTAGON.len() as u32;
        let num_indices_hexagon = buffers::INDICES_HEXAGON.len() as u32;

        let num_indices = [num_indices_pentagon, num_indices_hexagon];

        let vertex_buffers = [vertex_buffer_pentagon, vertex_buffer_hexagon];
        let index_buffers = [index_buffer_pentagon, index_buffer_hexagon];

        let selected_buffer_idx = 0;

        // let mut image_path = std::env::current_dir().unwrap();
        // image_path.push("assets");
        // image_path.push("memories.png");
        // let path_as_str = image_path.to_str().unwrap();

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            camera,
            camera_controller,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            render_pipelines,
            selected_rd_pipeline_idx,
            vertex_buffers,
            index_buffers,
            selected_buffer_idx,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
        }
    }
}
