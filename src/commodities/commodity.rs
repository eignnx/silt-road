use bevy::prelude::*;
use serde::Deserialize;

use crate::commodities::dollars_2024_to_dollars_1849;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Commodity>();

    app.add_systems(Startup, || {
        for comm in enum_iterator::all::<Commodity>() {
            println!(
                "{:?} -- {}",
                comm,
                display_unit_price(comm.base_unit_price(), comm.units())
            );
        }
    });
}

#[derive(
    Component,
    Debug,
    Clone,
    Copy,
    Reflect,
    Default,
    enum_iterator::Sequence,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[reflect(Component)]
pub enum Commodity {
    #[default]
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
    Nickel,
    Gold,
    Coal,
}

impl Commodity {
    pub fn units(&self) -> &'static str {
        use Commodity::*;

        match self {
            Sugar | Salt | SaltedMeat | Potatos | Flour | Grain => "pound",
            Cheese => "wheel",
            Spirits | Wine => "cask",
            Ammunition => "round",
            Firearms | Tools | Clothing => "piece",
            Wool => "skein",
            Iron | Copper | Nickel | Gold => "ingot",
            Coal => "ton",
            Lumber => "board-foot", // See: https://ekvintagewood.com/about-ek-vintage-reclaimed-wood/units-of-measure-glossary/
            Fabric => "yard",
        }
    }

    /// The (default) price of a single unit of this commodity in 1849 dollars.
    pub fn base_unit_price(&self) -> f32 {
        use Commodity::*;

        let dollars_2024 = match self {
            Grain => 1.10 / 5.0,
            Flour => 2.75 / 5.0,
            Spirits => 10.50,
            Wine => 14.00,
            Sugar => 3.0 / 2.0,
            Salt => 1.75 / 1.625,
            SaltedMeat => 4.595,
            Potatos => 0.935,
            Cheese => 5.731 * 55.0,     // per pound * 55 pounds per wheel
            Lumber => 15.55 / 4.,       // Price of home depot 1x12x4ft board
            Tools => 24.97 / 3.,        // Price of home depot 3 piece wrench set
            Ammunition => 450. / 1000., // Price of 1000 rounds of .45 ACP
            Firearms => 600.0,          // My estimate for price of a midrange handgun.
            Fabric => 6.0,              // Estimate of price of canvas fabric.
            Wool => 9.0,                // Estimate of price of skein of yarn.
            Clothing => 30.0,           // Estimate of price of a shirt.
            Iron => 0.23,               // price per 1000cm^3
            Copper => 80.,              // price per 1000cm^3
            Nickel => 203.,             // price per 1000cm^3
            Gold => 1.704e6,            // price per 1000cm^3
            Coal => 118.70,             // price per ton
        };

        dollars_2024_to_dollars_1849(dollars_2024)
    }
}

pub fn display_unit_price(unit_price: f32, unit: &'static str) -> String {
    if unit_price < 0.10 {
        format!("${:.2} per 100 {unit}", unit_price * 100.0)
    } else {
        format!("${unit_price:.2} per {unit}")
    }
}
