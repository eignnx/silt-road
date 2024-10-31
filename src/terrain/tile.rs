use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};

use crate::constants::{PIXELS_PER_TILE, TILES_PER_CHUNK};

use super::chunk::ChunkCoord;

pub(super) struct SpawnTile {
    pub(super) tile_pos: UVec2,
    pub(super) chunk_coord: ChunkCoord,
}

impl Command for SpawnTile {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_tile)
    }
}

pub(super) fn spawn_tile(
    In(spawn_cmd): In<SpawnTile>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("TexturedGrass.png");
    let texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16), // tile_size
        3,                // columns
        2,                // rows
        None,             // padding
        None,             // offset
    ));

    let screen_pos = (spawn_cmd.chunk_coord.xy() * TILES_PER_CHUNK as i32
        + spawn_cmd.tile_pos.as_ivec2())
        * PIXELS_PER_TILE as i32;

    let transform = Transform::from_translation(screen_pos.as_vec2().extend(0.0));

    commands.spawn((
        Name::new("Tile"),
        spawn_cmd.chunk_coord,
        SpriteBundle {
            transform,
            texture,
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 2,
        },
    ));
}
