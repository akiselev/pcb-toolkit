//! Input and result validation helpers shared by every calculator.
//!
//! All public calculation functions validate at the boundary: non-finite
//! values are rejected first (a plain `x <= 0.0` comparison lets `NaN`
//! through), then sign and range checks are applied. Results are checked
//! with [`finite_result`] before they are returned so that a model-domain
//! failure surfaces as [`CalcError::NonFiniteResult`] rather than as a
//! `NaN` inside an `Ok`.

use crate::CalcError;

/// Reject NaN and ±infinity.
pub fn finite(name: &'static str, value: f64) -> Result<f64, CalcError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(CalcError::NotFinite { name, value })
    }
}

/// Require a finite value strictly greater than zero (a dimension).
pub fn positive(name: &'static str, value: f64) -> Result<f64, CalcError> {
    finite(name, value)?;
    if value <= 0.0 {
        return Err(CalcError::NegativeDimension { name, value });
    }
    Ok(value)
}

/// Require a finite value greater than or equal to zero.
pub fn non_negative(name: &'static str, value: f64) -> Result<f64, CalcError> {
    finite(name, value)?;
    if value < 0.0 {
        return Err(CalcError::NegativeDimension { name, value });
    }
    Ok(value)
}

/// Require a finite value strictly greater than zero, reported as an
/// out-of-range quantity (for non-dimensional inputs such as time or voltage).
pub fn positive_quantity(name: &'static str, value: f64) -> Result<f64, CalcError> {
    finite(name, value)?;
    if value <= 0.0 {
        return Err(CalcError::OutOfRange {
            name,
            value,
            expected: "> 0",
        });
    }
    Ok(value)
}

/// Require a finite value within `[min, max]`.
pub fn in_range(
    name: &'static str,
    value: f64,
    min: f64,
    max: f64,
    expected: &'static str,
) -> Result<f64, CalcError> {
    finite(name, value)?;
    if value < min || value > max {
        return Err(CalcError::OutOfRange {
            name,
            value,
            expected,
        });
    }
    Ok(value)
}

/// Relative permittivity: finite and ≥ 1.
pub fn er(value: f64) -> Result<f64, CalcError> {
    finite("er", value)?;
    if value < 1.0 {
        return Err(CalcError::OutOfRange {
            name: "er",
            value,
            expected: ">= 1.0",
        });
    }
    Ok(value)
}

/// Check a computed quantity before it is placed in a result struct.
pub fn finite_result(name: &'static str, value: f64) -> Result<f64, CalcError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(CalcError::NonFiniteResult { name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nan_is_rejected_everywhere() {
        assert!(matches!(
            finite("x", f64::NAN),
            Err(CalcError::NotFinite { .. })
        ));
        assert!(matches!(
            positive("x", f64::NAN),
            Err(CalcError::NotFinite { .. })
        ));
        assert!(matches!(
            non_negative("x", f64::INFINITY),
            Err(CalcError::NotFinite { .. })
        ));
        assert!(matches!(er(f64::NAN), Err(CalcError::NotFinite { .. })));
        assert!(matches!(
            finite_result("z", f64::NAN),
            Err(CalcError::NonFiniteResult { .. })
        ));
    }

    #[test]
    fn sign_checks() {
        assert!(positive("x", 0.0).is_err());
        assert!(positive("x", 1e-300).is_ok());
        assert!(non_negative("x", 0.0).is_ok());
        assert!(non_negative("x", -1e-300).is_err());
        assert!(er(0.999).is_err());
        assert!(er(1.0).is_ok());
    }
}
