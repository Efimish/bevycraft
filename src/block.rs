use bevy::prelude::*;
use bevy::render::{mesh::{Indices, PrimitiveTopology, VertexAttributeValues}, render_asset::RenderAssetUsages};

pub struct SpawnDirtBlockPlugin;

impl Plugin for SpawnDirtBlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_dirt_block);
    }
}

fn spawn_dirt_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // There are duplicate verticies,
    // but they'll have different UV and normal
    // This is needed to properly set textures
    //
    // Comments after line go like this:
    // [current vertex index], [other vertexes indeces that are in the same position]
    let vertices: Vec<[f32; 3]> = vec![
        // top      (+y)
        [ 0.5,  0.5,  0.5], //  0,  8, 16
        [ 0.5,  0.5, -0.5], //  1,  9, 20
        [-0.5,  0.5,  0.5], //  2, 12, 18
        [-0.5,  0.5, -0.5], //  3, 13, 22
        // bottom   (-y)
        [ 0.5, -0.5,  0.5], //  4, 10, 17
        [ 0.5, -0.5, -0.5], //  5, 11, 21
        [-0.5, -0.5,  0.5], //  6, 14, 19
        [-0.5, -0.5, -0.5], //  7, 15, 23
        // right    (+x)
        [ 0.5,  0.5,  0.5], //  8,  0, 16
        [ 0.5,  0.5, -0.5], //  9,  1, 20
        [ 0.5, -0.5,  0.5], // 10,  4, 17
        [ 0.5, -0.5, -0.5], // 11,  5, 21
        // left     (-x)
        [-0.5,  0.5,  0.5], // 12,  2, 18
        [-0.5,  0.5, -0.5], // 13,  3, 18
        [-0.5, -0.5,  0.5], // 14,  6, 19
        [-0.5, -0.5, -0.5], // 15,  7, 23
        // back     (+z)
        [ 0.5,  0.5,  0.5], // 16,  0,  8
        [ 0.5, -0.5,  0.5], // 17,  4, 10
        [-0.5,  0.5,  0.5], // 18,  2, 12
        [-0.5, -0.5,  0.5], // 19,  6, 14
        // forward  (-z)
        [ 0.5,  0.5, -0.5], // 20,  1,  9
        [ 0.5, -0.5, -0.5], // 21,  5, 11
        [-0.5,  0.5, -0.5], // 22,  3, 13
        [-0.5, -0.5, -0.5], // 23,  7, 15
    ];

    // You can see a pattern here :)
    let indices = Indices::U32(vec![
         1,  3,  0, /**/  2,  0,  3, // top (+y)
         5,  4,  7, /**/  6,  7,  4, // bottom (-y)
         9,  8, 11, /**/ 10, 11,  8, // right (+x)
        13, 15, 12, /**/ 14, 12, 15, // left (-x)
        17, 16, 19, /**/ 18, 19, 16, // back (+z)
        21, 23, 20, /**/ 22, 20, 23, // forward (-z)
    ]);

    // How to set textures on sides
    let uvs = vec![
        [1.0, 0.0], [1.0, 1.0], [0.0, 0.0], [0.0, 1.0], // top
        [1.0, 0.0], [1.0, 1.0], [0.0, 0.0], [0.0, 1.0], // bottom
        [0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0], // right
        [1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0], // left
        [1.0, 0.0], [1.0, 1.0], [0.0, 0.0], [0.0, 1.0], // back (this one is correct)
        [0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0], // forward
    ];

    // For lightning to work properly
    let normals = vec![
        [0.0,  1.0, 0.0], [0.0,  1.0, 0.0], [0.0,  1.0, 0.0], [0.0,  1.0, 0.0], // top      (+y)
        [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], // bottom   (-y)
        [ 1.0, 0.0, 0.0], [ 1.0, 0.0, 0.0], [ 1.0, 0.0, 0.0], [ 1.0, 0.0, 0.0], // right    (+x)
        [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], // left     (-x)
        [0.0, 0.0,  1.0], [0.0, 0.0,  1.0], [0.0, 0.0,  1.0], [0.0, 0.0,  1.0], // back     (+z)
        [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], // forward  (-z)
    ];

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vertices)
        )
        .with_inserted_indices(indices)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(uvs)
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(normals)
        );

    let mesh_handle = meshes.add(mesh);

    let texture = asset_server.load("dirt.png");

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture),
                unlit: true, // remove later
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Name::new("Dirt block")
    ));
}