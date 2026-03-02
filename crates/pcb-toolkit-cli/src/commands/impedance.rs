use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::impedance::microstrip;

use crate::output;

#[derive(Args)]
pub struct ImpedanceArgs {
    #[command(subcommand)]
    pub topology: ImpedanceTopology,
}

#[derive(Subcommand)]
pub enum ImpedanceTopology {
    /// Microstrip (trace over ground plane).
    Microstrip {
        /// Conductor width (mils).
        #[arg(short, long)]
        width: f64,

        /// Dielectric height (mils).
        #[arg(long)]
        height: f64,

        /// Conductor thickness (mils). Default: 1.4 (1oz copper).
        #[arg(short, long, default_value = "1.4")]
        thickness: f64,

        /// Substrate relative permittivity. Default: 4.6 (FR-4).
        #[arg(long, default_value = "4.6")]
        er: f64,

        /// Frequency in MHz for dispersion correction. Default: 0 (static).
        #[arg(short, long, default_value = "0")]
        freq_mhz: f64,
    },
}

pub fn run(args: &ImpedanceArgs, json: bool) -> Result<()> {
    match &args.topology {
        ImpedanceTopology::Microstrip { width, height, thickness, er, freq_mhz } => {
            let result = microstrip::calculate(&microstrip::MicrostripInput {
                width: *width,
                height: *height,
                thickness: *thickness,
                er: *er,
                frequency: freq_mhz * 1e6,
            })
            .context("microstrip calculation failed")?;

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Microstrip Impedance");
                println!("────────────────────");
                println!("  Zo      = {:.4} Ω", result.zo);
                println!("  Er_eff  = {:.4}", result.er_eff);
                println!("  Tpd     = {:.4} ps/in", result.tpd_ps_per_in);
                println!("  Lo      = {:.4} nH/in", result.lo_nh_per_in);
                println!("  Co      = {:.4} pF/in", result.co_pf_per_in);
            }
            Ok(())
        }
    }
}
