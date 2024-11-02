use bevy::prelude::*;

mod commodity;
pub use commodity::*;
mod quality;
pub use quality::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Quantity>();
    app.add_plugins(commodity::plugin);
    app.add_plugins(quality::plugin);
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[reflect(Component)]
pub struct Quantity(pub u32);

pub fn dollars_2024_to_dollars_1849(dollars_2024: f32) -> f32 {
    // See: https://www.officialdata.org/us/inflation/2024?endYear=1849&amount=100
    dollars_2024 * 2.44 / 100.0
}
