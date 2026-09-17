use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::copper::EtchFactor;
use pcb_toolkit::current::{self, CurrentInput};
use pcb_toolkit::units::{Freq, Length, Temperature};

use crate::output;

/// Conductor current capacity per IPC-2221A.
#[derive(Args)]
pub struct CurrentArgs {
    /// Trace width [mil, mm, in, um].
    #[arg(short, long)]
    width: Length,

    /// Copper thickness [mil, mm, in, um]. Default: 1.4mil (1oz copper).
    #[arg(short, long, default_value = "1.4mil")]
    thickness: Length,

    /// Trace length [mil, mm, in, um].
    #[arg(short, long)]
    length: Length,

    /// Allowed temperature rise above ambient (°C).
    #[arg(long, default_value = "10")]
    temp_rise: f64,

    /// Ambient (board) temperature [C, F]. Default unit: °C. Sets the
    /// temperature of the reported DC resistance.
    #[arg(long, default_value = "25")]
    ambient: Temperature,

    /// Frequency [Hz, kHz, MHz, GHz]. Default: 0 (DC only).
    #[arg(short, long, default_value = "0")]
    freq: Freq,

    /// Etch factor: none, 1:1, or 2:1.
    #[arg(short, long, default_value = "none")]
    etch: String,

    /// Trace is on an internal layer.
    #[arg(long)]
    internal: bool,
}

pub fn run(args: &CurrentArgs, json: bool) -> Result<()> {
    let etch_factor = match args.etch.as_str() {
        "none" => EtchFactor::None,
        "1:1" => EtchFactor::OneToOne,
        "2:1" => EtchFactor::TwoToOne,
        other => anyhow::bail!("unknown etch factor '{}': use none, 1:1, or 2:1", other),
    };

    let result = current::calculate(&CurrentInput {
        width: args.width.mils(),
        thickness: args.thickness.mils(),
        length: args.length.mils(),
        temperature_rise: args.temp_rise,
        ambient_temp: args.ambient.celsius(),
        frequency: args.freq.hz(),
        etch_factor,
        is_internal: args.internal,
    })
    .context("current capacity calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Conductor Current Capacity (IPC-2221A)");
        println!("────────────────────────────────────────");
        println!("  Current capacity     = {:.4} A", result.current_capacity);
        println!("  Cross section        = {:.4} mil²", result.cross_section);
        println!(
            "  DC resistance        = {:.6} Ω  (at {:.1} °C ambient)",
            result.resistance_dc,
            args.ambient.celsius()
        );
        println!(
            "  DC resistance (hot)  = {:.6} Ω  (at ambient + {:.1} °C rise)",
            result.resistance_dc_at_rise, args.temp_rise
        );
        println!("  Voltage drop         = {:.6} V", result.voltage_drop);
        println!("  Power dissipation    = {:.6} W", result.power_dissipation);
        println!(
            "  Current density      = {:.6} A/mil²",
            result.current_density
        );
        println!("  Skin depth           = {:.4} mil", result.skin_depth_mils);
        output::print_model(&current::MODEL);
    }
    Ok(())
}
