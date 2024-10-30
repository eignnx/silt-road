use bevy::math::UVec2;
use bevy_ecs_tilemap::map::TilemapTileSize;

pub const PIXELS_PER_TILE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };

pub const TILES_PER_CHUNK: UVec2 = UVec2 { x: 16, y: 16 };

pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: TILES_PER_CHUNK.x * 4,
    y: TILES_PER_CHUNK.y * 4,
};
