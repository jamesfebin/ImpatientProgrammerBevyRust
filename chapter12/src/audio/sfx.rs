// src/audio/sfx.rs
use bevy::prelude::*;
use crate::combat::PowerType;
use super::assets::AudioAssets;

#[derive(Event, Clone, Copy)]
pub enum SfxKind {
    PlayerShoot(PowerType),
    EnemyShoot,
    Hit,
    PlayerDeath,
    EnemyDeath,
    Pickup,
    ButtonClick,
    Jump,
}

pub fn on_sfx(
    trigger: On<SfxKind>,
    audio_assets: Option<Res<AudioAssets>>,
    mut commands: Commands,
) {
    let Some(audio) = audio_assets else { return; };

    let handle = match *trigger.event() {
        SfxKind::PlayerShoot(PowerType::Fire) => audio.spell_fire.clone(),
        SfxKind::PlayerShoot(_) => audio.spell_generic.clone(),
        SfxKind::EnemyShoot => audio.enemy_shoot.clone(),
        SfxKind::Hit => audio.hit.clone(),
        SfxKind::PlayerDeath => audio.player_death.clone(),
        SfxKind::EnemyDeath => audio.enemy_death.clone(),
        SfxKind::Pickup => audio.pickup.clone(),
        SfxKind::ButtonClick => audio.button_click.clone(),
        SfxKind::Jump => audio.jump.clone(),
    };

    commands.spawn((AudioPlayer::new(handle), PlaybackSettings::DESPAWN));
}