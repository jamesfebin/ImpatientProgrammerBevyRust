use bevy::prelude::*;

use crate::collision::CollisionMap;
use crate::characters::physics::Velocity;
use crate::config::player::{COLLIDER_RADIUS};

/// A circular collider for collision detection.
/// 
/// The collider position is offset from the entity's transform,
/// typically to represent the character's feet position.
#[derive(Component, Debug, Clone)]
pub struct Collider {
    /// Radius of the circular collider in world units
    pub radius: f32,
    /// Offset from entity center (e.g., Vec2(0, -25) for feet)
    pub offset: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            radius: COLLIDER_RADIUS,
            offset: Vec2::ZERO,
        }
    }
}

impl Collider {
    /// Get the world position of this collider given an entity's transform.
    pub fn world_position(&self, transform: &Transform) -> Vec2 {
        transform.translation.truncate() + self.offset
    }
}

pub fn validate_movement(
    map: Option<Res<CollisionMap>>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Velocity, &Collider)>,
) {
    let Some(map) = map else { return };

    for (transform, mut velocity, collider) in query.iter_mut() {
        // Skip if not moving
        if !velocity.is_moving() {
            continue;
        }

        // Current collider position
        let current_pos = collider.world_position(transform);
        
        // Desired new position based on velocity
        let delta = velocity.0 * time.delta_secs();
        let desired_pos = current_pos + delta;

        // Use swept collision to find valid position
        let valid_pos = map.sweep_circle(current_pos, desired_pos, collider.radius);

        // Calculate what velocity would get us to valid_pos
        let actual_delta = valid_pos - current_pos;
        
        // Only update velocity if collision modified our path
        if (actual_delta - delta).length_squared() > 0.001 {
            // Convert position delta back to velocity
            let dt = time.delta_secs();
            if dt > 0.0 {
                velocity.0 = actual_delta / dt;
            }
        }
    }
}

/// Resolve collisions between entities (player and enemies)
/// Prevents entities from moving into each other
pub fn resolve_entity_collisions(
    mut query: Query<(Entity, &Transform, &mut Velocity, &Collider)>,
) {
    // Collect all entity positions first to avoid multiple mutable borrows
    let entities: Vec<_> = query
        .iter()
        .map(|(e, t, _, c)| (e, c.world_position(t), c.radius))
        .collect();

    // Check each entity against all others
    for (entity, transform, mut velocity, collider) in query.iter_mut() {
        // Skip if not moving
        if !velocity.is_moving() {
            continue;
        }

        let pos = collider.world_position(transform);
        let radius = collider.radius;

        for &(other_entity, other_pos, other_radius) in &entities {
            // Skip self
            if entity == other_entity {
                continue;
            }

            let delta = other_pos - pos;
            let distance = delta.length();
            let min_distance = radius + other_radius;

            // Check if entities are overlapping or very close
            if distance < min_distance * 1.1 {
                // Calculate the direction toward the other entity
                if distance > 0.01 {
                    let direction = delta / distance;
                    
                    // Project velocity onto the direction toward the other entity
                    let velocity_toward = velocity.0.dot(direction);
                    
                    // If moving toward the other entity, block that movement
                    if velocity_toward > 0.0 {
                        // Remove the component of velocity moving toward the other entity
                        velocity.0 -= direction * velocity_toward;
                    }
                }
            }
        }
    }
}