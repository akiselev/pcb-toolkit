//! Microstrip impedance calculator.
//!
//! Reference: Hammerstad & Jensen, "Accurate Models for Microstrip Computer-Aided
//! Design", IEEE MTT-S International Microwave Symposium Digest, 1980.

use crate::CalcError;
use crate::impedance::{common, types::ImpedanceResult};

/// Inputs for microstrip impedance calculation. All dimensions in mils.
pub struct MicrostripInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Dielectric height — distance from trace to ground plane (mils).
    pub height: f64,
    /// Conductor thickness (mils). Usually from copper weight.
    pub thickness: f64,
    /// Substrate relative permittivity (e.g., 4.6 for FR-4).
    pub er: f64,
    /// Frequency (Hz). Used for Kirschning-Jansen dispersion correction.
    pub frequency: f64,
}

/// Compute microstrip characteristic impedance and derived quantities.
pub fn calculate(input: &MicrostripInput) -> Result<ImpedanceResult, CalcError> {
    let MicrostripInput { width, height, thickness, er, .. } = *input;

    if width <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "width", value: width });
    }
    if height <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "height", value: height });
    }
    if er < 1.0 {
        return Err(CalcError::OutOfRange {
            name: "er",
            value: er,
            expected: ">= 1.0",
        });
    }

    // Apply thickness correction to get effective width
    let we = common::effective_width(width, height, thickness);
    let u = we / height;

    // Static effective dielectric constant
    let er_eff = common::er_eff_static(u, er);

    // Characteristic impedance (Hammerstad-Jensen)
    let zo = if u <= 1.0 {
        // Narrow trace
        (60.0 / er_eff.sqrt()) * (8.0 * height / we + we / (4.0 * height)).ln()
    } else {
        // Wide trace
        (120.0 * std::f64::consts::PI / er_eff.sqrt())
            / (u + 1.393 + 0.667 * (u + 1.444).ln())
    };

    // Derived quantities
    let tpd = common::propagation_delay(er_eff);
    let lo = common::inductance_per_length(zo, tpd);
    let co = common::capacitance_per_length(zo, tpd);

    Ok(ImpedanceResult {
        zo,
        er_eff,
        tpd_ps_per_in: tpd,
        lo_nh_per_in: lo,
        co_pf_per_in: co,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_microstrip() {
        let result = calculate(&MicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            frequency: 0.0,
        })
        .unwrap();

        // Zo should be in a reasonable range for this geometry
        assert!(result.zo > 20.0 && result.zo < 80.0, "Zo = {}", result.zo);
        assert!(result.er_eff > 1.0 && result.er_eff < 4.6, "Er_eff = {}", result.er_eff);
        assert!(result.tpd_ps_per_in > 0.0);
        assert!(result.lo_nh_per_in > 0.0);
        assert!(result.co_pf_per_in > 0.0);
    }

    #[test]
    fn narrow_trace_higher_impedance() {
        let narrow = calculate(&MicrostripInput {
            width: 3.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            frequency: 0.0,
        })
        .unwrap();

        let wide = calculate(&MicrostripInput {
            width: 20.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            frequency: 0.0,
        })
        .unwrap();

        assert!(
            narrow.zo > wide.zo,
            "narrow Zo {} should be > wide Zo {}",
            narrow.zo,
            wide.zo
        );
    }

    #[test]
    fn rejects_negative_width() {
        let result = calculate(&MicrostripInput {
            width: -1.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            frequency: 0.0,
        });
        assert!(result.is_err());
    }
}
