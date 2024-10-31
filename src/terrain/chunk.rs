use std::ops::Deref;

use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};

use crate::constants::{PIXELS_PER_CHUNK, PIXELS_PER_TILE, TILES_PER_CHUNK};

use super::tile::SpawnTile;

pub(super) fn plugin(app: &mut App) {
    app //<rustfmt ignore>
        .add_systems(Update, draw_gizmos);
}

#[derive(Component, Reflect)]
pub(super) struct Chunk(pub(super) ChunkCoord);

#[derive(Debug, Default, Component, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ChunkCoord(pub(super) IVec2);

impl ChunkCoord {
    pub(super) fn new(x: i32, y: i32) -> Self {
        Self(IVec2 { x, y })
    }
}

impl Deref for ChunkCoord {
    type Target = IVec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<IVec2> for ChunkCoord {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}

impl std::ops::Add for ChunkCoord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

pub(super) struct SpawnChunk {
    pub(super) chunk_coord: ChunkCoord,
}

impl Command for SpawnChunk {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_chunk)
    }
}

fn spawn_chunk(In(spawn_cmd): In<SpawnChunk>, mut commands: Commands) {
    let chunk_coord = spawn_cmd.chunk_coord;

    let chunk_entity = commands
        .spawn((
            Name::new("Chunk"),
            Chunk(chunk_coord),
            SpatialBundle::from_transform(Transform::from_translation(
                (chunk_coord.as_vec2() * PIXELS_PER_CHUNK).extend(0.0),
            )),
        ))
        .id();

    for x in 0..TILES_PER_CHUNK {
        for y in 0..TILES_PER_CHUNK {
            commands.add(SpawnTile {
                tile_pos: UVec2 { x, y },
                chunk_coord: spawn_cmd.chunk_coord,
                chunk_entity,
            });
        }
    }
}

fn draw_gizmos(mut gizmos: Gizmos, q_camera: Query<&Transform, With<Camera>>) {
    let cam_tsl = q_camera.single().translation;
    // From the origin to the camera location.
    gizmos.arrow_2d(Vec2::ZERO, cam_tsl.xy(), bevy::color::palettes::basic::RED);

    gizmos.grid_2d(
        Vec2::ZERO,
        0.0,
        UVec2::splat(10),
        UVec2::splat(PIXELS_PER_TILE * TILES_PER_CHUNK).as_vec2(),
        bevy::color::palettes::basic::AQUA,
    );
}
