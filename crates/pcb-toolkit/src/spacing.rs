//! IPC-2221C minimum conductor spacing lookup (Table 6-1).
//!
//! The table is stored in its source unit (millimetres) exactly as
//! published; mils are derived from it. The result is the table minimum, so
//! presenting it rounded must round up, never down.
//!
//! Scope: Table 6-1 gives electrical clearances for DC or AC peak voltage
//! between conductors under the construction and coating assumptions of
//! each category. A successful lookup is not a certification of insulation
//! or safety compliance; the standard's application notes, altitude and
//! pollution-degree conditions and the above-500 V procedure apply.

use serde::{Deserialize, Serialize};

use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "IPC-2221C Table 6-1 electrical conductor spacing",
    status: ModelStatus::Validated,
    reference: "IPC-2221C (2023) Table 6-1, p.58; above 500 V linear rule per category",
    validity: "0–500 V table lookup (all 72 cells checked against the published table); >500 V linear extrapolation per the standard's slope; DC or AC peak voltage",
};

/// Millimetres per mil.
const MM_PER_MIL: f64 = 0.0254;

/// IPC-2221C device type categories (Saturn PCB Toolkit labels).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    /// Internal conductors.
    B1,
    /// External conductors, uncoated, sea level to 3050 m.
    B2,
    /// External conductors, uncoated, over 3050 m or in a vacuum.
    B3,
    /// External conductors covered with solder mask (any elevation).
    B4,
    /// External conductors, coated (conformal coating), any elevation or vacuum.
    B5,
    /// External component lead/termination, coated, any elevation or vacuum.
    A6,
    /// External component lead/termination, uncoated, sea level to 3050 m.
    A7,
    /// External component lead/termination, uncoated, over 3050 m or in a vacuum.
    A8,
}

impl DeviceType {
    /// Human-readable category description.
    pub fn description(self) -> &'static str {
        match self {
            Self::B1 => "internal conductors",
            Self::B2 => "external conductors, uncoated, sea level to 3050 m",
            Self::B3 => "external conductors, uncoated, over 3050 m or in a vacuum",
            Self::B4 => "external conductors covered with solder mask, any elevation",
            Self::B5 => "external conductors, conformally coated, any elevation or vacuum",
            Self::A6 => "external component leads/terminations, coated, any elevation or vacuum",
            Self::A7 => "external component leads/terminations, uncoated, sea level to 3050 m",
            Self::A8 => {
                "external component leads/terminations, uncoated, over 3050 m or in a vacuum"
            }
        }
    }

    /// All categories, in table order.
    pub const ALL: [DeviceType; 8] = [
        Self::B1,
        Self::B2,
        Self::B3,
        Self::B4,
        Self::B5,
        Self::A6,
        Self::A7,
        Self::A8,
    ];
}

/// Inputs for the IPC-2221C conductor spacing lookup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingInput {
    /// Peak voltage across the conductor gap (V, DC or AC peak).
    pub voltage: f64,
    /// IPC-2221C device type category.
    pub device_type: DeviceType,
}

/// Results of an IPC-2221C conductor spacing lookup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingResult {
    /// Minimum conductor spacing in millimetres (the published value).
    pub spacing_mm: f64,
    /// Minimum conductor spacing in mils, derived from the millimetre value.
    pub spacing_mils: f64,
}

/// Upper bound (inclusive) of each voltage column (V).
const COLUMN_MAX_V: [f64; 9] = [15.0, 30.0, 50.0, 100.0, 150.0, 170.0, 250.0, 300.0, 500.0];

// Lookup table in millimetres: rows = device types [B1..A8], columns = voltage ranges.
// Voltage ranges: 0-15, 16-30, 31-50, 51-100, 101-150, 151-170, 171-250, 251-300, 301-500.
#[rustfmt::skip]
const TABLE_MM: [[f64; 9]; 8] = [
    //  0-15   16-30  31-50  51-100 101-150 151-170 171-250 251-300 301-500
    [ 0.05,  0.05,  0.10,  0.10,  0.20,   0.20,   0.20,   0.20,   0.25 ], // B1
    [ 0.10,  0.10,  0.64,  0.64,  0.64,   1.25,   1.25,   1.25,   2.50 ], // B2
    [ 0.10,  0.10,  0.64,  1.50,  3.20,   3.20,   6.40,  12.50,  12.50 ], // B3
    [ 0.075, 0.075, 0.30,  0.30,  0.80,   0.80,   0.80,   0.80,   1.60 ], // B4
    [ 0.075, 0.075, 0.13,  0.13,  0.40,   0.40,   0.40,   0.40,   0.80 ], // B5
    [ 0.13,  0.13,  0.13,  0.13,  0.40,   0.40,   0.40,   0.40,   0.80 ], // A6
    [ 0.13,  0.25,  0.40,  0.50,  0.80,   0.80,   0.80,   0.80,   1.50 ], // A7
    [ 0.13,  0.25,  0.80,  1.00,  1.60,   1.60,   1.60,   1.60,   3.00 ], // A8
];

/// Slope (mm/V) for voltages above 500 V: `spacing = table[301–500 V] + (V − 500) × slope`.
const SLOPE_MM_PER_V: [f64; 8] = [
    0.0025, 0.005, 0.025, 0.00305, 0.00305, 0.00305, 0.00305, 0.0061,
];

fn device_index(d: DeviceType) -> usize {
    DeviceType::ALL
        .iter()
        .position(|&x| x == d)
        .expect("every variant is listed")
}

/// Look up the IPC-2221C minimum conductor spacing for the given voltage and device type.
///
/// # Errors
/// Returns an error if `voltage` is negative or non-finite.
pub fn spacing(input: &SpacingInput) -> Result<SpacingResult, CalcError> {
    let voltage =
        validate::non_negative("voltage", input.voltage).map_err(|_| CalcError::OutOfRange {
            name: "voltage",
            value: input.voltage,
            expected: ">= 0 and finite",
        })?;
    let row = device_index(input.device_type);

    let spacing_mm = match COLUMN_MAX_V.iter().position(|&max| voltage <= max) {
        Some(col) => TABLE_MM[row][col],
        None => TABLE_MM[row][8] + (voltage - 500.0) * SLOPE_MM_PER_V[row],
    };
    let spacing_mm = validate::finite_result("spacing_mm", spacing_mm)?;

    Ok(SpacingResult {
        spacing_mm,
        spacing_mils: spacing_mm / MM_PER_MIL,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    fn lookup(voltage: f64, device_type: DeviceType) -> SpacingResult {
        spacing(&SpacingInput {
            voltage,
            device_type,
        })
        .unwrap()
    }

    #[test]
    fn b1_10v() {
        let r = lookup(10.0, DeviceType::B1);
        assert_relative_eq!(r.spacing_mm, 0.05, epsilon = 1e-12);
        assert_relative_eq!(r.spacing_mils, 1.9685, max_relative = 1e-4);
    }

    #[test]
    fn b3_40v() {
        assert_relative_eq!(
            lookup(40.0, DeviceType::B3).spacing_mm,
            0.64,
            epsilon = 1e-12
        );
    }

    #[test]
    fn minimum_is_never_undercut_by_rounding() {
        // 0.075 mm must not become 0.07493 mm through a rounded mil intermediate (audit A19).
        let r = lookup(10.0, DeviceType::B4);
        assert!(r.spacing_mm >= 0.075);
        assert!(r.spacing_mils * MM_PER_MIL >= 0.075 - 1e-15);
    }

    #[test]
    fn b1_600v_extrapolation() {
        let r = lookup(600.0, DeviceType::B1);
        assert_relative_eq!(r.spacing_mm, 0.25 + 100.0 * 0.0025, epsilon = 1e-12);
        assert_relative_eq!(r.spacing_mils, 19.685, max_relative = 1e-4);
    }

    #[test]
    fn column_boundaries_are_inclusive() {
        assert_eq!(lookup(0.0, DeviceType::B1).spacing_mm, TABLE_MM[0][0]);
        assert_eq!(lookup(15.0, DeviceType::B1).spacing_mm, TABLE_MM[0][0]);
        assert_eq!(lookup(16.0, DeviceType::B1).spacing_mm, TABLE_MM[0][1]);
        assert_eq!(lookup(30.0, DeviceType::B1).spacing_mm, TABLE_MM[0][1]);
        assert_eq!(lookup(500.0, DeviceType::B1).spacing_mm, TABLE_MM[0][8]);
        assert_relative_eq!(
            lookup(501.0, DeviceType::B1).spacing_mm,
            TABLE_MM[0][8] + 0.0025,
            epsilon = 1e-12
        );
    }

    #[test]
    fn extrapolation_is_continuous_at_500v_for_every_category() {
        for d in DeviceType::ALL {
            let a = lookup(500.0, d).spacing_mm;
            let b = lookup(500.0 + 1e-9, d).spacing_mm;
            assert!((a - b).abs() < 1e-9, "{d:?}");
        }
    }

    #[test]
    fn table_is_monotonic_in_voltage() {
        for row in TABLE_MM {
            for w in row.windows(2) {
                assert!(w[1] >= w[0]);
            }
        }
    }

    #[test]
    fn rejects_negative_and_nonfinite_voltage() {
        assert!(matches!(
            spacing(&SpacingInput {
                voltage: -1.0,
                device_type: DeviceType::B1
            }),
            Err(CalcError::OutOfRange {
                name: "voltage",
                ..
            })
        ));
        assert!(
            spacing(&SpacingInput {
                voltage: f64::NAN,
                device_type: DeviceType::B1
            })
            .is_err()
        );
        assert!(
            spacing(&SpacingInput {
                voltage: f64::INFINITY,
                device_type: DeviceType::B1
            })
            .is_err()
        );
    }
}
