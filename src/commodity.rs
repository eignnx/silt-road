use bevy::prelude::*;
use enum_iterator::Sequence;

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

    app.add_systems(Startup, || {
        let mut high_quality = Quantity(100);
        let mut average_quality = Quantity(100);
        let mut low_quality = Quantity(100);

        println!(
            "High: {}, Average: {}, Low: {}",
            high_quality.0, average_quality.0, low_quality.0
        );

        for _ in 0..10 {
            degrade_quality(
                &mut CommodityQualitySummary {
                    high_quality: &mut high_quality,
                    average_quality: &mut average_quality,
                    low_quality: &mut low_quality,
                },
                0.1,
            );
            println!(
                "High: {}, Average: {}, Low: {}",
                high_quality.0, average_quality.0, low_quality.0
            );
        }
    });
}

#[derive(Component, Debug, Clone, Copy, Reflect, Sequence)]
#[reflect(Component)]
pub enum Commodity {
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
            Iron | Copper | Nickle | Gold => "ingot",
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
            Nickle => 203.,             // price per 1000cm^3
            Gold => 1.704e6,            // price per 1000cm^3
        };

        // See: https://www.officialdata.org/us/inflation/2024?endYear=1849&amount=100
        dollars_2024 * 2.44 / 100.0
    }
}

pub fn display_unit_price(unit_price: f32, unit: &'static str) -> String {
    if unit_price < 0.10 {
        format!("${:.2} per 100 {unit}", unit_price * 100.0)
    } else {
        format!("${unit_price:.2} per {unit}")
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Quantity(pub u32);

#[derive(Component, Debug, Clone, Copy)]
pub enum Quality {
    Low,
    High,
}

impl Quality {
    #[allow(unused)]
    pub fn price_modifier(&self) -> f32 {
        match self {
            Quality::Low => 0.75,
            Quality::High => 1.25,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct CommodityQualitySummary<'a> {
    pub high_quality: &'a mut Quantity,
    pub average_quality: &'a mut Quantity,
    pub low_quality: &'a mut Quantity,
}

#[allow(unused)]
pub fn degrade_quality<'a: 'b, 'b>(
    quality_summary: &'b mut CommodityQualitySummary<'a>,
    degradation_pct: f32,
) {
    // Higher quality things degrade faster.
    let high_quality_degradation = degradation_pct * 1.25;
    let high_to_mid = (quality_summary.high_quality.0 as f32 * high_quality_degradation) as u32;
    let mid_to_low = (quality_summary.average_quality.0 as f32 * degradation_pct) as u32;

    quality_summary.high_quality.0 -= high_to_mid;
    quality_summary.average_quality.0 += high_to_mid;
    quality_summary.average_quality.0 -= mid_to_low;
    quality_summary.low_quality.0 += mid_to_low;
}
