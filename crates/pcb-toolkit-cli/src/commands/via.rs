use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::units::Length;
use pcb_toolkit::via::{self, ViaInput};

use crate::output;

#[derive(Args)]
pub struct ViaArgs {
    #[arg(long)]
    pub hole: Length,
    #[arg(long)]
    pub pad: Length,
    #[arg(long)]
    pub antipad: Length,
    #[arg(long)]
    pub height: Length,
    #[arg(long, default_value = "0.7mil")]
    pub plating: Length,
    #[arg(long, default_value = "4.6")]
    pub er: f64,
}

pub fn run(args: &ViaArgs, json: bool) -> Result<()> {
    let result = via::calculate(&ViaInput {
        hole_diameter_mils: args.hole.mils(),
        pad_diameter_mils: args.pad.mils(),
        antipad_diameter_mils: args.antipad.mils(),
        height_mils: args.height.mils(),
        plating_thickness_mils: args.plating.mils(),
        er: args.er,
    })
    .context("via calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Via Properties");
        println!("──────────────");
        println!("  C_via   = {:.4} pF", result.capacitance_pf);
        println!("  L_via   = {:.4} nH", result.inductance_nh);
        println!("  Z_via   = {:.4} Ω", result.impedance_ohms);
        println!("  f_res   = {:.4} MHz", result.resonant_freq_mhz);
    }
    Ok(())
}
