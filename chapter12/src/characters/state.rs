// src/characters/state.rs
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

/// Character states. Only one can be active at a time.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CharacterState {
    #[default]
    Idle,
    Walking,
    Running,
    Jumping,
}

// Append to src/characters/state.rs
impl CharacterState {
    /// Check if this is a grounded state (can jump from here)
    pub fn is_grounded(&self) -> bool {
        matches!(self, CharacterState::Idle | CharacterState::Walking | CharacterState::Running)
    }
}