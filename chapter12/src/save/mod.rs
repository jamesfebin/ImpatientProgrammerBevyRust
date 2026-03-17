pub mod data;
pub mod systems;
pub mod ui;

use bevy::prelude::*;
use crate::state::GameState;

pub use ui::{SaveLoadUIState, SaveLoadMode};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveLoadUIState>()
            .init_resource::<ui::PendingSaveLoadAction>()
            .add_systems(
                Update,
                ui::handle_save_load_ui
                    .run_if(in_state(GameState::Paused).or(in_state(GameState::MainMenu))),
            )
            .add_systems(
                Update,
                ui::handle_slot_buttons
                    .run_if(|ui_state: Res<SaveLoadUIState>| ui_state.active)
                    .run_if(in_state(GameState::Paused).or(in_state(GameState::MainMenu))),
            )
            .add_systems(
                Update,
                ui::handle_back_button
                    .run_if(|ui_state: Res<SaveLoadUIState>| ui_state.active)
                    .run_if(in_state(GameState::Paused).or(in_state(GameState::MainMenu))),
            )
            .add_systems(
                Update,
                ui::execute_save
                    .run_if(|p: Res<ui::PendingSaveLoadAction>| matches!(p.0, Some((SaveLoadMode::Save, _)))),
            )
            .add_systems(
                Update,
                ui::execute_load
                    .run_if(|p: Res<ui::PendingSaveLoadAction>| matches!(p.0, Some((SaveLoadMode::Load, _)))),
            );
    }
}