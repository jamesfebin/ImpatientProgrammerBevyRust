mod collision;
mod map;
mod player;

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowMode, MonitorSelection},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
    camera::Projection,
};
use bevy_procedural_tilemaps::prelude::*;

use crate::map::generate::{setup_generator, build_collision_map, CollisionMapBuilt};
use crate::player::PlayerPlugin;

#[cfg(debug_assertions)]
use crate::collision::{DebugCollisionEnabled, toggle_debug_collision, debug_draw_collision, debug_player_position, debug_log_tile_info};

#[derive(Component)]
struct CameraFollow;

#[derive(Component)]
struct FogOfWar;

// Custom material for circular fog of war vision
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CircularFogMaterial {
    #[uniform(0)]
    player_pos: Vec2,
    #[uniform(0)]
    vision_radius: f32,
}

impl Material2d for CircularFogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circular_fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Resource)]
struct VisionRadius(f32);

fn main() {
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
                .set(ImagePlugin::default_nearest()),
            Material2dPlugin::<CircularFogMaterial>::default(),
            ProcGenSimplePlugin::<Cartesian3D, Sprite>::default(),
            PlayerPlugin,
        ))
        .init_resource::<CollisionMapBuilt>()
        .add_systems(Startup, (setup_camera, setup_generator, setup_fog_of_war))
        .add_systems(Update, (build_collision_map, follow_player_and_fog, update_player_depth, configure_camera_projection, debug_tile_depths, debug_yellowgrass_tiles, debug_props_depth, debug_player_vs_props));

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

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), CameraFollow));
}

        /// System to update player depth based on Y position to match tilemap Z system
        /// This mirrors the same Z-depth calculation that bevy_procedural_tilemaps uses
        /// with with_z_offset_from_y(true)
        fn update_player_depth(mut player_query: Query<&mut Transform, With<crate::player::Player>>) {
            for mut transform in player_query.iter_mut() {
                let player_center_y = transform.translation.y;
                let old_z = transform.translation.z;
                
                // Map configuration (from generate.rs)
                const TILE_SIZE: f32 = 64.0;
                const GRID_Y: u32 = 18;
                
                // CRITICAL FIX: Use player's FEET position for depth sorting, not center!
                // The player sprite is anchored at center, but for proper depth sorting
                // we need to consider where the player's feet are (bottom of sprite)
                // Player scale is 1.2, so sprite height is TILE_SIZE * 1.2 = 76.8
                // Feet are at: center_y - (sprite_height / 2) = center_y - 38.4
                const PLAYER_SCALE: f32 = 1.2;
                const PLAYER_SPRITE_HEIGHT: f32 = TILE_SIZE * PLAYER_SCALE; // 76.8
                let player_feet_y = player_center_y - (PLAYER_SPRITE_HEIGHT / 2.0); // Bottom of player sprite
                
                let map_height = TILE_SIZE * GRID_Y as f32;
                let map_y0 = -TILE_SIZE * GRID_Y as f32 / 2.0; // Map origin Y (from generate.rs)
                
                // Normalize player FEET Y to [0, 1] across the whole grid height
                let t = ((player_feet_y - map_y0) / map_height).clamp(0.0, 1.0);
                
                // Use the Y-to-Z formula from bevy_procedural_tilemaps:
                // z = base_z + NODE_SIZE.z * (1.0 - y / grid_height)
                // Where NODE_SIZE.z = 1.0 and base_z varies by layer (1.0 for dirt, 3.0 for yellowgrass, etc)
                // Props (trees, rocks) typically have base_z ≈ 4.0-5.0
                // To ensure proper Y-sorting with props, we need to be in the SAME Z range as props
                // but with a small offset to ensure consistent rendering order
                const NODE_SIZE_Z: f32 = 1.0;
                const PLAYER_BASE_Z: f32 = 4.0; // Match props base Z range for proper Y-sorting
                const PLAYER_Z_OFFSET: f32 = 0.5; // Larger offset to ensure player is ALWAYS above props
                let player_z = PLAYER_BASE_Z + NODE_SIZE_Z * (1.0 - t) + PLAYER_Z_OFFSET;
                
                transform.translation.z = player_z;
                
                // Debug log every 60 frames (about once per second at 60fps)
                static mut FRAME_COUNT: u32 = 0;
                unsafe {
                    FRAME_COUNT += 1;
                    if FRAME_COUNT % 60 == 0 {
                        info!("🎮 Player depth debug - Center Y: {:.1}, Feet Y: {:.1}, Old Z: {:.3}, New Z: {:.3}, t: {:.3}", 
                              player_center_y, player_feet_y, old_z, player_z, t);
                    }
                }
            }
        }

/// System to configure camera projection to prevent Z-depth culling issues
fn configure_camera_projection(
    mut camera_query: Query<&mut Projection, (With<Camera2d>, With<CameraFollow>)>,
) {
    for mut projection in camera_query.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            // Widen the camera's clip range to prevent objects from being culled
            // This makes debugging less brittle and prevents Z-depth issues
            ortho.near = -2000.0;
            ortho.far = 2000.0;
        }
    }
}

/// Debug system to show tile Z values to understand the depth system
fn debug_tile_depths(
    tile_query: Query<(&Transform, &crate::collision::TileMarker)>,
) {
    // Debug log every 300 frames (about once per 5 seconds at 60fps)
    static mut FRAME_COUNT: u32 = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT % 300 == 0 {
            let mut tile_count = 0;
            let mut min_z = f32::MAX;
            let mut max_z = f32::MIN;
            let mut sample_tiles: Vec<(f32, f32, String)> = Vec::new(); // (Y, Z, Type)
            
            for (transform, tile_marker) in tile_query.iter() {
                tile_count += 1;
                let z = transform.translation.z;
                min_z = min_z.min(z);
                max_z = max_z.max(z);
                
                // Collect first 10 tiles as samples
                if sample_tiles.len() < 10 {
                    sample_tiles.push((
                        transform.translation.y,
                        z,
                        format!("{:?}", tile_marker.tile_type)
                    ));
                }
            }
            
            if tile_count > 0 {
                info!("🗺️ Tile depth debug - {} tiles, Z range: {:.3} to {:.3}", 
                      tile_count, min_z, max_z);
                info!("🗺️ Sample tiles (Y, Z, Type):");
                for (y, z, tile_type) in sample_tiles {
                    info!("   Y: {:.1}, Z: {:.3}, Type: {}", y, z, tile_type);
                }
            }
        }
    }
}

/// Debug system to specifically check YellowGrass Z values
fn debug_yellowgrass_tiles(
    tile_query: Query<(&Transform, &crate::collision::TileMarker)>,
) {
    // Debug log every 300 frames (about once per 5 seconds at 60fps)
    static mut FRAME_COUNT: u32 = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT % 300 == 0 {
            let mut yellowgrass_z_values: Vec<(f32, f32)> = Vec::new(); // (Y, Z)
            let mut yellowgrass_count = 0;
            let mut min_yg_z = f32::MAX;
            let mut max_yg_z = f32::MIN;
            
            for (transform, tile_marker) in tile_query.iter() {
                if tile_marker.tile_type == crate::collision::TileType::YellowGrass {
                    yellowgrass_count += 1;
                    let z = transform.translation.z;
                    min_yg_z = min_yg_z.min(z);
                    max_yg_z = max_yg_z.max(z);
                    
                    // Collect first 10 YellowGrass tiles
                    if yellowgrass_z_values.len() < 10 {
                        yellowgrass_z_values.push((transform.translation.y, z));
                    }
                }
            }
            
            if yellowgrass_count > 0 {
                info!("🌾 YellowGrass depth debug - {} tiles, Z range: {:.3} to {:.3}", 
                      yellowgrass_count, min_yg_z, max_yg_z);
                info!("🌾 Sample YellowGrass tiles (Y, Z):");
                for (y, z) in yellowgrass_z_values {
                    info!("   Y: {:.1}, Z: {:.3}", y, z);
                }
            }
        }
    }
}

/// Debug system to specifically check props (trees, rocks) Z values
fn debug_props_depth(
    tile_query: Query<(&Transform, &crate::collision::TileMarker)>,
) {
    // Debug log every 300 frames (about once per 5 seconds at 60fps)
    static mut FRAME_COUNT: u32 = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT % 300 == 0 {
            let mut tree_count = 0;
            let mut rock_count = 0;
            let mut tree_z_values: Vec<(f32, f32)> = Vec::new(); // (Y, Z)
            let mut rock_z_values: Vec<(f32, f32)> = Vec::new(); // (Y, Z)
            let mut min_tree_z = f32::MAX;
            let mut max_tree_z = f32::MIN;
            let mut min_rock_z = f32::MAX;
            let mut max_rock_z = f32::MIN;
            
            for (transform, tile_marker) in tile_query.iter() {
                match tile_marker.tile_type {
                    crate::collision::TileType::Tree => {
                        tree_count += 1;
                        let z = transform.translation.z;
                        min_tree_z = min_tree_z.min(z);
                        max_tree_z = max_tree_z.max(z);
                        
                        // Collect first 10 tree tiles
                        if tree_z_values.len() < 10 {
                            tree_z_values.push((transform.translation.y, z));
                        }
                    }
                    crate::collision::TileType::Rock => {
                        rock_count += 1;
                        let z = transform.translation.z;
                        min_rock_z = min_rock_z.min(z);
                        max_rock_z = max_rock_z.max(z);
                        
                        // Collect first 10 rock tiles
                        if rock_z_values.len() < 10 {
                            rock_z_values.push((transform.translation.y, z));
                        }
                    }
                    _ => {}
                }
            }
            
            if tree_count > 0 {
                info!("🌳 Tree depth debug - {} tiles, Z range: {:.3} to {:.3}", 
                      tree_count, min_tree_z, max_tree_z);
                info!("🌳 Sample tree tiles (Y, Z):");
                for (y, z) in tree_z_values {
                    info!("   Y: {:.1}, Z: {:.3}", y, z);
                }
            }
            
            if rock_count > 0 {
                info!("🪨 Rock depth debug - {} tiles, Z range: {:.3} to {:.3}", 
                      rock_count, min_rock_z, max_rock_z);
                info!("🪨 Sample rock tiles (Y, Z):");
                for (y, z) in rock_z_values {
                    info!("   Y: {:.1}, Z: {:.3}", y, z);
                }
            }
        }
    }
}

fn setup_fog_of_war(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CircularFogMaterial>>,
    vision_radius: Res<VisionRadius>,
) {
    let mesh = meshes.add(Rectangle::new(5000.0, 5000.0));
    let material = materials.add(CircularFogMaterial {
        player_pos: Vec2::ZERO,
        vision_radius: vision_radius.0,
    });
    
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 900.0)),
        FogOfWar,
    ));
}

fn follow_player_and_fog(
    player_query: Query<&Transform, With<crate::player::Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<crate::player::Player>, Without<FogOfWar>)>,
    mut fog_query: Query<(&mut Transform, &MeshMaterial2d<CircularFogMaterial>), (With<FogOfWar>, Without<Camera2d>, Without<crate::player::Player>)>,
    mut materials: ResMut<Assets<CircularFogMaterial>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);

    // Update camera with smooth following
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        let lerp_speed = 0.1;
        camera_transform.translation.x += (player_pos.x - camera_transform.translation.x) * lerp_speed;
        camera_transform.translation.y += (player_pos.y - camera_transform.translation.y) * lerp_speed;
        
        // Snap to pixel boundaries for crisp rendering
        camera_transform.translation.x = camera_transform.translation.x.round();
        camera_transform.translation.y = camera_transform.translation.y.round();
        camera_transform.translation.z = 1000.0;
    }

    // Update fog of war overlay
    if let Ok((mut fog_transform, material_handle)) = fog_query.single_mut() {
        fog_transform.translation.x = player_pos.x;
        fog_transform.translation.y = player_pos.y;
        fog_transform.translation.z = 900.0;

        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.player_pos = player_pos;
        }
    }
}

/// Debug system to compare player Z with nearby props
fn debug_player_vs_props(
    player_query: Query<&Transform, With<crate::player::Player>>,
    tile_query: Query<(&Transform, &crate::collision::TileMarker)>,
) {
    // Debug log every 300 frames (about once per 5 seconds at 60fps)
    static mut FRAME_COUNT: u32 = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT % 300 == 0 {
            let Ok(player_transform) = player_query.single() else {
                return;
            };
            
            let player_z = player_transform.translation.z;
            let player_y = player_transform.translation.y;
            
            // Find props within 2 tiles of player
            let mut nearby_props: Vec<(f32, f32, String)> = Vec::new(); // (Y, Z, Type)
            
            for (transform, tile_marker) in tile_query.iter() {
                match tile_marker.tile_type {
                    crate::collision::TileType::Tree | crate::collision::TileType::Rock => {
                        let prop_y = transform.translation.y;
                        let prop_z = transform.translation.z;
                        let distance = (prop_y - player_y).abs();
                        
                        // Only consider props within 2 tiles (128 pixels)
                        if distance <= 128.0 {
                            nearby_props.push((
                                prop_y,
                                prop_z,
                                format!("{:?}", tile_marker.tile_type)
                            ));
                        }
                    }
                    _ => {}
                }
            }
            
            if !nearby_props.is_empty() {
                info!("🎮 Player vs Props Z comparison:");
                info!("   Player: Y={:.1}, Z={:.3}", player_y, player_z);
                info!("   Nearby props:");
                for (y, z, tile_type) in nearby_props {
                    let z_diff = player_z - z;
                    let y_diff = player_y - y;
                    info!("     {}: Y={:.1}, Z={:.3}, Z_diff={:+.3}, Y_diff={:+.1}", 
                          tile_type, y, z, z_diff, y_diff);
                }
            }
        }
    }
}
