use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::differential::broadside_coupled::{self, BroadsideCoupledInput};
use pcb_toolkit::differential::edge_coupled_embedded::{self, EdgeCoupledEmbeddedInput};
use pcb_toolkit::differential::edge_coupled_external::{self, EdgeCoupledExternalInput};
use pcb_toolkit::differential::edge_coupled_internal_asym::{self, EdgeCoupledInternalAsymInput};
use pcb_toolkit::differential::edge_coupled_internal_sym::{self, EdgeCoupledInternalSymInput};
use pcb_toolkit::differential::types::DifferentialResult;
use pcb_toolkit::model::ModelInfo;
use pcb_toolkit::units::Length;

use crate::output;

#[derive(Args)]
pub struct DifferentialArgs {
    #[command(subcommand)]
    pub topology: DifferentialTopology,
}

#[derive(Subcommand)]
pub enum DifferentialTopology {
    /// Edge-coupled external (surface microstrip) pair. IPC-2141A / Saturn-compatible.
    EdgeCoupledExternal {
        #[arg(short, long)]
        width: Length,
        /// Edge-to-edge gap between the traces [mil, mm, in, um].
        #[arg(short, long)]
        spacing: Length,
        #[arg(long)]
        height: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
    },
    /// Edge-coupled internal symmetric (centered stripline) pair. Cohn 1955.
    EdgeCoupledInternalSym {
        #[arg(short, long)]
        width: Length,
        #[arg(short, long)]
        spacing: Length,
        /// Gap from each face of the trace to its ground plane [mil, mm, in, um].
        #[arg(long)]
        height: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
    },
    /// Edge-coupled internal asymmetric (offset stripline) pair.
    EdgeCoupledInternalAsym {
        #[arg(short, long)]
        width: Length,
        #[arg(short, long)]
        spacing: Length,
        /// Gap from the trace face to the top ground plane [mil, mm, in, um].
        #[arg(long)]
        height1: Length,
        /// Gap from the trace face to the bottom ground plane [mil, mm, in, um].
        #[arg(long)]
        height2: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
    },
    /// Edge-coupled embedded (buried microstrip) pair.
    EdgeCoupledEmbedded {
        #[arg(short, long)]
        width: Length,
        #[arg(short, long)]
        spacing: Length,
        #[arg(long)]
        height: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
        #[arg(long)]
        cover_height: Length,
    },
    /// Broadside-coupled (vertically stacked) pair between two ground planes.
    BroadsideCoupled {
        #[arg(short, long)]
        width: Length,
        /// Dielectric gap between the facing surfaces of the two strips [mil, mm, in, um].
        #[arg(long)]
        separation: Length,
        /// Ground-to-ground spacing [mil, mm, in, um].
        #[arg(long)]
        height_total: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
        /// Shielded (between two ground planes). Required: the unshielded
        /// configuration has no implemented model and is rejected.
        #[arg(long)]
        shielded: bool,
    },
}

fn print(title: &str, result: &DifferentialResult, model: &ModelInfo, json: bool) -> Result<()> {
    if json {
        output::print_result(result, true)?;
    } else {
        println!("{title}");
        println!("{}", "─".repeat(title.chars().count()));
        println!("  Zdiff    = {:.4} Ω", result.zdiff);
        println!("  Zo       = {:.4} Ω", result.zo);
        println!("  Zodd     = {:.4} Ω", result.zodd);
        println!("  Zeven    = {:.4} Ω", result.zeven);
        println!("  Kb       = {:.6}", result.kb);
        match result.kb_db {
            Some(db) => println!("  Kb       = {db:.4} dB"),
            None => println!("  Kb       = -inf dB (no coupling)"),
        }
        println!("  Kb_term  = {:.6}", result.kb_term);
        match result.kb_term_db {
            Some(db) => println!("  Kb_term  = {db:.4} dB"),
            None => println!("  Kb_term  = -inf dB (no coupling)"),
        }
        output::print_model(model);
    }
    Ok(())
}

pub fn run(args: &DifferentialArgs, json: bool) -> Result<()> {
    match &args.topology {
        DifferentialTopology::EdgeCoupledExternal {
            width,
            spacing,
            height,
            thickness,
            er,
        } => {
            let result = edge_coupled_external::calculate(&EdgeCoupledExternalInput {
                width: width.mils(),
                spacing: spacing.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("edge-coupled external calculation failed")?;
            print(
                "Edge-Coupled External Differential",
                &result,
                &edge_coupled_external::MODEL,
                json,
            )
        }
        DifferentialTopology::EdgeCoupledInternalSym {
            width,
            spacing,
            height,
            thickness,
            er,
        } => {
            let result = edge_coupled_internal_sym::calculate(&EdgeCoupledInternalSymInput {
                width: width.mils(),
                spacing: spacing.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("edge-coupled internal symmetric calculation failed")?;
            print(
                "Edge-Coupled Internal Symmetric Differential",
                &result,
                &edge_coupled_internal_sym::MODEL,
                json,
            )
        }
        DifferentialTopology::EdgeCoupledInternalAsym {
            width,
            spacing,
            height1,
            height2,
            thickness,
            er,
        } => {
            let result = edge_coupled_internal_asym::calculate(&EdgeCoupledInternalAsymInput {
                width: width.mils(),
                spacing: spacing.mils(),
                height1: height1.mils(),
                height2: height2.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("edge-coupled internal asymmetric calculation failed")?;
            print(
                "Edge-Coupled Internal Asymmetric Differential",
                &result,
                &edge_coupled_internal_asym::MODEL,
                json,
            )
        }
        DifferentialTopology::EdgeCoupledEmbedded {
            width,
            spacing,
            height,
            thickness,
            er,
            cover_height,
        } => {
            let result = edge_coupled_embedded::calculate(&EdgeCoupledEmbeddedInput {
                width: width.mils(),
                spacing: spacing.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
                cover_height: cover_height.mils(),
            })
            .context("edge-coupled embedded calculation failed")?;
            print(
                "Edge-Coupled Embedded Differential",
                &result,
                &edge_coupled_embedded::MODEL,
                json,
            )
        }
        DifferentialTopology::BroadsideCoupled {
            width,
            separation,
            height_total,
            thickness,
            er,
            shielded,
        } => {
            let result = broadside_coupled::calculate(&BroadsideCoupledInput {
                width: width.mils(),
                separation: separation.mils(),
                height_total: height_total.mils(),
                thickness: thickness.mils(),
                er: *er,
                shielded: *shielded,
            })
            .context("broadside-coupled calculation failed")?;
            print(
                "Broadside-Coupled Differential",
                &result,
                &broadside_coupled::MODEL,
                json,
            )
        }
    }
}
