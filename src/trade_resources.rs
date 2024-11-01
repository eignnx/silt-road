use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TradeResource>();

    app.add_systems(Startup, || {
        let resources = vec![
            TradeResource::Grain,
            TradeResource::Flour,
            TradeResource::Spirits,
            TradeResource::Wine,
            TradeResource::Sugar,
            TradeResource::Salt,
            TradeResource::SaltedMeat,
            TradeResource::Potatos,
            TradeResource::Cheese,
            TradeResource::Lumber,
            TradeResource::Tools,
            TradeResource::Ammunition,
            TradeResource::Firearms,
            TradeResource::Fabric,
            TradeResource::Wool,
            TradeResource::Clothing,
            TradeResource::Iron,
            TradeResource::Copper,
            TradeResource::Nickle,
            TradeResource::Gold,
        ];
        for resource in resources {
            info!(
                "The price of {:?} is ${:.2} per {}",
                resource,
                resource.unit_value(),
                resource.units()
            );
        }
    });
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
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
            Sugar | Salt | SaltedMeat | Potatos | Flour | Grain => "pound",
            Cheese => "wheel",
            Spirits | Wine => "cask",
            Ammunition => "round",
            Firearms | Tools | Clothing => "piece",
            Wool => "skein",
            Iron | Copper | Nickle | Gold => "ingot",
            Lumber => "board-foot", // See: https://ekvintagewood.com/about-ek-vintage-reclaimed-wood/units-of-measure-glossary/
            Fabric => "yard",
        }
    }

    pub fn unit_value(&self) -> f32 {
        use TradeResource::*;

        let dollars_2024 = match self {
            Grain => 1.10 / 5.0,
            Flour => 2.75 / 5.0,
            Spirits => 10.50,
            Wine => 14.00,
            Sugar => 3.0 / 2.0,
            Salt => 1.75 / 1.625,
            SaltedMeat => 4.595,
            Potatos => 0.935,
            Cheese => 5.731 * 55.0,       // per pound * 55 pounds per wheel
            Lumber => 15.55 / 4.,         // Price of home depot 1x12x4ft board
            Tools => 24.97 / 3.,          // Price of home depot 3 piece wrench set
            Ammunition => 409.99 / 1000., // Price of 1000 rounds of .45 ACP
            Firearms => 600.0,            // My estimate for price of a midrange handgun.
            Fabric => 6.0,                // Estimate of price of canvas fabric.
            Wool => 9.0,                  // Estimate of price of skein of yarn.
            Clothing => 30.0,             // Estimate of price of a shirt.
            Iron => 0.23,                 // price per 1000cm^3
            Copper => 80.,                // price per 1000cm^3
            Nickle => 203.,               // price per 1000cm^3
            Gold => 1.704e6,              // price per 1000cm^3
        };

        // See: https://www.officialdata.org/us/inflation/2024?endYear=1849&amount=100
        dollars_2024 * 2.44 / 100.0
    }
}

#[derive(Component)]
pub struct Quantity(pub u32);

#[derive(Component)]
pub enum Quality {
    Low,
    High,
}
