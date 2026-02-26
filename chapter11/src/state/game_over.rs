// src/state/game_over.rs
use bevy::prelude::*;

use crate::characters::input::Player;
use crate::characters::spawn::PlayerSpawned;
use crate::combat::healthbar::HealthBarOwner;
use crate::combat::systems::{Projectile, ProjectileEffect};
use crate::enemy::{spawn::EnemiesSpawned, Enemy};
use crate::particles::components::{Particle, ParticleEmitter};
use crate::collision::{TileMarker, CollisionMapBuilt};
use crate::inventory::Inventory;
use crate::map::generate::MapReady;

use super::GameState;

#[derive(Component)]
pub struct GameOverScreen;

pub fn spawn_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            GameOverScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER\n\nPress R to restart"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));
        });

    info!("Game over screen spawned");
}

pub fn despawn_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    info!("Game over screen despawned");
}

pub fn handle_restart_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        info!("Restarting game...");
        next_state.set(GameState::Loading);
    }
}

/// Despawns all gameplay entities and resets spawn flags so they re-trigger.
pub fn cleanup_game_world(
    mut commands: Commands,
    tiles: Query<Entity, With<TileMarker>>,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    projectiles: Query<Entity, With<Projectile>>,
    projectile_effects: Query<Entity, With<ProjectileEffect>>,
    emitters: Query<Entity, With<ParticleEmitter>>,
    particles: Query<Entity, With<Particle>>,
    healthbars: Query<Entity, With<HealthBarOwner>>,
    mut player_spawned: ResMut<PlayerSpawned>,
    mut enemies_spawned: ResMut<EnemiesSpawned>,
    mut collision_map_built: ResMut<CollisionMapBuilt>,
    mut inventory: ResMut<Inventory>
) {

    for entity in tiles.iter() {
        commands.entity(entity).despawn();
    }
    for entity in players.iter() {
        commands.entity(entity).despawn();
    }
    for entity in enemies.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectiles.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectile_effects.iter() {
        commands.entity(entity).despawn();
    }
    for entity in emitters.iter() {
        commands.entity(entity).despawn();
    }
    for entity in particles.iter() {
        commands.entity(entity).despawn();
    }
    for entity in healthbars.iter() {
        commands.entity(entity).despawn();
    }

    player_spawned.0 = false;
    enemies_spawned.0 = false;

    collision_map_built.0 = false;
    inventory.set_items(Default::default());
    commands.remove_resource::<MapReady>();

}