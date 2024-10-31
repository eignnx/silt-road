use bevy::prelude::*;

use crate::theme::widgets::{Containers, Widgets};

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    PlayerInventoryScreen,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Startup,
        (
            spawn_player_inventory,
            spawn_test_market_inventory,
            spawn_player_inventory_ui.after(spawn_player_inventory),
        ),
    );
    app.add_systems(
        OnEnter(AppState::PlayerInventoryScreen),
        spawn_player_inventory_ui,
    );
}

#[derive(Component)]
pub struct PlayerInventory;

#[derive(Component)]
pub struct MarketInventory;

#[derive(Component, Debug, Clone, Copy)]
pub enum Resource {
    Grain,
    Flour,
    Lumber,
}

#[derive(Component)]
pub struct Quantity(pub u32);

fn spawn_player_inventory(mut commands: Commands) {
    commands.spawn((PlayerInventory, Resource::Lumber, Quantity(45)));
    commands.spawn((PlayerInventory, Resource::Grain, Quantity(85)));
}

fn spawn_test_market_inventory(mut commands: Commands) {
    commands.spawn((MarketInventory, Resource::Flour, Quantity(350)));
    commands.spawn((MarketInventory, Resource::Grain, Quantity(20)));
    commands.spawn((MarketInventory, Resource::Lumber, Quantity(275)));
}

fn spawn_player_inventory_ui(
    mut commands: Commands,
    q_player_inventory: Query<(&Resource, &Quantity), With<PlayerInventory>>,
) {
    let menu_entity = commands
        .ui_root()
        .with_children(|parent| {
            parent.header("Your Inventory");
            parent.vlist().with_children(|parent| {
                for (resource, Quantity(qty)) in q_player_inventory.iter() {
                    parent.table_row().with_children(|parent| {
                        parent.label(format!("{resource:?}"));
                        parent.label(format!("x {qty}"));
                    });
                }
            });
        })
        .id();
}
