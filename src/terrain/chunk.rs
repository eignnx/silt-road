use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;
use libnoise::Generator;

use crate::constants::{PIXELS_PER_TILE, RENDER_CHUNK_SIZE, TILES_PER_CHUNK};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ChunkManager::default()).add_systems(
        Update,
        (spawn_chunks_around_camera, despawn_outofrange_chunks),
    );
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    // The entity points to the chunk entity.
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Debug, Default, Component)]
struct Chunk;

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 1)..=(camera_chunk_pos.y + 1) {
            for x in (camera_chunk_pos.x - 1)..=(camera_chunk_pos.x + 1) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &asset_server, IVec2::new(x, y));
                }
            }
        }
    }
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let tile_size = IVec2::new(PIXELS_PER_TILE.x as i32, PIXELS_PER_TILE.y as i32);
    camera_pos.as_ivec2() / (TILES_PER_CHUNK.as_ivec2() * tile_size)
}

fn spawn_chunk(commands: &mut Commands, asset_server: &AssetServer, chunk_pos: IVec2) {
    let tilemap_entity = commands.spawn_empty().id();

    let generator = libnoise::Source::simplex(1234).fbm(5, 0.013, 2.0, 0.5);
    let mut tile_storage = TileStorage::empty(TILES_PER_CHUNK.into());

    // Spawn the elements of the tilemap.
    for x in 0..TILES_PER_CHUNK.x {
        for y in 0..TILES_PER_CHUNK.y {
            let global_tile_pos = [
                (chunk_pos.x + x as i32) as f64,
                (chunk_pos.y + y as i32) as f64,
            ];
            let noise_val = generator.sample(global_tile_pos); // `noise_val` is between -1.0 and +1.0.
            let norm_noise_val = (noise_val + 1.0) / 2.0; // Now between 0.0 and +1.0.

            let tile_index = TileTextureIndex((norm_noise_val * 6.0).floor() as u32);

            spawn_tile(
                TilePos { x, y },
                tile_index,
                commands,
                tilemap_entity,
                &mut tile_storage,
            );
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * TILES_PER_CHUNK.x as f32 * PIXELS_PER_TILE.x,
        chunk_pos.y as f32 * TILES_PER_CHUNK.y as f32 * PIXELS_PER_TILE.y,
        0.0,
    ));

    let texture_handle: Handle<Image> = asset_server.load("TexturedGrass.png");

    commands.entity(tilemap_entity).insert((
        Chunk,
        #[cfg(feature = "dev")]
        Name::new(format!("Chunk: ({}, {})", chunk_pos.x, chunk_pos.y)),
        TilemapBundle {
            grid_size: PIXELS_PER_TILE.into(),
            size: TILES_PER_CHUNK.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: PIXELS_PER_TILE,
            transform,
            render_settings: TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn spawn_tile(
    position: TilePos,
    tile_texture_index: TileTextureIndex,
    commands: &mut Commands,
    chunk_entity: Entity,
    tile_storage: &mut TileStorage,
) {
    let tile_entity = commands
        .spawn((
            #[cfg(feature = "dev")]
            Name::new(format!("Tile: ({}, {})", position.x, position.y)),
            TileBundle {
                position,
                tilemap_id: TilemapId(chunk_entity),
                texture_index: tile_texture_index,
                ..Default::default()
            },
        ))
        .id();

    commands.entity(chunk_entity).add_child(tile_entity);
    tile_storage.set(&position, tile_entity);
}

fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &Transform), With<Chunk>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > TILES_PER_CHUNK.x as f32 * PIXELS_PER_TILE.x * 2.0 {
                let chunk_pos = camera_pos_to_chunk_pos(&chunk_pos);
                chunk_manager.spawned_chunks.remove(&chunk_pos);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
