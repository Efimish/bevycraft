mod face;
mod mesh;
mod block;
use bevy::prelude::*;
use face::Face;
use mesh::new_mesh;
use block::{Visibility, Voxel, Block};

const CHUNK_SIDE: usize = 16;
const CHUNK_LEN: usize = CHUNK_SIDE * CHUNK_SIDE * CHUNK_SIDE;

trait Chunk {
    type Output;

    const SIDE: usize;

    fn size() -> usize {
        Self::SIDE * Self::SIDE * Self::SIDE
    }

    fn linearize(x: usize, y: usize, z: usize) -> usize {
        x + (y * Self::SIDE) + (z * Self::SIDE * Self::SIDE)
    }

    fn delinearize(mut index: usize) -> (usize, usize, usize) {
        let z = index / (Self::SIDE * Self::SIDE);
        index -= z * Self::SIDE * Self::SIDE;

        let y = index / Self::SIDE;
        index -= y * Self::SIDE;

        let x = index;
        
        (x, y, z)
    }

    fn get(&self, pos: IVec3) -> Self::Output;
}

#[derive(Component)]
struct MyChunk {
    position: IVec3,
    voxels: [Block; CHUNK_LEN]
}

impl Default for MyChunk {
    fn default() -> Self {
        Self {
            position: IVec3::ZERO,
            voxels: [Block::default(); CHUNK_LEN]
        }
    }
}

impl MyChunk {
    fn flat() -> Self {
        let mut voxels = [Block::default(); CHUNK_LEN];
        for i in 0..Self::size() {
            let (_, y, _) = Self::delinearize(i);
            if y <= 3 {
                voxels[i] = Block::Dirt;
            }
        }
        Self {
            position: IVec3::ZERO,
            voxels
        }
    }

    fn with_position(mut self, position: IVec3) -> Self {
        self.position = position;
        self
    }
}

impl Chunk for MyChunk {
    type Output = Block;

    const SIDE: usize = CHUNK_SIDE;

    fn get(&self, pos: IVec3) -> Self::Output {
        if pos.min_element() < 0
        || pos.max_element() >= Self::SIDE as i32
        {
            Self::Output::default()
        } else {
            self.voxels[Self::linearize(pos.x as usize, pos.y as usize, pos.z as usize)]
        }
    }
}

fn simple_mesh<C, T>(chunk: &C) -> Vec<Face>
where
    C: Chunk<Output = T>,
    T: Voxel
{
    assert!(C::SIDE >= 2, "chunk side is too small");

    let mut buffer = Vec::new();
    for i in 0..C::size() {
        let (x, y, z) = C::delinearize(i);

        let pos = IVec3::new(x as i32, y as i32, z as i32);
        let voxel = chunk.get(pos);

        if voxel.visibility() == Visibility::Empty {
            continue;
        }

        let neighbors = [
            chunk.get(pos - IVec3::X),
            chunk.get(pos + IVec3::X),
            chunk.get(pos - IVec3::Y),
            chunk.get(pos + IVec3::Y),
            chunk.get(pos - IVec3::Z),
            chunk.get(pos + IVec3::Z),
        ];

        for (i, neighbor) in neighbors.into_iter().enumerate() {
            let generate = voxel.visible(&neighbor);
            if generate {
                buffer.push(Face {
                    side: i.into(),
                    position: pos.as_uvec3(),
                });
            }
        }
    }
    buffer
}

// ----------------------------------------

fn render_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    let chunks = [
        MyChunk::flat()
            .with_position(IVec3::new(0, 0, 0)),
        MyChunk::flat()
            .with_position(IVec3::new(-1, 0, 0)),
        MyChunk::flat()
            .with_position(IVec3::new(0, 0, -1)),
        MyChunk::flat()
            .with_position(IVec3::new(-1, 0, -1)),
    ];
    for chunk in chunks {
        let faces = simple_mesh(&chunk);
    
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
    
        for face in faces {
            indices.extend_from_slice(&face.indices(positions.len() as u32));
            positions.extend_from_slice(&face.positions(1.0));
            normals.extend_from_slice(&face.normals());
            uvs.extend_from_slice(&face.uvs());
        }
    
        let mesh = new_mesh(positions, indices, uvs, normals);
        let mesh_handle = meshes.add(mesh);
        let texture = asset_server.load("dirt.png");

        let name = format!("Chunk at {}, {}, {}", chunk.position.x, chunk.position.y, chunk.position.z);
        commands.spawn((
            PbrBundle {
                mesh: mesh_handle,
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture),
                    unlit: true, // remove later
                    ..default()
                }),
                transform: Transform::default()
                    .with_translation(chunk.position.as_vec3() * MyChunk::SIDE as f32),
                ..default()
            },
            Name::new(name)
        ));
    }
}


pub struct SpawnChunkPlugin;

impl Plugin for SpawnChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, render_chunk);
    }
}