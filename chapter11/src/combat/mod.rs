// src/combat/mod.rs
mod events; 
mod observers; 
pub mod health;
pub mod healthbar; 

mod player_combat;
mod power_type;
pub mod systems;

pub use health::Health; 
pub use healthbar::HealthBarOwner;

pub use player_combat::PlayerCombat;
pub use power_type::{PowerType, PowerVisuals};
pub use systems::{debug_switch_power, handle_power_input, spawn_projectile, ProjectileOwner}; 

use bevy::prelude::*;
use crate::state::GameState; 

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register observers for combat events
            .add_observer(observers::on_projectile_hit) 
            .add_observer(observers::on_entity_death) 
            .add_systems(
                Update,
                (
                    handle_power_input,
                    debug_switch_power,
                    systems::move_projectiles, 
                    systems::check_projectile_hits,
                    healthbar::spawn_healthbars,
                    healthbar::update_healthbars,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}