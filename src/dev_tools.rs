use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[allow(unused)]
#[cfg(feature = "dev")]
pub(super) fn plugin(app: &mut App) {
    use bevy::input::common_conditions::input_toggle_active;

    app.add_plugins((
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backquote)),
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
    ));
}
