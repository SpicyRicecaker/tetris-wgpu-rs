pub mod buffers;
mod bg;
pub mod render;
mod pipeline;
mod shader;
mod texture;
mod camera;

use bg::Background;

use camera::Camera;
use buffers::Uniforms;
use wgpu::{
    util::{DeviceExt, StagingBelt},
    BufferDescriptor, Color, TextureFormat,
};

// use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub struct FontInterface {
    staging_belt: StagingBelt,
    // glyph_brush: GlyphBrush<()>,
}

impl FontInterface {
    pub fn new(device: &wgpu::Device, format: TextureFormat) -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        // let visitor =
        //     ab_glyph::FontArc::try_from_slice(include_bytes!("..\\..\\..\\assets\\visitor2.ttf"))
        //         .unwrap();

        // let glyph_brush = GlyphBrushBuilder::using_font(visitor).build(device, format);

        Self {
            staging_belt,
            // glyph_brush,
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
        // self.glyph_brush.queue(Section {
        //     screen_position: (x, y),
        //     bounds: (size.width as f32, size.height as f32),
        //     text: vec![Text::new(text)
        //         .with_color([
        //             color.r as f32,
        //             color.g as f32,
        //             color.b as f32,
        //             color.a as f32,
        //         ])
        //         .with_scale(scale)],
        //     ..Section::default()
        // });
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: winit::dpi::PhysicalSize<u32>,
        // frame: &SwapChainTexture,
    ) {
        // self.glyph_brush
        //     .draw_queued(
        //         device,
        //         &mut self.staging_belt,
        //         encoder,
        //         &frame.view,
        //         size.width,
        //         size.height,
        //     )
        //     .expect("Draw queued");
    }
}

pub struct State {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,

    pub vertices: Vec<buffers::Vertex>,
    pub indices: Vec<u16>,

    pub font_interface: FontInterface,

    pub camera: Camera,

    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    pub background: bg::Background,
}

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        // First create the wgpu instance, choosing the primary backend
        // Currently only dx12 for compile times
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        // Create the surface to draw on (from window, which we get from winit)
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Unable to find adapter");

        let (device, queue) = adapter
            // Create the device from adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("Unable to create device");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            // Low latency vsync, falls back to Fifo,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        surface.configure(&device, &config);

        let camera = Camera::new(config.width as f32, config.height as f32);

        let mut uniforms = Uniforms::new(config.width as f32, config.height as f32);
        uniforms.update_view_proj(&camera);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // recall, bindgroup are resources that the gpu can access through specified shaders
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    // layout
                    binding: 0,
                    // visible only to vertex stage shaders
                    visibility: wgpu::ShaderStages::VERTEX,
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

        let render_pipeline_layout =
            Self::create_render_pipeline_layout(&device, &uniform_bind_group_layout);

        let render_pipeline =
            Self::create_render_pipeline(&render_pipeline_layout, &config, &device, &shader);

        let font_interface = FontInterface::new(&device, config.format);

        let vertices = Vec::new();
        let indices = Vec::new();

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Index Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let background = Background::default();
        Self {
            surface,
            config,
            device,
            queue,
            size,
            camera,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            render_pipeline,
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            font_interface,
            background,
        }
    }
}
