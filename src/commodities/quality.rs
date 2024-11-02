use bevy::prelude::*;

use crate::commodities::display_unit_price;

use super::{Commodity, Quantity};

pub(super) fn plugin(app: &mut App) {
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

    app.add_systems(Startup, dbg_print_prices_of_different_quality_commodities);
}

#[derive(Component, enum_iterator::Sequence, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub enum Quality {
    Low,
    High,
}

impl Quality {
    #[allow(unused)]
    pub fn price_modifier(&self, comm: &Commodity) -> f32 {
        use Commodity::*;

        enum HighQualityClass {
            OneOfAKind,
            LuxuryGood,
            QualityIsDurability,
            ExtremelyStableValue,
            Artisinal,
        }

        use HighQualityClass::*;

        let high_quality_class = match comm {
            SaltedMeat | Spirits | Wine => LuxuryGood,
            Tools => QualityIsDurability,
            Clothing | Firearms => OneOfAKind,
            Cheese | Fabric => Artisinal,
            Ammunition | Wool | Grain | Sugar | Salt | Flour | Potatos | Lumber | Iron | Copper
            | Nickel | Gold | Coal => ExtremelyStableValue,
        };

        enum LowQualityClass {
            NonDegradable,
            Damageable,
            MassProduced,
            Perishable,
        }

        use LowQualityClass::*;

        let low_quality_class = match comm {
            Copper | Nickel | Gold | Coal | Ammunition | Grain | Flour | Sugar | Salt | Wool => {
                NonDegradable
            }
            SaltedMeat | Cheese | Potatos => Perishable,
            Spirits | Wine | Tools | Firearms | Clothing | Fabric => MassProduced,
            Lumber | Iron => Damageable,
        };

        match self {
            Quality::Low => match low_quality_class {
                NonDegradable => 0.95,
                Damageable => 0.80,
                MassProduced => 0.60,
                Perishable => 0.30,
            },
            Quality::High => match high_quality_class {
                OneOfAKind => 3.5,
                LuxuryGood => 2.0,
                QualityIsDurability => 1.75,
                ExtremelyStableValue => 1.05,
                Artisinal => 1.35,
            },
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

#[allow(unused)]
pub fn quality_modified_base_unit_price(commodity: &Commodity, quality: &Quality) -> f32 {
    commodity.base_unit_price() * quality.price_modifier(commodity)
}

pub fn dbg_print_prices_of_different_quality_commodities() {
    for comm in enum_iterator::all::<Commodity>() {
        use std::fmt::Write;
        let mut msg = format!("QUALITY: {:?}", comm);
        for quality in [Some(Quality::Low), None, Some(Quality::High)].iter() {
            if let Some(quality) = quality {
                let unit_price = display_unit_price(
                    quality_modified_base_unit_price(&comm, quality),
                    comm.units(),
                );
                write!(&mut msg, " | {quality:?} -- {unit_price}").unwrap();
            } else {
                let unit_price = display_unit_price(comm.base_unit_price(), comm.units());
                write!(&mut msg, " | Average -- {unit_price}").unwrap();
            }
        }
        println!("{}", msg);
    }
}
