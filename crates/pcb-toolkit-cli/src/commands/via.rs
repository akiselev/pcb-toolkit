use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::units::Length;
use pcb_toolkit::via::{self, ViaInput};

use crate::output;

/// Via lumped parasitics (C, L, √(L/C), LC frequency) and barrel resistance.
#[derive(Args)]
pub struct ViaArgs {
    /// Drilled hole diameter [mil, mm, in, um].
    #[arg(long)]
    pub hole: Length,
    /// Pad diameter [mil, mm, in, um].
    #[arg(long)]
    pub pad: Length,
    /// Antipad (plane clearance) diameter [mil, mm, in, um].
    #[arg(long)]
    pub antipad: Length,
    /// Barrel height / board thickness [mil, mm, in, um].
    #[arg(long)]
    pub height: Length,
    /// Barrel plating thickness [mil, mm, in, um]. Used for the DC resistance.
    #[arg(long, default_value = "0.7mil")]
    pub plating: Length,
    /// Board relative permittivity.
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
        println!("  √(L/C)  = {:.4} Ω", result.impedance_ohms);
        println!("  f_LC    = {:.4} MHz", result.resonant_freq_mhz);
        println!("  R_dc    = {:.4} mΩ", result.resistance_mohm);
        output::print_model(&via::MODEL);
    }
    Ok(())
}
