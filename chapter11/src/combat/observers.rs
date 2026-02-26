// src/combat/observers.rs
use super::events::{EntityDeath, ProjectileHit};
use super::health::Health;
use bevy::prelude::*;
use crate::characters::input::Player; 
use crate::state::GameState;

/// Observer that handles projectile hits by applying damage to the target.
pub fn on_projectile_hit(
    hit: On<ProjectileHit>,
    mut healths: Query<&mut Health>,
    mut commands: Commands,
) {
    let Ok(mut health) = healths.get_mut(hit.target) else {
        return;
    };

    health.take_damage(&mut commands, hit.target, hit.damage);

    info!(
        "{:?} hit for {} damage! HP: {:.0}/{:.0}",
        hit.power_type, hit.damage, health.current, health.max
    );
}

/// Observer that handles entity death by despawning the entity.
pub fn on_entity_death(
    death: On<EntityDeath>,
    mut commands: Commands, 
    players: Query<(), With<Player>>, 
    mut next_state: ResMut<NextState<GameState>>,
) {
    let entity = death.entity;
    let is_player = players.get(entity).is_ok();

    info!("Entity {:?} defeated!", death.entity);
    commands.entity(death.entity).despawn();

    // Add this line
    if is_player { 
        info!("Player defeated! Game Over."); 
        next_state.set(GameState::GameOver);
    }
}