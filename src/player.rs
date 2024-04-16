use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{PrimaryWindow, CursorGrabMode};
use crate::keybinds::KeyBindsResource;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_movement,
                player_rotation,
                grab_cursor
            ));
    }
}

// {
//     Player,
//     transform,
//     children[
//         PlayerCamera,
//         Camera3dBundle
//     ]    
// }
#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct PlayerCamera;

pub fn spawn_player(
    mut commands: Commands
) {
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 10.0, 0.0),
        GlobalTransform::IDENTITY,
        Name::new("Player")
    ))
    .with_children(|parent| {
        parent.spawn((
            PlayerCamera,
            Camera3dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::rgb_u8(120, 167, 255)),
                    ..default()
                },
                ..default()
            },
            Name::new("Player camera")
        ));
    });
}

// Not used yet
#[allow(unused)]
pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn_recursive();
    }
}

fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBindsResource>,
    time: Res<Time>
) {
    let player_speed: f32 = 10.0;
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let local_z = if let Ok(camera_transform) = camera_query.get_single() {
            camera_transform.local_z()
        } else {
            player_transform.local_z()
        };
        let forward = Vec3::new(-local_z.x, 0.0, -local_z.z);
        let right = Vec3::new(local_z.z, 0.0, -local_z.x);

        if keyboard_input.pressed(keybinds.move_forward) {
            direction += forward;
        }
        if keyboard_input.pressed(keybinds.move_left) {
            direction -= right;
        }
        if keyboard_input.pressed(keybinds.move_backward) {
            direction -= forward;
        }
        if keyboard_input.pressed(keybinds.move_right) {
            direction += right;
        }
        if keyboard_input.pressed(keybinds.move_up) {
            direction += Vec3::Y;
        }
        if keyboard_input.pressed(keybinds.move_down) {
            direction -= Vec3::Y;
        }
        direction = direction.normalize_or_zero();

        player_transform.translation += direction * player_speed * time.delta_seconds();
    }
}

// Escape to grab cursor, then you can rotate the camera
fn player_rotation(
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut mouse_motion: EventReader<MouseMotion>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mouse_sensitivity: f32 = 0.2;
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(window) = window_query.get_single() {
            for event in mouse_motion.read() {
                if window.cursor.grab_mode != CursorGrabMode::Confined {
                    continue;
                }
                let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
                let delta = event.delta;
    
                pitch -= (delta.y * mouse_sensitivity).to_radians();
                yaw -= (delta.x * mouse_sensitivity).to_radians();
    
                pitch = pitch.clamp(-1.54, 1.54);
    
                camera_transform.rotation
                 = Quat::from_axis_angle(Vec3::Y, yaw)
                 * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    }
}

fn grab_cursor(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBindsResource>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        if keyboard_input.just_pressed(keybinds.grab_cursor) {
            match window.cursor.grab_mode {
                CursorGrabMode::None => {
                    window.cursor.grab_mode = CursorGrabMode::Confined;
                    window.cursor.visible = false;
                },
                _ => {
                    window.cursor.grab_mode = CursorGrabMode::None;
                    window.cursor.visible = true;
                }
            }
        }
    }
}