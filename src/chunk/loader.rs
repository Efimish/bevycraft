use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::player::Player;

use super::MyChunk;
use super::simple_mesh;

const RENDER_DISTANCE: usize = 12;

#[derive(Resource, PartialEq)]
struct CurrentChunk(IVec3);

#[derive(Resource)]
struct ChunkMap {
    chunks: HashMap<IVec3, Entity>
}

impl Default for ChunkMap {
    fn default() -> Self {
        Self { chunks: HashMap::with_capacity((RENDER_DISTANCE * RENDER_DISTANCE) as usize) }
    }
}

#[derive(Component, Default)]
enum ChunkState {
    #[default]
    Loading,
    Unloading,
    Refreshing,
    Loaded,
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentChunk(IVec3::ZERO))
            .insert_resource(ChunkMap::default())
            .add_systems(Update, update_current_chunk);
    }
}

fn update_current_chunk(
    mut current_chunk: ResMut<CurrentChunk>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = player_query.get_single() {
        let pos = player.translation;
        let chunk_pos = (pos / 16.0).floor().as_ivec3();
        if current_chunk.0 != chunk_pos {
            *current_chunk = CurrentChunk(chunk_pos);
        }
    }
}

// fn new_chunk(pos: IVec3) -> MyChunk {
//     MyChunk::flat(5)
//         .with_position(pos)
// }

// fn render_chunks(
//     chunks_query: Query<(Entity, &mut MyChunk, &ChunkState)>,
//     chunk_map: Res<ChunkMap>
// ) {
//     //
// }

// fn render_chunk(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>
// ) {
//     let chunk = MyChunk::flat(5);

//     let result = simple_mesh(&chunk);

//     let mut positions = Vec::new();
//     let mut indices = Vec::new();
//     let mut normals = Vec::new();
//     let mut uvs = Vec::new();

//     for face in result.iter() {
//         indices.extend_from_slice(&face.indices(positions.len() as u32));
//         positions.extend_from_slice(&face.positions(1.0));
//         normals.extend_from_slice(&face.normals());
//         uvs.extend_from_slice(&face.uvs());
//     }

//     let mesh = Mesh::new(
//         PrimitiveTopology::TriangleList,
//         RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
//     )
//         .with_inserted_attribute(
//             Mesh::ATTRIBUTE_POSITION,
//             VertexAttributeValues::Float32x3(positions)
//         )
//         .with_inserted_indices(
//             Indices::U32(indices)
//         )
//         .with_inserted_attribute(
//             Mesh::ATTRIBUTE_UV_0,
//             VertexAttributeValues::Float32x2(uvs)
//         )
//         .with_inserted_attribute(
//             Mesh::ATTRIBUTE_NORMAL,
//             VertexAttributeValues::Float32x3(normals)
//         );
    
//     let mesh_handle = meshes.add(mesh);
//     let texture = asset_server.load("dirt.png");

//     commands.spawn((
//         PbrBundle {
//             mesh: mesh_handle,
//             material: materials.add(StandardMaterial {
//                 base_color_texture: Some(texture),
//                 unlit: true, // remove later
//                 // cull_mode: None,
//                 ..default()
//             }),
//             transform: Transform::from_xyz(0.0, 0.0, 0.0),
//             ..default()
//         },
//         Name::new("BIG CHUNK (of your memory)")
//     ));
// }