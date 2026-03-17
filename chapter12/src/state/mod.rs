mod game_over;
mod game_state;
mod loading;
mod pause;
pub mod main_menu;

use bevy::prelude::*;
use crate::characters::spawn::CharactersListResource;
use crate::characters::config::CharactersList;
use crate::map::generate::MapReady;
use crate::save::SaveLoadUIState;

pub use game_state::GameState;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(
                OnEnter(GameState::MainMenu),
                (game_over::cleanup_game_world, main_menu::spawn_main_menu).chain(),
            )
            .add_systems(OnExit(GameState::MainMenu), main_menu::despawn_main_menu)
            .add_systems(
                Update,
                main_menu::handle_main_menu_buttons
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                Update,
                main_menu::handle_main_menu_hover
                    .run_if(in_state(GameState::MainMenu)),
            )
            // Loading state systems
            .add_systems(OnEnter(GameState::Loading), loading::spawn_loading_screen)
            .add_systems(Update, (
                check_assets_loaded,
                loading::animate_loading,
            ).run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), (
                loading::despawn_loading_screen
            ))
                // Pause state systems
            .add_systems(OnEnter(GameState::Paused), pause::spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), (pause::despawn_pause_menu, close_save_load_ui))
            .add_systems(
                Update,
                pause::handle_pause_buttons
                    .run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                Update,
                pause::handle_pause_hover
                    .run_if(in_state(GameState::Paused)),
            )
            // Pause toggle (works in Playing or Paused states)
            .add_systems(Update, 
                toggle_pause.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused)))
            )
            .add_systems(OnEnter(GameState::GameOver), game_over::spawn_game_over_screen)
            .add_systems(
                OnExit(GameState::GameOver),
                (game_over::despawn_game_over_screen, game_over::cleanup_game_world),
            )
            .add_systems(
                Update,
                game_over::handle_restart_input.run_if(in_state(GameState::GameOver)),
            );
    }
}

fn check_assets_loaded(
    characters_list_res: Option<Res<CharactersListResource>>,
    characters_lists: Res<Assets<CharactersList>>,
    map_ready: Option<Res<MapReady>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(res) = characters_list_res else {
        return;
    };
    
    if characters_lists.get(&res.handle).is_some() && map_ready.is_some()  {
        info!("Assets loaded, transitioning to Playing!");
        next_state.set(GameState::Playing);
    }
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    ui_state: Res<SaveLoadUIState>, 
) {
    if input.just_pressed(KeyCode::Escape) {

        if ui_state.active {
            return;
        }

        match current_state.get() {
            GameState::Playing => {
                info!("Game paused");
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                info!("Game resumed");
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}

fn close_save_load_ui(mut ui_state: ResMut<SaveLoadUIState>) {
    ui_state.active = false;
}