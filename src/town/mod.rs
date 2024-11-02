use bevy::prelude::*;
use rand::Rng;

use crate::commodities::Commodity;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Town>();
    app.register_type::<Business>();
    app.add_systems(Startup, spawn_test_town);
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Town {
    pub name: String,
    pub population: u32,
    /// 0.0 is remote, 1.0 is located on a major trade route.
    pub accessability: f32,
}

impl Town {
    pub fn business_rarity(&self, business: &Business) -> f32 {
        use Business::*;
        let pop_mag = (self.population as f32 + 1.0).log10() / 10.0;
        let accessibility = self.accessability;
        let remoteness = 1.0 - self.accessability;
        match business {
            TrainStation => accessibility,
            WagonShop => 0.90 + pop_mag,
            LivestockAuction => 0.50 + pop_mag,
            Warehouse => 0.30 + pop_mag + accessibility,
            LumberMill | Refinery => 0.3 + remoteness * 0.5,
            CoalMine => remoteness * 0.9,
            IronMine => remoteness * 0.8,
            CopperMine => remoteness * 0.4,
            NickelMine => remoteness * 0.2,
            GoldMine => remoteness * 0.1,
            Blacksmith => 0.9,
            Mill | Farm => remoteness + 0.5,
            Brewery | Winery => pop_mag,
            Tavern => pop_mag + 0.3,
            Weaver | Tailor | Butcher => 0.5 + pop_mag,
        }
        .clamp(0.0, 1.0)
    }
}

fn spawn_test_town(mut commands: Commands) {
    let town = Town {
        name: "Exampliburg".to_string(),
        population: 18_000,
        accessability: 0.50,
    };
    commands
        .spawn((Name::new("Exampliburg"), town.clone()))
        .with_children(|parent| {
            for business_type in enum_iterator::all::<Business>() {
                let rarity = town.business_rarity(&business_type);
                if rand::thread_rng().gen_bool(rarity as f64) {
                    parent.spawn((Name::new(format!("{:?}", business_type)), business_type));
                }
            }
        });
}

#[derive(Component, Reflect, Debug, enum_iterator::Sequence)]
#[reflect(Component)]
pub enum Business {
    WagonShop,
    LivestockAuction,
    Warehouse,
    /// Refines ore into metal.
    Refinery,
    LumberMill,
    Tavern,
    CoalMine,
    IronMine,
    CopperMine,
    NickelMine,
    GoldMine,
    /// Makes tools
    Blacksmith,
    Weaver,
    Tailor,
    Farm,
    Mill,
    Brewery,
    Winery,
    Butcher,
    TrainStation,
}

pub struct ConsumesProduces {
    pub consumes: Vec<Commodity>,
    pub produces: Vec<Commodity>,
}

impl Business {
    pub fn commodities_consumed(&self) -> ConsumesProduces {
        use Business::*;
        use Commodity::*;
        match self {
            WagonShop => ConsumesProduces {
                consumes: vec![Lumber, Iron],
                produces: vec![],
            },
            LivestockAuction => ConsumesProduces {
                consumes: vec![Grain],
                produces: vec![],
            },
            Warehouse => ConsumesProduces {
                consumes: vec![],
                produces: vec![],
            },
            Refinery => ConsumesProduces {
                consumes: vec![],
                produces: vec![Iron, Nickel, Copper],
            },
            LumberMill => ConsumesProduces {
                consumes: vec![Tools],
                produces: vec![Lumber],
            },
            Tavern => ConsumesProduces {
                consumes: vec![Wine, Spirits, Cheese, SaltedMeat],
                produces: vec![],
            },
            CoalMine => ConsumesProduces {
                consumes: vec![Tools],
                produces: vec![Coal],
            },
            IronMine => ConsumesProduces {
                consumes: vec![Tools],
                produces: vec![Iron],
            },
            CopperMine => ConsumesProduces {
                consumes: vec![Tools],
                produces: vec![Copper],
            },
            NickelMine => ConsumesProduces {
                consumes: vec![Tools],
                produces: vec![Nickel],
            },
            GoldMine => ConsumesProduces {
                consumes: vec![Tools],
                produces: vec![Gold],
            },
            Blacksmith => ConsumesProduces {
                consumes: vec![Iron],
                produces: vec![Tools],
            },
            Weaver => ConsumesProduces {
                consumes: vec![Wool],
                produces: vec![Fabric],
            },
            Tailor => ConsumesProduces {
                consumes: vec![Fabric],
                produces: vec![Clothing],
            },
            Farm => ConsumesProduces {
                consumes: vec![Tools],
                produces: vec![Grain, Potatos],
            },
            Mill => ConsumesProduces {
                consumes: vec![Grain],
                produces: vec![Flour],
            },
            Brewery => ConsumesProduces {
                consumes: vec![Grain, Potatos],
                produces: vec![Spirits],
            },
            Winery => ConsumesProduces {
                consumes: vec![],
                produces: vec![Wine, Cheese],
            },
            Butcher => ConsumesProduces {
                consumes: vec![Salt],
                produces: vec![SaltedMeat],
            },
            TrainStation => ConsumesProduces {
                consumes: vec![Coal],
                produces: vec![
                    Salt, Sugar, Cheese, SaltedMeat, Ammunition, Firearms, Tools, Clothing, Lumber,
                    Wine,
                ],
            },
        }
    }
}
