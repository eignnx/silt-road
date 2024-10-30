use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

mod chunk;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((TilemapPlugin, chunk::plugin));
}
