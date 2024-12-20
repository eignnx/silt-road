use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};

use crate::constants::PIXELS_PER_TILE;

use super::{chunk::ChunkCoord, world_gen::tile_type};

pub(super) struct SpawnTile {
    pub(super) tile_pos: UVec2,
    pub(super) chunk_coord: ChunkCoord,
    pub(super) chunk_entity: Entity,
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

    let screen_pos = IVec2::ZERO + spawn_cmd.tile_pos.as_ivec2() * PIXELS_PER_TILE as i32;
    let transform = Transform::from_translation(screen_pos.as_vec2().extend(0.0));

    let texture_index = tile_type(spawn_cmd.chunk_coord, spawn_cmd.tile_pos).texture_index();

    let tile_entity = commands
        .spawn((
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
                index: texture_index,
            },
        ))
        .id();

    commands
        .entity(spawn_cmd.chunk_entity)
        .add_child(tile_entity);
}
