// src/audio/music.rs
use bevy::prelude::*;
use super::assets::AudioAssets;

/// Marker for music entities so we can find and stop them.
#[derive(Component)]
pub struct MusicTrack;

pub fn start_menu_music(
    mut commands: Commands,
    audio_assets: Option<Res<AudioAssets>>,
    existing: Query<Entity, With<MusicTrack>>,
) {
    // 1. Stop any currently playing music
    for entity in existing.iter() {
        commands.entity(entity).despawn();
    }
    
    // 2. Safely get our loaded audio assets
    let Some(audio) = audio_assets else { return; };
    
    // 3. Spawn the new audio player
    commands.spawn((
        MusicTrack,
        AudioPlayer::new(audio.menu_music.clone()),
        PlaybackSettings::LOOP,
    ));
}

pub fn start_battle_music(
    mut commands: Commands,
    audio_assets: Option<Res<AudioAssets>>,
    existing: Query<Entity, With<MusicTrack>>,
) {
    for entity in existing.iter() {
        commands.entity(entity).despawn();
    }
    let Some(audio) = audio_assets else { return; };
    commands.spawn((
        MusicTrack,
        AudioPlayer::new(audio.battle_music.clone()),
        PlaybackSettings::LOOP,
    ));
}

pub fn stop_all_music(
    mut commands: Commands,
    existing: Query<Entity, With<MusicTrack>>,
) {
    for entity in existing.iter() {
        commands.entity(entity).despawn();
    }
}