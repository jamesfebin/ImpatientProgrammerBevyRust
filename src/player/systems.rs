use bevy::prelude::*;

use super::components::{
    ANIM_DT, AnimationState, AnimationTimer, DirectionalClips, Facing, MOVE_SPEED, PLAYER_Z,
    Player, TILE_SIZE, WALK_FRAMES,
};
use crate::map::Map;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("male_spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    let facing = Facing::Down;
    let directional_clips = DirectionalClips::walk(WALK_FRAMES);
    let start_index = directional_clips.clip(facing).start();

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::new(0., 0., PLAYER_Z)).with_scale(Vec3::splat(1.2)),
        Player,
        directional_clips,
        AnimationState {
            facing,
            moving: false,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    map: Option<Res<Map>>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        // Calculate the proposed new position
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let new_pos = current_pos + delta;
        
        // Use robust circle-based collision with swept movement
        // Player collider radius = 12 pixels (3/8 of 32px tile) - increased for safety
        let collider_radius = 24.0;
        
        let new_pos = if let Some(map) = map.as_ref() {
            // Use swept movement to prevent tunneling and ensure smooth collision
            map.try_move_circle(current_pos, new_pos, collider_radius)
        } else {
            new_pos
        };
        
        // Only move if position changed (collision prevented perfect movement)
        let can_move = new_pos != current_pos;
        
        // Only move if the destination is walkable
        if can_move {
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
            anim.moving = true;

            // Update facing direction
            if direction.x.abs() > direction.y.abs() {
                anim.facing = if direction.x > 0.0 {
                    Facing::Right
                } else {
                    Facing::Left
                };
            } else {
                anim.facing = if direction.y > 0.0 {
                    Facing::Up
                } else {
                    Facing::Down
                };
            }
        } else {
            // Blocked by unwalkable tile - stop moving
            anim.moving = false;
        }
    } else {
        anim.moving = false;
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<
        (
            &DirectionalClips,
            &mut AnimationState,
            &mut AnimationTimer,
            &mut Sprite,
        ),
        With<Player>,
    >,
) {
    let Ok((clips, mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    let atlas = match sprite.texture_atlas.as_mut() {
        Some(atlas) => atlas,
        None => return,
    };

    let clip = clips.clip(anim.facing);
    if !clip.contains(atlas.index) {
        atlas.index = clip.start();
        timer.reset();
    }

    let just_started = anim.moving && !anim.was_moving;
    let just_stopped = !anim.moving && anim.was_moving;

    if anim.moving {
        if just_started {
            atlas.index = clip.next(clip.start());
            timer.reset();
        } else {
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = clip.next(atlas.index);
            }
        }
    } else if just_stopped {
        atlas.index = clip.start();
        timer.reset();
    }

    anim.was_moving = anim.moving;
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, animate_player));
    }
}
