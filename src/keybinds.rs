use bevy::prelude::*;

pub struct KeyBindPlugin;

#[derive(Resource)]
pub struct KeyBindsResource {
    pub move_forward: KeyCode,
    pub move_left: KeyCode,
    pub move_backward: KeyCode,
    pub move_right: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub grab_cursor: KeyCode,
}

impl Default for KeyBindsResource {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_left: KeyCode::KeyA,
            move_backward: KeyCode::KeyS,
            move_right: KeyCode::KeyD,
            move_up: KeyCode::Space,
            move_down: KeyCode::ShiftLeft,
            grab_cursor: KeyCode::Escape,
        }
    }
}

impl Plugin for KeyBindPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<KeyBindsResource>();
    }
}