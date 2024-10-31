use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[allow(unused)]
#[cfg(feature = "dev")]
pub(super) fn plugin(app: &mut App) {
    use bevy::{dev_tools::states::log_transitions, input::common_conditions::input_toggle_active};

    use crate::screens::AppState;

    app.add_plugins((
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backquote)),
        LogDiagnosticsPlugin::default(),
        // FrameTimeDiagnosticsPlugin,
    ));
    app.add_systems(Update, log_transitions::<AppState>);
}
