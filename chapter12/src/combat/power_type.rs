use bevy::prelude::*;
use crate::particles::components::{EmissionShape, ParticleConfig};
use serde::{Serialize, Deserialize};

/// The different magical powers available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PowerType {
    #[default]
    Fire,
    Arcane,
    Shadow,
    Poison,
}

#[derive(Clone)]
pub struct PowerVisuals {
    pub primary: ParticleConfig,
    pub core: Option<ParticleConfig>,
    pub particles_per_spawn: u32,
    pub core_particles_per_spawn: u32,
}

impl PowerType {
    /// Get visual configuration for this power
    pub fn visuals(&self, direction: Vec3) -> PowerVisuals {
        match self {
            PowerType::Fire => Self::fire_visuals(direction),
            PowerType::Arcane => Self::arcane_visuals(direction),
            PowerType::Shadow => Self::shadow_visuals(direction),
            PowerType::Poison => Self::poison_visuals(direction),
        }
    }

    pub fn damage(&self) -> f32 {
        match self {
            PowerType::Fire => 25.0,
            PowerType::Arcane => 35.0,
            PowerType::Shadow => 20.0,
            PowerType::Poison => 15.0,
        }
    }

    /// Hitbox collision radius. Larger for powers that spread visually.
    pub fn hitbox_radius(&self) -> f32 {
        match self {
            PowerType::Fire => 30.0,
            PowerType::Arcane => 18.0,
            PowerType::Shadow => 15.0,
            PowerType::Poison => 25.0,
        }
    }

    fn fire_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.0,
                lifetime_variance: 0.2,
                speed: 350.0,
                speed_variance: 40.0,
                direction,
                direction_variance: 0.12,
                scale: 1.5,
                scale_variance: 0.5,
                color: Color::srgb(3.0, 0.5, 0.1),  // Bright orange-red
                angular_velocity: 3.0,
                angular_velocity_variance: 2.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 10.0 },
            },
            core: Some(ParticleConfig {
                lifetime: 0.8,
                lifetime_variance: 0.2,
                speed: 350.0,
                speed_variance: 30.0,
                direction,
                direction_variance: 0.08,
                scale: 1.0,
                scale_variance: 0.3,
                color: Color::srgb(4.0, 1.0, 0.2),  // Very bright yellow-white
                angular_velocity: 5.0,
                angular_velocity_variance: 2.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 5.0 },
            }),
            particles_per_spawn: 5,
            core_particles_per_spawn: 3,
        }
    }

    fn arcane_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.2,
                lifetime_variance: 0.2,
                speed: 300.0,
                speed_variance: 30.0,
                direction,
                direction_variance: 0.05,  // Very precise
                scale: 1.2,
                scale_variance: 0.3,
                color: Color::srgb(0.5, 0.8, 2.5),  // Blue arcane energy
                angular_velocity: 2.0,
                angular_velocity_variance: 1.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 6.0 },
            },
            core: Some(ParticleConfig {
                lifetime: 1.0,
                lifetime_variance: 0.1,
                speed: 300.0,
                speed_variance: 20.0,
                direction,
                direction_variance: 0.03,  // Even more precise
                scale: 0.8,
                scale_variance: 0.2,
                color: Color::srgb(0.9, 0.95, 3.0),  // Bright white-blue
                angular_velocity: 0.5,
                angular_velocity_variance: 0.5,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            }),
            particles_per_spawn: 4,
            core_particles_per_spawn: 2,
        }
    }

    fn shadow_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 0.6,  // Short-lived
                lifetime_variance: 0.15,
                speed: 600.0,  // Very fast
                speed_variance: 100.0,
                direction,
                direction_variance: 0.04,
                scale: 1.0,
                scale_variance: 0.4,
                color: Color::srgb(0.6, 0.2, 1.2),  // Dark purple
                angular_velocity: 8.0,  // Spins fast
                angular_velocity_variance: 4.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            },
            core: Some(ParticleConfig {
                lifetime: 0.5,
                lifetime_variance: 0.1,
                speed: 650.0,
                speed_variance: 80.0,
                direction,
                direction_variance: 0.02,
                scale: 1.3,
                scale_variance: 0.3,
                color: Color::srgb(1.0, 0.5, 1.8),  // Brighter purple core
                angular_velocity: 12.0,
                angular_velocity_variance: 5.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            }),
            particles_per_spawn: 7,  // Many particles
            core_particles_per_spawn: 3,
        }
    }

    fn poison_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.5,  // Long-lived
                lifetime_variance: 0.4,
                speed: 200.0,  // Slow
                speed_variance: 50.0,
                direction,
                direction_variance: 0.25,  // Spreads a lot
                scale: 1.8,  // Large particles
                scale_variance: 0.6,
                color: Color::srgb(0.3, 2.0, 0.3),  // Toxic green
                angular_velocity: 1.0,
                angular_velocity_variance: 2.0,
                acceleration: Vec3::new(0.0, 20.0, 0.0),  // Rises slightly
                emission_shape: EmissionShape::Circle { radius: 15.0 },
            },
            core: None,  // No core - just a cloud
            particles_per_spawn: 6,
            core_particles_per_spawn: 0,
        }
    }
}