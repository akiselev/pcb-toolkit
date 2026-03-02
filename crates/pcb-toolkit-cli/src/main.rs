use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod output;

#[derive(Parser)]
#[command(name = "pcb-toolkit", version, about = "PCB design calculator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output results as JSON.
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Transmission line impedance calculators.
    Impedance(commands::impedance::ImpedanceArgs),
    /// Differential pair impedance calculators.
    Differential(commands::differential::DifferentialArgs),
    /// Conductor current capacity (IPC-2221A).
    Current(commands::current::CurrentArgs),
    /// Fusing current (Onderdonk equation).
    Fusing(commands::fusing::FusingArgs),
    /// Via impedance and parasitic properties.
    Via(commands::via::ViaArgs),
    /// Planar spiral inductor.
    Inductor(commands::inductor::InductorArgs),
    /// Capacitive/inductive reactance and resonant frequency.
    Reactance(commands::reactance::ReactanceArgs),
    /// Wavelength in a dielectric.
    Wavelength(commands::wavelength::WavelengthArgs),
    /// Ohm's law, attenuators, and component combinations.
    #[command(name = "ohms-law")]
    OhmsLaw(commands::ohms_law::OhmsLawArgs),
    /// PPM/frequency conversion and crystal load capacitance.
    Ppm(commands::ppm::PpmArgs),
    /// Padstack geometry (thru-hole, corner-to-corner).
    Padstack(commands::padstack::PadstackArgs),
    /// Conductor spacing (IPC-2221C).
    Spacing(commands::spacing::SpacingArgs),
    /// AWG wire gauge properties.
    #[command(name = "wire-gauge")]
    WireGauge(commands::wire_gauge::WireGaugeArgs),
    /// PDN impedance calculator.
    Pdn(commands::pdn::PdnArgs),
    /// Thermal management (junction temperature).
    Thermal(commands::thermal::ThermalArgs),
    /// Crosstalk estimation (NEXT).
    Crosstalk(commands::crosstalk::CrosstalkArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Impedance(args) => commands::impedance::run(&args, cli.json),
        Commands::Differential(args) => commands::differential::run(&args, cli.json),
        Commands::Current(args) => commands::current::run(&args, cli.json),
        Commands::Fusing(args) => commands::fusing::run(&args, cli.json),
        Commands::Via(args) => commands::via::run(&args, cli.json),
        Commands::Inductor(args) => commands::inductor::run(&args, cli.json),
        Commands::Reactance(args) => commands::reactance::run(&args, cli.json),
        Commands::Wavelength(args) => commands::wavelength::run(&args, cli.json),
        Commands::OhmsLaw(args) => commands::ohms_law::run(&args, cli.json),
        Commands::Ppm(args) => commands::ppm::run(&args, cli.json),
        Commands::Padstack(args) => commands::padstack::run(&args, cli.json),
        Commands::Spacing(args) => commands::spacing::run(&args, cli.json),
        Commands::WireGauge(args) => commands::wire_gauge::run(&args, cli.json),
        Commands::Pdn(args) => commands::pdn::run(&args, cli.json),
        Commands::Thermal(args) => commands::thermal::run(&args, cli.json),
        Commands::Crosstalk(args) => commands::crosstalk::run(&args, cli.json),
    }
}
