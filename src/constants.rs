use bevy::math::UVec2;
use bevy_ecs_tilemap::map::TilemapTileSize;

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };

pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 4,
    y: CHUNK_SIZE.y * 4,
};
