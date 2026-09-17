use anyhow::{Context, Result};
use clap::{Args, Subcommand};

use pcb_toolkit::ohms_law;
use pcb_toolkit::units::{Capacitance, Inductance, Resistance};

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
        /// Resistance [Ohm, mOhm, kOhm/k, MOhm/M]. Default unit: Ohm.
        #[arg(long)]
        resistance: Option<Resistance>,
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
        /// System impedance [Ohm, kOhm/k]. Default unit: Ohm.
        #[arg(long)]
        impedance: Resistance,
    },

    /// Symmetric T-pad attenuator.
    TPad {
        /// Attenuation in dB (> 0).
        #[arg(long)]
        attenuation: f64,
        /// System impedance [Ohm, kOhm/k]. Default unit: Ohm.
        #[arg(long)]
        impedance: Resistance,
    },

    /// Resistors in series.
    ResistorsSeries {
        /// Resistor values [Ohm, mOhm, kOhm/k, MOhm/M]. Default unit: Ohm.
        #[arg(required = true, num_args = 1..)]
        values: Vec<Resistance>,
    },

    /// Resistors in parallel.
    ResistorsParallel {
        /// Resistor values [Ohm, mOhm, kOhm/k, MOhm/M]. Default unit: Ohm.
        #[arg(required = true, num_args = 1..)]
        values: Vec<Resistance>,
    },

    /// Capacitors in series.
    CapacitorsSeries {
        /// Capacitor values [F, uF, nF, pF]. Default unit: F.
        #[arg(required = true, num_args = 1..)]
        values: Vec<Capacitance>,
    },

    /// Capacitors in parallel.
    CapacitorsParallel {
        /// Capacitor values [F, uF, nF, pF]. Default unit: F.
        #[arg(required = true, num_args = 1..)]
        values: Vec<Capacitance>,
    },

    /// Inductors in series.
    InductorsSeries {
        /// Inductor values [H, mH, uH, nH]. Default unit: H.
        #[arg(required = true, num_args = 1..)]
        values: Vec<Inductance>,
    },

    /// Inductors in parallel.
    InductorsParallel {
        /// Inductor values [H, mH, uH, nH]. Default unit: H.
        #[arg(required = true, num_args = 1..)]
        values: Vec<Inductance>,
    },
}

pub fn run(args: &OhmsLawArgs, json: bool) -> Result<()> {
    match &args.sub {
        OhmsLawSub::Eir {
            voltage,
            current,
            resistance,
        } => {
            let result = ohms_law::eir(*voltage, *current, resistance.map(|r| r.ohms()))
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

        OhmsLawSub::LedBias {
            supply,
            led_v,
            led_current,
        } => {
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

        OhmsLawSub::PiPad {
            attenuation,
            impedance,
        } => {
            let result = ohms_law::pi_pad(*attenuation, impedance.ohms())
                .context("Pi-pad calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("Pi-Pad Attenuator");
                println!("─────────────────");
                println!("  Attenuation = {:.4} dB", result.attenuation_db);
                println!("  K           = {:.6}", result.k);
                println!(
                    "  R series    = {:.4} Ω  (single centre element)",
                    result.r_series_ohm
                );
                println!(
                    "  R shunt     = {:.4} Ω  (each of the two outer elements)",
                    result.r_shunt_ohm
                );
            }
        }

        OhmsLawSub::TPad {
            attenuation,
            impedance,
        } => {
            let result = ohms_law::t_pad(*attenuation, impedance.ohms())
                .context("T-pad calculation failed")?;
            if json {
                output::print_result(&result, true)?;
            } else {
                println!("T-Pad Attenuator");
                println!("────────────────");
                println!("  Attenuation = {:.4} dB", result.attenuation_db);
                println!("  K           = {:.6}", result.k);
                println!(
                    "  R series    = {:.4} Ω  (each of the two outer elements)",
                    result.r_series_ohm
                );
                println!(
                    "  R shunt     = {:.4} Ω  (single centre element)",
                    result.r_shunt_ohm
                );
            }
        }

        OhmsLawSub::ResistorsSeries { values } => {
            let result =
                ohms_law::resistors_series(&values.iter().map(|v| v.ohms()).collect::<Vec<_>>())
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
            let result =
                ohms_law::resistors_parallel(&values.iter().map(|v| v.ohms()).collect::<Vec<_>>())
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
            let result =
                ohms_law::capacitors_series(&values.iter().map(|v| v.farads()).collect::<Vec<_>>())
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
            let result = ohms_law::capacitors_parallel(
                &values.iter().map(|v| v.farads()).collect::<Vec<_>>(),
            )
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
            let result =
                ohms_law::inductors_series(&values.iter().map(|v| v.henries()).collect::<Vec<_>>())
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
            let result = ohms_law::inductors_parallel(
                &values.iter().map(|v| v.henries()).collect::<Vec<_>>(),
            )
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
