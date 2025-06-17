use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::VertexFormat;

pub const ATTRIBUTE_BASE_VOXEL_INDICES: MeshVertexAttribute =
    MeshVertexAttribute::new("BaseVoxelIndices", 988540917, VertexFormat::Uint32);
pub const ATTRIBUTE_OVERLAY_VOXEL_INDICES: MeshVertexAttribute =
    MeshVertexAttribute::new("OverlayVoxelIndices", 593015852, VertexFormat::Uint32);

#[derive(Asset, AsBindGroup, Clone, TypePath)]
pub struct ChunkMaterial {
    #[texture(0, dimension="2d_array")]
    #[sampler(1)]
    pub texture: Handle<Image>,

    #[texture(2, dimension="2d_array")]
    #[sampler(3)]
    pub pbr_texture: Handle<Image>
}

impl Material for ChunkMaterial {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/chunk.wgsl".into()
    }

    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/chunk.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayout,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError>
    {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_BASE_VOXEL_INDICES.at_shader_location(7),
            ATTRIBUTE_OVERLAY_VOXEL_INDICES.at_shader_location(8),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}