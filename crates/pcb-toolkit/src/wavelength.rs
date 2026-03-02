//! Signal wavelength calculator.
//!
//! λ = c / (f × √Er_eff)

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// Speed of light expressed as inches per nanosecond.
const C_IN_PER_NS: f64 = 11.803;

/// Result of a wavelength calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WavelengthResult {
    /// Full wavelength in inches.
    pub lambda_inches: f64,
    /// λ/2 in inches.
    pub lambda_half_inches: f64,
    /// λ/4 in inches.
    pub lambda_quarter_inches: f64,
    /// λ/7 in inches.
    pub lambda_seventh_inches: f64,
    /// λ/10 in inches.
    pub lambda_tenth_inches: f64,
    /// λ/20 in inches.
    pub lambda_twentieth_inches: f64,
    /// Period in nanoseconds (1 / f_hz × 1e9).
    pub period_ns: f64,
}

/// Calculate signal wavelength in a substrate.
///
/// # Arguments
/// - `freq_hz` — frequency in Hz (must be > 0)
/// - `er_eff` — effective relative permittivity (must be ≥ 1.0)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if inputs are out of valid range.
pub fn wavelength(freq_hz: f64, er_eff: f64) -> Result<WavelengthResult, CalcError> {
    if freq_hz <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "freq_hz",
            value: freq_hz,
            expected: "> 0",
        });
    }
    if er_eff < 1.0 {
        return Err(CalcError::OutOfRange {
            name: "er_eff",
            value: er_eff,
            expected: ">= 1.0",
        });
    }

    let period_ns = 1.0 / freq_hz * 1e9;
    // λ = c × T_ns / √Er_eff  (c in in/ns, T in ns → λ in inches)
    let lambda_inches = C_IN_PER_NS * period_ns / er_eff.sqrt();

    Ok(WavelengthResult {
        lambda_inches,
        lambda_half_inches: lambda_inches / 2.0,
        lambda_quarter_inches: lambda_inches / 4.0,
        lambda_seventh_inches: lambda_inches / 7.0,
        lambda_tenth_inches: lambda_inches / 10.0,
        lambda_twentieth_inches: lambda_inches / 20.0,
        period_ns,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn notes: Period=10ns (f=100MHz), Er_eff=4 → λ=59.01426 inches (but formula
    // gives 11.803 × 10 / √4 = 118.03 / 2 = 59.015 — rounding in the notes).
    // The note says 59.01426 which matches using c=11.80285 (exact c / in/ns).
    // Saturn uses c=11.803 → λ = 11.803 × 10 / 2 = 59.015 inches.
    #[test]
    fn saturn_notes_100mhz_er4() {
        let result = wavelength(100e6, 4.0).unwrap();
        assert_relative_eq!(result.period_ns, 10.0, epsilon = 1e-9);
        assert_relative_eq!(result.lambda_inches, 59.015, epsilon = 1e-2);
        assert_relative_eq!(result.lambda_half_inches, result.lambda_inches / 2.0, epsilon = 1e-10);
        assert_relative_eq!(result.lambda_quarter_inches, result.lambda_inches / 4.0, epsilon = 1e-10);
        assert_relative_eq!(result.lambda_seventh_inches, result.lambda_inches / 7.0, epsilon = 1e-10);
        assert_relative_eq!(result.lambda_tenth_inches, result.lambda_inches / 10.0, epsilon = 1e-10);
        assert_relative_eq!(result.lambda_twentieth_inches, result.lambda_inches / 20.0, epsilon = 1e-10);
    }

    #[test]
    fn error_on_zero_freq() {
        assert!(wavelength(0.0, 4.0).is_err());
    }

    #[test]
    fn error_on_er_below_one() {
        assert!(wavelength(100e6, 0.5).is_err());
    }

    #[test]
    fn vacuum_er1() {
        let result = wavelength(1e9, 1.0).unwrap();
        // At 1 GHz, period = 1 ns, λ = 11.803 inches in vacuum
        assert_relative_eq!(result.lambda_inches, 11.803, epsilon = 1e-3);
    }
}
