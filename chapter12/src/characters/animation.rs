use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::characters::config::{CharacterEntry, AnimationType};
use crate::characters::facing::Facing;
use crate::characters::state::CharacterState; 

// Default animation timing (10 FPS = 0.1 seconds per frame)
pub const DEFAULT_ANIMATION_FRAME_TIME: f32 = 0.1;

#[derive(Component, Default)]
pub struct AnimationController {
    pub current_animation: AnimationType,
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
    /// Get the animation clip for the current animation and facing direction.
    /// `facing` is passed in since it's now a separate component.
    pub fn get_clip(&self, config: &CharacterEntry, facing: Facing) -> Option<AnimationClip> {
        let def = config.animations.get(&self.current_animation)?;
        
        let row = if def.directional {
            def.start_row + facing.direction_index()
        } else {
            def.start_row
        };
        
        Some(AnimationClip::new(row, def.frame_count, config.atlas_columns))
    }
}


pub fn on_state_change_update_animation(
    mut query: Query<
        (&CharacterState, &mut AnimationController, &mut AnimationTimer),
        Changed<CharacterState>
    >,
) {
    for (state, mut controller, mut timer) in query.iter_mut() {
        // Select animation based on new state
        let new_animation = match state {
            CharacterState::Idle | CharacterState::Walking => AnimationType::Walk,
            CharacterState::Running => AnimationType::Run,
            CharacterState::Jumping => AnimationType::Jump,
        };
        
        // Only update and reset timer if animation actually changed
        if controller.current_animation != new_animation {
            controller.current_animation = new_animation;
            timer.0.reset();
        }
    }
}

pub fn animations_playback(
    time: Res<Time>,
    mut query: Query<(
        &CharacterState,
        &Facing,
        &AnimationController,
        &mut AnimationTimer,
        &mut Sprite,
        &CharacterEntry,
    )>,
) {
    for (state, facing, controller, mut timer, mut sprite, config) in query.iter_mut() {
        // Don't animate when idle
        if *state == CharacterState::Idle {
            // Ensure idle sprite is at frame 0
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                if let Some(clip) = controller.get_clip(config, *facing) {
                    if atlas.index != clip.start() {
                        atlas.index = clip.start();
                    }
                }
            }
            continue;
        }
        
        let Some(atlas) = sprite.texture_atlas.as_mut() else { continue; };
        let Some(clip) = controller.get_clip(config, *facing) else { continue; };
        let Some(anim_def) = config.animations.get(&controller.current_animation) else { continue; };
        
        // Safety: If we somehow ended up on a frame outside our clip, reset.
        if !clip.contains(atlas.index) {
            atlas.index = clip.start();
            timer.0.reset();
        }
        
        // Update timer duration if needed
        let expected_duration = std::time::Duration::from_secs_f32(anim_def.frame_time);
        if timer.0.duration() != expected_duration {
            timer.0.set_duration(expected_duration);
        }
        
        // Advance animation
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = clip.next(atlas.index);
        }
    }
}