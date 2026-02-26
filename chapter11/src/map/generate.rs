// src/map/generate.rs
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use bevy::prelude::*;
use bevy::tasks::{block_on, futures_lite::future::poll_once, AsyncComputeTaskPool, Task};
use bevy_procedural_tilemaps::prelude::*;
use bevy_procedural_tilemaps::proc_gen::generator::model::ModelInstance;
use bevy_procedural_tilemaps::proc_gen::generator::rules::Rules;
use bevy_procedural_tilemaps::proc_gen::grid::GridData;

use crate::config::map::{
    CHUNKS_X, CHUNKS_Y, GRID_X, GRID_Y, NODE_SIZE_Z, TILE_SIZE, TOTAL_GRID_X, TOTAL_GRID_Y,
};
use crate::map::{
    assets::{load_assets, prepare_tilemap_handles, TilemapHandles},
    rules::build_world,
};

const ASSETS_PATH: &str = "tile_layers";
const TILEMAP_FILE: &str = "tilemap.png";
const NODE_SIZE: Vec3 = Vec3::new(TILE_SIZE, TILE_SIZE, NODE_SIZE_Z);
const ASSETS_SCALE: Vec3 = Vec3::new(2.0, 2.0, 1.0);
const GRID_Z: u32 = 5;

/// Maximum unpin radius for progressive corner unpinning fallback.
const MAX_UNPIN_RADIUS: u32 = 5;

/// Shared progress counter for the loading screen.
#[derive(Resource)]
pub struct MapGenProgress {
    pub current: Arc<AtomicU32>,
    pub total: u32,
}

/// Marker resource: inserted when the map is fully spawned.
#[derive(Resource)]
pub struct MapReady;

/// Stores the spawner and grid template needed after background generation completes.
#[derive(Resource)]
pub struct MapSpawnResources {
    spawner: NodesSpawner<Sprite>,
    grid_template: CartesianGrid<Cartesian3D>,
}

/// Background task producing generated chunk data.
#[derive(Resource)]
pub struct MapGenTask(Task<Vec<ChunkResult>>);

struct ChunkResult {
    grid_data: GridData<Cartesian3D, ModelInstance, CartesianGrid<Cartesian3D>>,
    chunk_offset: Vec3,
    chunk_x: u32,
    chunk_y: u32,
}

pub fn prepare_tilemap_handles_resource(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tilemap_handles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, ASSETS_PATH, TILEMAP_FILE);
    commands.insert_resource(tilemap_handles);
}

pub fn setup_generator(
    mut commands: Commands,
    tilemap_handles: Res<TilemapHandles>,
) {
    // 1. Build rules, models, and assets (shared across all chunks)
    let (assets_definitions, models, socket_collection) = build_world();

    let rules = RulesBuilder::new_cartesian_3d(models, socket_collection)
        .with_rotation_axis(Direction::ZForward)
        .build()
        .unwrap();
    let rules_arc = Arc::new(rules);

    let grid_template =
        CartesianGrid::new_cartesian_3d(GRID_X, GRID_Y, GRID_Z, false, false, false);

    let models_assets = load_assets(&tilemap_handles, assets_definitions);
    let spawner = NodesSpawner::new(models_assets, NODE_SIZE, ASSETS_SCALE);

    // Store resources needed for spawning later
    commands.insert_resource(MapSpawnResources {
        spawner,
        grid_template: grid_template.clone(),
    });

    // Initialize progress tracking
    let progress = Arc::new(AtomicU32::new(0));
    commands.insert_resource(MapGenProgress {
        current: progress.clone(),
        total: CHUNKS_X * CHUNKS_Y,
    });

    // Spawn the background task
    let pool = AsyncComputeTaskPool::get();
    let task = pool.spawn(async move {
        generate_all_chunks(rules_arc, grid_template, progress)
    });
    commands.insert_resource(MapGenTask(task));
}

pub fn poll_map_generation(
    mut commands: Commands,
    task: Option<ResMut<MapGenTask>>,
    resources: Option<Res<MapSpawnResources>>,
) {
    let (Some(mut task), Some(resources)) = (task, resources) else {
        return;
    };

    // Check if the task is done
    let Some(chunks) = block_on(poll_once(&mut task.0)) else {
        return; // Still running...
    };

    // Task finished! Spawn everything.
    for chunk in &chunks {
        spawn_chunk_tiles(
            &mut commands,
            &resources.grid_template,
            &resources.spawner,
            &chunk.grid_data,
            chunk.chunk_offset,
            chunk.chunk_x,
            chunk.chunk_y,
        );
    }

    // Cleanup and mark as ready
    commands.remove_resource::<MapGenTask>();
    commands.remove_resource::<MapSpawnResources>();
    commands.remove_resource::<MapGenProgress>();
    commands.insert_resource(MapReady);

    info!(
        "Map generation complete: {}x{} chunks, {}x{} total tiles",
        CHUNKS_X, CHUNKS_Y, TOTAL_GRID_X, TOTAL_GRID_Y
    );
}

fn generate_all_chunks(
    rules_arc: Arc<Rules<Cartesian3D>>,
    grid_template: CartesianGrid<Cartesian3D>,
    progress: Arc<AtomicU32>,
) -> Vec<ChunkResult> {
    let mut generated_chunks: HashMap<
        (u32, u32),
        GridData<Cartesian3D, ModelInstance, CartesianGrid<Cartesian3D>>,
    > = HashMap::new();

    for cy in 0..CHUNKS_Y {
        for cx in 0..CHUNKS_X {
            // Seed borders from neighbors
            let initial_nodes = build_initial_nodes(cx, cy, &generated_chunks, &grid_template);
            let is_corner = cx > 0 && cy > 0;

            // Generate with fallback for robustness
            let grid_data = generate_chunk_with_fallback(
                &rules_arc,
                &grid_template,
                &initial_nodes,
                is_corner,
                cx,
                cy,
            );

            generated_chunks.insert((cx, cy), grid_data);
            progress.fetch_add(1, Ordering::Relaxed);
            info!("Generated chunk ({}, {})", cx, cy);
        }
    }

    // Convert HashMap into results for spawning
    let mut results = Vec::with_capacity((CHUNKS_X * CHUNKS_Y) as usize);
    for cy in 0..CHUNKS_Y {
        for cx in 0..CHUNKS_X {
            let grid_data = generated_chunks.remove(&(cx, cy)).unwrap();
            let chunk_offset = Vec3::new(
                (cx as f32 * (GRID_X - 1) as f32 - TOTAL_GRID_X as f32 / 2.0) * TILE_SIZE,
                (cy as f32 * (GRID_Y - 1) as f32 - TOTAL_GRID_Y as f32 / 2.0) * TILE_SIZE,
                0.0,
            );
            results.push(ChunkResult {
                grid_data,
                chunk_offset,
                chunk_x: cx,
                chunk_y: cy,
            });
        }
    }
    results
}


fn build_initial_nodes(
    cx: u32,
    cy: u32,
    generated_chunks: &HashMap<
        (u32, u32),
        GridData<Cartesian3D, ModelInstance, CartesianGrid<Cartesian3D>>,
    >,
    grid_template: &CartesianGrid<Cartesian3D>,
) -> Vec<((u32, u32, u32), ModelInstance)> {
    let mut initial_nodes = Vec::new();

    // Seed left column (x=0) from left neighbor's right column (x=GRID_X-1)
    if cx > 0 {
        let left_data = &generated_chunks[&(cx - 1, cy)];
        for y in 0..GRID_Y {
            for z in 0..GRID_Z {
                let src_index = grid_template.index_from_coords(GRID_X - 1, y, z);
                let model = *left_data.get(src_index);
                initial_nodes.push(((0, y, z), model));
            }
        }
    }

    // Seed bottom row (y=0) from bottom neighbor's top row (y=GRID_Y-1)
    if cy > 0 {
        let bottom_data = &generated_chunks[&(cx, cy - 1)];
        let start_x = if cx > 0 { 1 } else { 0 };
        for x in start_x..GRID_X {
            for z in 0..GRID_Z {
                let src_index = grid_template.index_from_coords(x, GRID_Y - 1, z);
                let model = *bottom_data.get(src_index);
                initial_nodes.push(((x, 0, z), model));
            }
        }
    }

    initial_nodes
}

fn generate_chunk_with_fallback(
    rules: &Arc<Rules<Cartesian3D>>,
    grid: &CartesianGrid<Cartesian3D>,
    initial_nodes: &[((u32, u32, u32), ModelInstance)],
    is_corner: bool,
    cx: u32,
    cy: u32,
) -> GridData<Cartesian3D, ModelInstance, CartesianGrid<Cartesian3D>> {
    // Try with full initial nodes first
    if let Some(data) = try_generate_chunk(rules, grid, initial_nodes) {
        return data;
    }

    if !is_corner {
        panic!("Non-corner chunk ({}, {}) failed -- check your rules!", cx, cy);
    }

    // Progressive unpinning: remove an L-shaped region at the corner
    for radius in 1..=MAX_UNPIN_RADIUS {
        let reduced: Vec<_> = initial_nodes
            .iter()
            .filter(|&&((x, y, _z), _)| {
                // Keep nodes NOT in the L-shaped corner region
                !((x == 0 && y < radius) || (y == 0 && x > 0 && x <= radius))
            })
            .copied()
            .collect();

        if let Some(data) = try_generate_chunk(rules, grid, &reduced) {
            warn!("Corner chunk ({}, {}) needed unpin radius {}", cx, cy, radius);
            return data;
        }
    }

    panic!("Corner chunk ({}, {}) failed to generate.", cx, cy);
}

fn try_generate_chunk(
    rules: &Arc<Rules<Cartesian3D>>,
    grid: &CartesianGrid<Cartesian3D>,
    initial_nodes: &[((u32, u32, u32), ModelInstance)],
) -> Option<GridData<Cartesian3D, ModelInstance, CartesianGrid<Cartesian3D>>> {
    // In v0.3 we explicitly set border zones
    let num_directions = 6;
    let mut border_zones = Vec::with_capacity(initial_nodes.len() * num_directions);
    for &((x, y, z), _) in initial_nodes {
        let idx = grid.index_from_coords(x, y, z);
        for dir in 0..num_directions {
            border_zones.push((idx, dir));
        }
    }

    let gen_builder = GeneratorBuilder::new()
        .with_shared_rules(rules.clone())
        .with_grid(grid.clone())
        .with_rng(RngMode::RandomSeed)
        .with_node_heuristic(NodeSelectionHeuristic::MinimumRemainingValue)
        .with_model_heuristic(ModelSelectionHeuristic::WeightedProbability)
        .with_border_zones(border_zones);

    let gen_builder = if !initial_nodes.is_empty() {
        match gen_builder.with_initial_nodes(initial_nodes.to_vec()) {
            Ok(b) => b,
            Err(_) => return None,
        }
    } else {
        gen_builder
    };

    let mut generator = match gen_builder.build() {
        Ok(g) => g,
        Err(_) => return None,
    };

    match generator.generate_grid() {
        Ok((_, data)) => Some(data),
        Err(_) => None,
    }
}

fn spawn_chunk_tiles(
    commands: &mut Commands,
    grid: &CartesianGrid<Cartesian3D>,
    spawner: &NodesSpawner<Sprite>,
    grid_data: &GridData<Cartesian3D, ModelInstance, CartesianGrid<Cartesian3D>>,
    chunk_offset: Vec3,
    chunk_x: u32,
    chunk_y: u32,
) {
    for (node_index, instance) in grid_data.iter().enumerate() {
        let Some(node_assets) = spawner.assets.get(&instance.model_index) else {
            continue;
        };
        let position = grid.pos_from_index(node_index);

        // Optimization: Skip overlap tiles
        // The right column and top row are spawned by the next chunk.
        if position.x == GRID_X - 1 && chunk_x < CHUNKS_X - 1 {
            continue;
        }
        if position.y == GRID_Y - 1 && chunk_y < CHUNKS_Y - 1 {
            continue;
        }

        for asset in node_assets.iter() {
            let mut local_pos = Vec3::new(
                asset.world_offset.x
                    + NODE_SIZE.x
                        * (position.x as f32 + asset.grid_offset.dx as f32 + 0.5),
                asset.world_offset.y
                    + NODE_SIZE.y
                        * (position.y as f32 + asset.grid_offset.dy as f32 + 0.5),
                asset.world_offset.z
                    + NODE_SIZE.z
                        * (position.z as f32 + asset.grid_offset.dz as f32 + 0.5),
            );

            // Global z_offset for correct depth sorting across all chunks
            let global_y = chunk_y * (GRID_Y - 1) + position.y;
            local_pos.z += NODE_SIZE_Z * (1.0 - global_y as f32 / TOTAL_GRID_Y as f32);

            let world_pos = Vec3::new(
                chunk_offset.x + local_pos.x,
                chunk_offset.y + local_pos.y,
                local_pos.z,
            );

            let entity = commands.spawn_empty().id();
            let entity_commands = &mut commands.entity(entity);
            asset.assets_bundle.insert_bundle(
                entity_commands,
                world_pos,
                ASSETS_SCALE,
                instance.rotation,
            );
            (asset.spawn_commands)(entity_commands);
        }
    }
}