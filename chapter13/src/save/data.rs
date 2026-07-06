use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::characters::facing::Facing;
use crate::collision::TileType;
use crate::combat::PowerType;
use crate::inventory::ItemKind;

pub const SAVE_VERSION: u32 = 1;
pub const MAX_SLOTS: usize = 5;

#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub checksum: u64,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub timestamp: String,
    pub slot_name: String,
    pub player: PlayerSave,
    pub enemies: Vec<EnemySave>,
    pub inventory: HashMap<ItemKind, u32>,
    pub tiles: Vec<TileSave>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSave {
    pub position: [f32; 3],
    pub health_current: f32,
    pub health_max: f32,
    pub power_type: PowerType,
    pub character_name: String,
    pub character_index: usize,
    pub facing: Facing,
}

#[derive(Serialize, Deserialize)]
pub struct EnemySave {
    pub position: [f32; 3],
    pub health_current: f32,
    pub health_max: f32,
    pub character_name: String,
    pub power_type: PowerType,
    pub facing: Facing,
}

#[derive(Serialize, Deserialize)]
pub struct TileSave {
    pub position: [f32; 3],
    pub rotation: [f32; 4],  // Quat (x, y, z, w) — WFC applies Z-axis rotation to tiles
    pub scale: [f32; 3],
    pub atlas_index: usize,
    pub tile_type: TileType,
    pub pickable: Option<ItemKind>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveMetadata {
    pub timestamp: String,
    pub character_name: String,
    pub player_health: f32,
    pub player_max_health: f32,
}

pub fn saves_directory() -> std::path::PathBuf {
    let mut path = std::env::current_exe()
        .unwrap_or_default()
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .to_path_buf();
    path.push("saves");
    path
}

pub fn save_file_path(slot: usize) -> std::path::PathBuf {
    saves_directory().join(format!("slot_{}.sav", slot))
}

pub fn meta_file_path(slot: usize) -> std::path::PathBuf {
    saves_directory().join(format!("slot_{}.meta", slot))
}

pub fn compute_checksum(data: &[u8]) -> u64 {
    // Simple FNV-1a hash
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}