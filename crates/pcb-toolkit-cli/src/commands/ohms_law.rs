use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::ohms_law;

use crate::output;

#[derive(Args)]
pub struct OhmsLawArgs {
    #[command(subcommand)]
    pub sub: OhmsLawSub,
}

#[derive(Subcommand)]
pub enum OhmsLawSub {
    /// Voltage, current, resistance, and power (V = IR).
    Eir {
        /// Voltage in Volts.
        #[arg(long)]
        voltage: Option<f64>,
        /// Current in Amperes.
        #[arg(long)]
        current: Option<f64>,
        /// Resistance in Ohms.
        #[arg(long)]
        resistance: Option<f64>,
    },

    /// LED bias resistor calculator.
    LedBias {
        /// Supply voltage (V).
        #[arg(long)]
        supply: f64,
        /// LED forward voltage (V).
        #[arg(long)]
        led_v: f64,
        /// LED operating current (A).
        #[arg(long)]
        led_current: f64,
    },

    /// Symmetric Pi-pad attenuator.
    PiPad {
        /// Attenuation in dB (> 0).
        #[arg(long)]
        attenuation: f64,
        /// System impedance in Ohms.
        #[arg(long)]
        impedance: f64,
    },

    /// Symmetric T-pad attenuator.
    TPad {
        /// Attenuation in dB (> 0).
        #[arg(long)]
        attenuation: f64,
        /// System impedance in Ohms.
        #[arg(long)]
        impedance: f64,
    },

    /// Resistors in series.
    ResistorsSeries {
        /// Resistor values in Ohms.
        #[arg(required = true, num_args = 1..)]
        values: Vec<f64>,
    },

    /// Resistors in parallel.
    ResistorsParallel {
        /// Resistor values in Ohms.
        #[arg(required = true, num_args = 1..)]
        values: Vec<f64>,
    },

    /// Capacitors in series.
    CapacitorsSeries {
        /// Capacitor values in Farads.
        #[arg(required = true, num_args = 1..)]
        values: Vec<f64>,
    },

    /// Capacitors in parallel.
    CapacitorsParallel {
        /// Capacitor values in Farads.
        #[arg(required = true, num_args = 1..)]
        values: Vec<f64>,
    },

    /// Inductors in series.
    InductorsSeries {
        /// Inductor values in Henries.
        #[arg(required = true, num_args = 1..)]
        values: Vec<f64>,
    },

    /// Inductors in parallel.
    InductorsParallel {
        /// Inductor values in Henries.
        #[arg(required = true, num_args = 1..)]
        values: Vec<f64>,
    },
}

pub fn run(args: &OhmsLawArgs, json: bool) -> Result<()> {
    match &args.sub {
        OhmsLawSub::Eir { voltage, current, resistance } => {
            let result = ohms_law::eir(*voltage, *current, *resistance)
                .context("E-I-R calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("E-I-R");
                println!("─────");
                println!("  Voltage    = {:.4} V", result.voltage_v);
                println!("  Current    = {:.4} A", result.current_a);
                println!("  Resistance = {:.4} Ω", result.resistance_ohm);
                println!("  Power      = {:.4} W", result.power_w);
            }
        }

        OhmsLawSub::LedBias { supply, led_v, led_current } => {
            let result = ohms_law::led_bias(*supply, *led_v, *led_current)
                .context("LED bias calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("LED Bias Resistor");
                println!("─────────────────");
                println!("  Resistance = {:.4} Ω", result.resistance_ohm);
                println!("  Power      = {:.4} W", result.power_w);
            }
        }

        OhmsLawSub::PiPad { attenuation, impedance } => {
            let result = ohms_law::pi_pad(*attenuation, *impedance)
                .context("Pi-pad calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Pi-Pad Attenuator");
                println!("─────────────────");
                println!("  Attenuation = {:.4} dB", result.attenuation_db);
                println!("  K           = {:.6}", result.k);
                println!("  R series    = {:.4} Ω", result.r_series_ohm);
                println!("  R shunt     = {:.4} Ω", result.r_shunt_ohm);
            }
        }

        OhmsLawSub::TPad { attenuation, impedance } => {
            let result = ohms_law::t_pad(*attenuation, *impedance)
                .context("T-pad calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("T-Pad Attenuator");
                println!("────────────────");
                println!("  Attenuation = {:.4} dB", result.attenuation_db);
                println!("  K           = {:.6}", result.k);
                println!("  R series    = {:.4} Ω", result.r_series_ohm);
                println!("  R shunt     = {:.4} Ω", result.r_shunt_ohm);
            }
        }

        OhmsLawSub::ResistorsSeries { values } => {
            let result = ohms_law::resistors_series(values)
                .context("resistors series calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Resistors in Series");
                println!("───────────────────");
                println!("  Total resistance = {:.4} Ω", result.resistance_ohm);
            }
        }

        OhmsLawSub::ResistorsParallel { values } => {
            let result = ohms_law::resistors_parallel(values)
                .context("resistors parallel calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Resistors in Parallel");
                println!("─────────────────────");
                println!("  Total resistance = {:.4} Ω", result.resistance_ohm);
            }
        }

        OhmsLawSub::CapacitorsSeries { values } => {
            let result = ohms_law::capacitors_series(values)
                .context("capacitors series calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Capacitors in Series");
                println!("────────────────────");
                println!("  Total capacitance = {:.6e} F", result.capacitance_f);
            }
        }

        OhmsLawSub::CapacitorsParallel { values } => {
            let result = ohms_law::capacitors_parallel(values)
                .context("capacitors parallel calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Capacitors in Parallel");
                println!("──────────────────────");
                println!("  Total capacitance = {:.6e} F", result.capacitance_f);
            }
        }

        OhmsLawSub::InductorsSeries { values } => {
            let result = ohms_law::inductors_series(values)
                .context("inductors series calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Inductors in Series");
                println!("───────────────────");
                println!("  Total inductance = {:.6e} H", result.inductance_h);
            }
        }

        OhmsLawSub::InductorsParallel { values } => {
            let result = ohms_law::inductors_parallel(values)
                .context("inductors parallel calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Inductors in Parallel");
                println!("─────────────────────");
                println!("  Total inductance = {:.6e} H", result.inductance_h);
            }
        }
    }
    Ok(())
}
