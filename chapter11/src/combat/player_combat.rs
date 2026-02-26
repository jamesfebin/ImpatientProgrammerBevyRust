use super::power_type::PowerType;
use bevy::prelude::*;

/// Attach to any entity that can use powers (player, NPCs)
#[derive(Component)]
pub struct PlayerCombat {
    pub power_type: PowerType,
    pub cooldown: Timer,
}

impl Default for PlayerCombat {
    fn default() -> Self {
        Self {
            power_type: PowerType::Fire,
            cooldown: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

impl PlayerCombat {
    pub fn new(power_type: PowerType) -> Self {
        Self {
            power_type,
            cooldown: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }

    pub fn with_cooldown(mut self, seconds: f32) -> Self {
        self.cooldown = Timer::from_seconds(seconds, TimerMode::Once);
        self
    }
}