use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::impedance::{coplanar, embedded, microstrip, stripline, types::ImpedanceResult};
use pcb_toolkit::model::ModelInfo;
use pcb_toolkit::units::{Freq, Length};

use crate::output;

#[derive(Args)]
pub struct ImpedanceArgs {
    #[command(subcommand)]
    pub topology: ImpedanceTopology,
}

#[derive(Subcommand)]
pub enum ImpedanceTopology {
    /// Microstrip (trace over ground plane). Hammerstad-Jensen + Kirschning-Jansen dispersion.
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

        /// Frequency [Hz, kHz, MHz, GHz]. Default: 0 (quasi-static). Non-zero
        /// applies Kirschning-Jansen dispersion (valid for H/λ₀ ≤ 0.13).
        #[arg(short, long, default_value = "0")]
        freq: Freq,
    },
    /// Stripline (trace between two ground planes). Cohn conformal mapping.
    Stripline {
        #[arg(short, long)]
        width: Length,
        /// Gap from each face of the trace to its ground plane [mil, mm, in, um].
        #[arg(long)]
        height: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
    },
    /// Embedded microstrip (covered trace over ground plane). Quasi-static only.
    Embedded {
        #[arg(short, long)]
        width: Length,
        #[arg(long)]
        height: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
        /// Cover height above trace [mil, mm, in, um].
        #[arg(long)]
        cover_height: Length,
    },
    /// Conductor-backed coplanar waveguide (trace with coplanar grounds over a ground plane).
    Coplanar {
        #[arg(short, long)]
        width: Length,
        /// Gap between trace and each coplanar ground [mil, mm, in, um].
        #[arg(short, long)]
        gap: Length,
        /// Substrate height to the bottom ground plane [mil, mm, in, um].
        #[arg(long)]
        height: Length,
        /// Conductor thickness. Pass 0 for the zero-thickness model.
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
    },
}

fn print(title: &str, result: &ImpedanceResult, model: &ModelInfo, json: bool) -> Result<()> {
    if json {
        output::print_result(result, true)?;
    } else {
        println!("{title}");
        println!("{}", "─".repeat(title.chars().count()));
        println!("  Zo      = {:.4} Ω", result.zo);
        println!("  Er_eff  = {:.4}", result.er_eff);
        println!("  Tpd     = {:.4} ps/in", result.tpd_ps_per_in);
        println!("  Lo      = {:.4} nH/in", result.lo_nh_per_in);
        println!("  Co      = {:.4} pF/in", result.co_pf_per_in);
        output::print_model(model);
    }
    Ok(())
}

pub fn run(args: &ImpedanceArgs, json: bool) -> Result<()> {
    match &args.topology {
        ImpedanceTopology::Microstrip {
            width,
            height,
            thickness,
            er,
            freq,
        } => {
            let result = microstrip::calculate(&microstrip::MicrostripInput {
                width: width.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
                frequency: freq.hz(),
            })
            .context("microstrip calculation failed")?;
            print("Microstrip Impedance", &result, &microstrip::MODEL, json)
        }
        ImpedanceTopology::Stripline {
            width,
            height,
            thickness,
            er,
        } => {
            let result = stripline::calculate(&stripline::StriplineInput {
                width: width.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("stripline calculation failed")?;
            print("Stripline Impedance", &result, &stripline::MODEL, json)
        }
        ImpedanceTopology::Embedded {
            width,
            height,
            thickness,
            er,
            cover_height,
        } => {
            let result = embedded::calculate(&embedded::EmbeddedMicrostripInput {
                width: width.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
                cover_height: cover_height.mils(),
                frequency: 0.0,
            })
            .context("embedded microstrip calculation failed")?;
            print(
                "Embedded Microstrip Impedance",
                &result,
                &embedded::MODEL,
                json,
            )
        }
        ImpedanceTopology::Coplanar {
            width,
            gap,
            height,
            thickness,
            er,
        } => {
            let result = coplanar::calculate(&coplanar::CoplanarInput {
                width: width.mils(),
                gap: gap.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("coplanar waveguide calculation failed")?;
            print(
                "Conductor-Backed Coplanar Waveguide Impedance",
                &result,
                &coplanar::MODEL,
                json,
            )
        }
    }
}
