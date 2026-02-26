use bevy::prelude::*;
use super::health::Health;

const HEALTHBAR_WIDTH: f32 = 50.0;
const HEALTHBAR_HEIGHT: f32 = 6.0;
const HEALTHBAR_Y_OFFSET: f32 = 43.0;
const HEALTHBAR_Z_OFFSET: f32 = 1.0;
/// Small z bump so the foreground always renders on top of the background.
const HEALTHBAR_FG_Z_BUMP: f32 = 0.01;

/// Marker: this entity is the colored fill of a healthbar.
#[derive(Component)]
pub struct HealthBarForeground;

/// Links a healthbar entity back to its owner character.
#[derive(Component)]
pub struct HealthBarOwner(pub Entity);

/// Spawns a background + foreground healthbar pair for each entity that gains Health.
pub fn spawn_healthbars(
    mut commands: Commands,
    new_health: Query<(Entity, &GlobalTransform, &Health), Added<Health>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (owner, transform, health) in &new_health {
        let pos = transform.translation();
        let bg_pos = Vec3::new(pos.x, pos.y + HEALTHBAR_Y_OFFSET, pos.z + HEALTHBAR_Z_OFFSET);
        let fg_pos = Vec3::new(pos.x, pos.y + HEALTHBAR_Y_OFFSET, pos.z + HEALTHBAR_Z_OFFSET + HEALTHBAR_FG_Z_BUMP);

        // Background: dark gray
        let bg_mesh = meshes.add(Rectangle::new(HEALTHBAR_WIDTH, HEALTHBAR_HEIGHT));
        let bg_mat = materials.add(ColorMaterial::from(Color::srgb(0.2, 0.2, 0.2)));
        commands.spawn((
            Mesh2d(bg_mesh),
            MeshMaterial2d(bg_mat),
            Transform::from_translation(bg_pos),
            HealthBarOwner(owner),
        ));

        // Foreground: color derived from actual health ratio
        let ratio = health.ratio();
        let fg_mesh = meshes.add(Rectangle::new(HEALTHBAR_WIDTH, HEALTHBAR_HEIGHT));
        let fg_mat = materials.add(ColorMaterial::from(health_color(ratio)));
        commands.spawn((
            Mesh2d(fg_mesh),
            MeshMaterial2d(fg_mat),
            Transform::from_translation(fg_pos).with_scale(Vec3::new(ratio.max(0.001), 1.0, 1.0)),
            HealthBarOwner(owner),
            HealthBarForeground,
        ));
    }
}


/// Despawns bars whose owner no longer exists.
pub fn update_healthbars(
    mut commands: Commands,
    mut bars: Query<(
        Entity,
        &HealthBarOwner,
        &mut Transform,
        Has<HealthBarForeground>,
        &MeshMaterial2d<ColorMaterial>,
    )>,
    owners: Query<(&GlobalTransform, &Health)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bar_entity, owner_ref, mut transform, is_foreground, mat_handle) in bars.iter_mut() {
        let Ok((owner_transform, health)) = owners.get(owner_ref.0) else {
            // If the owner is gone (despawned), clean up the health bar
            commands.entity(bar_entity).despawn();
            continue;
        };

        let owner_pos = owner_transform.translation();
        let ratio = health.ratio();

        if is_foreground {
            // Scale foreground width to match health ratio
            transform.scale.x = ratio.max(0.001);
            
            // Reposition foreground to stay left-aligned as it shrinks
            // (Scaling happens from center, so we need to offset position)
            transform.translation = Vec3::new(
                owner_pos.x - (HEALTHBAR_WIDTH * (1.0 - ratio) / 2.0),
                owner_pos.y + HEALTHBAR_Y_OFFSET,
                owner_pos.z + HEALTHBAR_Z_OFFSET + HEALTHBAR_FG_Z_BUMP,
            );
            
            // Update color (Green -> Yellow -> Red)
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.color = health_color(ratio);
            }
        } else {
            // Background simply stays centered above the owner
            transform.translation = Vec3::new(
                owner_pos.x,
                owner_pos.y + HEALTHBAR_Y_OFFSET,
                owner_pos.z + HEALTHBAR_Z_OFFSET,
            );
        }
    }
}

/// Green → Yellow → Red, continuous at ratio = 0.5.
fn health_color(ratio: f32) -> Color {
    if ratio >= 0.5 {
        let t = (ratio - 0.5) * 2.0; // 1.0 at full, 0.0 at half
        Color::srgb(1.0 - t * 0.8, 0.8, 0.2)
    } else {
        let t = ratio * 2.0; // 1.0 at half, 0.0 at empty
        Color::srgb(1.0, t * 0.8, 0.2)
    }
}