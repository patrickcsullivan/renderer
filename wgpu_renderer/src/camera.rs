use cgmath::{Matrix4, Point3, Rad, Vector3};
use wgpu::util::DeviceExt;

/// Uniform data that can be sent to the shaders. Contains the camera position
/// and view projection matrix.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniforms {
    pub position: [f32; 3],
    _padding: u32,
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniforms {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            position: [0.0; 3],
            _padding: 0,
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    /// Set the uniform to the given view projection matrix.
    fn update(&mut self, position: Vector3<f32>, view_proj_matrix: Matrix4<f32>) {
        self.position = position.into();
        self.view_proj = view_proj_matrix.into();
    }
}

// The coordinate system in wgpu is based on DirectX and Metal's coordinate
// systems. That means that in normalized device coordinates, the x axis and y
// axis range from -1.0 to +1.0, and the z axis ranges from 0.0 to +1.0. The
// cgmath crate (like most game math crates) is built for OpenGL's coordinate
// system. This matrix will scale and translate our scene from OpenGL's
// coordinate sytem to WGPU's.
#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    // TODO: uniform and ViewProjectionUniform could probably be private
    pub uniform: CameraUniforms,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new_perspective_camera<P: Into<Point3<f32>>, A: Into<Rad<f32>>>(
        device: &wgpu::Device,
        position: P,
        target: P,
        aspect: f32,
        fovy: A,
        znear: f32,
        zfar: f32,
    ) -> Self {
        let position: Point3<f32> = position.into();
        let target: Point3<f32> = target.into();

        let inv_transf_matrix = Matrix4::look_at_rh(position, target, Vector3::unit_y());
        let projection_matrix = cgmath::perspective(fovy, aspect, znear, zfar);
        let view_transf_matrix = OPENGL_TO_WGPU_MATRIX * projection_matrix * inv_transf_matrix;

        let mut uniform = CameraUniforms::new();
        uniform.update(
            Vector3::new(position.x, position.y, position.z),
            view_transf_matrix,
        );

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Perspective Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
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
            label: Some("Perspective Camera Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Perspective Camera Bind Group"),
        });

        Self {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
