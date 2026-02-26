// src/enemy/spawn.rs
use super::components::{AIBehavior, Enemy, EnemyCombat, EnemyPath};
use crate::characters::{
    animation::{AnimationController, AnimationTimer, DEFAULT_ANIMATION_FRAME_TIME},
    collider::Collider,
    config::{CharacterEntry, CharactersList},
    facing::Facing,
    physics::Velocity,
    spawn::CharactersListResource, // Add this line
    state::CharacterState,
};
use crate::collision::CollisionMap;
use crate::config::enemy::{ENEMY_SCALE, ENEMY_Z_POSITION};
use bevy::prelude::*;
use crate::combat::Health;

/// Spawn an enemy at the given position
pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &AssetServer,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    characters_list: &CharactersList,
    position: Vec3,
    character_name: &str,
) -> Option<Entity> {
    // Find the character entry by name
    let character_entry = characters_list
        .characters
        .iter()
        .find(|c| c.name == character_name)?;

    // Create atlas layout
    let max_row = character_entry.calculate_max_animation_row();
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ));

    // Load texture
    let texture = asset_server.load(&character_entry.texture_path);

    // Create sprite
    let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

    // Spawn enemy entity with all necessary components
    let entity = commands
        .spawn((
            Enemy,
            sprite,
            Transform::from_translation(position).with_scale(Vec3::splat(ENEMY_SCALE)),
            GlobalTransform::default(),
            AnimationController::default(),
            CharacterState::default(),
            Velocity::default(),
            Facing::default(),
            Collider::default(),
            EnemyCombat::default(),
            Health::new(character_entry.max_health), 
            AIBehavior::default(),
            EnemyPath::default(),  // Add this line
            AnimationTimer(Timer::from_seconds(
                DEFAULT_ANIMATION_FRAME_TIME,
                TimerMode::Repeating,
            )),
            character_entry.clone(),
        ))
        .id();

    info!("Spawned enemy '{}' at {:?}", character_name, position);

     Some(entity)
}

/// Resource to track if enemies have been spawned
#[derive(Resource, Default, PartialEq, Eq)]
pub struct EnemiesSpawned(pub bool);

/// Validate and adjust spawn position to ensure it's on a walkable tile
fn get_valid_spawn_position(collision_map: &CollisionMap, desired_pos: Vec2) -> Vec2 {
    // Use circle check with enemy collision radius for robust detection
    let enemy_radius = 12.0; // Approximate enemy collision radius
    
    // Check if the desired position is clear (considering radius)
    if collision_map.is_circle_clear(desired_pos, enemy_radius) {
        return desired_pos;
    }

    // Find nearest walkable tile
    let grid_pos = collision_map.world_to_grid(desired_pos);
    if let Some(walkable) = collision_map.find_nearest_walkable(grid_pos) {
        let world_pos = collision_map.grid_to_world(walkable.x, walkable.y);
        info!(
            "Adjusted spawn from {:?} to {:?} (was on obstacle)",
            desired_pos, world_pos
        );
        return world_pos;
    }

    // Fallback to original (shouldn't happen in a valid map)
    warn!("Could not find walkable spawn position near {:?}", desired_pos);
    desired_pos
}

/// System to spawn test enemies when collision map is ready
pub fn spawn_test_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_lists: Res<Assets<CharactersList>>,
    characters_list_res: Option<Res<CharactersListResource>>, // Add this line
    collision_map: Option<Res<CollisionMap>>,
    mut enemies_spawned: ResMut<EnemiesSpawned>,
) {
    // Wait for collision map
    let Some(collision_map) = collision_map else {
        return;
    };

    // Wait for character list resource
    let Some(characters_list_res) = characters_list_res else {
        return;
    };

    // Get the character list asset
    let Some(characters_list) = characters_lists.get(&characters_list_res.handle) else {
        return;
    };

    // Define desired spawn positions
    let spawn_positions = [Vec2::new(200.0, 0.0), Vec2::new(-200.0, 100.0)];

    for desired_pos in spawn_positions {
        // Validate position against collision map
        let valid_pos = get_valid_spawn_position(&collision_map, desired_pos);

        spawn_enemy(
            &mut commands,
            &asset_server,
            &mut atlas_layouts,
            characters_list,
            Vec3::new(valid_pos.x, valid_pos.y, ENEMY_Z_POSITION),
            "graveyard_reaper",
        );
    }

    // Mark enemies as spawned so this system doesn't run again
    enemies_spawned.0 = true;
    info!("Enemies spawned with validated positions");
}