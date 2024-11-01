use bevy::prelude::*;

use crate::character::SpawnRandomCharacter;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Wagon>();
    app.add_systems(Startup, spawn_test_caravan);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerCaravan;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Employee;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Wagon {
    weight_capacity: f32,
}

fn spawn_test_caravan(mut commands: Commands) {
    for _ in 0..10 {
        commands.add(SpawnRandomCharacter {
            bundle: Some(PlayerCaravan),
        });
    }

    commands.spawn((
        PlayerCaravan,
        Wagon {
            weight_capacity: 2400.0,
        },
    ));
}
