use bevy::prelude::*;
use crate::characters::animation::*;
use crate::characters::config::{CharacterEntry, CharactersList};
use crate::characters::movement::Player;

const PLAYER_SCALE: f32 = 0.8;
const PLAYER_Z_POSITION: f32 = 20.0;

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

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut character_index: ResMut<CurrentCharacterIndex>,
) {
    // Load the characters list
    let characters_list_handle: Handle<CharactersList> = asset_server.load("characters/characters.ron");
    
    // Store the handle in a resource
    commands.insert_resource(CharactersListResource {
        handle: characters_list_handle,
    });
    
    // Initialize with first character
    character_index.index = 0;
    
    // Spawn player entity (will be initialized once asset loads)
    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z_POSITION))
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        Sprite::default(),
    ));
}

pub fn initialize_player_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_lists: Res<Assets<CharactersList>>,
    character_index: Res<CurrentCharacterIndex>,
    characters_list_res: Option<Res<CharactersListResource>>,
    mut query: Query<Entity, (With<Player>, Without<AnimationController>)>,
) {
    let Some(characters_list_res) = characters_list_res else {
        return;
    };
    
    for entity in query.iter_mut() {
        let Some(characters_list) = characters_lists.get(&characters_list_res.handle) else {
            continue;
        };
        
        if character_index.index >= characters_list.characters.len() {
            continue;
        };
        
        let character_entry = &characters_list.characters[character_index.index];
        
        let texture = asset_server.load(&character_entry.texture_path);
        let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);
        
        let sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: 0,
            },
        );
        
        commands.entity(entity).insert((
            AnimationController::default(),
            AnimationState::default(),
            AnimationTimer(Timer::from_seconds(DEFAULT_ANIMATION_FRAME_TIME, TimerMode::Repeating)),
            character_entry.clone(),
            sprite,
        ));
    }
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