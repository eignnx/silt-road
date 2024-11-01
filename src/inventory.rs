use bevy::prelude::*;

use crate::trade_resources::{Quality, Quantity, TradeResource};

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
