mod map;
mod characters;
mod state; 
mod collision;
mod config;
mod inventory;
mod camera;
mod combat;
mod particles;
mod enemy;
mod save;
mod audio;

use bevy::{
    prelude::*,
    window::{MonitorSelection, Window, WindowMode, WindowPlugin}, // Line update alert
};

use bevy_procedural_tilemaps::prelude::*;
use crate::camera::CameraPlugin;
use crate::map::generate::{setup_generator, prepare_tilemap_handles_resource, poll_map_generation};
use crate::state::GameState;

fn get_assets_path() -> String {
    // Check for assets/ next to the executable first (release builds)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_assets = exe_dir.join("assets");
            if exe_assets.exists() {
                return exe_assets.to_string_lossy().to_string();
            }
        }
    }
    // Fallback for `cargo run` from project root
    "src/assets".to_string()
}

fn main() {
    let assets_path = get_assets_path();
    App::new()
        .insert_resource(ClearColor(Color::BLACK)) // Line update alert
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: assets_path.into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Game".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current), // Add this line
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(state::StatePlugin)
        .add_plugins(CameraPlugin) // Add this line
        .add_plugins(inventory::InventoryPlugin)
        .add_plugins(collision::CollisionPlugin)
        .add_plugins(characters::CharactersPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(enemy::EnemyPlugin) 
        .add_plugins(particles::ParticlesPlugin)
        .add_plugins(save::SavePlugin)
        .add_plugins(audio::AudioManagerPlugin)
        .add_systems(Startup, prepare_tilemap_handles_resource)
        .add_systems(OnEnter(GameState::Loading), setup_generator)
        .add_systems(Update, poll_map_generation.run_if(in_state(GameState::Loading)))
        .run();
}