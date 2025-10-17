mod map;
mod player;

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

use bevy_procedural_tilemaps::prelude::*;

use crate::map::generate::{setup_generator, build_collision_map, CollisionMapBuilt};
use crate::map::debug::{DebugCollisionEnabled, toggle_debug_collision, debug_draw_collision, debug_player_position, debug_log_tile_info};
use crate::player::PlayerPlugin;

// Camera follow component
#[derive(Component)]
struct CameraFollow;

#[derive(Component)]
struct FogOfWar;

// Custom material for circular fog of war
// Both fields share uniform(0) so they're packed into a single uniform buffer
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

fn main() {
    // Window size
    let window_width = 1280.0;
    let window_height = 960.0;
    
    // Circular vision radius
    let vision_radius = 320.0;

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(VisionRadius(vision_radius))
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "src/assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(window_width as u32, window_height as u32),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            Material2dPlugin::<CircularFogMaterial>::default(),
        ))
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        .init_resource::<CollisionMapBuilt>()
        .init_resource::<DebugCollisionEnabled>()
        .add_systems(Startup, (setup_camera, setup_generator, setup_fog_of_war))
        .add_systems(Update, (
            build_collision_map,
            follow_player_and_fog,
            toggle_debug_collision,
            debug_draw_collision,
            debug_player_position,
            debug_log_tile_info,
        ))
        .add_plugins(PlayerPlugin)
        .run();
}

#[derive(Resource)]
struct VisionRadius(f32);

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        CameraFollow,
    ));
}

fn setup_fog_of_war(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CircularFogMaterial>>,
    vision_radius: Res<VisionRadius>,
) {
    // Create a large quad that covers the entire view
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

/// Combined system to follow player with camera and update fog material
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

    // Update camera
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        let lerp_speed = 0.1;
        camera_transform.translation.x += (player_pos.x - camera_transform.translation.x) * lerp_speed;
        camera_transform.translation.y += (player_pos.y - camera_transform.translation.y) * lerp_speed;
        
        camera_transform.translation.x = camera_transform.translation.x.round();
        camera_transform.translation.y = camera_transform.translation.y.round();
        camera_transform.translation.z = 1000.0;
    }

    // Update fog overlay position and material
    if let Ok((mut fog_transform, material_handle)) = fog_query.single_mut() {
        fog_transform.translation.x = player_pos.x;
        fog_transform.translation.y = player_pos.y;
        fog_transform.translation.z = 900.0;

        // Update material with player position
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.player_pos = player_pos;
        }
    }
}
