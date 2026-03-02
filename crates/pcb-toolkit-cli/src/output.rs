//! Output formatting (plain text and JSON).

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
