// src/enemy/components.rs
use crate::combat::PowerType;
use bevy::prelude::*;

/// Marker component for enemy entities
#[derive(Component)]
pub struct Enemy;

/// Combat capabilities for enemies
#[derive(Component)]
pub struct EnemyCombat {
    pub power_type: PowerType,
    pub cooldown: Timer,
}

impl Default for EnemyCombat {
    fn default() -> Self {
        Self {
            power_type: PowerType::Shadow, // Graveyard reaper uses shadow magic
            cooldown: Timer::from_seconds(2.0, TimerMode::Once), // Slower than player
        }
    }
}

impl EnemyCombat {
    pub fn new(power_type: PowerType, cooldown_seconds: f32) -> Self {
        Self {
            power_type,
            cooldown: Timer::from_seconds(cooldown_seconds, TimerMode::Once),
        }
    }
}

/// AI behavior state for enemies
#[derive(Component)]
pub struct AIBehavior {
    pub attack_range: f32,
    pub detection_range: f32,
}

impl Default for AIBehavior {
    fn default() -> Self {
        Self {
            attack_range: 150.0,    // Stop and attack within this range
            detection_range: 500.0, // Start following player within this range
        }
    }
}

impl AIBehavior {
    pub fn new(attack_range: f32, detection_range: f32) -> Self {
        Self {
            attack_range,
            detection_range,
        }
    }
}

#[derive(Component, Default)]
pub struct EnemyPath {
    /// Waypoints in world coordinates
    pub waypoints: Vec<Vec2>,
    /// Current waypoint index
    pub current_index: usize,
    /// Timer for path recalculation
    pub recalc_timer: f32,
}

impl EnemyPath {
    /// Distance threshold to consider a waypoint reached
    pub const WAYPOINT_THRESHOLD: f32 = 16.0;
    /// How often to recalculate path (seconds)
    pub const RECALC_INTERVAL: f32 = 0.5;
    
    /// Get current waypoint position
    pub fn current_waypoint(&self) -> Option<Vec2> {
        self.waypoints.get(self.current_index).copied()
    }
    
    /// Advance to next waypoint, returns true if path completed
    pub fn advance(&mut self) -> bool {
        self.current_index += 1;
        self.current_index >= self.waypoints.len()
    }
    
    /// Set a new path (skips first waypoint since it's the starting position)
    pub fn set_path(&mut self, waypoints: Vec<Vec2>) {
        // Skip waypoint 0 - it's the enemy's current position
        // This prevents jitter from briefly facing backwards
        let new_waypoints = if waypoints.len() > 1 {
            waypoints[1..].to_vec()
        } else {
            waypoints
        };
        
        // If we have an existing path, check if new path is similar
        // This prevents flickering when paths are recalculated
        if let Some(current_target) = self.current_waypoint() {
            if let Some(new_first) = new_waypoints.first() {
                // If new first waypoint is close to our current target,
                // keep the current path - we're already heading the right way
                if current_target.distance(*new_first) < Self::WAYPOINT_THRESHOLD * 1.5 {
                    return;
                }
            }
        }
        
        self.waypoints = new_waypoints;
        self.current_index = 0;
    }
    
    /// Check if we have a valid path
    pub fn has_path(&self) -> bool {
        !self.waypoints.is_empty() && self.current_index < self.waypoints.len()
    }
}