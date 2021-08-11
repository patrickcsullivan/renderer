mod camera;
mod light;
mod mesh_buffers;
mod render_pipeline;
mod texture;
mod transformation;

use camera::Camera;
use cgmath::Rotation3;
use light::PointLight;
use mesh::Mesh;
use mesh_buffers::GpuMeshBuffers;
use texture::Texture;
use transformation::Transformation;

pub struct Config<'a> {
    pub mesh: &'a Mesh,
    pub dst_path: &'a str,
    pub width: u32,
    pub height: u32,
    pub model_translation: cgmath::Vector3<f32>,
    pub point_light_position: cgmath::Point3<f32>,
    pub camera_position: cgmath::Point3<f32>,
    pub camera_fovy: cgmath::Deg<f32>,
}

/// Generate a screenshot.
pub async fn render(config: Config<'_>) {
    let (device, queue) = request_device().await;

    let depth_texture = texture::Texture::create_depth_texture(
        &device,
        config.width,
        config.height,
        "Depth Texture",
    );

    let model_transformation = transformation::Transformation::new(
        &device,
        cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_x(), cgmath::Deg(-90.0)),
        config.model_translation,
    );

    let camera = Camera::new_perspective_camera(
        &device,
        config.camera_position,
        cgmath::Point3::new(0.0, 0.0, 0.0),
        config.width as f32 / config.height as f32,
        config.camera_fovy,
        0.1,
        1000.0,
    );

    let point_light =
        light::PointLight::new(&device, config.point_light_position.into(), (1.0, 1.0, 1.0));

    let output_texture = texture::Texture::create_rgba_output_texture(
        &device,
        config.width,
        config.height,
        "Output Texture",
    );
    let output_buffer = create_output_buffer(&device, config.width, config.height);
    let render_pipeline = render_pipeline::RenderPipeline::new(
        &device,
        &model_transformation.bind_group_layout,
        &camera.bind_group_layout,
        &point_light.bind_group_layout,
        depth_texture.desc.format,
        output_texture.desc.format,
    );

    let mesh_buffers = GpuMeshBuffers::load(&device, config.mesh);
    render_pipeline.render(
        &device,
        &queue,
        &mesh_buffers,
        &model_transformation.bind_group,
        &camera.bind_group,
        &point_light.bind_group,
        config.width,
        config.height,
        &depth_texture,
        &output_texture,
        &output_buffer,
    );
    save_buffer_to_image(
        &device,
        &output_buffer,
        config.dst_path,
        config.width,
        config.height,
    )
    .await;
    output_buffer.unmap();
}

/// Request the GPU device and its queue.
async fn request_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
        })
        .await
        .unwrap();
    adapter
        .request_device(&Default::default(), None)
        .await
        .unwrap()
}

/// Create the buffer onto which the output image will be written.
fn create_output_buffer(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Buffer {
    let u32_size = std::mem::size_of::<u32>() as u32;
    let output_buffer_size = (u32_size * width * height) as wgpu::BufferAddress;
    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsage::COPY_DST
            // this tells wpgu that we want to read this buffer from the cpu
            | wgpu::BufferUsage::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    device.create_buffer(&output_buffer_desc)
}

/// Poll data from the device and write the output buffer to the destination path.
async fn save_buffer_to_image(
    device: &wgpu::Device,
    output_buffer: &wgpu::Buffer,
    dst_path: &str,
    width: u32,
    height: u32,
) {
    let buffer_slice = output_buffer.slice(..);

    // We have to create the mapping THEN device.poll() before await the
    // future. Otherwise the application will freeze.
    let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
    device.poll(wgpu::Maintain::Wait);
    mapping.await.unwrap();

    let data = buffer_slice.get_mapped_range();

    use image::{ImageBuffer, Rgba};
    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, data).unwrap();
    buffer.save(dst_path).unwrap();
}
