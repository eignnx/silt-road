use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;
use libnoise::Generator;

use crate::constants::{PIXELS_PER_TILE, TILES_PER_CHUNK};

pub(super) fn plugin(app: &mut App) {
    app //<rustfmt ignore>
        .insert_resource(ChunkManager::default())
        .add_systems(
            Update,
            (
                spawn_chunks_around_camera,
                despawn_outofrange_chunks,
                draw_gizmos,
            ),
        );
    // .add_systems(Startup, spawn_single_chunk);
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<ChunkCoord>,
}

#[derive(Debug, Default, Component)]
struct Chunk;

#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash)]
struct ChunkCoord(pub IVec2);

fn spawn_single_chunk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let chunk_coord = ChunkCoord(IVec2::ZERO);
    chunk_manager.spawned_chunks.insert(chunk_coord);
    spawn_chunk(&mut commands, &asset_server, chunk_coord);
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let cam_tsf = camera_query.single();
    let camera_chunk_coord = world_pos_to_chunk_coord(&cam_tsf.translation.xy());
    let ChunkCoord(IVec2 { x: ccx, y: ccy }) = camera_chunk_coord;
    for y in (ccy - 1)..=(ccy + 1) {
        for x in (ccx - 1)..=(ccx + 1) {
            let chunk_coord = ChunkCoord(IVec2::new(x, y));
            if !chunk_manager.spawned_chunks.contains(&chunk_coord) {
                chunk_manager.spawned_chunks.insert(chunk_coord);
                spawn_chunk(&mut commands, &asset_server, chunk_coord);
            }
        }
    }
}

/// From a world position in pixels to a chunk coordinate.
fn world_pos_to_chunk_coord(world_pos: &Vec2) -> ChunkCoord {
    ChunkCoord(
        world_pos.as_ivec2() / (TILES_PER_CHUNK.as_ivec2() * PIXELS_PER_TILE.as_ivec2())
            - 1 / (2 * TILES_PER_CHUNK.as_ivec2()),
    )
}

fn chunk_and_tile_to_world_pos(chunk_coord: ChunkCoord, tile_coord: UVec2) -> Vec2 {
    let chunk_shift = chunk_coord.0.as_vec2() * TILES_PER_CHUNK.as_vec2();
    let tile_shift = tile_coord.as_vec2();
    // Shift by half a tile since chunk origin is at the center of tile (0,0).
    let fudge_tile_shift = Vec2::splat(0.5);
    PIXELS_PER_TILE.as_vec2() * (chunk_shift + tile_shift + fudge_tile_shift)
}

fn spawn_chunk(commands: &mut Commands, asset_server: &AssetServer, chunk_coord: ChunkCoord) {
    let tilemap_entity = commands.spawn_empty().id();

    let generator = libnoise::Source::simplex(1234).fbm(5, 0.013, 2.0, 0.5);
    let mut tile_storage = TileStorage::empty(TILES_PER_CHUNK.into());

    // Spawn the elements of the tilemap.
    for x in 0..TILES_PER_CHUNK.x as i32 {
        for y in 0..TILES_PER_CHUNK.y as i32 {
            let tile_pos = IVec2::new(x, y);
            let global_tile_pos = (chunk_coord.0 * TILES_PER_CHUNK.as_ivec2() + tile_pos)
                .as_dvec2()
                .to_array();
            let noise_val = generator.sample(global_tile_pos); // `noise_val` is between -1.0 and +1.0.
            let norm_noise_val = (noise_val + 1.0) / 2.0; // Now between 0.0 and +1.0.

            let tile_index = TileTextureIndex((norm_noise_val * 6.0).floor() as u32);

            spawn_tile(
                tile_pos.as_uvec2().into(),
                tile_index,
                commands,
                tilemap_entity,
                &mut tile_storage,
            );
        }
    }

    let transform = Transform::from_translation(
        chunk_and_tile_to_world_pos(chunk_coord, UVec2::ZERO).extend(0.0),
    );

    let texture_handle: Handle<Image> = asset_server.load("TexturedGrass.png");

    let grid_size = TilemapGridSize::from(PIXELS_PER_TILE.as_vec2());
    let tile_size = TilemapTileSize::from(PIXELS_PER_TILE.as_vec2());

    commands.entity(tilemap_entity).insert((
        Chunk,
        chunk_coord,
        #[cfg(feature = "dev")]
        Name::new(format!("Chunk: {:?}", chunk_coord)),
        TilemapBundle {
            grid_size,
            size: TILES_PER_CHUNK.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform,
            render_settings: TilemapRenderSettings {
                render_chunk_size: TILES_PER_CHUNK,
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
    q_chunk_entities: Query<(Entity, &ChunkCoord), With<Chunk>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let camera_transform = camera_query.single();

    for (chunk_entity, chunk_coord) in q_chunk_entities.iter() {
        let chunk_pos = chunk_and_tile_to_world_pos(*chunk_coord, UVec2::ZERO)
            + PIXELS_PER_TILE.as_vec2() * TILES_PER_CHUNK.as_vec2() / 2.0;
        let camera_pos = camera_transform.translation.xy();
        let distance = camera_pos.distance(chunk_pos);
        if distance > 2.0 * TILES_PER_CHUNK.x as f32 * PIXELS_PER_TILE.x as f32 {
            let chunk_coord = world_pos_to_chunk_coord(&chunk_pos);
            chunk_manager.spawned_chunks.remove(&chunk_coord);
            commands.entity(chunk_entity).despawn_recursive();
        }
    }
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    chunk_manager: Res<ChunkManager>,
    q_camera: Query<&Transform, With<Camera>>,
) {
    let cam_tsl = q_camera.single().translation;
    // From the origin to the camera location.
    gizmos.arrow_2d(Vec2::ZERO, cam_tsl.xy(), bevy::color::palettes::basic::RED);

    for chunk_coord in chunk_manager.spawned_chunks.iter() {
        gizmos.grid_2d(
            chunk_and_tile_to_world_pos(*chunk_coord, UVec2::ZERO)
                - PIXELS_PER_TILE.as_vec2() / 2.0,
            0.0,
            UVec2::splat(100),
            PIXELS_PER_TILE.as_vec2() * TILES_PER_CHUNK.as_vec2(),
            bevy::color::palettes::basic::AQUA,
        );
    }
}
