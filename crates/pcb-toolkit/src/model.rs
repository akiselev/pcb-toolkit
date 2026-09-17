//! Model provenance and validation status.
//!
//! Every calculator module exposes a `MODEL` constant describing which
//! published model it implements, where the expression comes from, the
//! geometry range over which it is applicable, and how much validation
//! evidence exists. "Matches Saturn" is a compatibility statement, not an
//! accuracy statement; the two are kept separate here.

use serde::{Deserialize, Serialize};

/// How much evidence backs a calculator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    /// Exact relation or a published closed form checked against an
    /// independent evaluation (field solver, conformal-mapping reference,
    /// or circuit oracle) inside its stated range.
    Validated,
    /// Published approximation kept for compatibility with Saturn PCB
    /// Toolkit or IPC-2141; internally consistent and sign-safe, but its
    /// accuracy has only been checked against the source's own examples.
    Compatibility,
    /// Heuristic or partially reconstructed model. Results are estimates
    /// and must not be used as an engineering decision without independent
    /// confirmation.
    Experimental,
}

impl ModelStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Validated => "validated",
            Self::Compatibility => "compatibility",
            Self::Experimental => "experimental",
        }
    }
}

/// Static description of a calculator's model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Human-readable model name.
    pub name: &'static str,
    /// Validation status.
    pub status: ModelStatus,
    /// Primary reference for the expressions.
    pub reference: &'static str,
    /// Geometry / parameter range over which the model is applicable, and
    /// the expected accuracy inside that range.
    pub validity: &'static str,
}

impl ModelInfo {
    /// One-line caveat suitable for CLI output.
    pub fn caveat(&self) -> String {
        format!(
            "Model: {} [{}] — {}. Range: {}",
            self.name,
            self.status.label(),
            self.reference,
            self.validity
        )
    }
}
