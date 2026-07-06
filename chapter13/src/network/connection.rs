use std::path::PathBuf;
use std::sync::atomic::Ordering;

use bevy::prelude::*;
use spacetimedb_sdk::{DbContext, Table};

use super::SpacetimeConnection;
use crate::map::generate::{MapGenProgress, MapReady};
use crate::map::seed::WorldSeed;
use crate::module_bindings::DbConnection;
use crate::module_bindings::player_table::{PlayerTableAccess, playerQueryTableAccess};
use crate::module_bindings::world_config_table::{
    WorldConfigTableAccess, world_configQueryTableAccess,
};
use crate::state::{GameMode, GameState};

const SPACETIMEDB_URI: &str = "http://127.0.0.1:3000";
const DATABASE_NAME: &str = "bevy-game";
const TOKEN_FILENAME: &str = "spacetimedb_token";

fn token_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    Some(exe.parent()?.join(TOKEN_FILENAME))
}

fn load_token() -> Option<String> {
    let path = token_path()?;
    let contents = std::fs::read_to_string(&path).ok()?;
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn save_token(token: &str) -> std::io::Result<()> {
    let path = token_path().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "could not determine executable path",
        )
    })?;
    std::fs::write(path, token)
}

pub fn connect_to_spacetimedb(mut commands: Commands) {
    let token = load_token();

    let conn = DbConnection::builder()
        .with_uri(SPACETIMEDB_URI)
        .with_database_name(DATABASE_NAME)
        .with_token(token)
        .on_connect(|ctx, _identity, token| {
            if let Err(e) = save_token(token) {
                error!("Failed to save SpacetimeDB token: {e}");
            }
            info!("Connected to SpacetimeDB");
            ctx.subscription_builder()
                .on_applied(|ctx| {
                    if let Some(identity) = ctx.try_identity() {
                        if let Some(player) = ctx.db.player().identity().find(&identity) {
                            info!("Playing as: {}", player.username);
                            return;
                        }
                    }
                    info!("Player subscription applied");
                })
                .on_error(|_ctx, err| {
                    error!("Subscription error: {err}");
                })
                .add_query(|q| q.from.player())
                .add_query(|q| q.from.world_config())
                .subscribe();
        })
        .on_connect_error(|_ctx, err| {
            error!("SpacetimeDB connection error: {err}");
        })
        .on_disconnect(|_ctx, err| {
            if let Some(e) = err {
                warn!("Disconnected from SpacetimeDB with error: {e}");
            } else {
                info!("Disconnected from SpacetimeDB");
            }
        })
        .build();

    match conn {
        Ok(conn) => {
            info!("SpacetimeDB connection initiated");
            commands.insert_resource(SpacetimeConnection { conn });
        }
        Err(e) => {
            error!("Failed to initiate SpacetimeDB connection: {e}");
        }
    }
}

pub fn fetch_world_seed(mut commands: Commands, connection: Res<SpacetimeConnection>) {
    if let Some(config) = connection.conn.db.world_config().id().find(&0) {
        info!("Received multiplayer world seed: {}", config.map_seed);
        commands.insert_resource(PendingWorldSeed(config.map_seed));
    }
}

pub fn process_spacetimedb_messages(connection: Res<SpacetimeConnection>) {
    if let Err(e) = connection.conn.frame_tick() {
        error!("SpacetimeDB frame_tick error: {e}");
    }
}

#[derive(Component)]
pub struct MultiplayerScreen;

#[derive(Component)]
pub struct ConnectionStatusText;

#[derive(Component)]
pub struct OnlinePlayersText;

pub fn spawn_multiplayer_screen(mut commands: Commands) {
    commands
        .spawn((
            MultiplayerScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Multiplayer"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));
            parent.spawn((
                ConnectionStatusText,
                Text::new("Connecting..."),
                TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            parent.spawn((
                OnlinePlayersText,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.9, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            parent
                .spawn((
                    JoinButton,
                    Button,
                    Node {
                        display: Display::None,
                        width: Val::Px(220.0),
                        height: Val::Px(55.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(30.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.15, 0.3, 0.15, 0.9)),
                ))
                .with_children(|btn_parent| {
                    btn_parent.spawn((
                        Text::new("Join"),
                        TextFont {
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent.spawn((
                Text::new("Press Backspace to return to Main Menu"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
            ));
        });
}

pub fn update_join_button(
    pending: Option<Res<PendingWorldSeed>>,
    world_seed: Option<Res<WorldSeed>>,
    mut query: Query<&mut Node, With<JoinButton>>,
) {
    let should_show = pending.is_some() && world_seed.is_none();
    for mut node in query.iter_mut() {
        node.display = if should_show {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn handle_join_button(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<JoinButton>)>,
    pending: Option<Res<PendingWorldSeed>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Some(pending) = pending.as_ref() else {
            continue;
        };
        info!("Joining multiplayer world with seed {}", pending.0);
        commands.insert_resource(WorldSeed(pending.0));
        commands.remove_resource::<PendingWorldSeed>();
    }
}

pub fn handle_join_button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<JoinButton>),
    >,
) {
    for (interaction, mut bg) in interaction_query.iter_mut() {
        *bg = match interaction {
            Interaction::Hovered => BackgroundColor(Color::srgba(0.25, 0.45, 0.25, 0.9)),
            Interaction::Pressed => BackgroundColor(Color::srgba(0.35, 0.6, 0.35, 0.9)),
            Interaction::None => BackgroundColor(Color::srgba(0.15, 0.3, 0.15, 0.9)),
        };
    }
}

pub fn update_multiplayer_screen(
    connection: Option<Res<SpacetimeConnection>>,
    pending_seed: Option<Res<PendingWorldSeed>>,
    world_seed: Option<Res<WorldSeed>>,
    progress: Option<Res<MapGenProgress>>,
    map_ready: Option<Res<MapReady>>,
    mut status_query: Query<&mut Text, (With<ConnectionStatusText>, Without<OnlinePlayersText>)>,
    mut online_query: Query<&mut Text, (With<OnlinePlayersText>, Without<ConnectionStatusText>)>,
) {
    let local_identity = connection.as_ref().and_then(|c| c.conn.try_identity());

    let status = if world_seed.is_some() {
        if map_ready.is_some() {
            "Entering world...".to_string()
        } else if let Some(progress) = progress.as_ref() {
            let current = progress.current.load(Ordering::Relaxed);
            format!("Generating world: {}/{} chunks", current, progress.total)
        } else {
            "Generating world...".to_string()
        }
    } else if let Some(conn) = &connection {
        if let Some(identity) = local_identity {
            if let Some(player) = conn.conn.db.player().identity().find(&identity) {
                if pending_seed.is_some() {
                    format!("Ready to join as: {}", player.username)
                } else {
                    format!("Connected as: {}", player.username)
                }
            } else {
                "Connected, waiting for player data...".to_string()
            }
        } else {
            "Authenticating...".to_string()
        }
    } else {
        "Connecting...".to_string()
    };

    let online_list = if let Some(conn) = &connection {
        let mut others: Vec<String> = conn
            .conn
            .db
            .player()
            .iter()
            .filter(|p| p.is_online && Some(p.identity) != local_identity)
            .map(|p| p.username)
            .collect();
        others.sort();

        if others.is_empty() {
            "No other players online".to_string()
        } else {
            let mut s = String::from("Online players:");
            for name in others {
                s.push_str("\n- ");
                s.push_str(&name);
            }
            s
        }
    } else {
        String::new()
    };

    for mut text in status_query.iter_mut() {
        text.0 = status.clone();
    }
    for mut text in online_query.iter_mut() {
        text.0 = online_list.clone();
    }
}

pub fn handle_multiplayer_back(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Backspace) {
        next_state.set(GameState::MainMenu);
    }
}

pub fn despawn_multiplayer_screen(
    mut commands: Commands,
    query: Query<Entity, With<MultiplayerScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_network(mut commands: Commands, connection: Res<SpacetimeConnection>) {
    let _ = connection.conn.disconnect();
    commands.remove_resource::<SpacetimeConnection>();
    commands.insert_resource(GameMode::SinglePlayer);
    info!("Network cleaned up");
}

#[derive(Resource)]
pub struct PendingWorldSeed(pub u64);

#[derive(Component)]
pub struct JoinButton;
