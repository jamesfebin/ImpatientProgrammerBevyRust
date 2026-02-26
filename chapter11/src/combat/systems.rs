// src/combat/systems.rs
use super::player_combat::PlayerCombat;
use super::power_type::{PowerType, PowerVisuals};
use crate::characters::facing::Facing;
use crate::characters::input::Player;
use crate::particles::components::ParticleEmitter;
use bevy::prelude::*;
use crate::enemy::Enemy;

/// Marker for projectile effects
#[derive(Component)]
pub struct ProjectileEffect {
    pub power_type: PowerType,
}

/// Who fired a projectile determines which entities it can hit.
#[derive(Component, Clone, Copy)]
pub enum ProjectileOwner {
    Player,
    Enemy,
}

/// Invisible hitbox that travels and checks for collisions.
#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub power_type: PowerType,
    pub owner: ProjectileOwner,
    pub radius: f32,
}

pub fn handle_power_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&GlobalTransform, &Facing, &mut PlayerCombat), With<Player>>,
) {
    let Ok((global_transform, facing, mut combat)) = player_query.single_mut() else {
        return;
    };

    combat.cooldown.tick(time.delta());

    let ctrl_pressed =
        input.just_pressed(KeyCode::ControlLeft) || input.just_pressed(KeyCode::ControlRight);

    if !ctrl_pressed {
        return;
    }

    // Only fire if cooldown has elapsed
    if combat.cooldown.elapsed_secs() < combat.cooldown.duration().as_secs_f32() {
        return;
    }

    combat.cooldown.reset();

    let position: Vec3 = global_transform.translation();
    let direction = facing_to_vec3(facing);
    let spawn_position = position + direction * 5.0;

    // Get visuals from power type
    let visuals = combat.power_type.visuals(direction);

    spawn_projectile(&mut commands, spawn_position, combat.power_type, &visuals, ProjectileOwner::Player);

    info!("{:?} projectile fired!", combat.power_type);
}



pub fn spawn_projectile(
    commands: &mut Commands,
    position: Vec3,
    power_type: PowerType,
    visuals: &PowerVisuals,
    owner: ProjectileOwner, 
) {
    // Primary particles
    let primary_emitter =
        ParticleEmitter::new(0.016, visuals.particles_per_spawn, visuals.primary.clone())
            .one_shot();

    commands.spawn((
        primary_emitter,
        Transform::from_translation(position),
        GlobalTransform::from(Transform::from_translation(position)),
        ProjectileEffect { power_type },
    ));

    // Core particles (if the power has a core)
    if let Some(ref core_config) = visuals.core {
        let core_emitter =
            ParticleEmitter::new(0.016, visuals.core_particles_per_spawn, core_config.clone())
                .one_shot();

        commands.spawn((
            core_emitter,
            Transform::from_translation(position),
            GlobalTransform::from(Transform::from_translation(position)),
            ProjectileEffect { power_type },
        ));
    }

    // Hitbox: invisible entity that moves and checks for hits
    let direction = visuals.primary.direction.normalize_or_zero();
    let speed = visuals.primary.speed;
    commands.spawn((
        Projectile {
            velocity: direction * speed,
            lifetime: 2.0,
            power_type,
            owner,
            radius: power_type.hitbox_radius(),
        },
        Transform::from_translation(position),
    ));
}

fn facing_to_vec3(facing: &Facing) -> Vec3 {
    match facing {
        Facing::Right => Vec3::X,
        Facing::Left => Vec3::NEG_X,
        Facing::Up => Vec3::Y,
        Facing::Down => Vec3::NEG_Y,
    }
}

pub fn debug_switch_power(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut PlayerCombat, With<Player>>,
) {
    let Ok(mut combat) = player_query.single_mut() else {
        return;
    };

    let new_power = if input.just_pressed(KeyCode::Digit1) {
        Some(PowerType::Fire)
    } else if input.just_pressed(KeyCode::Digit2) {
        Some(PowerType::Arcane)
    } else if input.just_pressed(KeyCode::Digit3) {
        Some(PowerType::Shadow)
    } else if input.just_pressed(KeyCode::Digit4) {
        Some(PowerType::Poison)
    } else {
        None
    };

    if let Some(power) = new_power {
        combat.power_type = power;
        info!("Switched to {:?}", power);
    }
}

/// Moves projectile hitboxes forward and despawns them on timeout.
pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Projectile, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut proj, mut transform) in projectiles.iter_mut() {
        proj.lifetime -= dt;
        if proj.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation += proj.velocity * dt;
    }
}

/// Checks each projectile hitbox against its valid targets; triggers hit events on collision.
pub fn check_projectile_hits(
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    players: Query<(Entity, &GlobalTransform), With<Player>>,
    enemies: Query<(Entity, &GlobalTransform), With<Enemy>>,
) {
    for (proj_entity, proj, proj_transform) in &projectiles {
        let proj_pos = proj_transform.translation;

        let hit_target = match proj.owner {
            ProjectileOwner::Player => enemies
                .iter()
                .find(|(_, t)| proj_pos.distance(t.translation()) <= proj.radius)
                .map(|(e, _)| e),
            ProjectileOwner::Enemy => players
                .iter()
                .find(|(_, t)| proj_pos.distance(t.translation()) <= proj.radius)
                .map(|(e, _)| e),
        };

        if let Some(target) = hit_target {
            // Trigger hit event instead of directly applying damage
            commands.trigger(super::events::ProjectileHit {
                target,
                damage: proj.power_type.damage(),
                power_type: proj.power_type,
            });
            commands.entity(proj_entity).despawn();
        }
    }
}