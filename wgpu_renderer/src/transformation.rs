use cgmath::{Matrix4, Quaternion, Vector3};
use wgpu::util::DeviceExt;

/// Uniform data that can be sent to the shaders. Contains the model
/// transformation matrix.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformationUniform {
    pub transformation: [[f32; 4]; 4],
}

impl TransformationUniform {
    /// Creates a new view transformation matrix uniform, initialized to the
    /// identity matrix.
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            transformation: Matrix4::identity().into(),
        }
    }

    /// Set the uniform to the given transformation matrix.
    fn update(&mut self, transformation: Matrix4<f32>) {
        self.transformation = transformation.into();
    }
}

pub struct Transformation {
    // TODO: uniform and TransformationUniform could probably be private
    pub uniform: TransformationUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Transformation {
    pub fn new(device: &wgpu::Device, transformation: Matrix4<f32>) -> Self {
        let mut uniform = TransformationUniform::new();
        uniform.update(transformation);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Transformation Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Model Transformation Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Model Transformation Bind Group"),
        });

        Self {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
