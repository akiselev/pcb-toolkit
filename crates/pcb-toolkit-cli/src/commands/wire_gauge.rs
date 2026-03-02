use anyhow::{bail, Result};
use clap::Args;

use pcb_toolkit::wire_gauge::{self, Awg};

use crate::output;

#[derive(Args)]
pub struct WireGaugeArgs {
    #[arg(short, long)]
    pub awg: String,
}

fn parse_awg(s: &str) -> Result<Awg> {
    match s.to_lowercase().trim_start_matches("awg").trim() {
        "4/0" | "0000" => Ok(Awg::Awg4_0),
        "3/0" | "000"  => Ok(Awg::Awg3_0),
        "2/0" | "00"   => Ok(Awg::Awg2_0),
        "1/0" | "0"    => Ok(Awg::Awg1_0),
        "1"  => Ok(Awg::Awg1),
        "2"  => Ok(Awg::Awg2),
        "3"  => Ok(Awg::Awg3),
        "4"  => Ok(Awg::Awg4),
        "5"  => Ok(Awg::Awg5),
        "6"  => Ok(Awg::Awg6),
        "7"  => Ok(Awg::Awg7),
        "8"  => Ok(Awg::Awg8),
        "9"  => Ok(Awg::Awg9),
        "10" => Ok(Awg::Awg10),
        "11" => Ok(Awg::Awg11),
        "12" => Ok(Awg::Awg12),
        "13" => Ok(Awg::Awg13),
        "14" => Ok(Awg::Awg14),
        "15" => Ok(Awg::Awg15),
        "16" => Ok(Awg::Awg16),
        "17" => Ok(Awg::Awg17),
        "18" => Ok(Awg::Awg18),
        "19" => Ok(Awg::Awg19),
        "20" => Ok(Awg::Awg20),
        "21" => Ok(Awg::Awg21),
        "22" => Ok(Awg::Awg22),
        "23" => Ok(Awg::Awg23),
        "24" => Ok(Awg::Awg24),
        "25" => Ok(Awg::Awg25),
        "26" => Ok(Awg::Awg26),
        "27" => Ok(Awg::Awg27),
        "28" => Ok(Awg::Awg28),
        "29" => Ok(Awg::Awg29),
        "30" => Ok(Awg::Awg30),
        "31" => Ok(Awg::Awg31),
        "32" => Ok(Awg::Awg32),
        "33" => Ok(Awg::Awg33),
        "34" => Ok(Awg::Awg34),
        "35" => Ok(Awg::Awg35),
        "36" => Ok(Awg::Awg36),
        "37" => Ok(Awg::Awg37),
        "38" => Ok(Awg::Awg38),
        "39" => Ok(Awg::Awg39),
        "40" => Ok(Awg::Awg40),
        _ => bail!(
            "unknown AWG gauge '{}' — valid range: 4/0 (largest) through 40 (smallest)",
            s
        ),
    }
}

pub fn run(args: &WireGaugeArgs, json: bool) -> Result<()> {
    let awg = parse_awg(&args.awg)?;
    let result = wire_gauge::lookup(awg);

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Wire Gauge Properties");
        println!("─────────────────────");
        println!("  AWG          = {}", result.awg_label);
        println!("  Diameter     = {:.4} in", result.diameter_in);
        println!("  Diameter     = {:.4} mil", result.diameter_mils);
        println!("  Resistance   = {:.4} Ω/kft", result.resistance_ohm_per_kft);
        println!("  Area         = {:.4} cmil", result.area_circular_mils);
        println!("  Area(Saturn) = {:.4}", result.area_saturn);
    }
    Ok(())
}
