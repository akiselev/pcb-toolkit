//! IPC-2221C minimum conductor spacing lookup.
//!
//! 8 device categories × 9 voltage ranges, plus linear extrapolation above 500 V.

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// IPC-2221C device type categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    B1,
    B2,
    B3,
    B4,
    B5,
    A6,
    A7,
    A8,
}

/// Inputs for the IPC-2221C conductor spacing lookup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingInput {
    /// Peak voltage across the conductor gap (V).
    pub voltage: f64,
    /// IPC-2221C device type category.
    pub device_type: DeviceType,
}

/// Results of an IPC-2221C conductor spacing lookup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingResult {
    /// Minimum conductor spacing in mils.
    pub spacing_mils: f64,
    /// Minimum conductor spacing in millimetres.
    pub spacing_mm: f64,
}

// Lookup table: rows = device types [B1..A8], columns = voltage ranges [0..8].
// Voltage ranges: 0-15, 16-30, 31-50, 51-100, 101-150, 151-170, 171-250, 251-300, 301-500.
// All values in mils.
#[rustfmt::skip]
const TABLE: [[f64; 9]; 8] = [
    //   0-15    16-30   31-50   51-100  101-150 151-170 171-250 251-300 301-500
    [  1.97,   1.97,   3.94,   3.94,   7.87,   7.87,   7.87,   7.87,   9.84 ], // B1
    [  3.94,   3.94,  25.17,  25.17,  25.17,  49.21,  49.21,  49.21,  98.43 ], // B2
    [  3.94,   3.94,  25.17,  59.06, 125.98, 125.98, 251.97, 492.13, 492.13 ], // B3
    [  2.95,   2.95,  11.81,  11.81,  31.50,  31.50,  31.50,  31.50,  62.99 ], // B4
    [  2.95,   2.95,   5.12,   5.12,  15.75,  15.75,  15.75,  15.75,  31.50 ], // B5
    [  5.12,   5.12,   5.12,   5.12,  15.75,  15.75,  15.75,  15.75,  31.50 ], // A6
    [  5.12,   9.84,  15.75,  19.69,  31.50,  31.50,  31.50,  31.50,  59.06 ], // A7
    [  5.12,   9.84,  31.50,  39.37,  62.99,  62.99,  62.99,  62.99, 118.11 ], // A8
];

/// Linear extrapolation coefficients for voltages above 500 V.
/// Each entry is (slope mils/V, intercept mils) for `spacing = (V - 500) * slope + intercept`.
#[rustfmt::skip]
const EXTRAP: [(f64, f64); 8] = [
    (0.098425, 9.8425),   // B1
    (0.196850, 98.4252),  // B2
    (0.984252, 492.1260), // B3
    (0.120070, 62.9900),  // B4
    (0.120070, 31.4961),  // B5
    (0.120070, 31.4961),  // A6
    (0.120070, 59.0551),  // A7
    (0.240157, 118.1100), // A8
];

fn device_index(d: DeviceType) -> usize {
    match d {
        DeviceType::B1 => 0,
        DeviceType::B2 => 1,
        DeviceType::B3 => 2,
        DeviceType::B4 => 3,
        DeviceType::B5 => 4,
        DeviceType::A6 => 5,
        DeviceType::A7 => 6,
        DeviceType::A8 => 7,
    }
}

fn voltage_column(voltage: f64) -> usize {
    if voltage <= 15.0 {
        0
    } else if voltage <= 30.0 {
        1
    } else if voltage <= 50.0 {
        2
    } else if voltage <= 100.0 {
        3
    } else if voltage <= 150.0 {
        4
    } else if voltage <= 170.0 {
        5
    } else if voltage <= 250.0 {
        6
    } else if voltage <= 300.0 {
        7
    } else {
        8
    }
}

/// Look up the IPC-2221C minimum conductor spacing for the given voltage and device type.
///
/// # Errors
///
/// Returns [`CalcError::OutOfRange`] if `voltage` is negative.
pub fn spacing(input: &SpacingInput) -> Result<SpacingResult, CalcError> {
    if input.voltage < 0.0 {
        return Err(CalcError::OutOfRange {
            name: "voltage",
            value: input.voltage,
            expected: ">= 0",
        });
    }

    let row = device_index(input.device_type);

    let spacing_mils = if input.voltage > 500.0 {
        let (slope, intercept) = EXTRAP[row];
        (input.voltage - 500.0) * slope + intercept
    } else {
        let col = voltage_column(input.voltage);
        TABLE[row][col]
    };

    Ok(SpacingResult {
        spacing_mils,
        spacing_mm: spacing_mils * 0.0254,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    fn lookup(voltage: f64, device_type: DeviceType) -> SpacingResult {
        spacing(&SpacingInput { voltage, device_type }).unwrap()
    }

    #[test]
    fn b1_10v() {
        let r = lookup(10.0, DeviceType::B1);
        assert_relative_eq!(r.spacing_mils, 1.97, epsilon = 1e-6);
    }

    #[test]
    fn b3_40v() {
        let r = lookup(40.0, DeviceType::B3);
        assert_relative_eq!(r.spacing_mils, 25.17, epsilon = 1e-6);
    }

    #[test]
    fn b1_600v_extrapolation() {
        let r = lookup(600.0, DeviceType::B1);
        let expected = (600.0 - 500.0) * 0.098425 + 9.8425;
        assert_relative_eq!(r.spacing_mils, expected, epsilon = 1e-6);
        assert_relative_eq!(r.spacing_mils, 19.685, epsilon = 1e-3);
    }

    #[test]
    fn boundary_0v() {
        // 0 V → column 0
        let r = lookup(0.0, DeviceType::B1);
        assert_relative_eq!(r.spacing_mils, TABLE[0][0], epsilon = 1e-9);
    }

    #[test]
    fn boundary_15v() {
        // 15 V → column 0 (inclusive upper bound)
        let r = lookup(15.0, DeviceType::B1);
        assert_relative_eq!(r.spacing_mils, TABLE[0][0], epsilon = 1e-9);
    }

    #[test]
    fn boundary_16v() {
        // 16 V → column 1
        let r = lookup(16.0, DeviceType::B1);
        assert_relative_eq!(r.spacing_mils, TABLE[0][1], epsilon = 1e-9);
    }

    #[test]
    fn boundary_30v() {
        // 30 V → column 1 (inclusive upper bound)
        let r = lookup(30.0, DeviceType::B1);
        assert_relative_eq!(r.spacing_mils, TABLE[0][1], epsilon = 1e-9);
    }

    #[test]
    fn boundary_500v() {
        // 500 V → column 8 (last table column)
        let r = lookup(500.0, DeviceType::B1);
        assert_relative_eq!(r.spacing_mils, TABLE[0][8], epsilon = 1e-9);
    }

    #[test]
    fn boundary_501v_extrapolation() {
        // 501 V → linear extrapolation
        let r = lookup(501.0, DeviceType::B1);
        let expected = 1.0 * 0.098425 + 9.8425;
        assert_relative_eq!(r.spacing_mils, expected, epsilon = 1e-6);
    }

    #[test]
    fn mm_conversion() {
        let r = lookup(10.0, DeviceType::B2);
        assert_relative_eq!(r.spacing_mm, r.spacing_mils * 0.0254, epsilon = 1e-9);
    }

    #[test]
    fn rejects_negative_voltage() {
        let result = spacing(&SpacingInput {
            voltage: -1.0,
            device_type: DeviceType::B1,
        });
        assert!(matches!(result, Err(CalcError::OutOfRange { name: "voltage", .. })));
    }
}
