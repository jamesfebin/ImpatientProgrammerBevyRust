use super::components::*;
use super::material::ParticleMaterial;
use bevy::prelude::*;
use rand::Rng;

/// System to update particle emitters and spawn new particles
pub fn update_emitters(
    mut commands: Commands,
    time: Res<Time>,
    mut emitters: Query<(Entity, &mut ParticleEmitter, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut emitter, global_transform) in emitters.iter_mut() {
        if !emitter.active {
            continue;
        }

        // Handle one-shot emitters
        if emitter.one_shot && emitter.has_spawned {
            emitter.active = false;
            continue;
        }

        emitter.spawn_timer.tick(time.delta());

        if emitter.spawn_timer.just_finished() {
            emitter.has_spawned = true;

            // Spawn particles
            for i in 0..emitter.particles_per_spawn {
                spawn_particle(
                    &mut commands,
                    &emitter.particle_config,
                    global_transform,
                    &mut rng,
                    &mut meshes,
                    &mut materials,
                    Some(entity),
                    i,
                );
            }

            if emitter.one_shot {
                emitter.active = false;
            }
        }
    }
}

pub fn spawn_particle(
    commands: &mut Commands,
    config: &ParticleConfig,
    global_transform: &GlobalTransform,
    rng: &mut rand::rngs::ThreadRng,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ParticleMaterial>>,
    owner: Option<Entity>,
    _particle_index: u32,
) {
    // Calculate randomized values
    let lifetime =
        config.lifetime + rng.gen_range(-config.lifetime_variance..config.lifetime_variance);
    let speed = config.speed + rng.gen_range(-config.speed_variance..config.speed_variance);
    let scale = config.scale + rng.gen_range(-config.scale_variance..config.scale_variance);
    let angular_velocity = config.angular_velocity
        + rng.gen_range(-config.angular_velocity_variance..config.angular_velocity_variance);

    // Calculate direction with variance
    let base_direction = config.direction.normalize_or_zero();
    let direction = if config.direction_variance > 0.0 {
        apply_direction_variance(base_direction, config.direction_variance, rng)
    } else {
        base_direction
    };

    // Calculate emission offset based on shape
    let emission_offset = match config.emission_shape {
        EmissionShape::Point => Vec3::ZERO,
        EmissionShape::Circle { radius } => {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(0.0..radius);
            Vec3::new(angle.cos() * distance, angle.sin() * distance, 0.0)
        }
        EmissionShape::Cone { angle } => {
            let cone_angle = rng.gen_range(-angle..angle);
            let rotated = rotate_vector_2d(base_direction, cone_angle);
            rotated * rng.gen_range(0.0..1.0)
        }
    };

    let velocity = direction * speed;

    // Get position directly from GlobalTransform
    let emitter_position = global_transform.translation();
    let mut position = emitter_position + emission_offset;

    // Ensure particles are at a visible Z layer (above player)
    position.z = 25.0;

    // Create particle with color curves
    let start_color = config.color;
    // Create color curve: bright → slightly dimmer → fade to black
    let mid_color = {
        let linear = config.color.to_linear();
        Color::LinearRgba(LinearRgba::new(
            linear.red * 0.7,
            linear.green * 0.7,
            linear.blue * 0.7,
            linear.alpha,
        ))
    };
    let end_color = Color::srgba(
        config.color.to_linear().red * 0.3,
        config.color.to_linear().green * 0.3,
        config.color.to_linear().blue * 0.3,
        0.0,
    ); // Fading to black

    let particle = Particle::new(velocity, lifetime, scale, start_color)
        .with_angular_velocity(angular_velocity)
        .with_acceleration(config.acceleration)
        .with_color_curve(mid_color, end_color)
        .with_scale_curve(scale * 0.2); // Shrink to 20%

    // Create a mesh for the particle
    let size = 24.0 * scale;
    let mesh = meshes.add(Rectangle::new(size, size));
    let material = materials.add(ParticleMaterial::new(start_color));

    commands.spawn((
        particle,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(position),
    ));
}

fn apply_direction_variance(
    direction: Vec3,
    variance: f32,
    rng: &mut rand::rngs::ThreadRng,
) -> Vec3 {
    let angle = rng.gen_range(-variance..variance);
    rotate_vector_2d(direction, angle)
}

/// Rotate a 2D vector by an angle (in radians)
fn rotate_vector_2d(vec: Vec3, angle: f32) -> Vec3 {
    let cos = angle.cos();
    let sin = angle.sin();
    Vec3::new(vec.x * cos - vec.y * sin, vec.x * sin + vec.y * cos, vec.z)
}

pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(
        Entity,
        &mut Particle,
        &mut Transform,
        &MeshMaterial2d<ParticleMaterial>,
    )>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
) {
    for (entity, mut particle, mut transform, material_handle) in particles.iter_mut() {
        particle.lifetime -= time.delta_secs();

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Update position
        let acceleration = particle.acceleration;
        particle.velocity += acceleration * time.delta_secs();
        transform.translation += particle.velocity * time.delta_secs();

        // Update rotation
        transform.rotate_z(particle.angular_velocity * time.delta_secs());

        // Apply color curve interpolation
        let current_color = particle.current_color();

        // Apply scale curve interpolation
        let current_scale = particle.current_scale();
        transform.scale = Vec3::splat(current_scale);

        // Update material color
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.color = current_color.to_linear();
        }
    }
}

pub fn cleanup_finished_emitters(
    mut commands: Commands,
    emitters: Query<(Entity, &ParticleEmitter)>,
) {
    for (entity, emitter) in emitters.iter() {
        if emitter.one_shot && !emitter.active {
            commands.entity(entity).despawn();
        }
    }
}