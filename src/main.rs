use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod block;
use player::PlayerPlugin;
use block::SpawnDirtBlockPlugin;

fn main() {
    App::new()
        .add_plugins((
            // Default nearest is needed to avoid blurry textures
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            // Press ` to open the inspector
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backquote)),
            PlayerPlugin,
            SpawnDirtBlockPlugin
        ))
        .run()
}