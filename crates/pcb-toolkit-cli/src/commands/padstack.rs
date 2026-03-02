use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::padstack::{self, ThruHoleInput};
use pcb_toolkit::units::Length;

use crate::output;

#[derive(Args)]
pub struct PadstackArgs {
    #[command(subcommand)]
    pub sub: PadstackSub,
}

#[derive(Subcommand)]
pub enum PadstackSub {
    /// Plated thru-hole pad dimensions.
    ThruHole {
        /// Drilled hole diameter [mil, mm, in, um].
        #[arg(long)]
        hole: Length,
        /// Annular ring width [mil, mm, in, um].
        #[arg(long)]
        ring: Length,
        /// Isolation width (clearance) [mil, mm, in, um].
        #[arg(long)]
        isolation: Length,
    },

    /// Corner-to-corner (diagonal) distance.
    CornerToCorner {
        /// Horizontal span [mil, mm, in, um].
        #[arg(long)]
        a: Length,
        /// Vertical span [mil, mm, in, um].
        #[arg(long)]
        b: Length,
    },
}

pub fn run(args: &PadstackArgs, json: bool) -> Result<()> {
    match &args.sub {
        PadstackSub::ThruHole { hole, ring, isolation } => {
            let result = padstack::thru_hole(&ThruHoleInput {
                hole_diameter_mils: hole.mils(),
                annular_ring_mils: ring.mils(),
                isolation_width_mils: isolation.mils(),
            })
            .context("thru-hole pad calculation failed")?;

            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Thru-Hole Pad");
                println!("─────────────");
                println!("  External layers        = {:.4} mil", result.pad_external_mils);
                println!("  Internal signal layers = {:.4} mil", result.pad_internal_signal_mils);
                println!("  Internal plane layers  = {:.4} mil", result.pad_internal_plane_mils);
            }
        }

        PadstackSub::CornerToCorner { a, b } => {
            let diagonal = padstack::corner_to_corner(a.mils(), b.mils())
                .context("corner-to-corner calculation failed")?;

            if json {
                let result = serde_json::json!({ "diagonal_mils": diagonal });
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("Corner to Corner");
                println!("────────────────");
                println!("  Diagonal = {:.4} mil", diagonal);
            }
        }
    }
    Ok(())
}
