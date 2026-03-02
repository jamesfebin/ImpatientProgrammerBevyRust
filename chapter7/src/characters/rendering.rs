use bevy::prelude::*;

use crate::characters::state::CharacterState;
use crate::characters::input::Player;
use crate::config::map::{GRID_Y, TILE_SIZE};
use crate::config::player::PLAYER_SCALE;

/// Z-depth constants for proper layering.
/// The tilemap uses `with_z_offset_from_y(true)` which assigns Z based on Y position.
/// We need to match this formula for the player.
const NODE_SIZE_Z: f32 = 1.0;  // Same as tilemap generator
const PLAYER_BASE_Z: f32 = 4.0;  // Match props layer Z range
const PLAYER_Z_OFFSET: f32 = 0.5;  // Small offset to stay above ground props

pub fn update_character_depth(
    mut player_query: Query<&mut Transform, (With<CharacterState>, Changed<Transform>)>,
) {
    // Map dimensions for normalization
    let map_height = TILE_SIZE * GRID_Y as f32;
    let map_y0 = -TILE_SIZE * GRID_Y as f32 / 2.0;  // Map origin Y (centered)
    
    // Player sprite height for feet position calculation
    let player_sprite_height = 64.0 * PLAYER_SCALE;

    for mut transform in player_query.iter_mut() {
        let player_center_y = transform.translation.y;

        // Use player's FEET position for depth sorting (not center)
        let player_feet_y = player_center_y - (player_sprite_height / 2.0);

        // Normalize feet Y to [0, 1] across the grid height
        let t = ((player_feet_y - map_y0) / map_height).clamp(0.0, 1.0);

        // Y-to-Z formula:
        // Lower Y (bottom of screen) = higher t = lower Z offset = rendered in front
        // Higher Y (top of screen) = lower t = higher Z offset = rendered behind
        let player_z = PLAYER_BASE_Z + NODE_SIZE_Z * (1.0 - t) + PLAYER_Z_OFFSET;

        transform.translation.z = player_z;
    }
}