use bevy::prelude::*;

use crate::characters::input::Player;
use super::inventory::{Pickable, Inventory};

/// System that checks for and processes item pickups.
pub fn handle_pickups(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    player_query: Query<&Transform, With<Player>>,
    pickables: Query<(Entity, &GlobalTransform, &Pickable)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let mut collected = Vec::new();

    // Check distance to each pickable
    for (entity, global_transform, pickable) in pickables.iter() {
        let item_pos = global_transform.translation().truncate();
        let distance_sq = player_pos.distance_squared(item_pos);
        
        if distance_sq <= pickable.radius * pickable.radius {
            collected.push((entity, pickable.kind));
        }
    }

    // Process collected items
    for (entity, kind) in collected {
        commands.entity(entity).despawn();
        let count = inventory.add(kind);
        info!(
            " Picked up {} (total: {}) — inventory: {}",
            kind, count, inventory.summary()
        );
    }
}