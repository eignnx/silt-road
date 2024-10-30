use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    utils::HashSet,
};
use bevy_ecs_tilemap::prelude::*;
use libnoise::Generator;

use crate::constants::{CHUNK_SIZE, RENDER_CHUNK_SIZE, TILE_SIZE};

#[cfg(feature = "dev")]
mod dev_tools;

mod constants;
mod helpers;

pub fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Silt Road"),
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(ChunkManager::default())
    .add_plugins(LogDiagnosticsPlugin::default())
    .add_plugins(FrameTimeDiagnosticsPlugin)
    .add_plugins(TilemapPlugin)
    .add_systems(Startup, startup)
    .add_systems(
        Update,
        (
            helpers::camera::movement,
            spawn_chunks_around_camera,
            despawn_outofrange_chunks,
        ),
    );

    app.add_plugins(dev_tools::plugin);
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &asset_server, IVec2::new(x, y));
                }
            }
        }
    }
}

fn spawn_chunk(commands: &mut Commands, asset_server: &AssetServer, chunk_pos: IVec2) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    let generator = libnoise::Source::simplex(1234).fbm(5, 0.013, 2.0, 0.5);

    // Spawn the elements of the tilemap.
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            // noise_val is between -1.0 and +1.0.
            let noise_val = generator.sample([
                (chunk_pos.x + x as i32) as f64 / 10.0,
                (chunk_pos.y + y as i32) as f64 / 10.0,
            ]);
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
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));

    let texture_handle: Handle<Image> = asset_server.load("TexturedGrass.png");

    commands.entity(tilemap_entity).insert((
        Name::new(format!("Chunk: ({}, {})", chunk_pos.x, chunk_pos.y)),
        TilemapBundle {
            grid_size: TILE_SIZE.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: TILE_SIZE,
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
    texture_index: TileTextureIndex,
    commands: &mut Commands<'_, '_>,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
) {
    let tile_entity = commands
        .spawn(TileBundle {
            position,
            tilemap_id: TilemapId(tilemap_entity),
            texture_index,
            ..Default::default()
        })
        .id();
    commands.entity(tilemap_entity).add_child(tile_entity);
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
            if distance > 320.0 {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Debug, Default, Component)]
struct Chunk;
