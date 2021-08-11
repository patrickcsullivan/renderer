pub struct Texture<'a> {
    pub desc: wgpu::TextureDescriptor<'a>,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: Option<wgpu::Sampler>,
}

impl<'a> Texture<'a> {
    /// Creates a texture onto which an image can be rendered.
    pub fn create_rgba_output_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        label: &'a str,
    ) -> Self {
        let desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::COPY_SRC | wgpu::TextureUsage::RENDER_ATTACHMENT,
            label: Some(label),
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            desc,
            texture,
            view,
            sampler: None,
        }
    }

    pub fn create_depth_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        label: &'a str,
    ) -> Self {
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT // 3.
                | wgpu::TextureUsage::SAMPLED,
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual), // 5.
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            desc,
            texture,
            view,
            sampler: Some(sampler),
        }
    }
}
