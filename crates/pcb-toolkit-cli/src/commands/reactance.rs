use anyhow::{Context, Result};
use clap::Args;

use pcb_toolkit::reactance;
use pcb_toolkit::units::{Capacitance, Freq, Inductance};

use crate::output;

#[derive(Args)]
pub struct ReactanceArgs {
    #[arg(short, long)]
    pub freq: Freq,
    #[arg(short, long)]
    pub cap: Option<Capacitance>,
    #[arg(short = 'l', long)]
    pub ind: Option<Inductance>,
}

pub fn run(args: &ReactanceArgs, json: bool) -> Result<()> {
    let result = reactance::reactance(
        args.freq.hz(),
        args.cap.map(|c| c.farads()),
        args.ind.map(|i| i.henries()),
    )
    .context("reactance calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("Reactance");
        println!("─────────");
        if let Some(xc) = result.xc_ohms {
            println!("  Xc     = {:.4} Ω", xc);
        }
        if let Some(xl) = result.xl_ohms {
            println!("  Xl     = {:.4} Ω", xl);
        }
        if let Some(f) = result.f_res_hz {
            println!("  f_res  = {:.4} Hz", f);
        }
    }
    Ok(())
}
