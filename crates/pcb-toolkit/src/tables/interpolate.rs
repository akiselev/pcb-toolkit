//! Generic linear interpolation for lookup tables.

/// Linear interpolation in a sorted table of (x, y) pairs.
///
/// If `x` is below the first entry, returns the first y value.
/// If `x` is above the last entry, returns the last y value.
/// Otherwise, linearly interpolates between the two surrounding entries.
///
/// # Panics
///
/// Panics if `table` is empty.
pub fn lerp(table: &[(f64, f64)], x: f64) -> f64 {
    assert!(!table.is_empty(), "interpolation table must not be empty");

    if table.len() == 1 || x <= table[0].0 {
        return table[0].1;
    }
    if x >= table[table.len() - 1].0 {
        return table[table.len() - 1].1;
    }

    // Binary search for the interval containing x
    let i = match table.binary_search_by(|entry| entry.0.partial_cmp(&x).unwrap()) {
        Ok(i) => return table[i].1, // exact match
        Err(i) => i - 1,
    };

    let (x0, y0) = table[i];
    let (x1, y1) = table[i + 1];
    let t = (x - x0) / (x1 - x0);
    y0 + t * (y1 - y0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match() {
        let table = &[(1.0, 10.0), (2.0, 20.0), (3.0, 30.0)];
        assert!((lerp(table, 2.0) - 20.0).abs() < 1e-10);
    }

    #[test]
    fn midpoint_interpolation() {
        let table = &[(0.0, 0.0), (10.0, 100.0)];
        assert!((lerp(table, 5.0) - 50.0).abs() < 1e-10);
    }

    #[test]
    fn clamp_below() {
        let table = &[(1.0, 10.0), (2.0, 20.0)];
        assert!((lerp(table, -5.0) - 10.0).abs() < 1e-10);
    }

    #[test]
    fn clamp_above() {
        let table = &[(1.0, 10.0), (2.0, 20.0)];
        assert!((lerp(table, 99.0) - 20.0).abs() < 1e-10);
    }
}
