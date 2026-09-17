use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::crosstalk::{self, CrosstalkInput};
use pcb_toolkit::units::Length;

use crate::output;

/// Backward crosstalk (NEXT) rule-of-thumb estimate. Experimental.
#[derive(Args)]
pub struct CrosstalkArgs {
    /// Signal rise time in nanoseconds.
    #[arg(long)]
    pub rise_time: f64,
    /// Aggressor signal swing (V).
    #[arg(long)]
    pub voltage: f64,
    /// Coupled (parallel) length [mil, mm, in, um].
    #[arg(short, long)]
    pub length: Length,
    /// Edge-to-edge spacing [mil, mm, in, um].
    #[arg(short, long)]
    pub spacing: Length,
    /// Dielectric height to the ground plane [mil, mm, in, um].
    #[arg(long)]
    pub height: Length,
    #[arg(long, default_value = "4.6")]
    pub er: f64,
    /// Trace width [mil, mm, in, um] (velocity estimate only).
    #[arg(short, long)]
    pub width: Length,
}

pub fn run(args: &CrosstalkArgs, json: bool) -> Result<()> {
    let result = crosstalk::calculate(&CrosstalkInput {
        rise_time_ns: args.rise_time,
        voltage: args.voltage,
        coupled_length_mils: args.length.mils(),
        spacing_mils: args.spacing.mils(),
        height_mils: args.height.mils(),
        er: args.er,
        trace_width_mils: args.width.mils(),
    })
    .context("crosstalk calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Crosstalk (NEXT) — estimate");
        println!("───────────────────────────");
        println!("  Kb       = {:.6}", result.kb);
        println!("  Xtalk    = {:.4} dB", result.crosstalk_db);
        println!("  V_couple = {:.4} V", result.coupled_voltage);
        println!("  NEXT     = {:.6}", result.next_coefficient);
        println!("  Lsat     = {:.4} mil", result.lsat_mils);
        output::print_model(&crosstalk::MODEL);
    }
    Ok(())
}
