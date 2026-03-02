use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::differential::broadside_coupled::{self, BroadsideCoupledInput};
use pcb_toolkit::differential::edge_coupled_embedded::{self, EdgeCoupledEmbeddedInput};
use pcb_toolkit::differential::edge_coupled_external::{self, EdgeCoupledExternalInput};
use pcb_toolkit::differential::edge_coupled_internal_asym::{self, EdgeCoupledInternalAsymInput};
use pcb_toolkit::differential::edge_coupled_internal_sym::{self, EdgeCoupledInternalSymInput};
use pcb_toolkit::differential::types::DifferentialResult;
use pcb_toolkit::units::Length;

use crate::output;

#[derive(Args)]
pub struct DifferentialArgs {
    #[command(subcommand)]
    pub topology: DifferentialTopology,
}

#[derive(Subcommand)]
pub enum DifferentialTopology {
    /// Edge-coupled external (surface microstrip) differential pair.
    EdgeCoupledExternal {
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
    },
    /// Edge-coupled internal symmetric (centered stripline) differential pair.
    EdgeCoupledInternalSym {
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
    },
    /// Edge-coupled internal asymmetric (offset stripline) differential pair.
    EdgeCoupledInternalAsym {
        #[arg(short, long)]
        width: Length,
        #[arg(short, long)]
        spacing: Length,
        #[arg(long)]
        height1: Length,
        #[arg(long)]
        height2: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
    },
    /// Edge-coupled embedded (buried microstrip) differential pair.
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
    /// Broadside-coupled differential pair.
    BroadsideCoupled {
        #[arg(short, long)]
        width: Length,
        #[arg(long)]
        separation: Length,
        #[arg(long)]
        height_total: Length,
        #[arg(short, long, default_value = "1.4mil")]
        thickness: Length,
        #[arg(long, default_value = "4.6")]
        er: f64,
        #[arg(long)]
        shielded: bool,
    },
}

fn print_diff_result(result: &DifferentialResult) {
    println!("  Zdiff    = {:.4} Ω", result.zdiff);
    println!("  Zo       = {:.4} Ω", result.zo);
    println!("  Zodd     = {:.4} Ω", result.zodd);
    println!("  Zeven    = {:.4} Ω", result.zeven);
    println!("  Kb       = {:.6}", result.kb);
    println!("  Kb       = {:.4} dB", result.kb_db);
    println!("  Kb_term  = {:.6}", result.kb_term);
    println!("  Kb_term  = {:.4} dB", result.kb_term_db);
}

pub fn run(args: &DifferentialArgs, json: bool) -> Result<()> {
    match &args.topology {
        DifferentialTopology::EdgeCoupledExternal { width, spacing, height, thickness, er } => {
            let result = edge_coupled_external::calculate(&EdgeCoupledExternalInput {
                width: width.mils(),
                spacing: spacing.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("edge-coupled external calculation failed")?;

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Edge-Coupled External Differential");
                println!("───────────────────────────────────");
                print_diff_result(&result);
            }
            Ok(())
        }
        DifferentialTopology::EdgeCoupledInternalSym { width, spacing, height, thickness, er } => {
            let result = edge_coupled_internal_sym::calculate(&EdgeCoupledInternalSymInput {
                width: width.mils(),
                spacing: spacing.mils(),
                height: height.mils(),
                thickness: thickness.mils(),
                er: *er,
            })
            .context("edge-coupled internal symmetric calculation failed")?;

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Edge-Coupled Internal Symmetric Differential");
                println!("─────────────────────────────────────────────");
                print_diff_result(&result);
            }
            Ok(())
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

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Edge-Coupled Internal Asymmetric Differential");
                println!("──────────────────────────────────────────────");
                print_diff_result(&result);
            }
            Ok(())
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

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Edge-Coupled Embedded Differential");
                println!("───────────────────────────────────");
                print_diff_result(&result);
            }
            Ok(())
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

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Broadside-Coupled Differential");
                println!("──────────────────────────────");
                print_diff_result(&result);
            }
            Ok(())
        }
    }
}
