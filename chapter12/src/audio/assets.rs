// src/audio/assets.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioAssets {
    pub menu_music: Handle<AudioSource>,
    pub battle_music: Handle<AudioSource>,
    pub spell_generic: Handle<AudioSource>,
    pub spell_fire: Handle<AudioSource>,
    pub enemy_shoot: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub player_death: Handle<AudioSource>,
    pub enemy_death: Handle<AudioSource>,
    pub pickup: Handle<AudioSource>,
    pub button_click: Handle<AudioSource>,
    pub jump: Handle<AudioSource>,
}

pub fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        menu_music: asset_server.load("audio/music/menu_ambient.mp3"),
        battle_music: asset_server.load("audio/music/battle_theme.ogg"),
        spell_generic: asset_server.load("audio/sfx/spell_01.ogg"),
        spell_fire: asset_server.load("audio/sfx/spell_fire.wav"),
        enemy_shoot: asset_server.load("audio/sfx/spell_enemy.ogg"),
        hit: asset_server.load("audio/sfx/hit.wav"),
        player_death: asset_server.load("audio/sfx/death_player.mp3"),
        enemy_death: asset_server.load("audio/sfx/death_enemy.mp3"),
        pickup: asset_server.load("audio/sfx/pickup.wav"),
        button_click: asset_server.load("audio/sfx/button_click.wav"),
        jump: asset_server.load("audio/sfx/jump.wav"),
    });
}