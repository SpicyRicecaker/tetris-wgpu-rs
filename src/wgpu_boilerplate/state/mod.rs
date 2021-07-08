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
    diffuse_bind_group: wgpu::BindGroup,
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

        use image::GenericImageView;

        let diffuse_bytes = include_bytes!("..\\..\\..\\assets\\memories.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.as_rgba8().unwrap();

        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("diffuse_texture"),
            // 1d/2d texture, sizes will be 1? 2d array textures size is # of 2d textures
            size: texture_size,
            // mipmaps are like downscaled textures that are used more at a distance, to reduce cpu & gpu load and reduce effects
            // obv we don't have any rn so just set it to 1
            mip_level_count: 1,
            sample_count: 1,
            // 2d dimension
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // Sampled allows use in bind group, copy dst allows texture to be destintation in queue::write_texture
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        dbg!(dimensions.0, dimensions.1);

        queue.write_texture(
            wgpu::ImageCopyTexture {
                // Texture to be copied to/from
                texture: &diffuse_texture,
                mip_level: 0,
                // Place the texture is in corresponding to mipmap lvl
                origin: wgpu::Origin3d::ZERO,
            },
            diffuse_rgba,
            wgpu::ImageDataLayout {
                // For non-compressed = 1
                offset: 0,
                // Show just be wxh , but
                // why 4x?
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            texture_size,
        );

        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            // What to do if coordinate is outside texture
            // u = x
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            // v = y
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            // w = z
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // when image needs to be magnified, [linear] blend in the fragments, smooth but blurry
            mag_filter: wgpu::FilterMode::Linear,
            // when image needs to be scaled down, [nearest] color of nearest pixel, will be pixelated
            min_filter: wgpu::FilterMode::Nearest,
            // filtering between mipmap lvls
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

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
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let shader = Self::create_shader(&device);

        let render_pipeline_layout = Self::create_render_pipeline_layout(&device, &texture_bind_group_layout);

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
            render_pipelines,
            selected_rd_pipeline_idx,
            vertex_buffers,
            index_buffers,
            selected_buffer_idx,
            num_indices,
            diffuse_bind_group,
        }
    }
}
