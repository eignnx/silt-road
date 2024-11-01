use bevy::prelude::*;

use crate::commodities::{Commodity, Quality, Quantity};

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

fn spawn_player_inventory(mut commands: Commands) {
    commands.spawn((PlayerInventory, Commodity::Lumber, Quantity(45)));
    commands.spawn((
        PlayerInventory,
        Commodity::Firearms,
        Quality::Low,
        Quantity(12),
    ));
    commands.spawn((
        PlayerInventory,
        Commodity::Cheese,
        Quality::High,
        Quantity(36),
    ));
    commands.spawn((PlayerInventory, Commodity::Grain, Quantity(85)));
}

fn spawn_test_market_inventory(mut commands: Commands) {
    commands.spawn((MarketInventory, Commodity::Flour, Quantity(350)));
    commands.spawn((MarketInventory, Commodity::Grain, Quantity(20)));
    commands.spawn((MarketInventory, Commodity::Lumber, Quantity(275)));
}
