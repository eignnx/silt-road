use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

mod chunk;
mod tile;
mod world_gen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((TilemapPlugin, chunk::plugin, world_gen::plugin));
}
