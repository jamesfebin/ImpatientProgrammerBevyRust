// src/audio/mod.rs
mod assets;
mod music;
mod sfx;

pub use sfx::SfxKind;

use crate::state::GameState;
use bevy::prelude::*;

pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            // SFX handled via global observer
            .add_observer(sfx::on_sfx)
            
            // Start loading assets and music on app startup
            .add_systems(
                Startup,
                (assets::load_audio_assets, music::start_menu_music).chain(),
            )
            
            // Music follows game state.
            .add_systems(OnEnter(GameState::MainMenu), music::start_menu_music)
            .add_systems(OnEnter(GameState::Playing), music::start_battle_music)
            .add_systems(OnEnter(GameState::GameOver), music::stop_all_music);
    }
}