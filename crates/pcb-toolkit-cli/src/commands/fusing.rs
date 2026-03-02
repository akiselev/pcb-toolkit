use anyhow::{bail, Context, Result};
use clap::Args;

use pcb_toolkit::copper::{CopperWeight, EtchFactor, PlatingThickness};
use pcb_toolkit::fusing;
use pcb_toolkit::units::Length;

use crate::output;

#[derive(Args)]
pub struct FusingArgs {
    #[arg(short, long)]
    pub width: Length,
    #[arg(short, long)]
    pub copper: String,
    #[arg(short, long, default_value = "bare")]
    pub plating: String,
    #[arg(short, long, default_value = "none")]
    pub etch: String,
    #[arg(short, long)]
    pub time: f64,
    #[arg(short, long, default_value = "25")]
    pub ambient: f64,
}

fn parse_plating(s: &str) -> Result<PlatingThickness> {
    match s.to_lowercase().trim_end_matches("oz").trim() {
        "bare" | "0" => Ok(PlatingThickness::Bare),
        "0.5" => Ok(PlatingThickness::Oz05),
        "1"   => Ok(PlatingThickness::Oz1),
        "1.5" => Ok(PlatingThickness::Oz15),
        "2"   => Ok(PlatingThickness::Oz2),
        "2.5" => Ok(PlatingThickness::Oz25),
        "3"   => Ok(PlatingThickness::Oz3),
        _ => bail!(
            "unknown plating '{}' — valid values: bare, 0.5oz, 1oz, 1.5oz, 2oz, 2.5oz, 3oz",
            s
        ),
    }
}

fn parse_etch(s: &str) -> Result<EtchFactor> {
    match s.to_lowercase().as_str() {
        "none" | "0"   => Ok(EtchFactor::None),
        "1:1" | "1to1" | "onetone" => Ok(EtchFactor::OneToOne),
        "2:1" | "2to1" | "twotoone" => Ok(EtchFactor::TwoToOne),
        _ => bail!(
            "unknown etch factor '{}' — valid values: none, 1:1, 2:1",
            s
        ),
    }
}

pub fn run(args: &FusingArgs, json: bool) -> Result<()> {
    let copper_weight = CopperWeight::from_str_oz(&args.copper)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let plating = parse_plating(&args.plating)?;
    let etch_factor = parse_etch(&args.etch)?;

    let result = fusing::fusing_current_trace(
        args.width.mils(),
        copper_weight,
        plating,
        etch_factor,
        args.time,
        args.ambient,
    )
    .context("fusing calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Fusing Current");
        println!("──────────────");
        println!("  Cu thickness = {:.4} mil", result.copper_thickness_mils);
        println!("  Area         = {:.4} sq mil", result.area_sq_mils);
        println!("  Area         = {:.4} cmil", result.area_circular_mils);
        println!("  I_fuse       = {:.4} A", result.fusing_current_a);
        println!("  T_melt       = {:.4} °C", result.melting_temp_c);
    }
    Ok(())
}
