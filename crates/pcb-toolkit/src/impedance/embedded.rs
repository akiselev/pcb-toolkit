//! Embedded microstrip impedance calculator.
//!
//! An embedded (buried) microstrip is a surface microstrip covered by a
//! dielectric overlay. The burial reduces both Zo and Er_eff compared to
//! a surface microstrip.
//!
//! Reference: Brooks, "Signal Integrity Issues and Printed Circuit Board Design".
//!
//! Z0_embedded = Z0_surface × (1 − exp(−2 × cover_height / height))
//! Er_eff_embedded = Er − (Er − Er_eff_surface) × exp(−2 × cover_height / height)

use crate::CalcError;
use crate::impedance::{common, microstrip, types::ImpedanceResult};

/// Inputs for embedded microstrip impedance calculation. All dimensions in mils.
pub struct EmbeddedMicrostripInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Dielectric height — distance from trace to ground plane (mils).
    pub height: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
    /// Cover height — dielectric above the trace (mils).
    /// When 0, result equals the surface microstrip.
    pub cover_height: f64,
    /// Frequency (Hz). Passed through to the surface microstrip calculation.
    pub frequency: f64,
}

/// Compute embedded microstrip impedance and derived quantities.
///
/// Delegates to [`microstrip::calculate`] for the surface result, then applies
/// the burial correction factor.
///
/// # TODO
/// - Verify exact formula against Saturn binary (Brooks vs IPC-2141)
pub fn calculate(input: &EmbeddedMicrostripInput) -> Result<ImpedanceResult, CalcError> {
    let EmbeddedMicrostripInput {
        width, height, thickness, er, cover_height, frequency,
    } = *input;

    if cover_height < 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "cover_height",
            value: cover_height,
        });
    }

    // Compute surface microstrip first
    let surface = microstrip::calculate(&microstrip::MicrostripInput {
        width,
        height,
        thickness,
        er,
        frequency,
    })?;

    // Early return when cover=0 (surface microstrip)
    if cover_height == 0.0 {
        return Ok(surface);
    }

    // Burial correction factor
    let exp_factor = (-2.0 * cover_height / height).exp();

    let zo = surface.zo * (1.0 - exp_factor);
    let er_eff = er - (er - surface.er_eff) * exp_factor;

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
    use approx::assert_relative_eq;

    #[test]
    fn zero_cover_equals_surface() {
        let surface = microstrip::calculate(&microstrip::MicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            frequency: 0.0,
        })
        .unwrap();

        let embedded = calculate(&EmbeddedMicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            cover_height: 0.0,
            frequency: 0.0,
        })
        .unwrap();

        assert_relative_eq!(embedded.zo, surface.zo, max_relative = 1e-10);
        assert_relative_eq!(embedded.er_eff, surface.er_eff, max_relative = 1e-10);
    }

    #[test]
    fn burial_reduces_impedance() {
        let surface = microstrip::calculate(&microstrip::MicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            frequency: 0.0,
        })
        .unwrap();

        let embedded = calculate(&EmbeddedMicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            cover_height: 5.0,
            frequency: 0.0,
        })
        .unwrap();

        assert!(
            embedded.zo < surface.zo,
            "embedded Zo {} should be < surface Zo {}",
            embedded.zo,
            surface.zo
        );
    }

    #[test]
    fn burial_increases_er_eff() {
        let surface = microstrip::calculate(&microstrip::MicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            frequency: 0.0,
        })
        .unwrap();

        let embedded = calculate(&EmbeddedMicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            cover_height: 5.0,
            frequency: 0.0,
        })
        .unwrap();

        assert!(
            embedded.er_eff > surface.er_eff,
            "embedded er_eff {} should be > surface er_eff {}",
            embedded.er_eff,
            surface.er_eff
        );
    }

    #[test]
    fn deep_burial_approaches_er() {
        let embedded = calculate(&EmbeddedMicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            cover_height: 50.0, // very deep
            frequency: 0.0,
        })
        .unwrap();

        // er_eff should approach er for deep burial
        assert_relative_eq!(embedded.er_eff, 4.6, max_relative = 0.01);
    }

    #[test]
    fn rejects_negative_cover() {
        let result = calculate(&EmbeddedMicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er: 4.6,
            cover_height: -1.0,
            frequency: 0.0,
        });
        assert!(result.is_err());
    }
}
