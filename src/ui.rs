use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, add_ui_camera);
    }
}

pub fn add_ui_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        Name::new("UI camera")
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("crosshair.png"),
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 0.0)),
            ..default()
        },
        Name::new("crosshair")
    ));
}