/// Errors returned by pcb-toolkit calculation functions.
#[derive(Debug, thiserror::Error)]
pub enum CalcError {
    #[error("W/H ratio {ratio:.3} outside valid range [{min}, {max}]")]
    InvalidRatio {
        ratio: f64,
        min: f64,
        max: f64,
    },

    #[error("negative dimension: {name} = {value}")]
    NegativeDimension { name: &'static str, value: f64 },

    #[error("value out of range: {name} = {value} (expected {expected})")]
    OutOfRange {
        name: &'static str,
        value: f64,
        expected: &'static str,
    },

    #[error("unknown material: {0}")]
    UnknownMaterial(String),

    #[error("unknown copper weight: {0}")]
    UnknownCopperWeight(String),

    #[error("insufficient inputs: {0}")]
    InsufficientInputs(&'static str),
}
