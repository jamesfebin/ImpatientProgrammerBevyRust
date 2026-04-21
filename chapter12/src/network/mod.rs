mod connection;

use bevy::prelude::*;

use crate::module_bindings::DbConnection;
use crate::state::{GameState, in_multiplayer};

use connection::{
    cleanup_network, connect_to_spacetimedb, despawn_multiplayer_screen, handle_multiplayer_back,
    process_spacetimedb_messages, spawn_multiplayer_screen, update_multiplayer_screen,
};

#[derive(Resource)]
pub struct SpacetimeConnection {
    pub conn: DbConnection,
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Loading),
            (connect_to_spacetimedb, spawn_multiplayer_screen).run_if(in_multiplayer),
        )
        .add_systems(
            Update,
            (
                process_spacetimedb_messages.run_if(resource_exists::<SpacetimeConnection>),
                update_multiplayer_screen.run_if(in_state(GameState::Loading)),
                handle_multiplayer_back.run_if(in_state(GameState::Loading)),
            )
                .run_if(in_multiplayer),
        )
        .add_systems(OnExit(GameState::Loading), despawn_multiplayer_screen)
        .add_systems(
            OnEnter(GameState::MainMenu),
            cleanup_network.run_if(resource_exists::<SpacetimeConnection>),
        );
    }
}
