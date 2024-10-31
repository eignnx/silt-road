use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

#[cfg(feature = "dev")]
mod dev_tools;

mod constants;
mod helpers;
mod terrain;

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
    .add_plugins((
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
        terrain::plugin,
    ))
    .add_systems(Startup, startup)
    .add_systems(Update, helpers::camera::movement);

    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
