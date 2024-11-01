#![allow(unused)]

use bevy::prelude::*;

use crate::{character::SpawnRandomCharacter, commodities::dollars_2024_to_dollars_1849};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Wagon>();
    app.register_type::<Team>();
    app.register_type::<DraughtAnimal>();
    app.add_systems(Startup, spawn_test_caravan);
    app.add_systems(Startup, dbg_print_draught_animal_prices);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct InPlayerCaravan;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Employee;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum Wagon {
    /// The classic covered wagon.
    Conestoga,
    /// The largest cargo wagon. Requires lots of  draught animals.
    #[allow(clippy::enum_variant_names)]
    TeamWagon,
    /// Medium-sized cargo wagon.
    Flatbed,
    /// A 2-wheeled cart.
    Cart,
    /// Carries food, cooking equipment.
    #[allow(clippy::enum_variant_names)]
    ChuckWagon,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum Team {
    Single(Entity),
    Yoke(Vec<(Entity, Entity)>),
}

impl Team {
    pub fn team_horsepower(&self, q_draught_animal: Query<&DraughtAnimal>) -> f32 {
        match self {
            Team::Single(animal) => q_draught_animal.get(*animal).unwrap().base_horsepower(),
            Team::Yoke(animals) => {
                let mut horsepower = 0.0;
                for (entity_a, entity_b) in animals {
                    let kind_a = q_draught_animal.get(*entity_a).unwrap();
                    let kind_b = q_draught_animal.get(*entity_b).unwrap();
                    let pair_unity = kind_a.yoke_unity(kind_b);
                    horsepower +=
                        (kind_a.base_horsepower() + kind_b.base_horsepower()) * pair_unity;
                }
                horsepower
            }
        }
    }

    pub fn team_speed(&self, load_weight: f32, q_draught_animal: Query<&DraughtAnimal>) -> f32 {
        let horsepower = self.team_horsepower(q_draught_animal);
        horsepower / load_weight
    }
}

#[derive(Component, Reflect, Debug, enum_iterator::Sequence)]
#[reflect(Component)]
/// An animal that pulls a wagon.
pub enum DraughtAnimal {
    Horse,
    Ox,
    Mule,
}

impl DraughtAnimal {
    pub fn haul_speed_mph(&self) -> f32 {
        match self {
            DraughtAnimal::Horse => 12.5,
            DraughtAnimal::Ox => 7.5,
            DraughtAnimal::Mule => 10.0,
        }
    }

    pub fn base_horsepower(&self) -> f32 {
        match self {
            DraughtAnimal::Horse => 1.0,
            DraughtAnimal::Ox => 2.5,
            DraughtAnimal::Mule => 0.70,
        }
    }

    pub fn base_price(&self) -> f32 {
        let price_dollars_2024 = match self {
            DraughtAnimal::Horse => 3000.00,
            DraughtAnimal::Ox => 1700.00,
            DraughtAnimal::Mule => 850.00,
        };

        dollars_2024_to_dollars_1849(price_dollars_2024)
    }

    pub fn yoke_unity(&self, other: &Self) -> f32 {
        use DraughtAnimal::*;

        match (self, other) {
            (Horse, Horse) | (Ox, Ox) | (Mule, Mule) => 1.0,
            (Horse, Mule) | (Mule, Horse) => 0.8,
            (Horse, Ox) | (Ox, Horse) => 0.5,
            (Ox, Mule) | (Mule, Ox) => 0.6,
        }
    }
}

fn dbg_print_draught_animal_prices() {
    for animal in enum_iterator::all::<DraughtAnimal>() {
        println!(
            "DRAUGHT_ANIMAL: One {:?} costs ${:.2}",
            animal,
            animal.base_price()
        );
    }
}

fn spawn_test_caravan(mut commands: Commands) {
    for _ in 0..10 {
        commands.add(SpawnRandomCharacter {
            bundle: Some(InPlayerCaravan),
        });
    }

    let horse1 = commands.spawn(DraughtAnimal::Horse).id();
    let horse2 = commands.spawn(DraughtAnimal::Horse).id();
    commands
        .spawn((InPlayerCaravan, Wagon::Conestoga))
        .with_children(|parent| {
            parent.spawn(Team::Yoke(vec![(horse1, horse2)]));
        });

    let mule1 = commands.spawn(DraughtAnimal::Mule).id();
    commands
        .spawn((InPlayerCaravan, Wagon::ChuckWagon))
        .with_children(|parent| {
            parent.spawn(Team::Single(mule1));
        });
}
