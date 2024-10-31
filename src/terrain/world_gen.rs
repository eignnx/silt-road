use bevy::prelude::*;
use libnoise::Generator;

use crate::constants::TILES_PER_CHUNK;

use super::chunk::ChunkCoord;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug, Clone, Copy)]
pub enum TerrainTile {
    Grass(u8),
    DryGrass(u8),
}

pub const WORLD_SEED: u64 = 314159;

pub(super) fn tile_type(chunk_coord: ChunkCoord, tile_coord: UVec2) -> TerrainTile {
    let perlin_source = libnoise::Source::improved_perlin(WORLD_SEED);
    let absolute_coord = chunk_coord.xy() * TILES_PER_CHUNK as i32 + tile_coord.as_ivec2();

    let soil_stat = perlin_source
        .scale([0.005f64, 0.005f64])
        .add(1.0)
        .mul(0.5)
        .sample(absolute_coord.as_dvec2().to_array());

    match soil_stat {
        _ if soil_stat < 0.1 => TerrainTile::Grass(1),
        _ if soil_stat < 0.2 => TerrainTile::Grass(2),
        _ if soil_stat < 0.3 => TerrainTile::Grass(3),
        _ if soil_stat < 0.4 => TerrainTile::DryGrass(1),
        _ if soil_stat < 0.5 => TerrainTile::DryGrass(2),
        _ if soil_stat < 0.6 => TerrainTile::DryGrass(3),
        _ => TerrainTile::DryGrass(3),
    }
}

impl TerrainTile {
    pub fn texture_index(&self) -> usize {
        match self {
            TerrainTile::Grass(n) => *n as usize,
            TerrainTile::DryGrass(n) => *n as usize + 3,
        }
    }
}
