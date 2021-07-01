pub mod buffers;
pub mod challenges;
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::{
    wgpu_boilerplate::buffers::{Vertex, INDICES, VERTICES},
    World,
};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: PhysicalSize<u32>,
    render_pipelines: Vec<wgpu::RenderPipeline>,
    selected_rd_pipeline_idx: usize,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // First create the wgpu instance, choosing the primary backend
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // Create the surface to draw on (from window, which we get from winit)
        let surface = unsafe { instance.create_surface(window) };

        // Create the adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        // Create the device from adapter
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            flags: wgpu::ShaderFlags::all(),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipelin"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                // Specify the entry point function for shaders, set by [[stage(fragment)]]
                entry_point: "main",
                // We should pass in info into the shader itself, right now we're creating it in the shader for hello world
                buffers: &[Vertex::desc()],
            },
            // Fragment technically opt
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main",
                // Target color output for swap chain, replace old pixels, and write to all colors
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                // Each of 3 vertices (of x, y) correspond to vertices of traingle
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // Ccw = triangle is facing forward
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let render_pipeline_2 = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                // Specify the entry point function for shaders, set by [[stage(fragment)]]
                entry_point: "main",
                // We should pass in info into the shader itself, right now we're creating it in the shader for hello world
                buffers: &[Vertex::desc()],
            },
            // Fragment technically opt
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main_2",
                // Target color output for swap chain, replace old pixels, and write to all colors
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                // Each of 3 vertices (of x, y) correspond to vertices of traingle
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // Ccw = triangle is facing forward
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let render_pipelines = vec![render_pipeline, render_pipeline_2];

        let selected_rd_pipeline_idx = 0;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(buffers::VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipelines,
            selected_rd_pipeline_idx,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn input(&mut self, event: &WindowEvent, world: &mut World) -> bool {
        // return false;
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                *world.cursor_pos_mut() = (position.x, position.y);
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipelines[self.selected_rd_pipeline_idx]);
            // slot = what buffer slot to use for buffer (can have mult buffers)
            // 2nd = slice of buffer to use
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer( self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices,0, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    /// Get a reference to the state's size.
    pub fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }
}

impl State {
    fn update_sc_size(&mut self) {
        self.sc_desc.width = self.size.width;
        self.sc_desc.height = self.size.height;
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        // Update swap chain description based off new size
        self.update_sc_size();
        // update swap chain based of new swap description
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}
