use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::units::Freq;
use pcb_toolkit::wavelength;

use crate::output;

#[derive(Args)]
pub struct WavelengthArgs {
    #[arg(short, long)]
    pub freq: Freq,
    #[arg(long, default_value = "1.0")]
    pub er: f64,
}

pub fn run(args: &WavelengthArgs, json: bool) -> Result<()> {
    let result = wavelength::wavelength(args.freq.hz(), args.er)
        .context("wavelength calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Wavelength");
        println!("──────────");
        println!("  λ       = {:.4} in", result.lambda_inches);
        println!("  λ/2     = {:.4} in", result.lambda_half_inches);
        println!("  λ/4     = {:.4} in", result.lambda_quarter_inches);
        println!("  λ/7     = {:.4} in", result.lambda_seventh_inches);
        println!("  λ/10    = {:.4} in", result.lambda_tenth_inches);
        println!("  λ/20    = {:.4} in", result.lambda_twentieth_inches);
        println!("  Period  = {:.4} ns", result.period_ns);
    }
    Ok(())
}
