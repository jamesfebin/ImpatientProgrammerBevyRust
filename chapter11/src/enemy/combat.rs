// src/enemy/combat.rs
use super::components::{AIBehavior, Enemy, EnemyCombat};
use crate::characters::input::Player;
use crate::combat::systems::{spawn_projectile, ProjectileOwner};
use bevy::prelude::*;

/// System that handles enemy attacks
pub fn enemy_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&GlobalTransform, &mut EnemyCombat, &AIBehavior), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (enemy_transform, mut combat, ai) in enemy_query.iter_mut() {
        // Tick the cooldown timer
        combat.cooldown.tick(time.delta());

        let enemy_pos = enemy_transform.translation();
        let player_pos = player_transform.translation;
        
        // Calculate distance to player
        let distance = enemy_pos.distance(player_pos);

        // Attack if in range and cooldown is ready
        if distance <= ai.attack_range && combat.cooldown.elapsed() >= combat.cooldown.duration() {
            // Calculate direction to player
            let to_player = (player_pos - enemy_pos).normalize();
            let spawn_position = enemy_pos + to_player * 5.0;

            // Get visuals from power type (using actual direction to player)
            let visuals = combat.power_type.visuals(to_player);

            // Spawn projectile (reuse existing function!)
            spawn_projectile(&mut commands, spawn_position, combat.power_type, &visuals, ProjectileOwner::Enemy);


            // Reset cooldown for next attack
            combat.cooldown.reset();

            info!("Enemy fired {:?} projectile at player!", combat.power_type);
        }
    }
}