// src/inventory/inventory.rs
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;


use crate::config::pickup::DEFAULT_RADIUS;

/// Types of items that can be collected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemKind {
    Plant1,
    Plant2,
    Plant3,
    Plant4,
}

impl ItemKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            ItemKind::Plant1 => "Herb",
            ItemKind::Plant2 => "Flower",
            ItemKind::Plant3 => "Mushroom",
            ItemKind::Plant4 => "Fern",
        }
    }
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[derive(Component, Debug)]
pub struct Pickable {
    pub kind: ItemKind,
    pub radius: f32,
}

impl Pickable {
    pub fn new(kind: ItemKind) -> Self {
        Self {
            kind,
            radius: DEFAULT_RADIUS,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct Inventory {
    items: HashMap<ItemKind, u32>,
}

impl Inventory {
    /// Add an item to the inventory, returns new count.
    pub fn add(&mut self, kind: ItemKind) -> u32 {
        let entry = self.items.entry(kind).or_insert(0);
        *entry += 1;
        *entry
    }

    /// Get a summary string of inventory contents.
    pub fn summary(&self) -> String {
        if self.items.is_empty() {
            return "empty".to_string();
        }

        let mut parts: Vec<String> = self
            .items
            .iter()
            .map(|(kind, count)| format!("{}: {}", kind, count))
            .collect();
        parts.sort();
        parts.join(", ")
    }

     /// Get a reference to the items map for serialization.
     pub fn items(&self) -> &HashMap<ItemKind, u32> {
        &self.items
    }

    /// Replace the entire items map (used when loading saves).
    pub fn set_items(&mut self, items: HashMap<ItemKind, u32>) {
        self.items = items;
    }
}
