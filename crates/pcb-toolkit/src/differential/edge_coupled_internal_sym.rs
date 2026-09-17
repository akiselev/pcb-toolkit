//! Edge-coupled internal symmetric (centered stripline) differential pair.
//!
//! Cohn, "Shielded Coupled-Strip Transmission Line", IRE Trans. MTT-3, 1955:
//!
//! * Zero thickness (exact conformal mapping):
//!   `ke = tanh(πW/2B)·tanh(π(W+S)/2B)`, `ko = tanh(πW/2B)·coth(π(W+S)/2B)`,
//!   `Zeven = (30π/√εr)·K(ke')/K(ke)`, `Zodd = (30π/√εr)·K(ko')/K(ko)`.
//! * Finite thickness (Cohn 1955 eq. 18, 20, 22, via fringing-capacitance
//!   ratios): the even mode uses eq. 18; the odd mode uses eq. 20 for
//!   S ≥ 5T and eq. 22 (which adds the parallel-plate capacitance between
//!   the facing edges) for S < 5T. Cohn's two odd-mode expressions differ
//!   by about 2% at S = 5T; eq. 22 is offset in admittance so the result is
//!   continuous there.
//!
//! The single-strip impedances come from [`crate::impedance::stripline`].

use super::types::{self, DifferentialResult};
use crate::impedance::stripline::z0_symmetric;
use crate::math::elliptic_ratio;
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, constants, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Cohn 1955 coupled stripline (conformal mapping + thickness corrections)",
    status: ModelStatus::Validated,
    reference: "Cohn, IRE Trans. MTT-3 (1955) eq. 2–6, 18, 20, 22; Wadell 1991 §4.3",
    validity: "Any W/B, S/B; T < B; exact for T = 0; thickness corrections ±2% for T/B ≤ 0.25",
};

/// Inputs for edge-coupled internal symmetric (centered stripline) differential pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeCoupledInternalSymInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Gap between traces (mils).
    pub spacing: f64,
    /// Dielectric height — gap from each face of the trace to its ground
    /// plane (mils). Plane-to-plane spacing is `2 × height + thickness`.
    pub height: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
}

/// Zero-thickness even- and odd-mode impedances (Cohn 1955 eq. 2–6) for
/// strips of width `w`, gap `s`, plane spacing `b`.
pub fn cohn_modes_thin(w: f64, s: f64, b: f64, er: f64) -> Result<(f64, f64), CalcError> {
    let a = (std::f64::consts::PI * w / (2.0 * b)).tanh();
    let c = (std::f64::consts::PI * (w + s) / (2.0 * b)).tanh();
    let ke = a * c;
    let ko = a / c;
    let scale = 30.0 * std::f64::consts::PI / er.sqrt();
    let ze = scale / elliptic_ratio(ke)?;
    let zo = scale / elliptic_ratio(ko)?;
    Ok((ze, zo))
}

/// Ratio of the thick-edge fringing capacitance to its zero-thickness value
/// (Cohn 1954 eq. 12 form), `x = T/B`.
fn fringe_ratio(x: f64) -> f64 {
    let y = 1.0 / (1.0 - x);
    (2.0 * y * (y + 1.0).ln() - (y - 1.0) * (y * y - 1.0).ln()) / (2.0 * std::f64::consts::LN_2)
}

/// Even- and odd-mode impedances of a coupled pair of width `w`, gap `s`,
/// thickness `t`, plane spacing `b` in dielectric `er`.
pub fn coupled_modes(w: f64, s: f64, b: f64, t: f64, er: f64) -> Result<(f64, f64), CalcError> {
    validate::positive("spacing", s)?;
    validate::er(er)?;
    let z0s = z0_symmetric(w, b, t, er)?; // validates w, b, t
    if t <= 0.0 {
        return cohn_modes_thin(w, s, b, er);
    }
    let z0s0 = z0_symmetric(w, b, 0.0, er)?;
    let (ze0, zo0) = cohn_modes_thin(w, s, b, er)?;
    let r = fringe_ratio(t / b);
    let eta = constants::ETA_0 / er.sqrt();
    let cf0 = 2.0 * std::f64::consts::LN_2 / std::f64::consts::PI;
    let cft = r * cf0;

    // Cohn eq. 18.
    let ye = 1.0 / z0s - r * (1.0 / z0s0 - 1.0 / ze0);

    // Cohn eq. 20 (S ≥ 5T) and eq. 22 (S < 5T).
    let y20 = |zo0: f64| 1.0 / z0s + r * (1.0 / zo0 - 1.0 / z0s0);
    let y22 = |zo0: f64, s: f64| {
        1.0 / zo0 + (1.0 / z0s - 1.0 / z0s0) - (2.0 / eta) * (cft - cf0) + 2.0 * t / (eta * s)
    };
    let yo = if s >= 5.0 * t {
        y20(zo0)
    } else {
        let s5 = 5.0 * t;
        let (_, zo0_5) = cohn_modes_thin(w, s5, b, er)?;
        y22(zo0, s) - y22(zo0_5, s5) + y20(zo0_5)
    };
    Ok((1.0 / ye, 1.0 / yo))
}

/// Compute differential impedance for an edge-coupled centered stripline pair.
pub fn calculate(input: &EdgeCoupledInternalSymInput) -> Result<DifferentialResult, CalcError> {
    let EdgeCoupledInternalSymInput {
        width,
        spacing,
        height,
        thickness,
        er,
    } = *input;
    validate::positive("height", height)?;
    validate::non_negative("thickness", thickness)?;
    let b = 2.0 * height + thickness;
    let zo = z0_symmetric(width, b, thickness, er)?;
    let (zeven, zodd) = coupled_modes(width, spacing, b, thickness, er)?;
    types::build(zo, zodd, zeven)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn input(
        width: f64,
        spacing: f64,
        height: f64,
        thickness: f64,
        er: f64,
    ) -> EdgeCoupledInternalSymInput {
        EdgeCoupledInternalSymInput {
            width,
            spacing,
            height,
            thickness,
            er,
        }
    }

    #[test]
    fn zero_thickness_matches_independent_cohn_evaluation() {
        // W=10, S=5, B=21.4, εr=4.6 (AGM evaluation): Zeven 56.153, Zodd 39.686.
        let (ze, zo) = cohn_modes_thin(10.0, 5.0, 21.4, 4.6).unwrap();
        assert_relative_eq!(ze, 56.1530, max_relative = 1e-5);
        assert_relative_eq!(zo, 39.6860, max_relative = 1e-5);
    }

    #[test]
    fn thick_case_matches_independent_evaluation() {
        // W=10, S=5, H=10, T=1.4, εr=4.6 (independent evaluation of Cohn eq. 18 and
        // the continuity-shifted eq. 22): Zeven 49.525, Zodd 33.565, Zo 42.423.
        let r = calculate(&input(10.0, 5.0, 10.0, 1.4, 4.6)).unwrap();
        assert_relative_eq!(r.zeven, 49.525, max_relative = 1e-3);
        assert_relative_eq!(r.zodd, 33.565, max_relative = 1e-3);
        assert_relative_eq!(r.zo, 42.423, max_relative = 1e-3);
        assert_relative_eq!(r.zdiff, 2.0 * r.zodd, max_relative = 1e-12);
    }

    #[test]
    fn modes_converge_to_single_strip_for_wide_spacing() {
        let r = calculate(&input(10.0, 500.0, 10.0, 1.4, 4.6)).unwrap();
        assert_relative_eq!(r.zodd, r.zo, max_relative = 1e-3);
        assert_relative_eq!(r.zeven, r.zo, max_relative = 1e-3);
        assert!(r.kb < 1e-3);
    }

    #[test]
    fn odd_mode_is_continuous_across_five_t_boundary() {
        let t = 1.4;
        let below = calculate(&input(10.0, 5.0 * t - 1e-6, 10.0, t, 4.6)).unwrap();
        let above = calculate(&input(10.0, 5.0 * t + 1e-6, 10.0, t, 4.6)).unwrap();
        assert_relative_eq!(below.zodd, above.zodd, max_relative = 1e-5);
        assert_relative_eq!(below.zeven, above.zeven, max_relative = 1e-5);
    }

    #[test]
    fn odd_mode_monotonic_in_spacing() {
        let mut prev = 0.0;
        let mut s = 0.5;
        while s < 100.0 {
            let z = calculate(&input(10.0, s, 10.0, 1.4, 4.6)).unwrap().zodd;
            assert!(z > prev, "s={s}: {z} <= {prev}");
            prev = z;
            s *= 1.1;
        }
    }

    #[test]
    fn wider_spacing_reduces_coupling() {
        let close = calculate(&input(10.0, 5.0, 63.0, 1.2, 4.0)).unwrap();
        let far = calculate(&input(10.0, 20.0, 63.0, 1.2, 4.0)).unwrap();
        assert!(far.kb < close.kb);
    }

    #[test]
    fn higher_er_gives_lower_z0() {
        let low_er = calculate(&input(10.0, 10.0, 63.0, 1.2, 2.2)).unwrap();
        let high_er = calculate(&input(10.0, 10.0, 63.0, 1.2, 4.6)).unwrap();
        assert!(high_er.zo < low_er.zo);
        assert_relative_eq!(
            high_er.zo * 4.6_f64.sqrt(),
            low_er.zo * 2.2_f64.sqrt(),
            max_relative = 1e-12
        );
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(calculate(&input(-1.0, 10.0, 63.0, 1.2, 4.0)).is_err());
        assert!(calculate(&input(10.0, 0.0, 63.0, 1.2, 4.0)).is_err());
        assert!(calculate(&input(10.0, 10.0, 63.0, 1.2, 0.5)).is_err());
        assert!(calculate(&input(10.0, f64::INFINITY, 63.0, 1.2, 4.0)).is_err());
    }
}
