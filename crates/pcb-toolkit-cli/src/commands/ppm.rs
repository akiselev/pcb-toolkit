use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::ppm;
use pcb_toolkit::units::{Capacitance, Freq};

use crate::output;

#[derive(Args)]
pub struct PpmArgs {
    #[command(subcommand)]
    pub sub: PpmSub,
}

#[derive(Subcommand)]
pub enum PpmSub {
    /// Convert Hz deviation to PPM.
    HzToPpm {
        /// Center (nominal) frequency [Hz, kHz, MHz, GHz].
        #[arg(long)]
        center: Freq,
        /// Upper frequency limit [Hz, kHz, MHz, GHz].
        #[arg(long)]
        max: Freq,
    },

    /// Convert PPM to Hz deviation.
    PpmToHz {
        /// Center (nominal) frequency [Hz, kHz, MHz, GHz].
        #[arg(long)]
        center: Freq,
        /// Parts-per-million deviation (> 0).
        #[arg(long)]
        ppm: f64,
    },

    /// Crystal load capacitance.
    XtalLoad {
        /// Stray PCB capacitance [F, nF, pF, uF].
        #[arg(long)]
        stray: Capacitance,
        /// Load capacitor 1 [F, nF, pF, uF].
        #[arg(long)]
        c1: Capacitance,
        /// Load capacitor 2 [F, nF, pF, uF].
        #[arg(long)]
        c2: Capacitance,
    },
}

pub fn run(args: &PpmArgs, json: bool) -> Result<()> {
    match &args.sub {
        PpmSub::HzToPpm { center, max } => {
            let result = ppm::hz_to_ppm(center.hz(), max.hz())
                .context("Hz to PPM calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Hz to PPM");
                println!("─────────");
                println!("  Variation = {:.4} Hz", result.variation_hz);
                println!("  PPM       = {:.4}", result.ppm);
            }
        }

        PpmSub::PpmToHz { center, ppm } => {
            let result = ppm::ppm_to_hz(center.hz(), *ppm)
                .context("PPM to Hz calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("PPM to Hz");
                println!("─────────");
                println!("  Variation = {:.4} Hz", result.variation_hz);
                println!("  Max freq  = {:.4} Hz", result.max_hz);
                println!("  Min freq  = {:.4} Hz", result.min_hz);
            }
        }

        PpmSub::XtalLoad { stray, c1, c2 } => {
            let result = ppm::xtal_load(stray.farads(), c1.farads(), c2.farads())
                .context("XTAL load capacitance calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Crystal Load Capacitance");
                println!("────────────────────────");
                println!("  C load (calc)         = {:.4} pF", result.c_load_calc_f * 1e12);
                println!("  C load (rule of thumb)= {:.4} pF", result.c_load_rule_of_thumb_f * 1e12);
            }
        }
    }
    Ok(())
}
