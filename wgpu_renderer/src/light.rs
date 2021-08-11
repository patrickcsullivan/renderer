use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLightUniform {
    pub position: [f32; 3],
    // The light position will be used as a uniform and uniforms must by 16
    // bytes wide, so we need to add 4 bytes of padding before the color.
    _padding: u32,
    pub color_rgb: [f32; 3],
}

impl PointLightUniform {
    pub fn new(position: (f32, f32, f32), color_rgb: (f32, f32, f32)) -> Self {
        Self {
            position: [position.0, position.1, position.2],
            _padding: 0,
            color_rgb: [color_rgb.0, color_rgb.1, color_rgb.2],
        }
    }
}

pub struct PointLight {
    // TODO: uniform and PointLightUniform could probably be private
    pub uniform: PointLightUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl PointLight {
    pub fn new(
        device: &wgpu::Device,
        position: (f32, f32, f32),
        color_rgb: (f32, f32, f32),
    ) -> Self {
        let uniform = PointLightUniform::new(position, color_rgb);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Light"),
            contents: bytemuck::cast_slice(&[uniform]),
            // COPY_DST enables us to copy new data into this buffer is we want to change the light.
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
