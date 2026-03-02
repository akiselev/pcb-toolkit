use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::impedance::{coplanar, embedded, microstrip, stripline};
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
    /// Stripline (trace between two ground planes).
    Stripline {
        #[arg(short, long)]
        width: Length,
        #[arg(long)]
        height: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
    },
    /// Embedded microstrip (covered trace over ground plane).
    Embedded {
        #[arg(short, long)]
        width: Length,
        #[arg(long)]
        height: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
        /// Cover height above trace.
        #[arg(long)]
        cover_height: Length,
        #[arg(short, long, default_value = "0")]
        freq: Freq,
    },
    /// Coplanar waveguide (trace with coplanar ground).
    Coplanar {
        #[arg(short, long)]
        width: Length,
        /// Gap between trace and coplanar ground.
        #[arg(short, long)]
        gap: Length,
        #[arg(long)]
        height: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
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
        ImpedanceTopology::Stripline { width, height, thickness, er } => {
            let result = stripline::calculate(&stripline::StriplineInput {
                width: width.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("stripline calculation failed")?;

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Stripline Impedance");
                println!("───────────────────");
                println!("  Zo      = {:.4} Ω", result.zo);
                println!("  Er_eff  = {:.4}", result.er_eff);
                println!("  Tpd     = {:.4} ps/in", result.tpd_ps_per_in);
                println!("  Lo      = {:.4} nH/in", result.lo_nh_per_in);
                println!("  Co      = {:.4} pF/in", result.co_pf_per_in);
            }
            Ok(())
        }
        ImpedanceTopology::Embedded { width, height, thickness, er, cover_height, freq } => {
            let result = embedded::calculate(&embedded::EmbeddedMicrostripInput {
                width: width.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
                cover_height: cover_height.mils(),
                frequency: freq.hz(),
            })
            .context("embedded microstrip calculation failed")?;

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Embedded Microstrip Impedance");
                println!("─────────────────────────────");
                println!("  Zo      = {:.4} Ω", result.zo);
                println!("  Er_eff  = {:.4}", result.er_eff);
                println!("  Tpd     = {:.4} ps/in", result.tpd_ps_per_in);
                println!("  Lo      = {:.4} nH/in", result.lo_nh_per_in);
                println!("  Co      = {:.4} pF/in", result.co_pf_per_in);
            }
            Ok(())
        }
        ImpedanceTopology::Coplanar { width, gap, height, thickness, er } => {
            let result = coplanar::calculate(&coplanar::CoplanarInput {
                width: width.mils(),
                gap: gap.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("coplanar waveguide calculation failed")?;

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Coplanar Waveguide Impedance");
                println!("────────────────────────────");
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
