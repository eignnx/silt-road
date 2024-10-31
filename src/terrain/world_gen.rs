use bevy::{prelude::*, utils::HashSet};
use libnoise::Generator;

use crate::constants::{PIXELS_PER_TILE, TILES_PER_CHUNK};

use super::chunk::{ChunkCoord, SpawnChunk};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ChunkManager>();
    app.add_systems(
        Update,
        (
            spawn_chunks_around_camera,
            // despawn_outofrange_chunks,
        ),
    );
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<ChunkCoord>,
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    q_camera: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let cam_tsl = q_camera.single().translation;
    let cam_chunk_center_offset = IVec2::splat(PIXELS_PER_TILE as i32 * TILES_PER_CHUNK as i32 / 2);
    let cam_pos = cam_tsl.xy().as_ivec2() - cam_chunk_center_offset;
    let cam_chunk = ChunkCoord::from(cam_pos / PIXELS_PER_TILE as i32 / TILES_PER_CHUNK as i32);

    for dx in -2..=2 {
        for dy in -2..=2 {
            let offset = ChunkCoord::new(dx, dy);
            let chunk_coord = cam_chunk + offset;
            if !chunk_manager.spawned_chunks.contains(&chunk_coord) {
                commands.add(SpawnChunk { chunk_coord });
                chunk_manager.spawned_chunks.insert(chunk_coord);
            }
        }
    }
}
