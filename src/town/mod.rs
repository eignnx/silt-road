use bevy::{prelude::*, utils::HashMap};
use bevy_easy_config::EasyConfigPlugin;
use rand::Rng;
use serde::Deserialize;

use crate::commodities::Commodity;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Town>();
    app.register_type::<Business>();
    app.register_type::<BusinessSupplyDemandSettings>();
    app.register_type::<SuppliesDemands>();
    app.add_plugins(EasyConfigPlugin::<BusinessSupplyDemandSettings>::new(
        "config/BusinessSupplyDemand.ron",
    ));
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
        name: "Quake City".to_string(),
        population: 18_000,
        accessability: 0.50,
    };
    commands
        .spawn((Name::new(town.name.clone()), town.clone()))
        .with_children(|parent| {
            for business_type in enum_iterator::all::<Business>() {
                let rarity = town.business_rarity(&business_type);
                if rand::thread_rng().gen_bool(rarity as f64) {
                    parent.spawn((Name::new(format!("{:?}", business_type)), business_type));
                }
            }
        });
}

#[derive(
    Component,
    Reflect,
    Debug,
    enum_iterator::Sequence,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
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

#[derive(Asset, Resource, Reflect, Deserialize, Debug, Clone)]
#[reflect(Resource)]
pub struct BusinessSupplyDemandSettings {
    supplies_demands: HashMap<Business, SuppliesDemands>,
}

impl Default for BusinessSupplyDemandSettings {
    fn default() -> Self {
        Self {
            supplies_demands: enum_iterator::all::<Business>()
                .map(|business| (business, SuppliesDemands::default()))
                .collect(),
        }
    }
}

#[derive(Reflect, Debug, Deserialize, Default, Clone)]
pub struct SuppliesDemands {
    pub demands: Vec<Commodity>,
    pub supplies: Vec<Commodity>,
}
