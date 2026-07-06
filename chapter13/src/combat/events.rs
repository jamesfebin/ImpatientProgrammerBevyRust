// src/combat/events.rs
use super::power_type::PowerType;
use bevy::prelude::*;

/// Event triggered when a projectile hits a target entity.
#[derive(Event)]
pub struct ProjectileHit {
    pub target: Entity,
    pub damage: f32,
    pub power_type: PowerType,
}

/// Event triggered when an entity's health reaches zero.
#[derive(Event)]
pub struct EntityDeath {
    pub entity: Entity,
}