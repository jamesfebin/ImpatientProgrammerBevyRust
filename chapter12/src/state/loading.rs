use std::sync::atomic::Ordering;

use bevy::prelude::*;

use crate::map::generate::{MapGenProgress, MapReady};

#[derive(Component)]
pub struct LoadingScreen;

#[derive(Component)]
pub struct LoadingText;

pub fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        LoadingScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
    )).with_children(|parent| {
        parent.spawn((
            LoadingText,
            Text::new("Loading..."),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
    
    info!("Loading screen spawned");
}

pub fn animate_loading(
    time: Res<Time>,
    mut query: Query<&mut Text, With<LoadingText>>,
    progress: Option<Res<MapGenProgress>>,
    map_ready: Option<Res<MapReady>>,
) {
    for mut text in query.iter_mut() {
        if map_ready.is_some() {
            **text = "Starting...".to_string();
        } else if let Some(ref progress) = progress {
            // Read atomic progress safely
            let current = progress.current.load(Ordering::Relaxed);
            **text = format!("Generating world: {}/{}", current, progress.total);
        } else {
            let dots = (time.elapsed_secs() * 2.0) as usize % 4;
            **text = format!("Loading{}", ".".repeat(dots));
        }
    }
}

pub fn despawn_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    
    info!("Loading screen despawned");
}