use bevy::{prelude::*, window::WindowResolution};

#[cfg(feature = "dev")]
mod dev_tools;

mod constants;
mod helpers;

#[allow(unused)]
mod terrain;

mod caravan;
mod character;
mod commodities;
mod inventory;
mod screens;
mod theme;

pub fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Silt Road"),
                    skip_taskbar: true,
                    resizable: true,
                    resolution: WindowResolution::new(640.0, 360.0),
                    ..Default::default()
                }),

                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_plugins((
        // terrain::plugin,
        caravan::plugin,
        character::plugin,
        inventory::plugin,
        screens::plugin,
        theme::plugin,
        commodities::plugin,
    ));

    app.add_systems(Startup, startup);
    app.add_systems(Update, helpers::camera::movement);

    #[cfg(feature = "dev")]
    app.add_plugins((dev_tools::plugin,));
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
