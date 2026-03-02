use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::pdn::{self, PdnInput};

use crate::output;

#[derive(Args)]
pub struct PdnArgs {
    /// Supply voltage (V).
    #[arg(long)]
    voltage: f64,

    /// Maximum current draw (A).
    #[arg(long)]
    current: f64,

    /// Load step as percentage of max current (%).
    #[arg(long)]
    i_step: f64,

    /// Allowed voltage ripple as percentage of supply (%).
    #[arg(long)]
    v_ripple: f64,

    /// Plane area (square inches).
    #[arg(long)]
    area: f64,

    /// Dielectric relative permittivity.
    #[arg(long)]
    er: f64,

    /// Dielectric thickness (mils).
    #[arg(long)]
    distance: f64,

    /// Frequency (MHz). Default: 0.
    #[arg(long, default_value = "0")]
    freq: f64,
}

pub fn run(args: &PdnArgs, json: bool) -> Result<()> {
    let result = pdn::calculate(&PdnInput {
        v_supply: args.voltage,
        i_max: args.current,
        i_step_pct: args.i_step,
        v_ripple_pct: args.v_ripple,
        area_sq_in: args.area,
        er: args.er,
        d_mils: args.distance,
        freq_mhz: args.freq,
    })
    .context("PDN impedance calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("PDN Impedance");
        println!("─────────────");
        println!("  Z target   = {:.6} Ω", result.z_target_ohms);
        println!("  C plane    = {:.4} pF", result.c_plane_pf);
        if let Some(xc) = result.xc_ohms {
            println!("  Xc         = {:.6} Ω", xc);
        }
    }
    Ok(())
}
