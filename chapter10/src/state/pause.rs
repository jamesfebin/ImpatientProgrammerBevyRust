use bevy::prelude::*;

use crate::save::SaveLoadUIState;
use crate::save::ui::SaveLoadMode;

use super::GameState;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseButton {
    Resume,
    SaveGame,
    LoadGame,
    MainMenu,
    Quit,
}

pub fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            PauseMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: FontSize::Px(42.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            let buttons = [
                (PauseButton::Resume, "Resume"),
                (PauseButton::SaveGame, "Save Game"),
                (PauseButton::LoadGame, "Load Game"),
                (PauseButton::MainMenu, "Main Menu"),
                (PauseButton::Quit, "Quit"),
            ];

            for (btn_type, label) in buttons {
                parent
                    .spawn((
                        btn_type,
                        Button,
                        Node {
                            width: Val::Px(250.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
                    ))
                    .with_children(|btn_parent| {
                        btn_parent.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: FontSize::Px(24.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });

    info!("Pause menu spawned");
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    info!("Pause menu despawned");
}

pub fn handle_pause_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut ui_state: ResMut<SaveLoadUIState>,
    interaction_query: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
    mut exit: MessageWriter<AppExit>,
) {
    if ui_state.active {
        return;
    }

    for (interaction, button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            PauseButton::Resume => {
                next_state.set(GameState::Playing);
            }
            PauseButton::SaveGame => {
                ui_state.active = true;
                ui_state.mode = SaveLoadMode::Save;
            }
            PauseButton::LoadGame => {
                ui_state.active = true;
                ui_state.mode = SaveLoadMode::Load;
            }
            PauseButton::MainMenu => {
                next_state.set(GameState::MainMenu);
            }
            PauseButton::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

pub fn handle_pause_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PauseButton>),
    >,
) {
    for (interaction, mut bg) in interaction_query.iter_mut() {
        *bg = match interaction {
            Interaction::Hovered => BackgroundColor(Color::srgba(0.25, 0.25, 0.5, 0.9)),
            Interaction::Pressed => BackgroundColor(Color::srgba(0.35, 0.35, 0.6, 0.9)),
            Interaction::None => BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
        };
    }
}
