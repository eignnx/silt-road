use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
    utils::HashSet,
};

use super::chunk::{Chunk, ChunkCoord, SpawnChunk};
use crate::constants::{PIXELS_PER_CHUNK, PIXELS_PER_TILE, TILES_PER_CHUNK};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ChunkManager>();
    app.add_systems(Startup, |mut commands: Commands| {
        commands.trigger(ChunkChangeEv {});
    });
    app.add_systems(Update, watch_for_camera_chunk_change);
    app.observe(despawn_outofrange_chunks);
    app.observe(spawn_chunks_around_camera);
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<ChunkCoord>,
    /// The chunk the camera is currently "in".
    pub cam_chunk: ChunkCoord,
}

fn watch_for_camera_chunk_change(
    mut commands: Commands,
    q_camera: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let current_cam_chunk = chunk_containing_the_camera(q_camera.single());
    let old = chunk_manager.cam_chunk;
    if current_cam_chunk != old {
        chunk_manager.cam_chunk = current_cam_chunk;
        commands.trigger(ChunkChangeEv {});
    }
}

fn chunk_containing_the_camera(cam_tsf: &Transform) -> ChunkCoord {
    let cam_tsl = cam_tsf.translation;
    let cam_chunk_center_offset = IVec2::splat(PIXELS_PER_TILE as i32 * TILES_PER_CHUNK as i32 / 2);
    let cam_pos = cam_tsl.xy().as_ivec2() - cam_chunk_center_offset;
    ChunkCoord::from(cam_pos / PIXELS_PER_TILE as i32 / TILES_PER_CHUNK as i32)
}

#[derive(Debug, Event)]
struct ChunkChangeEv {}

fn despawn_outofrange_chunks(
    _trigger: Trigger<ChunkChangeEv>,
    mut commands: Commands,
    q_camera: Query<&Transform, With<Camera>>,
    q_chunk_entities: Query<(Entity, &Chunk)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let cam_pos = q_camera.single().translation.xy();
    for (chunk_entity, Chunk(chunk_coord)) in q_chunk_entities.iter() {
        if !chunk_manager.spawned_chunks.contains(chunk_coord) {
            continue;
        }
        let chunk_center_pos = chunk_coord.as_vec2() * PIXELS_PER_CHUNK + PIXELS_PER_CHUNK / 2.0;
        let dist_to_cam = chunk_center_pos.distance(cam_pos);
        if dist_to_cam > 3.0 * PIXELS_PER_CHUNK {
            println!("Despawning chunk: {:?}", chunk_coord);
            chunk_manager.spawned_chunks.remove(chunk_coord);
            commands.entity(chunk_entity).despawn_recursive();
        }
    }

    commands.trigger(SpawnChunksEv {});
}

#[derive(Debug, Event)]
struct SpawnChunksEv {}

fn spawn_chunks_around_camera(
    _trigger: Trigger<SpawnChunksEv>,
    mut commands: Commands,
    q_camera: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let cam_chunk = chunk_containing_the_camera(q_camera.single());

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
