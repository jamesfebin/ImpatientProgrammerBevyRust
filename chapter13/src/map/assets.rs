use bevy::{prelude::*};
use bevy_procedural_tilemaps::prelude::*;
use crate::collision::{TileMarker, TileType};
use crate::map::tilemap::TILEMAP; 
use crate::inventory::{ItemKind, Pickable};



#[derive(Clone)]
pub struct SpawnableAsset {
    /// Name of the sprite inside our tilemap atlas
    sprite_name: &'static str,
    /// Offset in grid coordinates (for multi-tile objects)
    grid_offset: GridDelta,
    /// Offset in world coordinates (fine positioning)
    offset: Vec3,
    /// Function to add custom components (like collision, physics, etc.)
    tile_type: Option<TileType>,

    pickable: Option<ItemKind>, 
}

impl SpawnableAsset {
    pub fn new(sprite_name: &'static str) -> Self {
        Self {
            sprite_name,
            grid_offset: GridDelta::new(0, 0, 0),
            offset: Vec3::ZERO,
            tile_type: None, // Default: no extra components
            pickable: None, 
        }
    }

    pub fn with_grid_offset(mut self, offset: GridDelta) -> Self {
        self.grid_offset = offset;
        self
    }

    pub fn with_tile_type(mut self, tile_type: TileType) -> Self { 
        self.tile_type = Some(tile_type);
        self
    }

    pub fn with_pickable(mut self, kind: ItemKind) -> Self {
        self.pickable = Some(kind);
        self
    }
}

#[derive(Clone, Resource)]
pub struct TilemapHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl TilemapHandles {
    pub fn sprite(&self, atlas_index: usize) -> Sprite {
        Sprite::from_atlas_image(
            self.image.clone(),
            TextureAtlas::from(self.layout.clone()).with_index(atlas_index),
        )
    }
}

pub fn prepare_tilemap_handles(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    assets_directory: &str,
    tilemap_file: &str,
) -> TilemapHandles {
    let image = asset_server.load::<Image>(format!("{assets_directory}/{tilemap_file}"));
    let mut layout = TextureAtlasLayout::new_empty(TILEMAP.atlas_size());
    for index in 0..TILEMAP.sprites.len() {
        layout.add_texture(TILEMAP.sprite_rect(index));
    }
    let layout = atlas_layouts.add(layout);

    TilemapHandles { image, layout }
}

pub fn load_assets(
    tilemap_handles: &TilemapHandles,
    assets_definitions: Vec<Vec<SpawnableAsset>>,
) -> ModelsAssets<Sprite> {
    let mut models_assets = ModelsAssets::<Sprite>::new();
    
    for (model_index, assets) in assets_definitions.into_iter().enumerate() {
        for asset_def in assets {
            let SpawnableAsset {
                sprite_name,
                grid_offset,
                offset,
                tile_type,
                pickable, // Add this line
            } = asset_def;

            let Some(atlas_index) = TILEMAP.sprite_index(sprite_name) else {
                panic!("Unknown atlas sprite '{}'", sprite_name);
            };

            // Create the spawner function that adds components
            let spawner = create_spawner(tile_type, pickable); // Line update alert

            models_assets.add(
                model_index,
                ModelAsset {
                    assets_bundle: tilemap_handles.sprite(atlas_index),
                    grid_offset,
                    world_offset: offset,
                    spawn_commands: spawner,
                },
            );
        }
    }
    models_assets
}

fn create_spawner(
    tile_type: Option<TileType>,
    pickable: Option<ItemKind>,
) -> fn(&mut EntityCommands) {
    match (tile_type, pickable) {
        // Tile types without pickable
        (Some(TileType::Dirt), None) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Dirt));
        },
        (Some(TileType::Grass), None) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Grass));
        },
        (Some(TileType::YellowGrass), None) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::YellowGrass));
        },
        (Some(TileType::Water), None) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Water));
        },
        (Some(TileType::Shore), None) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Shore));
        },
        (Some(TileType::Tree), None) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Tree));
        },
        (Some(TileType::Rock), None) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Rock));
        },
        (Some(TileType::Empty), None) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Empty));
        },

        // Pickable plants (with grass tile type)
        (Some(TileType::Grass), Some(ItemKind::Plant1)) => |e: &mut EntityCommands| {
            e.insert((TileMarker::new(TileType::Grass), Pickable::new(ItemKind::Plant1)));
        },
        (Some(TileType::Grass), Some(ItemKind::Plant2)) => |e: &mut EntityCommands| {
            e.insert((TileMarker::new(TileType::Grass), Pickable::new(ItemKind::Plant2)));
        },
        (Some(TileType::Grass), Some(ItemKind::Plant3)) => |e: &mut EntityCommands| {
            e.insert((TileMarker::new(TileType::Grass), Pickable::new(ItemKind::Plant3)));
        },
        (Some(TileType::Grass), Some(ItemKind::Plant4)) => |e: &mut EntityCommands| {
            e.insert((TileMarker::new(TileType::Grass), Pickable::new(ItemKind::Plant4)));
        },

        // Default: no components
        _ => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Empty)); 
        },
    }
}