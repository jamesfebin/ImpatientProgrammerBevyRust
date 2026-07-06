use bevy::prelude::*;
use super::{state::CharacterState, config::CharacterEntry};

/// Linear velocity in world units per second.
/// Systems that want to move an entity modify this.
/// A physics system reads this to update Transform.
#[derive(Component, Debug, Clone, Copy, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub const ZERO: Self = Self(Vec2::ZERO);
    
    pub fn is_moving(&self) -> bool {
        self.0 != Vec2::ZERO
    }
}

pub fn calculate_velocity(
    state: CharacterState,
    direction: Vec2,
    character: &CharacterEntry,
) -> Velocity {
    match state {
        CharacterState::Idle => Velocity::ZERO,
        CharacterState::Jumping => Velocity::ZERO,  // No movement during jump
        CharacterState::Walking => {
            Velocity(direction.normalize_or_zero() * character.base_move_speed)
        }
        CharacterState::Running => {
            Velocity(direction.normalize_or_zero() * character.base_move_speed * character.run_speed_multiplier)
        }
    }
}


pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        if velocity.is_moving() {
            transform.translation += velocity.0.extend(0.0) * time.delta_secs();
        }
    }
}