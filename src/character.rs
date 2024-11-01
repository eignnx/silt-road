use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use rand::{seq::SliceRandom, Rng};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Character>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Character {
    name: String,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct HiredGun;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Cook;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Teamster;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Laborer;

pub struct SpawnRandomCharacter<B: Bundle> {
    pub bundle: Option<B>,
}

impl<B: Bundle> Command for SpawnRandomCharacter<B> {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, |In(cmd): In<Self>, mut commands: Commands| {
            let mut rng = rand::thread_rng();

            let name = random_name();

            let mut entity_commands = commands.spawn((
                Character { name: name.clone() },
                Name::new(format!("Character: {name}")),
            ));

            if let Some(bundle) = cmd.bundle {
                entity_commands.insert(bundle);
            }

            let mut skills = 0;

            // Ensure at least one skill.
            while skills < 1 {
                if rng.gen_bool(0.10) {
                    entity_commands.insert(HiredGun);
                    skills += 1;
                }
                if rng.gen_bool(0.25) {
                    entity_commands.insert(Cook);
                    skills += 1;
                }
                if rng.gen_bool(0.20) {
                    entity_commands.insert(Teamster);
                    skills += 1;
                }
                if rng.gen_bool(0.35) {
                    entity_commands.insert(Laborer);
                    skills += 1;
                }
            }
        });
    }
}

fn random_name() -> String {
    let first_name = FIRST_NAMES.choose(&mut rand::thread_rng()).unwrap();
    let last_name = LAST_NAMES.choose(&mut rand::thread_rng()).unwrap();
    format!("{} {}", first_name, last_name)
}

const FIRST_NAMES: &[&str] = &[
    "Abigail",
    "Ace",
    "Amos",
    "Annie",
    "Beau",
    "Belle",
    "Buck",
    "Calamity",
    "Cheyenne",
    "Clara",
    "Clementine",
    "Clint",
    "Clyde",
    "Daisy",
    "Dakota",
    "Dusty",
    "Earl",
    "Eleanor",
    "Eli",
    "Emmett",
    "Etta",
    "Floyd",
    "Gus",
    "Hank",
    "Hattie",
    "Hezekiah",
    "Ike",
    "Jasper",
    "Jesse",
    "Jesse",
    "Lillie",
    "Mabel",
    "Mae",
    "Maverick",
    "Millie",
    "Nettie",
    "Otis",
    "Pearl",
    "Ringo",
    "Rufus",
    "Sadie",
    "Silas",
    "Sundance",
    "Tess",
    "Tex",
    "Virgil",
    "Wade",
    "Wes",
    "Willa",
    "Wyatt",
    "Zane",
    "Zeke",
];

const LAST_NAMES: &[&str] = &[
    "Baker",
    "Black",
    "Brown",
    "Carter",
    "Clark",
    "Cole",
    "Collins",
    "Cook",
    "Cooper",
    "Davis",
    "Diaz",
    "Evans",
    "Fisher",
    "Flores",
    "Foster",
    "Garcia",
    "Gonzalez",
    "Gray",
    "Green",
    "Hall",
    "Harris",
    "Hernandez",
    "Hill",
    "Howard",
    "Hughes",
    "Jackson",
    "James",
    "Jenkins",
    "Johnson",
    "Jones",
    "King",
    "Lee",
    "Lewis",
    "Long",
    "Lopez",
    "Martin",
    "Martinez",
    "Miller",
    "Mitchell",
    "Moore",
    "Morris",
    "Murphy",
    "Nelson",
    "Parker",
    "Perez",
    "Perry",
    "Peterson",
    "Phillips",
    "Powell",
    "Price",
    "Ramirez",
    "Reed",
    "Reyes",
    "Reynolds",
    "Richardson",
    "Rivera",
    "Roberts",
    "Robinson",
    "Rodriguez",
    "Rogers",
    "Ross",
    "Russell",
    "Sanchez",
    "Sanders",
    "Scott",
    "Simmons",
    "Smith",
    "Stewart",
    "Taylor",
    "Thomas",
    "Thompson",
    "Torres",
    "Turner",
    "Walker",
    "Ward",
    "Watson",
    "White",
    "Williams",
    "Wilson",
    "Wood",
    "Wright",
    "Young",
];
