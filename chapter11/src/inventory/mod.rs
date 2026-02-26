use bevy::prelude::*;

use crate::state::GameState;

mod inventory;
mod systems;

pub use inventory::{ItemKind, Pickable, Inventory};
use systems::handle_pickups;

/// Plugin for inventory and pickup functionality.
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_systems(
                Update,
                handle_pickups.run_if(in_state(GameState::Playing)),
            );
    }
}