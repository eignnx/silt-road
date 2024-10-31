use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Startup,
        (spawn_player_inventory, spawn_test_market_inventory),
    );
}

#[derive(Component)]
pub struct PlayerInventory;

#[derive(Component)]
pub struct MarketInventory;

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub enum TradeResource {
    Grain,
    Flour,
    Spirits,
    Wine,
    Sugar,
    Salt,
    SaltedMeat,
    Potatos,
    Cheese,
    Lumber,
    Tools,
    Ammunition,
    Firearms,
    Fabric,
    Wool,
    Clothing,
    Iron,
    Copper,
    Nickle,
    Gold,
}

impl TradeResource {
    pub fn units(&self) -> &'static str {
        use TradeResource::*;

        match self {
            Sugar | Salt | SaltedMeat | Potatos | Flour | Grain => "lbs",
            Cheese => "wheels",
            Spirits | Wine => "casks",
            Ammunition => "rounds",
            Firearms | Tools | Clothing => "pieces",
            Wool => "skeins",
            Iron | Copper | Nickle | Gold => "ingots",
            Lumber => "board-feet", // See: https://ekvintagewood.com/about-ek-vintage-reclaimed-wood/units-of-measure-glossary/
            Fabric => "yards",
        }
    }
}

#[derive(Component)]
pub struct Quantity(pub u32);

#[derive(Component)]
pub enum Quality {
    Low,
    High,
}

fn spawn_player_inventory(mut commands: Commands) {
    commands.spawn((PlayerInventory, TradeResource::Lumber, Quantity(45)));
    commands.spawn((
        PlayerInventory,
        TradeResource::Firearms,
        Quality::Low,
        Quantity(12),
    ));
    commands.spawn((
        PlayerInventory,
        TradeResource::Cheese,
        Quality::High,
        Quantity(36),
    ));
    commands.spawn((PlayerInventory, TradeResource::Grain, Quantity(85)));
}

fn spawn_test_market_inventory(mut commands: Commands) {
    commands.spawn((MarketInventory, TradeResource::Flour, Quantity(350)));
    commands.spawn((MarketInventory, TradeResource::Grain, Quantity(20)));
    commands.spawn((MarketInventory, TradeResource::Lumber, Quantity(275)));
}
