use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
#[derive(Resource)]
pub struct WorldSeed(pub u64);
pub fn init_single_player_seed(mut commands: Commands) {
    let seed = StdRng::from_entropy().next_u64();
    info!("Single-player world seed: {}", seed);
    commands.insert_resource(WorldSeed(seed));
}
