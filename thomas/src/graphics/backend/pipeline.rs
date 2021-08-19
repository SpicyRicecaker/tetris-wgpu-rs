use super::buffers;
use super::State;

impl State {
    pub fn create_render_pipeline_layout(
        device: &wgpu::Device,
        // texture_bind_group_layout: &wgpu::BindGroupLayout,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[uniform_bind_group_layout],
            push_constant_ranges: &[],
        })
    }

    pub fn create_render_pipeline(
        render_pipeline_layout: &wgpu::PipelineLayout,
        sc_desc: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                // Specify the entry point function for shaders, set by [[stage(fragment)]]
                entry_point: "main",
                // We should pass in info into the shader itself, right now we're creating it in the shader for hello world
                buffers: &[buffers::Vertex::desc()],
            },
            // Fragment technically opt
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "main",
                // Target color output for swap chain, replace old pixels, and write to all colors
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
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
        })
    }
}
