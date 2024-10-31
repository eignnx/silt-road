use bevy::prelude::*;

#[cfg(feature = "dev")]
mod dev_tools;

mod constants;
mod helpers;

#[allow(unused)]
mod terrain;

mod inventory;
mod theme;

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
    );

    app.add_plugins((
        // terrain::plugin,
        inventory::plugin,
    ));

    app.add_systems(Startup, startup);
    app.add_systems(Update, helpers::camera::movement);

    #[cfg(feature = "dev")]
    app.add_plugins((dev_tools::plugin,));
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
