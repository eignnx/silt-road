use bevy::prelude::*;

use crate::{
    commodity::{Commodity, Quality, Quantity},
    inventory::PlayerInventory,
    theme::{
        interaction::OnPress,
        widgets::{Containers, Widgets},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<AppState>();
    // app.add_systems(Startup, spawn_player_inventory_ui);
    app.add_systems(
        OnEnter(AppState::PlayerInventoryScreen),
        spawn_player_inventory_ui,
    );
    app.add_systems(OnEnter(AppState::TownScreen), spawn_town_screen_ui);
    app.add_systems(OnExit(AppState::PlayerInventoryScreen), despawn_active_pane);
    app.add_systems(OnExit(AppState::TownScreen), despawn_active_pane);
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    PlayerInventoryScreen,
    #[default]
    TownScreen,
}

#[derive(Resource)]
pub struct ActiveMenu {
    pub menu_pane_entity: Entity,
}

fn spawn_player_inventory_ui(
    mut commands: Commands,
    q_player_inventory: Query<(&Commodity, &Quantity, Option<&Quality>), With<PlayerInventory>>,
) {
    let menu_entity = commands
        .ui_root()
        .with_children(|parent| {
            parent.header("Your Inventory");
            parent.vlist().with_children(|parent| {
                for (resource, Quantity(qty), quality) in q_player_inventory.iter() {
                    let units = resource.units();
                    parent.table_row().with_children(|parent| {
                        parent.label(match quality {
                            Some(Quality::High) => format!("high quality {resource:?}"),
                            Some(Quality::Low) => format!("low grade {resource:?}"),
                            None => format!("{resource:?}"),
                        });
                        parent.label(format!("x {qty} {units}"));
                    });
                }
            });
            parent.button("Back").observe(enter_town_screen);
        })
        .id();

    commands.insert_resource(ActiveMenu {
        menu_pane_entity: menu_entity,
    });
}

fn despawn_active_pane(mut commands: Commands, active_menu: Res<ActiveMenu>) {
    commands
        .entity(active_menu.menu_pane_entity)
        .despawn_recursive();
    commands.remove_resource::<ActiveMenu>();
}

fn spawn_town_screen_ui(mut commands: Commands) {
    let menu_entity = commands
        .ui_root()
        .with_children(|parent| {
            parent.header("The Town");
            parent
                .button("Your Inventory")
                .observe(enter_player_inventory_screen);
            parent.button("Market");
            // .observe(enter_player_inventory_screen);
        })
        .id();

    commands.insert_resource(ActiveMenu {
        menu_pane_entity: menu_entity,
    });
}

fn enter_player_inventory_screen(
    _trigger: Trigger<OnPress>,
    mut next_screen: ResMut<NextState<AppState>>,
) {
    next_screen.set(AppState::PlayerInventoryScreen);
}

fn enter_town_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::TownScreen);
}
