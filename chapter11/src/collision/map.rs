use bevy::prelude::*;
use super::TileType;
use pathfinding::prelude::astar;

/// Collision map resource that stores walkability information.
/// Provides efficient spatial queries for movement validation.
#[derive(Resource)]
pub struct CollisionMap {
    /// Flat array of tile types (row-major order)
    tiles: Vec<TileType>,
    /// Grid dimensions
    width: i32,
    height: i32,
    /// Size of each tile in world units
    tile_size: f32,
    /// World position of grid origin (bottom-left corner)
    origin_x: f32,
    origin_y: f32,
}

impl CollisionMap {
    /// Create a new collision map with specified dimensions and origin.
    pub fn new(width: i32, height: i32, tile_size: f32, origin_x: f32, origin_y: f32) -> Self {
        let size = (width * height) as usize;
        Self {
            tiles: vec![TileType::Empty; size],
            width,
            height,
            tile_size,
            origin_x,
            origin_y,
        }
    }

    /// Convert 2D grid coordinates to 1D array index.
    #[inline]
    fn xy_to_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// Check if grid coordinates are within bounds.
    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn world_to_grid(&self, world_pos: Vec2) -> IVec2 {
        let grid_x = ((world_pos.x - self.origin_x) / self.tile_size).floor() as i32;
        let grid_y = ((world_pos.y - self.origin_y) / self.tile_size).floor() as i32;
        IVec2::new(grid_x, grid_y)
    }

    /// Convert grid coordinates to world position (tile center).
    pub fn grid_to_world(&self, grid_x: i32, grid_y: i32) -> Vec2 {
        Vec2::new(
            self.origin_x + (grid_x as f32 + 0.5) * self.tile_size,
            self.origin_y + (grid_y as f32 + 0.5) * self.tile_size,
        )
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<TileType> {
        if self.in_bounds(x, y) {
            Some(self.tiles[self.xy_to_idx(x, y)])
        } else {
            None
        }
    }

    /// Set a tile at grid coordinates.
    pub fn set_tile(&mut self, x: i32, y: i32, tile_type: TileType) {
        if self.in_bounds(x, y) {
            let idx = self.xy_to_idx(x, y);
            self.tiles[idx] = tile_type;
        }
    }

    /// Check if a grid position is walkable.
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y).map_or(false, |t| t.is_walkable())
    }

    /// Check if a world position is walkable.
    pub fn is_world_pos_walkable(&self, world_pos: Vec2) -> bool {
        let grid_pos = self.world_to_grid(world_pos);
        self.is_walkable(grid_pos.x, grid_pos.y)
    }

    fn circle_intersects_tile(&self, center: Vec2, radius: f32, gx: i32, gy: i32) -> bool {
        // Tile bounding box
        let tile_min = Vec2::new(
            self.origin_x + gx as f32 * self.tile_size,
            self.origin_y + gy as f32 * self.tile_size,
        );
        let tile_max = tile_min + Vec2::splat(self.tile_size);

        // Find closest point on tile to circle center
        let closest = Vec2::new(
            center.x.clamp(tile_min.x, tile_max.x),
            center.y.clamp(tile_min.y, tile_max.y),
        );

        // Check if closest point is within radius
        center.distance_squared(closest) <= radius * radius
    }

    fn is_within_bounds(&self, center: Vec2, radius: f32) -> bool {
        let left = self.origin_x;
        let right = self.origin_x + self.width as f32 * self.tile_size;
        let bottom = self.origin_y;
        let top = self.origin_y + self.height as f32 * self.tile_size;

        center.x - radius >= left
            && center.x + radius <= right
            && center.y - radius >= bottom
            && center.y + radius <= top
    }

    pub fn is_circle_clear(&self, center: Vec2, radius: f32) -> bool {
        // Early bounds check
        if !self.is_within_bounds(center, radius) {
            return false;
        }

        // Point collision if no radius
        if radius <= 0.0 {
            return self.is_world_pos_walkable(center);
        }

        // Find grid cells that could overlap the circle
        let min_gx = ((center.x - radius - self.origin_x) / self.tile_size).floor() as i32;
        let max_gx = ((center.x + radius - self.origin_x) / self.tile_size).floor() as i32;
        let min_gy = ((center.y - radius - self.origin_y) / self.tile_size).floor() as i32;
        let max_gy = ((center.y + radius - self.origin_y) / self.tile_size).floor() as i32;

        for gy in min_gy..=max_gy {
            for gx in min_gx..=max_gx {
                if !self.in_bounds(gx, gy) {
                    return false;  // Out of bounds = blocked
                }

                if let Some(tile) = self.get_tile(gx, gy) {
                    if !tile.is_walkable() {
                        // Apply tile-specific collision adjustment
                        let effective_radius = radius + tile.collision_adjustment() * self.tile_size;
                        
                        if self.circle_intersects_tile(center, effective_radius, gx, gy) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn sweep_circle(&self, start: Vec2, end: Vec2, radius: f32) -> Vec2 {
        let delta = end - start;
        
        // No movement needed
        if delta.length() < 0.001 {
            return start;
        }

        // Step size (quarter tile for smooth collision)
        let max_step = self.tile_size * 0.25;
        let steps = (delta.length() / max_step).ceil().max(1.0) as i32;
        let step_vec = delta / steps as f32;

        let mut pos = start;
        for _ in 0..steps {
            let candidate = pos + step_vec;

            if self.is_circle_clear(candidate, radius) {
                pos = candidate;
            } else {
                // Try sliding along X axis only
                let try_x = Vec2::new(candidate.x, pos.y);
                if self.is_circle_clear(try_x, radius) {
                    pos = try_x;
                    continue;
                }

                // Try sliding along Y axis only
                let try_y = Vec2::new(pos.x, candidate.y);
                if self.is_circle_clear(try_y, radius) {
                    pos = try_y;
                    continue;
                }

                // Completely blocked
                break;
            }
        }
        pos
    }

    #[cfg(debug_assertions)]
    pub fn width(&self) -> i32 { self.width }
    
    #[cfg(debug_assertions)]
    pub fn height(&self) -> i32 { self.height }
    
    #[cfg(debug_assertions)]
    pub fn tile_size(&self) -> f32 { self.tile_size }
    
    #[cfg(debug_assertions)]
    pub fn origin(&self) -> Vec2 { Vec2::new(self.origin_x, self.origin_y) }

    pub fn get_neighbors(&self, pos: IVec2) -> Vec<IVec2> {
        let mut neighbors = Vec::new();
        
        // Cardinal directions (always allowed if walkable)
        let cardinals = [
            IVec2::new(0, 1), IVec2::new(0, -1), IVec2::new(-1, 0), IVec2::new(1, 0),
        ];
        
        for dir in cardinals {
            let neighbor = pos + dir;
            if self.is_walkable(neighbor.x, neighbor.y) {
                neighbors.push(neighbor);
            }
        }
        
        // Diagonal directions - only if both adjacent cardinals are clear
        // This prevents corner-cutting through diagonal walls
        let diagonals = [
            (IVec2::new(-1, 1), IVec2::new(-1, 0), IVec2::new(0, 1)),   // Up-Left
            (IVec2::new(1, 1), IVec2::new(1, 0), IVec2::new(0, 1)),     // Up-Right
            (IVec2::new(-1, -1), IVec2::new(-1, 0), IVec2::new(0, -1)), // Down-Left
            (IVec2::new(1, -1), IVec2::new(1, 0), IVec2::new(0, -1)),   // Down-Right
        ];
        
        for (diagonal, adj1, adj2) in diagonals {
            let diag_pos = pos + diagonal;
            let adj1_pos = pos + adj1;
            let adj2_pos = pos + adj2;
            
            // Only allow diagonal if destination AND both adjacent cells are walkable
            if self.is_walkable(diag_pos.x, diag_pos.y)
                && self.is_walkable(adj1_pos.x, adj1_pos.y)
                && self.is_walkable(adj2_pos.x, adj2_pos.y)
            {
                neighbors.push(diag_pos);
            }
        }
        
        neighbors
    }
    
    /// Find path using A* algorithm
    pub fn find_path(&self, start: Vec2, goal: Vec2) -> Option<Vec<Vec2>> {
        use pathfinding::prelude::astar;
        
        let start_grid = self.world_to_grid(start);
        let goal_grid = self.world_to_grid(goal);
        
        if !self.is_walkable(start_grid.x, start_grid.y) {
            return None;
        }
        
        let actual_goal = if self.is_walkable(goal_grid.x, goal_grid.y) {
            goal_grid
        } else {
            self.find_nearest_walkable(goal_grid)?
        };
        
        let result = astar(
            &start_grid,
            |pos| {
                let pos = *pos;
                self.get_neighbors(pos).into_iter().map(move |n| {
                    let cost = if (n.x - pos.x).abs() + (n.y - pos.y).abs() == 2 {
                        14u32 // Diagonal
                    } else {
                        10u32 // Cardinal
                    };
                    (n, cost)
                })
            },
            |pos| {
                let dx = (pos.x - actual_goal.x).abs();
                let dy = (pos.y - actual_goal.y).abs();
                ((dx + dy) * 10) as u32
            },
            |pos| *pos == actual_goal,
        );
        
        result.map(|(path, _cost)| {
            path.into_iter().map(|p| self.grid_to_world(p.x, p.y)).collect()
        })
    }
    
    /// Find nearest walkable cell
    pub fn find_nearest_walkable(&self, pos: IVec2) -> Option<IVec2> {
        for radius in 1i32..10 {
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    if dx.abs() == radius || dy.abs() == radius {
                        let check = IVec2::new(pos.x + dx, pos.y + dy);
                        if self.is_walkable(check.x, check.y) {
                            return Some(check);
                        }
                    }
                }
            }
        }
        None
    }


}