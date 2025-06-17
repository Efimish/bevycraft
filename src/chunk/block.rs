#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Empty,
    Transparent,
    Opaque
}

pub trait Voxel: Eq {
    fn visibility(&self) -> Visibility;
    fn visible(&self, other: &Self) -> bool;
    fn texture(&self) -> Option<&'static str>;
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    #[default]
    Air,
    Dirt,
    Stone,
    Glass
}

impl Voxel for Block {
    fn visibility(&self) -> Visibility {
        match self {
            Self::Air => Visibility::Empty,
            Self::Dirt => Visibility::Opaque,
            Self::Stone => Visibility::Opaque,
            Self::Glass => Visibility::Transparent
        }
    }

    fn visible(&self, other: &Self) -> bool {
        let self_vis = self.visibility();
        let other_vis = other.visibility();
        match (self_vis, other_vis) {
            (Visibility::Opaque, Visibility::Empty) |
            (Visibility::Opaque, Visibility::Transparent) |
            (Visibility::Transparent, Visibility::Empty) => true,

            (Visibility::Transparent, Visibility::Transparent) => {
                self != other
            },

            _ => false
        }
    }

    fn texture(&self) -> Option<&'static str> {
        Some(match self {
            Self::Dirt => "dirt.png",
            Self::Stone => "stone.png",
            Self::Glass => "glass.png",
            _ => return None,
        })
    }
}