use bevy::prelude::*;
use super::CollisionMap;
use crate::characters::input::Player;
use crate::characters::collider::Collider;

/// Resource to toggle debug visualization.
#[derive(Resource, Default)]
pub struct DebugCollisionEnabled(pub bool);

pub fn toggle_debug_collision(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_enabled: ResMut<DebugCollisionEnabled>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        debug_enabled.0 = !debug_enabled.0;
        if debug_enabled.0 {
            info!("🔍 Collision debug ENABLED (F3 to toggle)");
        } else {
            info!("Collision debug disabled");
        }
    }
}

pub fn debug_draw_collision(
    map: Option<Res<CollisionMap>>,
    debug_enabled: Res<DebugCollisionEnabled>,
    mut gizmos: Gizmos,
) {
    if !debug_enabled.0 {
        return;
    }

    let Some(map) = map else { return };

    let tile_size = map.tile_size();
    let origin = map.origin();

    // Draw each tile
    for y in 0..map.height() {
        for x in 0..map.width() {
            let world_pos = Vec2::new(
                origin.x + (x as f32 + 0.5) * tile_size,
                origin.y + (y as f32 + 0.5) * tile_size,
            );

            let color = if map.is_walkable(x, y) {
                Color::srgba(0.0, 1.0, 0.0, 0.25)  // Green, 25% opacity
            } else {
                Color::srgba(1.0, 0.0, 0.0, 0.4)   // Red, 40% opacity
            };

            gizmos.rect_2d(
                world_pos,
                Vec2::splat(tile_size * 0.9),
                color,
            );
        }
    }
}

pub fn debug_player_position(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    map: Option<Res<CollisionMap>>,
    debug_enabled: Res<DebugCollisionEnabled>,
    mut gizmos: Gizmos,
) {
    if !debug_enabled.0 {
        return;
    }

    let Some(map) = map else { return };
    let Ok((transform, collider)) = player_query.single() else { return };

    let center = transform.translation.truncate();
    
    // Get actual collider position
    let collider_pos = collider.world_position(transform);
    let grid = map.world_to_grid(collider_pos);

    // Draw line from center to collider (shows offset)
    gizmos.line_2d(center, collider_pos, Color::srgba(1.0, 1.0, 0.0, 0.5));

    // Draw actual collider circle
    gizmos.circle_2d(collider_pos, collider.radius, Color::srgb(0.0, 1.0, 1.0));

    // Draw current grid cell outline
    if map.in_bounds(grid.x, grid.y) {
        let cell_center = map.grid_to_world(grid.x, grid.y);
        gizmos.rect_2d(
            cell_center,
            Vec2::splat(map.tile_size()),
            Color::srgb(1.0, 1.0, 0.0),
        );

        // Draw red X if on unwalkable tile
        if !map.is_walkable(grid.x, grid.y) {
            let offset = 15.0;
            gizmos.line_2d(
                collider_pos + Vec2::new(-offset, -offset),
                collider_pos + Vec2::new(offset, offset),
                Color::srgb(1.0, 0.0, 0.0),
            );
            gizmos.line_2d(
                collider_pos + Vec2::new(-offset, offset),
                collider_pos + Vec2::new(offset, -offset),
                Color::srgb(1.0, 0.0, 0.0),
            );
        }
    }
}