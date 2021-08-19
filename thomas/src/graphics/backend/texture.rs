use image::GenericImageView;
use anyhow::Result;
use wgpu::TextureFormat;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: TextureFormat,
        bytes: &[u8],
        label: &str
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, format, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        format: TextureFormat,
        label: Option<&str>
    ) -> Result<Self> {
        let rgba = img.as_rgba8().unwrap();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("diffuse_texture"),
            // 1d/2d texture, sizes will be 1? 2d array textures size is # of 2d textures
            size,
            // mipmaps are like downscaled textures that are used more at a distance, to reduce cpu & gpu load and reduce effects
            // obv we don't have any rn so just set it to 1
            mip_level_count: 1,
            sample_count: 1,
            // 2d dimension
            dimension: wgpu::TextureDimension::D2,
            format,
            // Sampled allows use in bind group, copy dst allows texture to be destintation in queue::write_texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                // Texture to be copied to/from
                texture: &texture,
                mip_level: 0,
                // Place the texture is in corresponding to mipmap lvl
                origin: wgpu::Origin3d::ZERO,
                // TODO
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::ImageDataLayout {
                // For non-compressed = 1
                offset: 0,
                // Show just be wxh , but
                // why 4x?
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view =
            texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            // What to do if coordinate is outside texture
            // u = x
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            // v = y
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            // w = z
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // when image needs to be magnified, [linear] blend in the fragments, smooth but blurry
            mag_filter: wgpu::FilterMode::Nearest,
            // when image needs to be scaled down, [nearest] color of nearest pixel, will be pixelated
            min_filter: wgpu::FilterMode::Nearest,
            // filtering between mipmap lvls
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {texture, view, sampler})

    }
}