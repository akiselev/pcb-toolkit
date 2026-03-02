use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::crosstalk::{self, CrosstalkInput};
use pcb_toolkit::units::Length;

use crate::output;

#[derive(Args)]
pub struct CrosstalkArgs {
    #[arg(long)]
    pub rise_time: f64,
    #[arg(long)]
    pub voltage: f64,
    #[arg(short, long)]
    pub length: Length,
    #[arg(short, long)]
    pub spacing: Length,
    #[arg(long)]
    pub height: Length,
    #[arg(long, default_value = "4.6")]
    pub er: f64,
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
        println!("Crosstalk (NEXT)");
        println!("────────────────");
        println!("  Kb       = {:.6}", result.kb);
        println!("  Xtalk    = {:.4} dB", result.crosstalk_db);
        println!("  V_couple = {:.4} V", result.coupled_voltage);
        println!("  NEXT     = {:.6}", result.next_coefficient);
        println!("  Lsat     = {:.4} mil", result.lsat_mils);
    }
    Ok(())
}
