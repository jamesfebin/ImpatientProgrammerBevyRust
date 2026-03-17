pub mod components;
pub mod material;
pub mod systems;

use crate::state::GameState;
use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub use material::*;
pub use systems::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing ParticlesPlugin");
        app.add_plugins(Material2dPlugin::<ParticleMaterial>::default())
            .add_systems(
                Update,
                (update_emitters, update_particles, cleanup_finished_emitters)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
        info!("ParticlesPlugin initialized");
    }
}