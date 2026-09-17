//! Stripline impedance calculator (centered and offset strips between two
//! ground planes).
//!
//! * Zero-thickness strip: exact conformal-mapping solution
//!   (Cohn, IRE Trans. MTT-2, 1954):
//!   `Zo = (30π/√εr) · K(k)/K(k')`, `k = sech(πW/2B)`.
//! * Finite thickness: Wadell's effective-width correction (*Transmission
//!   Line Design Handbook*, 1991, §3.4.1; as used by wcalc). The thick strip
//!   of width `W` between planes `B` apart is replaced by a zero-thickness
//!   strip of width `W + ΔW` between planes `B − T` apart, and the exact
//!   zero-thickness solution is evaluated for that geometry. This reproduces
//!   the parallel-plate limit for wide strips and is continuous in every input.
//! * Offset (asymmetric) strip: the capacitance to each plane is taken as
//!   one half of the capacitance of a centered strip with twice that
//!   spacing, so `Zo = 2·Z1·Z2/(Z1 + Z2)` with `Zi = Zo,sym(W, 2Hi + T)`
//!   (Wadell §3.4.2). Exact when H1 = H2; fringing between the two half
//!   spaces is neglected otherwise.
//!
//! The complete elliptic integrals are evaluated with the arithmetic-
//! geometric mean, so the impedance is positive for every geometry the
//! model accepts; the logarithmic IPC-2141 approximation that previously
//! produced negative values for wide strips is no longer used.

use crate::impedance::{microstrip, types::ImpedanceResult};
use crate::math::elliptic_ratio;
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Cohn 1954 conformal-mapping stripline with Wadell thickness correction",
    status: ModelStatus::Validated,
    reference: "Cohn, IRE Trans. MTT-2 (1954); Wadell 1991 §3.4.1–3.4.2",
    validity: "Any W/B; T < B; W/B ≲ 400 (elliptic modulus underflow beyond). Exact for T = 0; ±1% for T/B ≤ 0.25",
};

/// Inputs for centered stripline impedance calculation. All dimensions in mils.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StriplineInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Dielectric height — gap from each face of the trace to its ground plane
    /// (mils). Plane-to-plane spacing is `2 × height + thickness`.
    pub height: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
}

/// Wadell effective-width increase ΔW for a strip of width `w` and
/// thickness `t` between planes `b` apart (all in the same unit).
pub fn wadell_delta_w(w: f64, b: f64, t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let m = 6.0 * (b - t) / (3.0 * b - t);
    (t / std::f64::consts::PI)
        * (1.0 - 0.5 * ((t / (2.0 * b - t)).powi(2) + (0.0796 * t / (w + 1.1 * t)).powf(m)).ln())
}

/// Validate a strip geometry and return the equivalent zero-thickness
/// geometry `(w_eff, b_eff)`.
pub(crate) fn thick_geometry(w: f64, b: f64, t: f64) -> Result<(f64, f64), CalcError> {
    validate::positive("width", w)?;
    validate::positive("plane spacing", b)?;
    validate::non_negative("thickness", t)?;
    if t >= b {
        return Err(CalcError::OutOfRange {
            name: "thickness",
            value: t,
            expected: "< plane-to-plane spacing",
        });
    }
    Ok((w + wadell_delta_w(w, b, t), b - t))
}

/// Characteristic impedance of a strip of width `w` and thickness `t`
/// centered between ground planes `b` apart, in a homogeneous dielectric `er`.
pub fn z0_symmetric(w: f64, b: f64, t: f64, er: f64) -> Result<f64, CalcError> {
    validate::er(er)?;
    let (we, be) = thick_geometry(w, b, t)?;
    let k = 1.0 / (std::f64::consts::PI * we / (2.0 * be)).cosh();
    let zo = 30.0 * std::f64::consts::PI / er.sqrt() * elliptic_ratio(k)?;
    validate::finite_result("zo", zo)
}

/// Characteristic impedance of an offset strip with gaps `h1` and `h2`
/// (face of the strip to each plane).
pub fn z0_offset(w: f64, h1: f64, h2: f64, t: f64, er: f64) -> Result<f64, CalcError> {
    validate::positive("height1", h1)?;
    validate::positive("height2", h2)?;
    let z1 = z0_symmetric(w, 2.0 * h1 + t, t, er)?;
    let z2 = z0_symmetric(w, 2.0 * h2 + t, t, er)?;
    validate::finite_result("zo", 2.0 * z1 * z2 / (z1 + z2))
}

/// Compute centered stripline characteristic impedance and derived quantities.
pub fn calculate(input: &StriplineInput) -> Result<ImpedanceResult, CalcError> {
    let StriplineInput {
        width,
        height,
        thickness,
        er,
    } = *input;
    validate::positive("height", height)?;
    validate::non_negative("thickness", thickness)?;
    let zo = z0_symmetric(width, 2.0 * height + thickness, thickness, er)?;
    // Fully embedded in one dielectric: εeff = εr, no dispersion.
    microstrip::finish(zo, er)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn sl(width: f64, height: f64, thickness: f64, er: f64) -> StriplineInput {
        StriplineInput {
            width,
            height,
            thickness,
            er,
        }
    }

    #[test]
    fn zero_thickness_matches_cohn_reference_points() {
        // K(k)/K(k') evaluated independently (AGM): W/B = 1 → 65.3989 Ω (εr = 1).
        assert_relative_eq!(
            z0_symmetric(10.0, 10.0, 0.0, 1.0).unwrap(),
            65.3989,
            max_relative = 1e-5
        );
        // W/B = 0.1 → 194.361 Ω; W/B = 10 → 9.0265 Ω.
        assert_relative_eq!(
            z0_symmetric(1.0, 10.0, 0.0, 1.0).unwrap(),
            194.361,
            max_relative = 1e-5
        );
        assert_relative_eq!(
            z0_symmetric(100.0, 10.0, 0.0, 1.0).unwrap(),
            9.0265,
            max_relative = 1e-4
        );
    }

    #[test]
    fn wide_strip_approaches_parallel_plate() {
        // Zo → 30π·B/(W√εr) / (1 + 0.441·B/W) as W/B → ∞.
        let z = z0_symmetric(1000.0, 10.0, 0.0, 4.6).unwrap();
        let pp = 30.0 * std::f64::consts::PI
            / 4.6_f64.sqrt()
            / (100.0 + 2.0 * 2f64.ln() / std::f64::consts::PI);
        assert_relative_eq!(z, pp, max_relative = 1e-4);
    }

    #[test]
    fn matches_wheeler_closed_form_within_half_percent() {
        // Wadell/Wheeler: Zo = 30/√εr · ln(1 + A(2A + √(4A² + 6.27))), A = 4(B−T)/(πW').
        for (w, h, t) in [
            (10.0, 10.0, 1.4),
            (3.0, 10.0, 1.4),
            (10.0, 4.0, 0.7),
            (25.0, 10.0, 2.8),
        ] {
            let b = 2.0 * h + t;
            let we = w + wadell_delta_w(w, b, t);
            let a = 4.0 * (b - t) / (std::f64::consts::PI * we);
            let wheeler =
                30.0 / 4.6_f64.sqrt() * (1.0 + a * (2.0 * a + (4.0 * a * a + 6.27).sqrt())).ln();
            let ours = calculate(&sl(w, h, t, 4.6)).unwrap().zo;
            assert_relative_eq!(ours, wheeler, max_relative = 5e-3);
        }
    }

    #[test]
    fn typical_geometry() {
        // W=10, H=10, T=1.4, εr=4.6 → 42.42 Ω (independent evaluation).
        let r = calculate(&sl(10.0, 10.0, 1.4, 4.6)).unwrap();
        assert_relative_eq!(r.zo, 42.423, max_relative = 1e-3);
        assert_relative_eq!(r.er_eff, 4.6, epsilon = 1e-12);
        assert!(r.tpd_ps_per_in > 0.0 && r.lo_nh_per_in > 0.0 && r.co_pf_per_in > 0.0);
    }

    #[test]
    fn wide_strip_is_positive_not_negative() {
        // Audit A07 case: the IPC-2141 log formula gave −37 Ω here.
        let r = calculate(&sl(100.0, 5.0, 1.4, 4.6)).unwrap();
        assert!(r.zo > 0.0 && r.co_pf_per_in > 0.0 && r.lo_nh_per_in > 0.0);
        assert_relative_eq!(r.zo, 4.1427, max_relative = 1e-3);
    }

    #[test]
    fn monotonic_in_width_and_er() {
        let mut prev = f64::INFINITY;
        let mut w = 0.5;
        while w < 500.0 {
            let z = calculate(&sl(w, 10.0, 1.4, 4.6)).unwrap().zo;
            assert!(z < prev, "w={w}");
            prev = z;
            w *= 1.05;
        }
        let low = calculate(&sl(10.0, 10.0, 1.4, 2.2)).unwrap();
        let high = calculate(&sl(10.0, 10.0, 1.4, 4.6)).unwrap();
        assert!(low.zo > high.zo);
    }

    #[test]
    fn thickness_lowers_impedance_continuously() {
        let z0 = calculate(&sl(10.0, 10.0, 0.0, 4.6)).unwrap().zo;
        let z_tiny = calculate(&sl(10.0, 10.0, 1e-6, 4.6)).unwrap().zo;
        let z_thick = calculate(&sl(10.0, 10.0, 1.4, 4.6)).unwrap().zo;
        assert_relative_eq!(z_tiny, z0, max_relative = 1e-4);
        assert!(z_thick < z0);
    }

    #[test]
    fn offset_is_symmetric_and_reduces_to_centered() {
        let centered = z0_offset(10.0, 10.0, 10.0, 1.4, 4.6).unwrap();
        assert_relative_eq!(
            centered,
            calculate(&sl(10.0, 10.0, 1.4, 4.6)).unwrap().zo,
            max_relative = 1e-12
        );
        let a = z0_offset(10.0, 1.0, 19.0, 1.4, 4.6).unwrap();
        let b = z0_offset(10.0, 19.0, 1.0, 1.4, 4.6).unwrap();
        assert_relative_eq!(a, b, max_relative = 1e-12);
        assert!(a < centered, "moving towards a plane must lower Zo");
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(calculate(&sl(-1.0, 10.0, 1.4, 4.6)).is_err());
        assert!(calculate(&sl(10.0, 10.0, 1.4, 0.5)).is_err());
        assert!(calculate(&sl(f64::NAN, 10.0, 1.4, 4.6)).is_err());
        assert!(calculate(&sl(10.0, 0.0, 1.4, 4.6)).is_err());
        assert!(z0_symmetric(10.0, 1.0, 1.0, 4.6).is_err()); // t >= b
    }
}
