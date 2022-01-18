use std::mem::size_of;

use bytemuck;
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
            attributes: Self::ATTRIBUTES,
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

impl Mesh {
    pub fn count(&self) -> u32{
        match &self.index{
            Indeces::Index(inx) => inx.len() as u32,
            Indeces::Count(x) => x.clone(),
        }
    }
    pub fn pentagon() -> Self {
        Self {
            name: "PENTAGON".to_string(),
            verts: vec![
                Vertex {
                    position: [-0.0868241, 0.49240386, 0.0],
                    color: [0.5, 0.9, 0.1],
                },
                Vertex {
                    position: [-0.49513406, 0.06958647, 0.0],
                    color: [0.5, 0.1, 0.2],
                },
                Vertex {
                    position: [-0.21918549, -0.44939706, 0.0],
                    color: [0.5, 0.8, 0.3],
                },
                Vertex {
                    position: [0.35966998, -0.3473291, 0.0],
                    color: [0.5, 0.2, 0.4],
                },
                Vertex {
                    position: [0.44147372, 0.2347359, 0.0],
                    color: [0.5, 0.6, 0.5],
                },
            ],
            index: Indeces::Index(vec![0, 1, 4, 1, 2, 4, 2, 3, 4, 0]),
        }
    }
}
pub struct RenderObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    /// has to equal number of indices of number of vertices
    pub count: u32,
}

impl RenderObject {
    pub fn from_mesh(mesh: Mesh, device: &wgpu::Device) -> Self {
        Self {
            vertex_buffer: mesh.to_vertex_buffer(device),
            index_buffer: mesh.to_index_buffer(device),
            count: mesh.count(),
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
