use bevy::prelude::*;

/// A single particle in the particle system
#[derive(Component, Clone)]
pub struct Particle {
    pub velocity: Vec3,           // Movement speed and direction (units/sec)
    pub lifetime: f32,             // Remaining time before death (seconds)
    pub max_lifetime: f32,         // Original lifetime for progress calculation
    pub scale: f32,                // Current size multiplier
    pub angular_velocity: f32,     // Rotation speed (radians/sec)
    pub acceleration: Vec3,        // Forces like gravity (units/sec²)
    // Color curve support (start → mid → end)
    pub start_color: Color,        // Color at birth (0% lifetime)
    pub mid_color: Color,          // Color at midpoint (50% lifetime)
    pub end_color: Color,          // Color at death (100% lifetime)
    // Scale curve support
    pub start_scale: f32,          // Size at birth
    pub end_scale: f32,            // Size at death (usually smaller)
}

impl Particle {
    pub fn new(velocity: Vec3, lifetime: f32, scale: f32, start_color: Color) -> Self {
        Self {
            velocity,
            lifetime,
            max_lifetime: lifetime,
            scale,
            angular_velocity: 0.0,
            acceleration: Vec3::ZERO,
            start_color,
            mid_color: start_color,  // Default to same color
            end_color: start_color,
            start_scale: scale,
            end_scale: scale * 0.5,  // Default: shrink to half
        }
    }

    pub fn with_angular_velocity(mut self, angular_velocity: f32) -> Self {
        self.angular_velocity = angular_velocity;
        self
    }

    pub fn with_acceleration(mut self, acceleration: Vec3) -> Self {
        self.acceleration = acceleration;
        self
    }

    /// Set color curve for smooth color transitions
    pub fn with_color_curve(mut self, mid: Color, end: Color) -> Self {
        self.mid_color = mid;
        self.end_color = end;
        self
    }

    /// Set scale curve
    pub fn with_scale_curve(mut self, end_scale: f32) -> Self {
        self.end_scale = end_scale;
        self
    }

    /// Returns the normalized lifetime progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        1.0 - (self.lifetime / self.max_lifetime)
    }
    
    /// Get interpolated color based on lifetime progress
    pub fn current_color(&self) -> Color {
        let progress = self.progress();
        
        if progress < 0.5 {
            // First half: start → mid
            let t = progress * 2.0;  // Remap 0.0-0.5 to 0.0-1.0
            self.start_color.mix(&self.mid_color, t)
        } else {
            // Second half: mid → end
            let t = (progress - 0.5) * 2.0;  // Remap 0.5-1.0 to 0.0-1.0
            self.mid_color.mix(&self.end_color, t)
        }
    }
    
    /// Get interpolated scale based on lifetime progress
    pub fn current_scale(&self) -> f32 {
        let progress = self.progress();
        self.start_scale.lerp(self.end_scale, progress)
    }
}

#[derive(Component, Clone)]
pub struct ParticleEmitter {
    pub spawn_timer: Timer,
    pub particles_per_spawn: u32,
    pub particle_config: ParticleConfig,
    pub active: bool,
    pub one_shot: bool,
    pub has_spawned: bool,
}

impl ParticleEmitter {
    pub fn new(spawn_rate: f32, particles_per_spawn: u32, particle_config: ParticleConfig) -> Self {
        Self {
            spawn_timer: Timer::from_seconds(spawn_rate, TimerMode::Repeating),
            particles_per_spawn,
            particle_config,
            active: true,
            one_shot: false,
            has_spawned: false,
        }
    }

    pub fn one_shot(mut self) -> Self {
        self.one_shot = true;
        self
    }
}

#[derive(Clone)]
pub struct ParticleConfig {
    pub lifetime: f32,
    pub lifetime_variance: f32,
    pub speed: f32,
    pub speed_variance: f32,
    pub direction: Vec3,
    pub direction_variance: f32,  // In radians
    pub scale: f32,
    pub scale_variance: f32,
    pub color: Color,
    pub angular_velocity: f32,
    pub angular_velocity_variance: f32,
    pub acceleration: Vec3,
    pub emission_shape: EmissionShape,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            lifetime: 1.0,
            lifetime_variance: 0.1,
            speed: 100.0,
            speed_variance: 10.0,
            direction: Vec3::X,
            direction_variance: 0.1,
            scale: 1.0,
            scale_variance: 0.1,
            color: Color::WHITE,
            angular_velocity: 0.0,
            angular_velocity_variance: 0.0,
            acceleration: Vec3::ZERO,
            emission_shape: EmissionShape::Point,
        }
    }
}

#[derive(Clone)]
pub enum EmissionShape {
    Point,
    Circle { radius: f32 },
    Cone { angle: f32 },
}