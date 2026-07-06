use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Playing,
    Paused,
    GameOver,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub enum GameMode {
    SinglePlayer,
    Multiplayer,
}

pub fn in_multiplayer(mode: Option<Res<GameMode>>) -> bool {
    mode.is_some_and(|m| *m == GameMode::Multiplayer)
}
