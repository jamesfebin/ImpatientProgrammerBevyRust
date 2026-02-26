// src/characters/rendering.rs
use bevy::prelude::*;

use crate::characters::state::CharacterState; // Line update alert: Change from Player to CharacterState
use crate::config::map::{TOTAL_GRID_Y, TILE_SIZE};
use crate::config::player::PLAYER_SCALE;

/// Z-depth constants for proper layering.
/// The tilemap uses `with_z_offset_from_y(true)` which assigns Z based on Y position.
/// We need to match this formula for all characters (player and enemies). // Line update alert
const NODE_SIZE_Z: f32 = 1.0;  // Same as tilemap generator
const CHARACTER_BASE_Z: f32 = 4.0;  // Match props layer Z range // Line update alert
const CHARACTER_Z_OFFSET: f32 = 0.5;  // Small offset to stay above ground props // Line update alert

pub fn update_character_depth( // Line update alert: Renamed from update_player_depth
    mut character_query: Query<&mut Transform, (With<CharacterState>, Changed<Transform>)>, // Line update alert
) {
    // Map dimensions for normalization
    let map_height = TILE_SIZE * TOTAL_GRID_Y as f32;
    let map_y0 = -TILE_SIZE * TOTAL_GRID_Y as f32 / 2.0;  
    
    // Character sprite height for feet position calculation // Line update alert
    let character_sprite_height = 64.0 * PLAYER_SCALE; // Line update alert

    for mut transform in character_query.iter_mut() { // Line update alert
        let character_center_y = transform.translation.y; // Line update alert

        // Use character's FEET position for depth sorting (not center) // Line update alert
        let character_feet_y = character_center_y - (character_sprite_height / 2.0); // Line update alert

        // Normalize feet Y to [0, 1] across the grid height
        let t = ((character_feet_y - map_y0) / map_height).clamp(0.0, 1.0); // Line update alert

        // Y-to-Z formula:
        // Lower Y (bottom of screen) = higher t = lower Z offset = rendered in front
        // Higher Y (top of screen) = lower t = higher Z offset = rendered behind
        let character_z = CHARACTER_BASE_Z + NODE_SIZE_Z * (1.0 - t) + CHARACTER_Z_OFFSET; // Line update alert

        transform.translation.z = character_z; // Line update alert
    }
}