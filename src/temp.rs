#![allow(unused)]

// ------------------------------

enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

struct Player {
    game_mode: GameMode
}

// ------------------------------

struct Block {
    is_liquid: bool,
    is_solid: bool,
    is_translucent: bool,
    can_burn: bool,
}

// ------------------------------

// fn chunk_border<C>() -> Mesh
// where
//     C: Chunk
// {
//     let len = C::SIDE as f32;
//     let positions = vec![
//         [0.0, 0.0, 0.0],
//         [len, 0.0, 0.0],
//         [0.0, 0.0, len],
//         [len, 0.0, len],
//         [0.0, len, 0.0],
//         [len, len, 0.0],
//         [0.0, len, len],
//         [len, len, len],
//     ];
//     let indices = vec![
//         0, 1, 0, 2, 3, 1, 3, 2, // bottom 
//         4, 5, 4, 6, 7, 5, 7, 6, // top
//         0, 4, 1, 5, 2, 6, 3, 7 // sides
//     ];
//     new_mesh(positions, indices, vec![], vec![])
// }

// ------------------------------

enum WorldType {
    Overworld,
    Nether,
    End
}
