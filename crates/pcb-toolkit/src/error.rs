/// Errors returned by pcb-toolkit calculation functions.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CalcError {
    #[error("W/H ratio {ratio:.3} outside valid range [{min}, {max}]")]
    InvalidRatio { ratio: f64, min: f64, max: f64 },

    #[error("negative dimension: {name} = {value}")]
    NegativeDimension { name: &'static str, value: f64 },

    #[error("value out of range: {name} = {value} (expected {expected})")]
    OutOfRange {
        name: &'static str,
        value: f64,
        expected: &'static str,
    },

    /// An input was NaN or infinite. Every public calculation rejects
    /// non-finite inputs before evaluating any formula.
    #[error("value is not finite: {name} = {value}")]
    NotFinite { name: &'static str, value: f64 },

    /// A formula produced NaN or infinity for an input combination that
    /// passed the boundary checks. This is a model-domain failure and is
    /// reported instead of being serialized as a number or JSON `null`.
    #[error("calculation produced a non-finite result for {name}")]
    NonFiniteResult { name: &'static str },

    /// The requested configuration has no implemented, validated model.
    #[error("unsupported: {0}")]
    Unsupported(&'static str),

    #[error("unknown material: {0}")]
    UnknownMaterial(String),

    #[error("unknown copper weight: {0}")]
    UnknownCopperWeight(String),

    #[error("insufficient inputs: {0}")]
    InsufficientInputs(&'static str),
}
