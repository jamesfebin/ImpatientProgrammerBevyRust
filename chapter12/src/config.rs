pub mod player {
    /// Collision radius for the player's collider (in world units)
    pub const COLLIDER_RADIUS: f32 = 24.0; // Line update alert
    
    /// Z-position for player rendering (above terrain, below UI)
    pub const PLAYER_Z_POSITION: f32 = 20.0;
    
    /// Visual scale of the player sprite
    pub const PLAYER_SCALE: f32 = 1.2; // Line update alert (was 0.8)
}

pub mod pickup {
    /// Default radius for item pickup detection (in world units)
    pub const DEFAULT_RADIUS: f32 = 40.0;
}

pub mod enemy {
    /// Z-position for enemy rendering (same as player for consistent layering)
    pub const ENEMY_Z_POSITION: f32 = 20.0;

    /// Visual scale of enemy sprites (same as player for consistency)
    pub const ENEMY_SCALE: f32 = 1.2;
} 

/// Map/terrain configuration
pub mod map {
    /// Size of a single tile in world units (64px base * 1.0 scale = 64)
    /// NOTE: This must match TILE_SIZE in generate.rs!
    pub const TILE_SIZE: f32 = 64.0; // Line update alert (was 32.0)
    
    /// Grid dimensions
    pub const GRID_X: u32 = 25;
    pub const GRID_Y: u32 = 18;

    /// Number of chunks in each direction
    /// We're going big: 10x10 = 100 chunks!
    pub const CHUNKS_X: u32 = 10;
    pub const CHUNKS_Y: u32 = 10;

    /// Total unique grid dimensions (chunks overlap by 1 tile at borders)
    pub const TOTAL_GRID_X: u32 = CHUNKS_X * GRID_X - (CHUNKS_X - 1);
    pub const TOTAL_GRID_Y: u32 = CHUNKS_Y * GRID_Y - (CHUNKS_Y - 1);
    
    /// Z-height of each layer (used for Y-based depth sorting)
    pub const NODE_SIZE_Z: f32 = 1.0; // Add this line
}

pub mod camera {
    /// How fast the camera interpolates toward the player (higher = snappier)
    pub const CAMERA_LERP_SPEED: f32 = 6.0;
    
    /// Z position for the camera (must be high to see all layers)
    pub const CAMERA_Z: f32 = 1000.0;
}