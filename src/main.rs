mod camera;
mod collision;
mod map;
mod performance;
mod player;
mod profiling;

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowMode, MonitorSelection},
    sprite_render::Material2dPlugin,
};
use bevy_procedural_tilemaps::prelude::*;

use crate::map::generate::{setup_generator, build_collision_map, CollisionMapBuilt};
use crate::player::PlayerPlugin;
use crate::camera::{setup_camera, follow_camera, configure_camera_projection};
use crate::camera::fog::{setup_fog_of_war, follow_fog, VisionRadius, CircularFogMaterial};
use crate::camera::rendering::update_player_depth;
use crate::profiling::init_profiling;
use crate::performance::{PerformanceMonitor, monitor_performance};

#[cfg(debug_assertions)]
use crate::collision::{DebugCollisionEnabled, toggle_debug_collision, debug_draw_collision, debug_player_position, debug_log_tile_info};


fn main() {
    // Initialize profiling first
    init_profiling();
    
    let vision_radius = 320.0;

    let mut app = App::new();
    
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(VisionRadius(vision_radius))
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "src/assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Game".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .disable::<bevy::log::LogPlugin>(), // Disable Bevy's logging to avoid conflict
            Material2dPlugin::<CircularFogMaterial>::default(),
            ProcGenSimplePlugin::<Cartesian3D, Sprite>::default(),
            PlayerPlugin,
        ))
        .init_resource::<CollisionMapBuilt>()
        .init_resource::<PerformanceMonitor>()
        .add_systems(Startup, (setup_camera, setup_generator, setup_fog_of_war))
        .add_systems(Update, (
            // Performance monitoring (run first)
            monitor_performance,
            // Main game systems
            build_collision_map, 
            follow_camera, 
            follow_fog, 
            update_player_depth, 
            configure_camera_projection
        ));

    // Debug systems - only in debug builds
    #[cfg(debug_assertions)]
    {
        app.init_resource::<DebugCollisionEnabled>()
            .add_systems(Update, (
                toggle_debug_collision,
                debug_draw_collision,
                debug_player_position,
                debug_log_tile_info,
            ));
    }

    app.run();
}


