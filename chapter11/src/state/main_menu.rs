use bevy::prelude::*;

use crate::save::SaveLoadUIState;
use crate::save::ui::SaveLoadMode;

use super::GameState;

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub enum MainMenuButton {
    NewGame,
    LoadGame,
    Quit,
}

pub fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenuScreen,
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
                Text::new("Main Menu"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
            ));

            let buttons = [
                (MainMenuButton::NewGame, "New Game"),
                (MainMenuButton::LoadGame, "Load Game"),
                (MainMenuButton::Quit, "Quit"),
            ];

            for (btn_type, label) in buttons {
                parent
                    .spawn((
                        btn_type,
                        Button,
                        Node {
                            width: Val::Px(300.0),
                            height: Val::Px(55.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
                    ))
                    .with_children(|btn_parent| {
                        btn_parent.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_main_menu_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut ui_state: ResMut<SaveLoadUIState>,
    interaction_query: Query<(&Interaction, &MainMenuButton), Changed<Interaction>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            MainMenuButton::NewGame => {
                next_state.set(GameState::Loading);
            }
            MainMenuButton::LoadGame => {
                ui_state.active = true;
                ui_state.mode = SaveLoadMode::Load;
            }
            MainMenuButton::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

pub fn handle_main_menu_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
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