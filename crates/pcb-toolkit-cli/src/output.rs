//! Output formatting (plain text and JSON).

use pcb_toolkit::model::{ModelInfo, ModelStatus};

/// Print a result as either formatted text or JSON.
pub fn print_result<T: serde::Serialize + std::fmt::Debug>(
    result: &T,
    json: bool,
) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(result)?);
    } else {
        println!("{result:#?}");
    }
    Ok(())
}

/// Print the model caveat line for text output. Validated models print
/// nothing; compatibility and experimental models always print their
/// status, source and validity range so an estimate is never mistaken for
/// a validated result.
pub fn print_model(model: &ModelInfo) {
    match model.status {
        ModelStatus::Validated => {}
        ModelStatus::Compatibility | ModelStatus::Experimental => {
            println!();
            println!("  {}", model.caveat());
        }
    }
}
