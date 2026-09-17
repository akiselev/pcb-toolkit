use anyhow::{Context, Result, bail};
use clap::Args;

use pcb_toolkit::spacing::{self, DeviceType, SpacingInput};

use crate::output;

/// IPC-2221C Table 6-1 minimum electrical conductor spacing.
///
/// Device types (Saturn labels): B1 internal conductors; B2 external,
/// uncoated, sea level to 3050 m; B3 external, uncoated, over 3050 m or
/// vacuum; B4 external under solder mask; B5 external, conformally coated;
/// A6 component leads, coated; A7 component leads, uncoated, to 3050 m;
/// A8 component leads, uncoated, over 3050 m or vacuum.
#[derive(Args)]
pub struct SpacingArgs {
    /// Peak voltage between the conductors (V, DC or AC peak).
    #[arg(short, long)]
    pub voltage: f64,
    /// Device type: b1, b2, b3, b4, b5, a6, a7, a8 (see command help for definitions).
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
        _ => bail!(
            "unknown device type '{}' — valid values: b1, b2, b3, b4, b5, a6, a7, a8",
            s
        ),
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
        println!(
            "  Category = {:?}: {}",
            device_type,
            device_type.description()
        );
        println!("  Spacing  = {:.3} mm (table minimum)", result.spacing_mm);
        // Round up so the printed mils never undercut the table minimum.
        println!(
            "  Spacing  = {:.2} mil",
            (result.spacing_mils * 100.0).ceil() / 100.0
        );
        println!();
        println!("  Table 6-1 electrical clearance only; not a safety/insulation certification.");
        output::print_model(&spacing::MODEL);
    }
    Ok(())
}
