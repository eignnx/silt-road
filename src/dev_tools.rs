use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[allow(unused)]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WorldInspectorPlugin::new());
}
