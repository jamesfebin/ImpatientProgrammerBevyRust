use bevy::prelude::*;

use serde::{Serialize, Deserialize};

/// Tile types for collision detection.
/// Each type has different walkability and collision behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TileType {
    // Walkable terrain
    #[default]
    Empty,
    Dirt,
    Grass,
    YellowGrass,
    Shore,  // Water edges (walkable)
    // Non-walkable obstacles
    Water,
    Tree,
    Rock,
}

impl TileType {
    /// Check if this tile type allows movement through it.
    pub fn is_walkable(&self) -> bool {
        !matches!(self, TileType::Water | TileType::Tree | TileType::Rock)
    }

    /// Get the collision adjustment for this tile type.
    /// Positive = push player away, negative = allow corner cutting.
    pub fn collision_adjustment(&self) -> f32 {
        match self {
            TileType::Tree | TileType::Rock => -0.2,  // Allow cutting corners
            _ => 0.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct TileMarker {
    pub tile_type: TileType,
}

impl TileMarker {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type }
    }
}