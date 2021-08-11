// use anyhow::*;
use mesh::Mesh;
use wgpu::util::DeviceExt;

pub trait DescribeBufferLayout {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuVertex {
    position: [f32; 3],
    normal: [f32; 3],
}

impl DescribeBufferLayout for GpuVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<GpuVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

/// Contains GPU-accessible buffers for a mesh.
#[derive(Debug)]
pub struct GpuMeshBuffers {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl GpuMeshBuffers {
    /// Load the mesh into GPU-accessible buffers.
    pub fn load(device: &wgpu::Device, mesh: &Mesh) -> Self {
        let vertices: Vec<GpuVertex> = mesh
            .positions
            .iter()
            .zip(mesh.normals.iter())
            .map(|(p, n)| GpuVertex {
                position: [p.x, p.y, p.z],
                normal: [n.x, n.y, n.z],
            })
            .collect();

        let num_elements = vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&(0..num_elements).collect::<Vec<u32>>()),
            usage: wgpu::BufferUsage::INDEX,
        });

        Self {
            name: "Mesh".to_string(),
            vertex_buffer,
            index_buffer,
            num_indices: num_elements,
        }
    }
}
