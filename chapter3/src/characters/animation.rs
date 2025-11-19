use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::characters::config::{CharacterEntry, AnimationType};

// Default animation timing (10 FPS = 0.1 seconds per frame)
pub const DEFAULT_ANIMATION_FRAME_TIME: f32 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Facing {
    Up,
    Left,
    Down,
    Right,
}

impl Facing {
    // Convert a velocity vector into a discrete direction
    pub fn from_direction(direction: Vec2) -> Self {
        if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 { Facing::Right } else { Facing::Left }
        } else {
            if direction.y > 0.0 { Facing::Up } else { Facing::Down }
        }
    }
    
    // Helper to map direction to row offset (0, 1, 2, 3)
    fn direction_index(self) -> usize {
        match self {
            Facing::Up => 0,
            Facing::Left => 1,
            Facing::Down => 2,
            Facing::Right => 3,
        }
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub current_animation: AnimationType,
    pub facing: Facing,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            current_animation: AnimationType::Walk,
            facing: Facing::Down,
        }
    }
}

#[derive(Component, Default)]
pub struct AnimationState {
    pub is_moving: bool,
    pub was_moving: bool,
    pub is_jumping: bool,
    pub was_jumping: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Copy)]
pub struct AnimationClip {
    first: usize,
    last: usize,
}

impl AnimationClip {
    pub fn new(row: usize, frame_count: usize, atlas_columns: usize) -> Self {
        let first = row * atlas_columns;
        Self {
            first,
            last: first + frame_count - 1,
        }
    }
    
    pub fn start(self) -> usize {
        self.first
    }
    
    // Check if a frame index belongs to this clip
    pub fn contains(self, index: usize) -> bool {
        (self.first..=self.last).contains(&index)
    }
    
    // Calculate the next frame, looping back to start if needed
    pub fn next(self, index: usize) -> usize {
        if index >= self.last {
            self.first
        } else {
            index + 1
        }
    }
    
    // Check if animation has completed (used for non-looping animations like Jump)
    pub fn is_complete(self, current_index: usize, timer_finished: bool) -> bool {
        current_index >= self.last && timer_finished
    }
}

impl AnimationController {
    pub fn get_clip(&self, config: &CharacterEntry) -> Option<AnimationClip> {
        // 1. Get the definition (e.g. "Walk" data)
        let def = config.animations.get(&self.current_animation)?;
        
        // 2. Calculate the actual row based on facing direction
        let row = if def.directional {
            def.start_row + self.facing.direction_index()
        } else {
            def.start_row
        };
        
        // 3. Create the clip
        Some(AnimationClip::new(row, def.frame_count, config.atlas_columns))
    }
}

pub fn animate_characters(
    time: Res<Time>,
    mut query: Query<(
        &AnimationController,
        &AnimationState,
        &mut AnimationTimer,
        &mut Sprite,
        &CharacterEntry,
    )>,
) {
    for (animated, state, mut timer, mut sprite, config) in query.iter_mut() {
        
        let Some(atlas) = sprite.texture_atlas.as_mut() else { continue; };
        
        // Get the correct clip for current state/facing
        let Some(clip) = animated.get_clip(config) else { continue; };
        
        // Get timing info
        let Some(anim_def) = config.animations.get(&animated.current_animation) else { continue; };
        
        // Safety: If we somehow ended up on a frame outside our clip, reset.
        if !clip.contains(atlas.index) {
            atlas.index = clip.start();
            timer.0.reset();
        }
        
        // Detect state changes
        let just_started_moving = state.is_moving && !state.was_moving;
        let just_stopped_moving = !state.is_moving && state.was_moving;
        let just_started_jumping = state.is_jumping && !state.was_jumping;
        let just_stopped_jumping = !state.is_jumping && state.was_jumping;
        
        let should_animate = state.is_jumping || state.is_moving;
        let animation_changed = just_started_moving || just_started_jumping 
                              || just_stopped_moving || just_stopped_jumping;
        
        if animation_changed {
            // Reset animation
            atlas.index = clip.start();
            timer.0.set_duration(std::time::Duration::from_secs_f32(anim_def.frame_time));
            timer.0.reset();
        } else if should_animate {
            // Advance animation
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = clip.next(atlas.index);
            }
        } else {
            // When idle (not moving or jumping), stay on frame 0
            if atlas.index != clip.start() {
                atlas.index = clip.start();
            }
        }
    }
}

// Helper to update "was_moving" flags at the end of the frame
pub fn update_animation_flags(mut query: Query<&mut AnimationState>) {
    for mut state in query.iter_mut() {
        state.was_moving = state.is_moving;
        state.was_jumping = state.is_jumping;
    }
}
