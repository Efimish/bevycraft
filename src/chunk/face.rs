use bevy::prelude::*;

/// Simply a direction, see [`Face`]
#[derive(Clone, Copy)]
pub enum Side {
    Left,   // -x
    Right,  // +x
    Bottom, // -y
    Top,    // +y
    Front,  // -z
    Back,   // +z
}

impl From<usize> for Side {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Left,   // X-
            1 => Self::Right,  // X+
            2 => Self::Bottom, // Y-
            3 => Self::Top,    // Y+
            4 => Self::Front,  // Z-
            5 => Self::Back,   // Z+
            _ => unimplemented!()
        }
    }
}

impl Into<usize> for Side {
    fn into(self) -> usize {
        match self {
            Self::Left   => 0,
            Self::Right  => 1,
            Self::Bottom => 2,
            Self::Top    => 3,
            Self::Front  => 4,
            Self::Back   => 5,
        }
    }
}

impl Into<Vec3> for Side {
    fn into(self) -> Vec3 {
        match self {
            Self::Right  => Vec3::X,
            Self::Left   => Vec3::NEG_X,
            Self::Top    => Vec3::Y,
            Self::Bottom => Vec3::NEG_Y,
            Self::Back   => Vec3::Z,
            Self::Front  => Vec3::NEG_Z,
        }
    }
}

impl Side {
    pub fn indices(&self, start: u32) -> [u32; 6] {
        [start, start + 2, start + 1, start + 1, start + 2, start + 3]
    }

    pub fn normal(&self) -> [f32; 3] {
        <Self as Into<Vec3>>::into(*self).to_array()
    }

    pub fn normals(&self) -> [[f32; 3]; 4] {
        [self.normal(); 4]
    }

    pub fn uvs(&self) -> [[f32; 2]; 4] {
        [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]]
    }

    pub fn positions(&self) -> [[f32; 3]; 4] {
        match &self {
            Self::Right  => [[1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]],
            Self::Left   => [[0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
            Self::Top    => [[0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]],
            Self::Bottom => [[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            Self::Back   => [[1.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
            Self::Front  => [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
        }
    }
}

/// [`Side`] with position
#[derive(Clone, Copy)]
pub struct Face {
    pub position: UVec3,
    pub side: Side,
}

impl Face {
    pub fn indices(&self, start: u32) -> [u32; 6] {
        self.side.indices(start)
    }
  
    pub fn positions(&self, voxel_size: f32) -> [[f32; 3]; 4] {
        let positions = self.side.positions();

        let [x, y, z] = self.position.as_vec3().to_array();

        [
            [
                (x + positions[0][0]) * voxel_size,
                (y + positions[0][1]) * voxel_size,
                (z + positions[0][2]) * voxel_size,
            ],
            [
                (x + positions[1][0]) * voxel_size,
                (y + positions[1][1]) * voxel_size,
                (z + positions[1][2]) * voxel_size,
            ],
            [
                (x + positions[2][0]) * voxel_size,
                (y + positions[2][1]) * voxel_size,
                (z + positions[2][2]) * voxel_size,
            ],
            [
                (x + positions[3][0]) * voxel_size,
                (y + positions[3][1]) * voxel_size,
                (z + positions[3][2]) * voxel_size,
            ],
        ]
    }
  
    pub fn normals(&self) -> [[f32; 3]; 4] {
        self.side.normals()
    }
  
    pub fn uvs(&self) -> [[f32; 2]; 4] {
        self.side.uvs()
    }
}