use bevy::prelude::*;
use bevy::render::{
    mesh::{
        Indices,
        PrimitiveTopology,
        VertexAttributeValues
    }, render_asset::RenderAssetUsages
};

pub fn new_mesh(
    positions: Vec<[f32; 3]>,
    indices: Vec<u32>,
    uvs: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>
) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions)
        )
        .with_inserted_indices(
            Indices::U32(indices)
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(uvs)
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(normals)
        )
}