use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::pdn::{self, PdnInput};
use pcb_toolkit::units::{Freq, Length};

use crate::output;

/// PDN target impedance and plane capacitance (first-order budget).
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

    /// Plane area in square inches.
    #[arg(long = "area-sq-in")]
    area_sq_in: f64,

    /// Dielectric relative permittivity.
    #[arg(long)]
    er: f64,

    /// Dielectric thickness between the planes [mil, mm, in, um]. Default unit: mil.
    #[arg(long)]
    distance: Length,

    /// Frequency for the plane reactance [Hz, kHz, MHz, GHz]. Default: 0 (DC, no reactance).
    #[arg(long, default_value = "0")]
    freq: Freq,
}

pub fn run(args: &PdnArgs, json: bool) -> Result<()> {
    let result = pdn::calculate(&PdnInput {
        v_supply: args.voltage,
        i_max: args.current,
        i_step_pct: args.i_step,
        v_ripple_pct: args.v_ripple,
        area_sq_in: args.area_sq_in,
        er: args.er,
        d_mils: args.distance.mils(),
        freq_mhz: args.freq.hz() / 1e6,
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
        output::print_model(&pdn::MODEL);
    }
    Ok(())
}
