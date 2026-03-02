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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Impedance(args) => commands::impedance::run(&args, cli.json),
    }
}
