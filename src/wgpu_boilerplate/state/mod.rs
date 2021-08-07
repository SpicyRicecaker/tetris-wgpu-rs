mod adapter;
pub mod buffer_queue;
mod clear_background;
mod device_queue;
pub mod render;
mod render_pipeline;
mod shader;
mod surface;
mod swap_chain;
mod texture;

use crate::wgpu_boilerplate::state::clear_background::Background;

use super::buffers;
use super::camera::camera_controller::CameraController;
use super::camera::Camera;
use buffers::Uniforms;
use wgpu::{
    util::{DeviceExt, StagingBelt},
    BufferDescriptor, Color, SwapChainTexture, TextureFormat,
};

use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub struct FontInterface {
    staging_belt: StagingBelt,
    glyph_brush: GlyphBrush<()>,
}

impl FontInterface {
    pub fn new(device: &wgpu::Device, format: TextureFormat) -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let visitor =
            ab_glyph::FontArc::try_from_slice(include_bytes!("..\\..\\..\\assets\\visitor2.ttf"))
                .unwrap();

        let glyph_brush = GlyphBrushBuilder::using_font(visitor).build(device, format);

        Self {
            staging_belt,
            glyph_brush,
        }
    }

    pub fn finish(&mut self) {
        self.staging_belt.finish()
    }

    pub fn queue(
        &mut self,
        size: winit::dpi::PhysicalSize<u32>,
        text: &str,
        x: f32,
        y: f32,
        color: Color,
        scale: f32,
    ) {
        self.glyph_brush.queue(Section {
            screen_position: (x, y),
            bounds: (size.width as f32, size.height as f32),
            text: vec![Text::new(text)
                .with_color([
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    color.a as f32,
                ])
                .with_scale(scale)],
            ..Section::default()
        });
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: winit::dpi::PhysicalSize<u32>,
        frame: &SwapChainTexture,
    ) {
        self.glyph_brush
            .draw_queued(
                device,
                &mut self.staging_belt,
                encoder,
                &frame.view,
                size.width,
                size.height,
            )
            .expect("Draw queued");
    }
}

pub struct State {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,

    instances: Vec<buffers::Instance>,
    instance_buffer: wgpu::Buffer,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,

    pub vertices: Vec<super::buffers::Vertex>,
    pub indices: Vec<u16>,

    pub font_interface: FontInterface,

    pub camera: Camera,
    camera_controller: CameraController,

    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    pub background: clear_background::Background,
}

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        // First create the wgpu instance, choosing the primary backend
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let surface = Self::create_surface(&instance, window);

        let adapter = Self::create_adapter(&instance, &surface).await;

        let (device, queue) = Self::create_device_queue(&adapter).await;
        let sc_desc = Self::create_swap_chain_descriptor(&size, &adapter, &surface);

        let swap_chain = Self::create_swap_chain(&sc_desc, &surface, &device);

        let camera = Camera::new(sc_desc.width as f32, sc_desc.height as f32);

        let camera_controller: CameraController = CameraController::new(0.2);

        let mut uniforms = Uniforms::new(sc_desc.width as f32, sc_desc.height as f32);
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
                label: Some("Uniform Bind Group Layout"),
            });

        // create uniform bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let shader = Self::create_shader(&device);

        let render_pipeline_layout = Self::create_render_pipeline_layout(
            &device,
            // &texture_bind_group_layout,
            &uniform_bind_group_layout,
        );

        let render_pipeline =
            Self::create_render_pipeline(&render_pipeline_layout, &sc_desc, &device, &shader);

        let font_interface = FontInterface::new(&device, sc_desc.format);

        let vertices = Vec::new();
        let indices = Vec::new();

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 0,
            usage: wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Index Buffer"),
            size: 0,
            usage: wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let instance = buffers::Instance {
            position: cgmath::vec3(0.0, 0.0, 0.0),
        };
        let instances = vec![instance];
        let instances_data = instances.iter().map(|i| i.to_raw()).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: 2000,
            usage: wgpu::BufferUsage::INDEX,
            mapped_at_creation: true,
        });
        let background = Background::default();
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
            render_pipeline,
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            instances,
            instance_buffer,
            // diffuse_bind_group,
            // diffuse_texture,
            font_interface,
            background,
        }
    }
}
