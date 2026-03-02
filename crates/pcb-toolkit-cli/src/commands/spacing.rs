use anyhow::{bail, Context, Result};
use clap::Args;

use pcb_toolkit::spacing::{self, DeviceType, SpacingInput};

use crate::output;

#[derive(Args)]
pub struct SpacingArgs {
    #[arg(short, long)]
    pub voltage: f64,
    #[arg(short, long)]
    pub device_type: String,
}

fn parse_device_type(s: &str) -> Result<DeviceType> {
    match s.to_lowercase().as_str() {
        "b1" => Ok(DeviceType::B1),
        "b2" => Ok(DeviceType::B2),
        "b3" => Ok(DeviceType::B3),
        "b4" => Ok(DeviceType::B4),
        "b5" => Ok(DeviceType::B5),
        "a6" => Ok(DeviceType::A6),
        "a7" => Ok(DeviceType::A7),
        "a8" => Ok(DeviceType::A8),
        _ => bail!("unknown device type '{}' — valid values: b1, b2, b3, b4, b5, a6, a7, a8", s),
    }
}

pub fn run(args: &SpacingArgs, json: bool) -> Result<()> {
    let device_type = parse_device_type(&args.device_type)?;
    let result = spacing::spacing(&SpacingInput {
        voltage: args.voltage,
        device_type,
    })
    .context("spacing calculation failed")?;

    if json {
        output::print_result(&result, true)?;
    } else {
        println!("IPC-2221C Conductor Spacing");
        println!("───────────────────────────");
        println!("  Spacing = {:.4} mil", result.spacing_mils);
        println!("  Spacing = {:.4} mm", result.spacing_mm);
    }
    Ok(())
}
