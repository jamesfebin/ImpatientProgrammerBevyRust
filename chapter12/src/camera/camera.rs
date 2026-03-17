// src/camera/camera.rs
use bevy::prelude::*;

use crate::characters::input::Player;
use crate::config::camera::{CAMERA_LERP_SPEED, CAMERA_Z};

/// Marker component for the main game camera.
#[derive(Component)]
pub struct MainCamera;

/// Spawn the main 2D camera.
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainCamera));
}

/// Smoothly follow the player with the camera.
///
/// Uses linear interpolation for smooth movement and snaps to pixel boundaries
/// to prevent subpixel rendering artifacts (grid shimmer).
pub fn follow_camera(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    // Only update when player moves (Changed<Transform>)
    let Some(player_transform) = player_query.iter().next() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let camera_pos = camera_transform.translation.truncate();

    // Early exit if camera is already very close (within 0.5 pixels)
    let distance = player_pos.distance(camera_pos);
    if distance < 0.5 {
        return;
    }

    // Smooth interpolation toward player
    let lerp_factor = (CAMERA_LERP_SPEED * time.delta_secs()).clamp(0.0, 1.0);
    let new_pos = camera_pos.lerp(player_pos, lerp_factor);

    // Snap to pixel boundaries to prevent grid shimmer
    camera_transform.translation.x = new_pos.x.round();
    camera_transform.translation.y = new_pos.y.round();
    camera_transform.translation.z = CAMERA_Z;
}
