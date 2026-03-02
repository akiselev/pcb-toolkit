use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::impedance::microstrip;
use pcb_toolkit::units::{Freq, Length};

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
        /// Conductor width [mil, mm, in, um]. Default unit: mil.
        #[arg(short, long)]
        width: Length,

        /// Dielectric height [mil, mm, in, um]. Default unit: mil.
        #[arg(long)]
        height: Length,

        /// Conductor thickness [mil, mm, in, um]. Default: 1.4mil (1oz copper).
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,

        /// Substrate relative permittivity. Default: 4.6 (FR-4).
        #[arg(long, default_value = "4.6")]
        er: f64,

        /// Frequency [Hz, kHz, MHz, GHz]. Default: 0 (static).
        #[arg(short, long, default_value = "0")]
        freq: Freq,
    },
}

pub fn run(args: &ImpedanceArgs, json: bool) -> Result<()> {
    match &args.topology {
        ImpedanceTopology::Microstrip { width, height, thickness, er, freq } => {
            let result = microstrip::calculate(&microstrip::MicrostripInput {
                width: width.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
                frequency: freq.hz(),
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
