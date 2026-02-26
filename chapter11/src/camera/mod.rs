// src/camera/mod.rs
mod camera;

use bevy::prelude::*;
use crate::state::GameState;

// Re-export public items
pub use camera::MainCamera;

/// Plugin for camera systems.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                Startup,
                camera::setup_camera,
            )
            .add_systems(
                Update,
                camera::follow_camera.run_if(in_state(GameState::Playing)),
            );
    }
}