use bevy::prelude::*;

mod commodity;
pub use commodity::*;
mod quality;
pub use quality::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(commodity::plugin);
    app.add_plugins(quality::plugin);
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Quantity(pub u32);
