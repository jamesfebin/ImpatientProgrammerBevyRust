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
use crate::collision::{CollisionMap, TileType};
use crate::config::enemy::{ENEMY_SCALE, ENEMY_Z_POSITION};
use crate::config::player::COLLIDER_RADIUS;
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
    let grid = collision_map.world_to_grid(desired_pos);
    let tile_at_pos = collision_map.get_tile(grid.x, grid.y);
    let circle_clear = collision_map.is_circle_clear(desired_pos, COLLIDER_RADIUS);
    info!(
        "[ENEMY SPAWN DEBUG] desired={:?}, grid=({}, {}), tile={:?}, is_circle_clear(r={})={}",
        desired_pos, grid.x, grid.y, tile_at_pos, COLLIDER_RADIUS, circle_clear
    );

    if circle_clear {
        // Double-check: log surrounding tiles
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                let t = collision_map.get_tile(grid.x + dx, grid.y + dy);
                if let Some(t) = t {
                    if !t.is_walkable() {
                        info!(
                            "[ENEMY SPAWN DEBUG]   neighbor({},{}) = {:?} (unwalkable!)",
                            grid.x + dx, grid.y + dy, t
                        );
                    }
                }
            }
        }
        return desired_pos;
    }

    if let Some(clear_pos) = collision_map.find_nearest_clear_position(desired_pos, COLLIDER_RADIUS) {
        let new_grid = collision_map.world_to_grid(clear_pos);
        let new_tile = collision_map.get_tile(new_grid.x, new_grid.y);
        info!(
            "[ENEMY SPAWN DEBUG] Adjusted spawn: {:?} -> {:?}, new_grid=({}, {}), new_tile={:?}",
            desired_pos, clear_pos, new_grid.x, new_grid.y, new_tile
        );
        return clear_pos;
    }

    warn!(
        "[ENEMY SPAWN DEBUG] FAILED to find clear position near {:?} — returning invalid pos!",
        desired_pos
    );
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