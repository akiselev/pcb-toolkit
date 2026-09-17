//! Linear interpolation for lookup tables.

use crate::CalcError;

/// Linear interpolation in a table of `(x, y)` pairs sorted by strictly
/// increasing `x`.
///
/// Below the first entry the first `y` is returned; above the last entry the
/// last `y` is returned; otherwise the value is interpolated linearly.
///
/// # Errors
/// Returns an error if the table is empty, any coordinate is non-finite, the
/// `x` values are not strictly increasing, or `x` is non-finite. The function
/// never panics.
pub fn lerp(table: &[(f64, f64)], x: f64) -> Result<f64, CalcError> {
    if table.is_empty() {
        return Err(CalcError::InsufficientInputs(
            "interpolation table must not be empty",
        ));
    }
    if !x.is_finite() {
        return Err(CalcError::NotFinite {
            name: "interpolation x",
            value: x,
        });
    }
    for w in table.windows(2) {
        if !(w[0].0.is_finite() && w[0].1.is_finite() && w[1].0.is_finite() && w[1].1.is_finite()) {
            return Err(CalcError::NotFinite {
                name: "interpolation table entry",
                value: f64::NAN,
            });
        }
        if w[1].0 <= w[0].0 {
            return Err(CalcError::OutOfRange {
                name: "interpolation table x",
                value: w[1].0,
                expected: "strictly increasing x coordinates",
            });
        }
    }
    if !(table[0].0.is_finite() && table[0].1.is_finite()) {
        return Err(CalcError::NotFinite {
            name: "interpolation table entry",
            value: f64::NAN,
        });
    }

    if x <= table[0].0 {
        return Ok(table[0].1);
    }
    let last = table[table.len() - 1];
    if x >= last.0 {
        return Ok(last.1);
    }

    // Index of the first entry with x0 > x; the interval is [i-1, i].
    let i = table.partition_point(|entry| entry.0 <= x);
    let (x0, y0) = table[i - 1];
    let (x1, y1) = table[i];
    let t = (x - x0) / (x1 - x0);
    Ok(y0 + t * (y1 - y0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match() {
        let table = &[(1.0, 10.0), (2.0, 20.0), (3.0, 30.0)];
        assert!((lerp(table, 2.0).unwrap() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn midpoint_interpolation() {
        let table = &[(0.0, 0.0), (10.0, 100.0)];
        assert!((lerp(table, 5.0).unwrap() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn clamp_below_and_above() {
        let table = &[(1.0, 10.0), (2.0, 20.0)];
        assert!((lerp(table, -5.0).unwrap() - 10.0).abs() < 1e-10);
        assert!((lerp(table, 99.0).unwrap() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn single_entry_table() {
        assert!((lerp(&[(1.0, 7.0)], 3.0).unwrap() - 7.0).abs() < 1e-10);
    }

    #[test]
    fn nan_query_and_bad_tables_are_errors_not_panics() {
        let table = &[(1.0, 10.0), (2.0, 20.0)];
        assert!(lerp(table, f64::NAN).is_err());
        assert!(lerp(&[], 1.0).is_err());
        assert!(lerp(&[(1.0, 1.0), (f64::NAN, 2.0)], 1.5).is_err());
        assert!(lerp(&[(2.0, 1.0), (1.0, 2.0)], 1.5).is_err());
    }
}
