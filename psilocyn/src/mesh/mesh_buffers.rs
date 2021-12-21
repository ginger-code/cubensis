use hyphae::scene::geometry::GeometrySource;
use hyphae::scene::primitives::PrimitiveType;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        log::trace!("Retrieving Vertex descriptor");
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
pub struct MeshBuffers {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) indirect_buffer: wgpu::Buffer,
    pub(crate) index_count: u32,
}
impl MeshBuffers {
    pub fn new(device: &wgpu::Device, geometry_source: &GeometrySource) -> Self {
        log::debug!("Creating mesh buffers");
        let vertices = get_vertices(geometry_source);
        let indices = get_indices(geometry_source);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::STORAGE,
        });
        let indirect_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Indirect Buffer"),
            contents: bytemuck::cast_slice(
                vec![DrawIndirect {
                    vertex_count: indices.len() as u32,
                    instance_count: 1,
                    base_vertex: 0,
                    base_instance: 0,
                }]
                .as_slice(),
            ),
            usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::STORAGE,
        });
        let index_count = indices.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            indirect_buffer,
            index_count,
        }
    }
}

fn get_vertices(geometry: &GeometrySource) -> &[Vertex] {
    log::trace!("Retrieving vertices");
    match geometry {
        GeometrySource::Primitive(primitive) => match primitive {
            PrimitiveType::Quad => QUAD_VERTICES,
        },
        GeometrySource::ComputeShader(_) => QUAD_VERTICES,
    }
}

fn get_indices(geometry: &GeometrySource) -> &[u16] {
    log::trace!("Retrieving indices");
    match geometry {
        GeometrySource::Primitive(primitive) => match primitive {
            PrimitiveType::Quad => QUAD_INDICES,
        },
        GeometrySource::ComputeShader(_) => QUAD_INDICES,
    }
}

pub const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, 0.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        uv: [1.0, 1.0],
    },
];

#[rustfmt::skip]
pub const QUAD_INDICES: &[u16] = &[
    0, 1, 2,
    0, 3, 1,
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct DrawIndirect {
    vertex_count: u32,   // The number of vertices to draw.
    instance_count: u32, // The number of instances to draw.
    base_vertex: u32,    // The Index of the first vertex to draw.
    base_instance: u32,  // The instance ID of the first instance to draw.
}
