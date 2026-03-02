use anyhow::{bail, Context, Result};
use clap::Args;

use pcb_toolkit::inductor::{self, SpiralShape};
use pcb_toolkit::units::Length;

use crate::output;

#[derive(Args)]
pub struct InductorArgs {
    #[arg(long)]
    pub dout: Length,
    #[arg(long)]
    pub turns: u32,
    #[arg(short, long)]
    pub width: Length,
    #[arg(short, long)]
    pub spacing: Length,
    #[arg(long, default_value = "square")]
    pub shape: String,
}

fn parse_shape(s: &str) -> Result<SpiralShape> {
    match s.to_lowercase().as_str() {
        "square"     => Ok(SpiralShape::Square),
        "hexagonal"  => Ok(SpiralShape::Hexagonal),
        "octagonal"  => Ok(SpiralShape::Octagonal),
        "circle" | "circular" => Ok(SpiralShape::Circle),
        _ => bail!(
            "unknown shape '{}' — valid values: square, hexagonal, octagonal, circle",
            s
        ),
    }
}

pub fn run(args: &InductorArgs, json: bool) -> Result<()> {
    let shape = parse_shape(&args.shape)?;
    let result = inductor::planar_spiral(
        args.turns,
        args.width.mils(),
        args.spacing.mils(),
        args.dout.mils(),
        shape,
    )
    .context("inductor calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Planar Spiral Inductor");
        println!("──────────────────────");
        println!("  din      = {:.4} mil", result.din_mils);
        println!("  rho      = {:.6}", result.rho);
        println!("  d_avg    = {:.4} mil", result.d_avg_mils);
        println!("  L        = {:.4} nH", result.inductance_nh);
    }
    Ok(())
}
