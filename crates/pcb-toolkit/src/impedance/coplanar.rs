//! Conductor-backed coplanar waveguide (CBCPW / grounded CPW) impedance.
//!
//! Zero-thickness model (Ghione & Naldi, Electronics Letters 1983; Wadell
//! 1991 §3.6.2; the same expressions as scikit-rf's `CPW` with
//! `has_metal_backside=True`):
//!
//! ```text
//! k  = W / (W + 2S)
//! k3 = tanh(πW/4H) / tanh(π(W + 2S)/4H)
//! q  = K(k)/K(k'),   q3 = K(k3)/K(k3')
//! εeff = (q + εr·q3) / (q + q3)
//! Zo   = 60π / (√εeff · (q + q3))
//! ```
//!
//! Finite thickness (Gupta, Garg, Bahl & Bhartia, *Microstrip Lines and
//! Slotlines*, §7.2.3): the strip is widened and the gaps narrowed by
//! `Δ = (1.25·T/π)·(1 + ln(4πW/T))`, and εeff is reduced by
//! `0.7·(εeff − 1)·(T/S) / (q + 0.7·T/S)`.
//!
//! The coplanar ground strips are assumed to be wide compared with the gaps.

use crate::impedance::{microstrip, types::ImpedanceResult};
use crate::math::elliptic_ratio;
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Conductor-backed CPW (Ghione-Naldi conformal mapping) with Gupta thickness correction",
    status: ModelStatus::Validated,
    reference: "Ghione & Naldi, Electron. Lett. 19 (1983); Wadell 1991 §3.6.2; Gupta et al. §7.2.3",
    validity: "Wide coplanar grounds; thickness correction assumes T ≪ S (t/S ≲ 0.1) and is rejected once Δ ≥ S; H not ≪ (W+2S)/24 (modulus resolution). Zero-thickness form checked to 1e-10 against an independent AGM evaluation",
};

/// Inputs for conductor-backed coplanar waveguide impedance calculation.
/// All dimensions in mils.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoplanarInput {
    /// Center conductor width (mils).
    pub width: f64,
    /// Gap between center conductor and each coplanar ground (mils).
    pub gap: f64,
    /// Substrate height to the bottom ground plane (mils).
    pub height: f64,
    /// Conductor thickness (mils). 0 selects the zero-thickness model.
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
}

/// Compute conductor-backed CPW impedance and derived quantities.
pub fn calculate(input: &CoplanarInput) -> Result<ImpedanceResult, CalcError> {
    let CoplanarInput {
        width,
        gap,
        height,
        thickness,
        er,
    } = *input;

    validate::positive("width", width)?;
    validate::positive("gap", gap)?;
    validate::positive("height", height)?;
    validate::non_negative("thickness", thickness)?;
    validate::er(er)?;

    // Thickness: widen the strip and narrow the gaps (Gupta et al.).
    let delta = if thickness > 0.0 {
        (1.25 * thickness / std::f64::consts::PI)
            * (1.0 + (4.0 * std::f64::consts::PI * width / thickness).ln())
    } else {
        0.0
    };
    if delta >= gap {
        return Err(CalcError::OutOfRange {
            name: "thickness",
            value: thickness,
            expected: "thickness correction Δ = 1.25T/π·(1 + ln(4πW/T)) must be smaller than the gap",
        });
    }
    let w = width + delta;
    let s = gap - delta;

    let k = w / (w + 2.0 * s);
    let k3 = (std::f64::consts::PI * w / (4.0 * height)).tanh()
        / (std::f64::consts::PI * (w + 2.0 * s) / (4.0 * height)).tanh();

    let q = elliptic_ratio(k)?;
    let q3 = elliptic_ratio(k3)?;

    let mut er_eff = (q + er * q3) / (q + q3);
    if thickness > 0.0 {
        let ts = thickness / gap;
        er_eff -= 0.7 * (er_eff - 1.0) * ts / (q + 0.7 * ts);
    }
    let zo = 60.0 * std::f64::consts::PI / (er_eff.sqrt() * (q + q3));

    microstrip::finish(zo, er_eff)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn cpw(width: f64, gap: f64, height: f64, thickness: f64, er: f64) -> CoplanarInput {
        CoplanarInput {
            width,
            gap,
            height,
            thickness,
            er,
        }
    }

    fn typical() -> CoplanarInput {
        cpw(10.0, 5.0, 10.0, 1.4, 4.6)
    }

    #[test]
    fn matches_independent_zero_thickness_reference() {
        // Independent AGM evaluation of the backed-CPW conformal-mapping model
        // (audit A03): W=10, S=5, H=5, εr=4.6 → 46.0957448 Ω, εeff 3.3419380.
        let r = calculate(&cpw(10.0, 5.0, 5.0, 0.0, 4.6)).unwrap();
        assert_relative_eq!(r.zo, 46.0957448064, max_relative = 1e-9);
        assert_relative_eq!(r.er_eff, 3.34193797552, max_relative = 1e-9);
    }

    #[test]
    fn er_eff_is_bounded_by_its_materials() {
        for (h, er) in [(2.0, 4.6), (5.0, 4.6), (100.0, 10.2), (0.5, 2.2)] {
            let r = calculate(&cpw(10.0, 5.0, h, 0.0, er)).unwrap();
            assert!(
                r.er_eff >= 1.0 && r.er_eff <= er,
                "h={h} er={er} -> {}",
                r.er_eff
            );
        }
    }

    #[test]
    fn air_filled_line_depends_on_ground_height() {
        let low = calculate(&cpw(10.0, 5.0, 2.0, 0.0, 1.0)).unwrap();
        let high = calculate(&cpw(10.0, 5.0, 100.0, 0.0, 1.0)).unwrap();
        assert!(low.zo < high.zo - 1.0, "low {} high {}", low.zo, high.zo);
        assert_relative_eq!(low.zo, 50.6287, max_relative = 1e-4);
        assert_relative_eq!(high.zo, 120.3543, max_relative = 1e-4);
    }

    #[test]
    fn far_bottom_ground_approaches_ungrounded_cpw() {
        // As H → ∞, k3 → k and εeff → (εr + 1)/2, Zo → 30π/(√εeff·q).
        let r = calculate(&cpw(10.0, 5.0, 1e5, 0.0, 4.6)).unwrap();
        let k: f64 = 0.5;
        let q = elliptic_ratio(k).unwrap();
        let expected = 30.0 * std::f64::consts::PI / (2.8_f64.sqrt() * q);
        assert_relative_eq!(r.er_eff, 2.8, max_relative = 1e-4);
        assert_relative_eq!(r.zo, expected, max_relative = 1e-4);
    }

    #[test]
    fn thickness_is_not_ignored_and_lowers_impedance() {
        let thin = calculate(&cpw(10.0, 5.0, 10.0, 0.0, 4.6)).unwrap();
        let thick = calculate(&typical()).unwrap();
        assert!(thick.zo < thin.zo);
        assert!(thick.er_eff < thin.er_eff);
        // Continuity at T → 0.
        let tiny = calculate(&cpw(10.0, 5.0, 10.0, 1e-9, 4.6)).unwrap();
        assert_relative_eq!(tiny.zo, thin.zo, max_relative = 1e-6);
    }

    #[test]
    fn narrower_gap_lowers_impedance() {
        let wide_gap = calculate(&typical()).unwrap();
        let narrow_gap = calculate(&CoplanarInput {
            gap: 4.0,
            ..typical()
        })
        .unwrap();
        assert!(narrow_gap.zo < wide_gap.zo);
        let wide_gap = calculate(&CoplanarInput {
            thickness: 0.0,
            ..typical()
        })
        .unwrap();
        let narrow_gap = calculate(&CoplanarInput {
            gap: 2.0,
            thickness: 0.0,
            ..typical()
        })
        .unwrap();
        assert!(narrow_gap.zo < wide_gap.zo);
    }

    #[test]
    fn higher_er_lowers_impedance() {
        let low_er = calculate(&typical()).unwrap();
        let high_er = calculate(&CoplanarInput {
            er: 9.8,
            ..typical()
        })
        .unwrap();
        assert!(high_er.zo < low_er.zo);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(
            calculate(&CoplanarInput {
                width: 0.0,
                ..typical()
            })
            .is_err()
        );
        assert!(
            calculate(&CoplanarInput {
                gap: -5.0,
                ..typical()
            })
            .is_err()
        );
        assert!(
            calculate(&CoplanarInput {
                height: 0.0,
                ..typical()
            })
            .is_err()
        );
        assert!(
            calculate(&CoplanarInput {
                er: 0.5,
                ..typical()
            })
            .is_err()
        );
        assert!(
            calculate(&CoplanarInput {
                width: f64::NAN,
                ..typical()
            })
            .is_err()
        );
        // Thickness correction larger than the gap.
        assert!(calculate(&cpw(10.0, 1.0, 10.0, 1.4, 4.6)).is_err());
    }

    #[test]
    fn derived_quantities_consistent() {
        let r = calculate(&typical()).unwrap();
        assert_relative_eq!(
            r.lo_nh_per_in,
            r.zo * r.tpd_ps_per_in / 1000.0,
            max_relative = 1e-10
        );
        assert_relative_eq!(r.co_pf_per_in, r.tpd_ps_per_in / r.zo, max_relative = 1e-10);
    }
}
