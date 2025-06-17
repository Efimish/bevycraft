use bevy::pbr::wireframe::{WireframePlugin, WireframeConfig};
use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod keybinds;
mod player;
mod ui;
mod chunk;
use keybinds::KeyBindPlugin;
use player::PlayerPlugin;
use ui::GameUiPlugin;
use chunk::SpawnChunkPlugin;
use chunk::loader::ChunkPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "BevyCraft".to_string(),
                    ..default()
                }),
                ..default()
            })
            // Default nearest is needed to avoid blurry textures
                .set(ImagePlugin::default_nearest()),
            // Press ` to open the inspector
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backquote)),
            KeyBindPlugin,
            PlayerPlugin,
            GameUiPlugin,
            SpawnChunkPlugin,
            ChunkPlugin
        ))
        .add_plugins(WireframePlugin)
        // .insert_resource(WireframeConfig {
        //     global: true,
        //     default_color: Color::WHITE
        // })
        .run()
}