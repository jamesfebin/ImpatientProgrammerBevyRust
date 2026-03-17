// src/enemy/ai.rs
use super::components::{AIBehavior, Enemy, EnemyPath};
use crate::characters::{
    config::CharacterEntry,
    facing::Facing,
    input::Player,
    physics::{Velocity, calculate_velocity},
    state::CharacterState,
};
use crate::collision::CollisionMap;
use bevy::prelude::*;

/// AI system that makes enemies follow the player using A* pathfinding
pub fn enemy_follow_player(
    time: Res<Time>,
    collision_map: Option<Res<CollisionMap>>,
    mut enemy_query: Query<
        (
            &Transform,
            &mut CharacterState,
            &mut Velocity,
            &mut Facing,
            &CharacterEntry,
            &AIBehavior,
            &mut EnemyPath,
        ),
        With<Enemy>,
    >,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    
    let Some(collision_map) = collision_map else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let delta = time.delta_secs();

    for (enemy_transform, mut state, mut velocity, mut facing, character, ai, mut path) in
        enemy_query.iter_mut()
    {
        let enemy_pos = enemy_transform.translation.truncate();
        let to_player = player_pos - enemy_pos;
        let distance = to_player.length();

        // Outside detection range - go idle
        if distance > ai.detection_range {
            if *state != CharacterState::Idle {
                *state = CharacterState::Idle;
            }
            *velocity = Velocity::ZERO;
            continue;
        }

        // Within attack range - stop and attack
        // Use hysteresis: different threshold for staying vs entering attack mode
        // This prevents oscillation at the boundary
        let attack_threshold = if *state == CharacterState::Idle {
            ai.attack_range + 20.0 // Stay in attack mode even if player moves slightly away
        } else {
            ai.attack_range // Enter attack mode at normal range
        };
        
        if distance <= attack_threshold {
            if *state != CharacterState::Idle {
                *state = CharacterState::Idle;
            }
            *velocity = Velocity::ZERO;
            
            // Face the player while attacking
            let direction = to_player.normalize_or_zero();
            if direction != Vec2::ZERO {
                let new_facing = Facing::from_velocity(direction);
                if *facing != new_facing {
                    *facing = new_facing;
                }
            }
            continue;
        }

        // Need to move toward player - use pathfinding
        path.recalc_timer -= delta;
        
        // Recalculate path if we don't have one
        if !path.has_path() {
            if let Some(waypoints) = collision_map.find_path(enemy_pos, player_pos) {
                path.set_path(waypoints);
                path.recalc_timer = EnemyPath::RECALC_INTERVAL;
            }
        } else if path.recalc_timer <= 0.0 {
            // Periodically update existing path  
            path.recalc_timer = EnemyPath::RECALC_INTERVAL;
            
            if let Some(waypoints) = collision_map.find_path(enemy_pos, player_pos) {
                path.set_path(waypoints);
            }
        }

        // Follow current waypoint
        if let Some(waypoint) = path.current_waypoint() {
            let to_waypoint = waypoint - enemy_pos;
            let waypoint_distance = to_waypoint.length();
            
            // Check if we reached the waypoint
            if waypoint_distance < EnemyPath::WAYPOINT_THRESHOLD {
                path.advance();
            }
            
            // Recalculate direction for current waypoint (might have advanced)
            if let Some(current_wp) = path.current_waypoint() {
                let to_waypoint = current_wp - enemy_pos;
                let direction = to_waypoint.normalize_or_zero();
            
            // Update state
            if *state != CharacterState::Walking {
                *state = CharacterState::Walking;
            }
            
            // Update facing
            if direction != Vec2::ZERO {
                let new_facing = Facing::from_velocity(direction);
                if *facing != new_facing {
                    *facing = new_facing;
                }
            }
                
                // Calculate velocity toward waypoint
                *velocity = calculate_velocity(*state, direction, character);
            }
        } else {
            // No path available - fallback to direct movement
            let direction = to_player.normalize_or_zero();
            
            if *state != CharacterState::Walking {
                *state = CharacterState::Walking;
            }
            
            if direction != Vec2::ZERO {
                let new_facing = Facing::from_velocity(direction);
                if *facing != new_facing {
                    *facing = new_facing;
                }
            }
            
            *velocity = calculate_velocity(*state, direction, character);
        }
    }
}