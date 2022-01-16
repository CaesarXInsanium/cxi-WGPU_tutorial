use std::mem::size_of;

use bytemuck::{self, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    /// easier method of creating vertex positions and offsets and good stuff
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    /// Must make sure that layout described in this function
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
enum Indeces {
    Index(Vec<u32>),
    Count(u32),
}

pub struct Mesh {
    name: String,
    verts: Vec<Vertex>,
    index: Indeces,
}
pub struct RenderObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub count: u32,
}

impl RenderObject {
    pub fn from_mesh(mesh: Mesh, device: &wgpu::Device) -> Self {
        Self {
            vertex_buffer: mesh.to_vertex_buffer(device),
            index_buffer: mesh.to_index_buffer(device),
            count: mesh.verts.len() as u32,
        }
    }
}

impl Mesh {
    pub fn to_vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(self.name.as_str()),
            contents: bytemuck::cast_slice(self.verts.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
    pub fn to_index_buffer(&self, device: &wgpu::Device) -> Option<wgpu::Buffer> {
        let label = self.name.to_string() + "_indeces";
        let indices = match &self.index {
            Indeces::Index(v) => v,
            Indeces::Count(_) => {
                return None;
            }
        };
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label.as_str()),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Some(buffer)
    }
    pub fn triangle() -> Self {
        Self {
            name: "Triangle".to_string(),
            verts: TRIANGLE_VERTICES.to_vec(),
            index: Indeces::Count(TRIANGLE_VERTICES.len() as u32),
        }
    }
}
