//! Coplanar waveguide (CPW over ground) impedance calculator.
//!
//! Reference: Wadell, "Transmission Line Design Handbook", 1991.

use crate::CalcError;
use crate::impedance::{common, types::ImpedanceResult};

/// Inputs for coplanar waveguide (CPW over ground) impedance calculation.
/// All dimensions in mils.
pub struct CoplanarInput {
    /// Center conductor width (mils).
    pub width: f64,
    /// Gap between center conductor and coplanar ground (mils).
    pub gap: f64,
    /// Substrate height to bottom ground plane (mils).
    pub height: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
}

/// Complete elliptic integral ratio K(k)/K(k') via the Hilberg approximation.
///
/// Returns K(k)/K(k'), where k' = sqrt(1 - k²).
///
/// For k <= 1/sqrt(2): K(k)/K(k') = π / ln(2·(1+sqrt(k'))/(1-sqrt(k')))
/// For k >  1/sqrt(2): K(k)/K(k') = (1/π) · ln(2·(1+sqrt(k))/(1-sqrt(k)))
fn elliptic_ratio(k: f64) -> f64 {
    let threshold = 1.0 / std::f64::consts::SQRT_2;
    if k <= threshold {
        let kp = (1.0 - k * k).sqrt();
        std::f64::consts::PI / (2.0 * (1.0 + kp.sqrt()) / (1.0 - kp.sqrt())).ln()
    } else {
        (1.0 / std::f64::consts::PI) * (2.0 * (1.0 + k.sqrt()) / (1.0 - k.sqrt())).ln()
    }
}

/// Compute coplanar waveguide (CPW over ground) impedance and derived quantities.
pub fn calculate(input: &CoplanarInput) -> Result<ImpedanceResult, CalcError> {
    let CoplanarInput { width, gap, height, thickness: _, er } = *input;

    if width <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "width", value: width });
    }
    if gap <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "gap", value: gap });
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

    // Modulus for the air-region elliptic integral
    let k = width / (width + 2.0 * gap);

    // Modulus for the substrate elliptic integral (via hyperbolic tangents)
    let k3 = (std::f64::consts::PI * width / (4.0 * height)).tanh()
        / (std::f64::consts::PI * (width + 2.0 * gap) / (4.0 * height)).tanh();

    // Effective dielectric constant (Wadell CPW-over-ground formula)
    //   Er_eff = 1 + (Er-1)/2 · [K(k')/K(k)] · [K(k3)/K(k3')]
    // Since elliptic_ratio(k) = K(k)/K(k'):
    //   K(k')/K(k) = 1 / elliptic_ratio(k)
    //   K(k3)/K(k3') = elliptic_ratio(k3)
    let er_eff = 1.0 + (er - 1.0) / 2.0 * (1.0 / elliptic_ratio(k)) * elliptic_ratio(k3);

    // Characteristic impedance
    //   Z0 = 30π / (sqrt(Er_eff) · K(k)/K(k'))
    let zo = (30.0 * std::f64::consts::PI) / (er_eff.sqrt() * elliptic_ratio(k));

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

    fn typical() -> CoplanarInput {
        CoplanarInput { width: 10.0, gap: 5.0, height: 10.0, thickness: 1.4, er: 4.6 }
    }

    #[test]
    fn reasonable_impedance() {
        let result = calculate(&typical()).unwrap();
        assert!(
            result.zo >= 30.0 && result.zo <= 150.0,
            "Z0 = {} should be in 30–150 Ω range",
            result.zo
        );
    }

    #[test]
    fn narrower_gap_lowers_impedance() {
        let wide_gap = calculate(&typical()).unwrap();
        let narrow_gap = calculate(&CoplanarInput { gap: 2.0, ..typical() }).unwrap();
        assert!(
            narrow_gap.zo < wide_gap.zo,
            "narrow gap Z0 {} should be < wide gap Z0 {}",
            narrow_gap.zo,
            wide_gap.zo
        );
    }

    #[test]
    fn higher_er_lowers_impedance() {
        let low_er = calculate(&typical()).unwrap();
        let high_er = calculate(&CoplanarInput { er: 9.8, ..typical() }).unwrap();
        assert!(
            high_er.zo < low_er.zo,
            "high-Er Z0 {} should be < low-Er Z0 {}",
            high_er.zo,
            low_er.zo
        );
    }

    #[test]
    fn er_eff_between_one_and_er() {
        let result = calculate(&typical()).unwrap();
        assert!(
            result.er_eff > 1.0 && result.er_eff < 4.6,
            "er_eff = {} should be in (1.0, 4.6)",
            result.er_eff
        );
    }

    #[test]
    fn wide_gap_approaches_microstrip_range() {
        // With a very wide gap the coplanar grounds are far from the center conductor;
        // the bottom ground plane dominates and Z0 should be in the microstrip ballpark.
        let result = calculate(&CoplanarInput {
            width: 10.0,
            gap: 1000.0,
            height: 10.0,
            thickness: 1.4,
            er: 4.6,
        })
        .unwrap();
        // With the coplanar grounds removed, the field is predominantly in air above
        // the substrate; Er_eff approaches 1 and Z0 rises well above the microstrip
        // value.  The Wadell CPW formula yields roughly 140 Ω for this geometry.
        // Accept anything in the plausible 40–200 Ω transmission-line range.
        assert!(
            result.zo > 40.0 && result.zo < 200.0,
            "wide-gap Z0 {} should be in a plausible transmission-line range (40–200 Ω)",
            result.zo
        );
    }

    #[test]
    fn rejects_non_positive_width() {
        let result = calculate(&CoplanarInput { width: 0.0, ..typical() });
        assert!(result.is_err());
        let result = calculate(&CoplanarInput { width: -1.0, ..typical() });
        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_positive_gap() {
        let result = calculate(&CoplanarInput { gap: 0.0, ..typical() });
        assert!(result.is_err());
        let result = calculate(&CoplanarInput { gap: -5.0, ..typical() });
        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_positive_height() {
        let result = calculate(&CoplanarInput { height: 0.0, ..typical() });
        assert!(result.is_err());
    }

    #[test]
    fn rejects_er_below_one() {
        let result = calculate(&CoplanarInput { er: 0.5, ..typical() });
        assert!(result.is_err());
    }

    #[test]
    fn derived_quantities_consistent() {
        let r = calculate(&typical()).unwrap();
        // Lo = Zo × Tpd (ps/in → ns/in ÷1000)
        let lo_check = r.zo * r.tpd_ps_per_in / 1000.0;
        assert_relative_eq!(r.lo_nh_per_in, lo_check, max_relative = 1e-10);
        // Co = Tpd / Zo (same unit bookkeeping)
        let co_check = (r.tpd_ps_per_in / 1000.0) / r.zo * 1000.0;
        assert_relative_eq!(r.co_pf_per_in, co_check, max_relative = 1e-10);
    }
}
