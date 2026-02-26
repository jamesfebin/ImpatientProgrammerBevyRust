use bevy::prelude::*;
use std::collections::{HashMap, hash_map::Entry};

use super::{CollisionMap, TileMarker, TileType};
use crate::config::map::{TILE_SIZE, TOTAL_GRID_X, TOTAL_GRID_Y};

/// Resource to track if collision map has been built.
#[derive(Resource, Default, PartialEq, Eq)]
pub struct CollisionMapBuilt(pub bool);

pub fn build_collision_map(
    mut commands: Commands,
    mut built: ResMut<CollisionMapBuilt>,
    tile_query: Query<(&TileMarker, &Transform)>,
) {
    // Need at least one tile to proceed
    let mut tile_iter = tile_query.iter();
    let Some((first_marker, first_transform)) = tile_iter.next() else {
        return; // WFC hasn't generated tiles yet
    };

    // Calculate grid origin (centered map)
    let grid_origin_x = -TILE_SIZE * TOTAL_GRID_X as f32 / 2.0;
    let grid_origin_y = -TILE_SIZE * TOTAL_GRID_Y as f32 / 2.0;

    // Track bounds and layer info
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
    let mut layer_tracker: HashMap<(i32, i32), (TileType, f32)> = HashMap::new();
    let mut tile_count: usize = 0;

    // Process all tiles, keeping only the topmost at each position
    let mut process_tile = |marker: &TileMarker, transform: &Transform| {
        tile_count += 1;

        let world_x = transform.translation.x;
        let world_y = transform.translation.y;
        let world_z = transform.translation.z;
        
        let grid_x = ((world_x - grid_origin_x) / TILE_SIZE).floor() as i32;
        let grid_y = ((world_y - grid_origin_y) / TILE_SIZE).floor() as i32;

        min_x = min_x.min(grid_x);
        max_x = max_x.max(grid_x);
        min_y = min_y.min(grid_y);
        max_y = max_y.max(grid_y);

        // Keep only the topmost layer (highest Z)
        match layer_tracker.entry((grid_x, grid_y)) {
            Entry::Occupied(mut entry) => {
                if world_z > entry.get().1 {
                    *entry.get_mut() = (marker.tile_type, world_z);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert((marker.tile_type, world_z));
            }
        }
    };

    // Process first tile and remaining
    process_tile(first_marker, first_transform);
    for (marker, transform) in tile_iter {
        process_tile(marker, transform);
    }

    // Calculate actual dimensions
    let actual_width = (max_x - min_x + 1) as i32;
    let actual_height = (max_y - min_y + 1) as i32;

    // Create the collision map
    let mut map = CollisionMap::new(
        actual_width,
        actual_height,
        TILE_SIZE,
        grid_origin_x,
        grid_origin_y,
    );

    // Populate the map from layer tracker
    for ((grid_x, grid_y), (tile_type, _z)) in layer_tracker.iter() {
        // Convert world grid to local array coordinates
        let local_x = grid_x - min_x;
        let local_y = grid_y - min_y;
        map.set_tile(local_x, local_y, *tile_type);
    }

    // Post-processing: Convert water edges to shore
    convert_water_edges_to_shore(&mut map);
    // Insert as resource and mark built
    commands.insert_resource(map);
    built.0 = true;
}

fn convert_water_edges_to_shore(map: &mut CollisionMap) {
    let mut shores = Vec::new();

    // Find water tiles that touch walkable tiles
    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get_tile(x, y) != Some(TileType::Water) {
                continue;
            }

            // Check 8 neighbors
            let neighbors = [
                (x - 1, y),     (x + 1, y),     // left, right
                (x, y - 1),     (x, y + 1),     // down, up
                (x - 1, y - 1), (x + 1, y - 1), // bottom corners
                (x - 1, y + 1), (x + 1, y + 1), // top corners
            ];

            for (nx, ny) in neighbors {
                if map.is_walkable(nx, ny) {
                    shores.push((x, y));
                    break;
                }
            }
        }
    }

    for (x, y) in shores {
        map.set_tile(x, y, TileType::Shore);
    }


}