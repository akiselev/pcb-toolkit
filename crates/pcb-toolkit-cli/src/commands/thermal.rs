use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::thermal::{self, ThermalInput};

use crate::output;

#[derive(Args)]
pub struct ThermalArgs {
    /// Thermal resistance junction-to-ambient (°C/W).
    #[arg(long)]
    r_theta: f64,

    /// Power dissipation (W).
    #[arg(long)]
    power: f64,

    /// Ambient temperature (°C).
    #[arg(long, default_value = "25")]
    ambient: f64,
}

pub fn run(args: &ThermalArgs, json: bool) -> Result<()> {
    let result = thermal::calculate(&ThermalInput {
        r_theta_ja: args.r_theta,
        power_w: args.power,
        t_ambient_c: args.ambient,
    })
    .context("thermal calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Thermal Management");
        println!("──────────────────");
        println!("  T junction = {:.4} °C", result.t_junction_c);
        println!("  T junction = {:.4} °F", result.t_junction_f);
    }
    Ok(())
}
