use std::fs;

use super::data::*;

pub fn load_save_data(slot: usize) -> Result<SaveData, String> {
    let path = save_file_path(slot);
    let file_bytes = fs::read(&path).map_err(|e| format!("Read error: {}", e))?;

    let save_file: SaveFile =
        bincode::deserialize(&file_bytes).map_err(|e| format!("Deserialize error: {}", e))?;

    // Verify checksum
    let computed = compute_checksum(&save_file.data);
    if computed != save_file.checksum {
        return Err("Save file corrupted or tampered with".into());
    }

    let save_data: SaveData = bincode::deserialize(&save_file.data)
        .map_err(|e| format!("Data deserialize error: {}", e))?;

    if save_data.version != SAVE_VERSION {
        return Err(format!(
            "Incompatible save version: {} (expected {})",
            save_data.version, SAVE_VERSION
        ));
    }

    Ok(save_data)
}

pub fn load_slot_metadata(slot: usize) -> Option<SaveMetadata> {
    let path = meta_file_path(slot);
    let bytes = fs::read(&path).ok()?;
    bincode::deserialize(&bytes).ok()
}