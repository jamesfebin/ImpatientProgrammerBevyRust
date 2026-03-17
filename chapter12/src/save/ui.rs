use bevy::prelude::*;

use crate::characters::config::{CharacterEntry, CharactersList};
use crate::characters::facing::Facing;
use crate::characters::input::Player;
use crate::characters::spawn::{CharactersListResource, CurrentCharacterIndex, PlayerSpawned};
use crate::collision::{CollisionMapBuilt, TileMarker};
use crate::combat::healthbar::HealthBarOwner;
use crate::combat::systems::{Projectile, ProjectileEffect};
use crate::combat::{Health, PlayerCombat};
use crate::enemy::spawn::EnemiesSpawned;
use crate::enemy::Enemy;
use crate::inventory::{Inventory, Pickable};
use crate::map::assets::TilemapHandles;
use crate::particles::components::{Particle, ParticleEmitter};
use crate::state::GameState;
use crate::characters::animation::*;
use crate::characters::collider::Collider;
use crate::characters::physics::Velocity;
use crate::characters::state::CharacterState;
use crate::config::player::PLAYER_SCALE;


use super::data::*;
use super::systems;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveLoadMode {
    Save,
    Load,
}

#[derive(Resource, Default)]
pub struct PendingSaveLoadAction(pub Option<(SaveLoadMode, usize)>);

#[derive(Resource)]
pub struct SaveLoadUIState {
    pub active: bool,
    pub mode: SaveLoadMode,
}

impl Default for SaveLoadUIState {
    fn default() -> Self {
        Self {
            active: false,
            mode: SaveLoadMode::Save,
        }
    }
}

#[derive(Component)]
pub struct SaveLoadUI;

#[derive(Component)]
pub struct SlotButton(pub usize);

#[derive(Component)]
pub struct BackButton;

pub fn handle_save_load_ui(
    mut commands: Commands,
    ui_state: Res<SaveLoadUIState>,
    existing_ui: Query<Entity, With<SaveLoadUI>>,
) {
    if !ui_state.is_changed() {
        return;
    }

    for entity in existing_ui.iter() {
        commands.entity(entity).despawn();
    }

    if !ui_state.active {
        return;
    }

    let title = match ui_state.mode {
        SaveLoadMode::Save => "SAVE GAME",
        SaveLoadMode::Load => "LOAD GAME",
    };

    let mut slot_infos: Vec<Option<SaveMetadata>> = Vec::new();
    for slot in 0..MAX_SLOTS {
        slot_infos.push(systems::load_slot_metadata(slot));
    }

    commands
        .spawn((
            SaveLoadUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 1.0)),
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(title),
                TextFont {
                    font_size: 42.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            for slot in 0..MAX_SLOTS {
                let info = &slot_infos[slot];
                let label = match info {
                    Some(meta) => format!(
                        "Slot {} — {}",
                        slot + 1,
                        meta.timestamp,
                    ),
                    None => format!("Slot {} — Empty", slot + 1),
                };

                let is_empty = info.is_none();
                let is_load_mode = ui_state.mode == SaveLoadMode::Load;
                let disabled = is_load_mode && is_empty;

                let bg_color = if disabled {
                    Color::srgba(0.2, 0.2, 0.2, 0.5)
                } else {
                    Color::srgba(0.15, 0.15, 0.3, 0.9)
                };

                let text_color = if disabled {
                    Color::srgba(0.5, 0.5, 0.5, 1.0)
                } else {
                    Color::WHITE
                };

                let mut btn = parent.spawn((
                    SlotButton(slot),
                    Button,
                    Node {
                        width: Val::Px(500.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(bg_color),
                ));

                if disabled {
                    btn.remove::<Button>();
                }

                btn.with_children(|btn_parent| {
                    btn_parent.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                });
            }

            parent
                .spawn((
                    BackButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(45.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.4, 0.1, 0.1, 0.9)),
                ))
                .with_children(|btn_parent| {
                    btn_parent.spawn((
                        Text::new("Back"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}


pub fn handle_slot_buttons(
    mut ui_state: ResMut<SaveLoadUIState>,
    mut pending: ResMut<PendingSaveLoadAction>,
    interaction_query: Query<(&Interaction, &SlotButton), Changed<Interaction>>,
) {
    for (interaction, slot_btn) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        // Button was just clicked!
        // 1. Record what needs to happen (Save or Load this slot)
        pending.0 = Some((ui_state.mode, slot_btn.0));
        // 2. Close the UI
        ui_state.active = false;
    }
}
 
pub fn handle_back_button(
    mut ui_state: ResMut<SaveLoadUIState>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Close on ESC
    if input.just_pressed(KeyCode::Escape) {
        ui_state.active = false;
        return;
    }
    // Close on Back button click
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            ui_state.active = false;
        }
    }
}

pub fn execute_save(
    mut pending: ResMut<PendingSaveLoadAction>,
    tile_query: Query<(&Transform, &Sprite, &TileMarker, Option<&Pickable>)>,
    player_query: Query<
        (&Transform, &Health, &PlayerCombat, &CharacterEntry, &Facing),
        With<Player>,
    >,
    enemy_query: Query<(&Transform, &Health, &CharacterEntry, &Facing), With<Enemy>>,
    inventory: Res<Inventory>,
    character_index: Res<CurrentCharacterIndex>,
) {
    let Some((SaveLoadMode::Save, slot)) = pending.0 else {
        return;
    };
    pending.0 = None;

    let Ok((player_tf, player_health, player_combat, player_entry, player_facing)) =
        player_query.single()
    else {
        error!("No player found for save");
        return;
    };

    let player_save = PlayerSave {
        position: [
            player_tf.translation.x,
            player_tf.translation.y,
            player_tf.translation.z,
        ],
        health_current: player_health.current,
        health_max: player_health.max,
        power_type: player_combat.power_type,
        character_name: player_entry.name.clone(),
        character_index: character_index.index,
        facing: *player_facing,
    };

    let mut enemies = Vec::new();
    for (tf, health, entry, facing) in enemy_query.iter() {
        enemies.push(EnemySave {
            position: [tf.translation.x, tf.translation.y, tf.translation.z],
            health_current: health.current,
            health_max: health.max,
            character_name: entry.name.clone(),
            power_type: crate::combat::PowerType::Fire,
            facing: *facing,
        });
    }

    let mut tiles = Vec::new();
    for (tf, sprite, tile_marker, pickable) in tile_query.iter() {
        let atlas_index = sprite
            .texture_atlas
            .as_ref()
            .map(|a| a.index)
            .unwrap_or(0);

        let rot = tf.rotation;
        tiles.push(TileSave {
            position: [tf.translation.x, tf.translation.y, tf.translation.z],
            rotation: [rot.x, rot.y, rot.z, rot.w],
            scale: [tf.scale.x, tf.scale.y, tf.scale.z],
            atlas_index,
            tile_type: tile_marker.tile_type,
            pickable: pickable.map(|p| p.kind),
        });
    }

    let timestamp = chrono::Local::now().format("%d %b %Y, %I:%M %p").to_string();

    let save_data = SaveData {
        version: SAVE_VERSION,
        timestamp: timestamp.clone(),
        slot_name: format!("Slot {}", slot + 1),
        player: player_save,
        enemies,
        inventory: inventory.items().clone(),
        tiles,
    };

    match do_write_save(slot, &save_data, &timestamp) {
        Ok(()) => info!("Saved to slot {}", slot + 1),
        Err(e) => error!("Failed to save: {}", e),
    }
}

fn do_write_save(slot: usize, save_data: &SaveData, timestamp: &str) -> Result<(), String> {
    let data_bytes =
        bincode::serialize(save_data).map_err(|e| format!("Serialize error: {}", e))?;
    let checksum = compute_checksum(&data_bytes);
    let save_file = SaveFile {
        checksum,
        data: data_bytes,
    };
    let file_bytes =
        bincode::serialize(&save_file).map_err(|e| format!("Serialize error: {}", e))?;

    let dir = saves_directory();
    std::fs::create_dir_all(&dir).map_err(|e| format!("Create dir error: {}", e))?;
    std::fs::write(save_file_path(slot), &file_bytes)
        .map_err(|e| format!("Write error: {}", e))?;

    let metadata = SaveMetadata {
        timestamp: timestamp.to_string(),
        character_name: save_data.player.character_name.clone(),
        player_health: save_data.player.health_current,
        player_max_health: save_data.player.health_max,
    };
    let meta_bytes =
        bincode::serialize(&metadata).map_err(|e| format!("Meta serialize error: {}", e))?;
    std::fs::write(meta_file_path(slot), &meta_bytes)
        .map_err(|e| format!("Meta write error: {}", e))?;

    Ok(())
}

pub fn execute_load(world: &mut World) {
    let slot = {
        let pending = world.resource::<PendingSaveLoadAction>();
        match pending.0 {
            Some((SaveLoadMode::Load, slot)) => slot,
            _ => return,
        }
    };
    world.resource_mut::<PendingSaveLoadAction>().0 = None;

    let save_data = match systems::load_save_data(slot) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to load: {}", e);
            return;
        }
    };

    // Despawn all gameplay entities
    let mut to_despawn = Vec::new();
    for entity in world.query_filtered::<Entity, With<TileMarker>>().iter(world) {
        to_despawn.push(entity);
    }
    for entity in world.query_filtered::<Entity, With<Player>>().iter(world) {
        to_despawn.push(entity);
    }
    for entity in world.query_filtered::<Entity, With<Enemy>>().iter(world) {
        to_despawn.push(entity);
    }
    for entity in world.query_filtered::<Entity, With<Projectile>>().iter(world) {
        to_despawn.push(entity);
    }
    for entity in world.query_filtered::<Entity, With<ProjectileEffect>>().iter(world) {
        to_despawn.push(entity);
    }
    for entity in world.query_filtered::<Entity, With<ParticleEmitter>>().iter(world) {
        to_despawn.push(entity);
    }
    for entity in world.query_filtered::<Entity, With<Particle>>().iter(world) {
        to_despawn.push(entity);
    }
    for entity in world.query_filtered::<Entity, With<HealthBarOwner>>().iter(world) {
        to_despawn.push(entity);
    }
    for entity in to_despawn {
        world.despawn(entity);
    }

    let tilemap_handles = match world.get_resource::<TilemapHandles>() {
        Some(h) => h.clone(),
        None => {
            error!("TilemapHandles not available for loading");
            return;
        }
    };

    // Spawn tiles
    for tile in &save_data.tiles {
        let sprite = tilemap_handles.sprite(tile.atlas_index);
        let transform = Transform {
            translation: Vec3::new(tile.position[0], tile.position[1], tile.position[2]),
            rotation: Quat::from_xyzw(
                tile.rotation[0],
                tile.rotation[1],
                tile.rotation[2],
                tile.rotation[3],
            ),
            scale: Vec3::new(tile.scale[0], tile.scale[1], tile.scale[2]),
        };

        let mut entity = world.spawn((sprite, transform, TileMarker::new(tile.tile_type)));
        if let Some(item_kind) = tile.pickable {
            entity.insert(Pickable::new(item_kind));
        }
    }

    let characters_list_handle = {
        let Some(res) = world.get_resource::<CharactersListResource>() else {
            error!("CharactersListResource not available");
            return;
        };
        res.handle.clone()
    };
    let characters_list = {
        let lists = world.resource::<Assets<CharactersList>>();
        let Some(list) = lists.get(&characters_list_handle) else {
            error!("Characters list not loaded");
            return;
        };
        list.clone()
    };

    // Spawn player
    let player_data = &save_data.player;
    let char_idx = player_data.character_index.min(characters_list.characters.len() - 1);
    let character_entry = characters_list.characters[char_idx].clone();

    let max_row = character_entry.calculate_max_animation_row();
    let layout = {
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(character_entry.tile_size),
            character_entry.atlas_columns as u32,
            (max_row + 1) as u32,
            None,
            None,
        ))
    };
    let texture = {
        let asset_server = world.resource::<AssetServer>();
        asset_server.load(&character_entry.texture_path)
    };
    let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

    world.spawn((
        Player,
        Transform::from_translation(Vec3::new(
            player_data.position[0],
            player_data.position[1],
            player_data.position[2],
        ))
        .with_scale(Vec3::splat(PLAYER_SCALE)),
        sprite,
        AnimationController::default(),
        CharacterState::default(),
        Velocity::default(),
        player_data.facing,
        Collider::default(),
        PlayerCombat::new(player_data.power_type),
        Health {
            current: player_data.health_current,
            max: player_data.health_max,
        },
        AnimationTimer(Timer::from_seconds(
            DEFAULT_ANIMATION_FRAME_TIME,
            TimerMode::Repeating,
        )),
        character_entry,
    ));

    // Spawn enemies
    for enemy_data in &save_data.enemies {
        let enemy_entry = characters_list
            .characters
            .iter()
            .find(|c| c.name == enemy_data.character_name);

        let Some(enemy_entry) = enemy_entry else {
            warn!("Unknown enemy character: {}", enemy_data.character_name);
            continue;
        };
        let enemy_entry = enemy_entry.clone();

        let max_row = enemy_entry.calculate_max_animation_row();
        let layout = {
            let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
            layouts.add(TextureAtlasLayout::from_grid(
                UVec2::splat(enemy_entry.tile_size),
                enemy_entry.atlas_columns as u32,
                (max_row + 1) as u32,
                None,
                None,
            ))
        };
        let texture = {
            let asset_server = world.resource::<AssetServer>();
            asset_server.load(&enemy_entry.texture_path)
        };
        let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

        use crate::config::enemy::ENEMY_SCALE;
        use crate::enemy::components::*;

        world.spawn((
            Enemy,
            sprite,
            Transform::from_translation(Vec3::new(
                enemy_data.position[0],
                enemy_data.position[1],
                enemy_data.position[2],
            ))
            .with_scale(Vec3::splat(ENEMY_SCALE)),
            GlobalTransform::default(),
            AnimationController::default(),
            CharacterState::default(),
            Velocity::default(),
            enemy_data.facing,
            Collider::default(),
            EnemyCombat::default(),
            Health {
                current: enemy_data.health_current,
                max: enemy_data.health_max,
            },
            AIBehavior::default(),
            EnemyPath::default(),
            AnimationTimer(Timer::from_seconds(
                DEFAULT_ANIMATION_FRAME_TIME,
                TimerMode::Repeating,
            )),
            enemy_entry,
        ));
    }

    world
        .resource_mut::<Inventory>()
        .set_items(save_data.inventory);

    world.resource_mut::<PlayerSpawned>().0 = true;
    world.resource_mut::<EnemiesSpawned>().0 = true;
    world.resource_mut::<CurrentCharacterIndex>().index = save_data.player.character_index;
    world.resource_mut::<CollisionMapBuilt>().0 = false;
    world.insert_resource(crate::map::generate::MapReady);

    world
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);

    info!("Game loaded from slot {}", slot + 1);
}