use bevy::prelude::*;
use crate::characters::animation::*;
use crate::characters::config::{CharacterEntry, CharactersList};
use crate::characters::input::Player;  // Changed from movement::Player
use crate::characters::state::CharacterState;  // Line update alert
use crate::characters::physics::Velocity;  // Line update alert
use crate::characters::facing::Facing;  // Line update alert
use crate::characters::collider::Collider; 
use crate::config::player::{PLAYER_SCALE, PLAYER_Z_POSITION}; 
use crate::combat::PlayerCombat;
use crate::collision::CollisionMap;
use crate::combat::Health;

#[derive(Resource, Default)]
pub struct CurrentCharacterIndex {
    pub index: usize,
}

#[derive(Resource)]
pub struct CharactersListResource {
    pub handle: Handle<CharactersList>,
}

fn create_character_atlas_layout(
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    character_entry: &CharacterEntry,
) -> Handle<TextureAtlasLayout> {
    let max_row = character_entry.calculate_max_animation_row();
    
    atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ))
}

pub fn switch_character(
    input: Res<ButtonInput<KeyCode>>,
    mut character_index: ResMut<CurrentCharacterIndex>,
    characters_lists: Res<Assets<CharactersList>>,
    characters_list_res: Option<Res<CharactersListResource>>,
    mut query: Query<(
        &mut CharacterEntry,
        &mut Sprite,
    ), With<Player>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // Map digit keys to indices
    const DIGIT_KEYS: [KeyCode; 9] = [
        KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3,
        KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6,
        KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9,
    ];
    
    // Find which digit key was pressed
    let new_index = DIGIT_KEYS.iter()
        .position(|&key| input.just_pressed(key));
    
    let Some(new_index) = new_index else {
        return;
    };
    
    let Some(characters_list_res) = characters_list_res else {
        return;
    };
    
    let Some(characters_list) = characters_lists.get(&characters_list_res.handle) else {
        return;
    };
    
    if new_index >= characters_list.characters.len() {
        return;
    }
    
    // Update character index
    character_index.index = new_index;
    
    // Update player entity
    let Ok((mut current_entry, mut sprite)) = query.single_mut() else {
        return;
    };
    
    let character_entry = &characters_list.characters[new_index];
    
    // Update character entry
    *current_entry = character_entry.clone();
    
    // Update sprite with new texture
    let texture = asset_server.load(&character_entry.texture_path);
    let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);
    
    *sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout,
            index: 0,
        },
    );
}

#[derive(Resource, Default, PartialEq, Eq)]
pub struct PlayerSpawned(pub bool);

// Add this helper function after create_character_atlas_layout
/// Get a valid spawn position, checking collision map and adjusting if needed
fn get_valid_spawn_position(collision_map: &CollisionMap, desired_pos: Vec2) -> Vec2 {
    let player_radius = 12.0; // Approximate player collision radius
    
    // Check if the desired position is clear
    if collision_map.is_circle_clear(desired_pos, player_radius) {
        return desired_pos;
    }
    
    // Find nearest walkable tile
    let grid_pos = collision_map.world_to_grid(desired_pos);
    if let Some(walkable) = collision_map.find_nearest_walkable(grid_pos) {
        let world_pos = collision_map.grid_to_world(walkable.x, walkable.y);
        info!(
            "Adjusted player spawn from {:?} to {:?} (was on obstacle)",
            desired_pos, world_pos
        );
        return world_pos;
    }
    
    // Fallback to original
    warn!("Could not find walkable spawn position near {:?}", desired_pos);
    desired_pos
}

// Replace spawn_player with this
/// Load character assets at startup (before collision map is built)
pub fn load_character_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut character_index: ResMut<CurrentCharacterIndex>,
) {
    // Load the characters list
    let characters_list_handle: Handle<CharactersList> =
        asset_server.load("characters/characters.ron");

    // Store the handle in a resource
    commands.insert_resource(CharactersListResource {
        handle: characters_list_handle,
    });

    // Initialize with first character
    character_index.index = 0;
    
    info!("Character assets loading started");
}

// Add this new function
/// Spawn player at a valid position AFTER collision map is built
pub fn spawn_player_at_valid_position(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_lists: Res<Assets<CharactersList>>,
    character_index: Res<CurrentCharacterIndex>,
    characters_list_res: Option<Res<CharactersListResource>>,
    collision_map: Option<Res<CollisionMap>>,
    mut player_spawned: ResMut<PlayerSpawned>,
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
    
    if character_index.index >= characters_list.characters.len() {
        warn!("Invalid character index: {}", character_index.index);
        return;
    }
    
    let character_entry = &characters_list.characters[character_index.index];
    
    // Calculate valid spawn position
    let desired_pos = Vec2::new(0.0, 0.0);
    let valid_pos = get_valid_spawn_position(&collision_map, desired_pos);
    
    // Create sprite
    let texture = asset_server.load(&character_entry.texture_path);
    let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);
    let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });
    
    // Spawn player with all components at valid position
    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(valid_pos.x, valid_pos.y, PLAYER_Z_POSITION))
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        sprite,
        AnimationController::default(),
        CharacterState::default(),
        Velocity::default(),
        Facing::default(),
        Collider::default(),
        PlayerCombat::default(),
        Health::new(character_entry.max_health),
        AnimationTimer(Timer::from_seconds(
            DEFAULT_ANIMATION_FRAME_TIME,
            TimerMode::Repeating,
        )),
        character_entry.clone(),
    ));
    
    // Mark player as spawned
    player_spawned.0 = true;
    info!("Player spawned at validated position {:?}", valid_pos);
}