use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

mod chunk;
mod chunk_manager;
mod tile;
mod world_gen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        TilemapPlugin,
        chunk::plugin,
        chunk_manager::plugin,
        world_gen::plugin,
    ));
}
