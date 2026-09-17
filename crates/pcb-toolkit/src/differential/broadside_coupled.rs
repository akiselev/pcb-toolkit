//! Broadside-coupled differential pair (two strips stacked vertically).
//!
//! # Shielded (between two ground planes)
//!
//! With the strips at gap `S`, thickness `T`, planes `B` apart, each strip
//! is `H = (B − S − 2T)/2` from its own plane. By symmetry the mid-plane
//! between the strips is an electric wall for the odd mode and a magnetic
//! wall for the even mode, so:
//!
//! * `Zodd`  = offset stripline with gaps `S/2` (to the electric wall) and `H`;
//! * `Zeven` = strip with a ground at `H` and nothing on the other side
//!   (`2·Zsym(2H + T)`, the magnetic-wall limit of the half-space model);
//! * `Zo`    = one strip alone: offset stripline with gaps `S + T + H` and `H`.
//!
//! All three use [`crate::impedance::stripline`] (Cohn 1954 + Wadell
//! thickness). The construction reproduces the parallel-plate limit
//! `Zdiff → η₀·S/(W·√εr)` for close, wide strips and never cancels to zero.
//!
//! # Unshielded (single ground plane)
//!
//! Not implemented. No validated closed form exists for this asymmetric
//! inhomogeneous structure in this crate; the request is rejected with
//! [`CalcError::Unsupported`] rather than answered with a guess.

use super::types::{self, DifferentialResult};
use crate::impedance::stripline::{z0_offset, z0_symmetric};
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Broadside-coupled stripline via electric/magnetic-wall symmetry and Cohn 1954 strips",
    status: ModelStatus::Compatibility,
    reference: "Cohn 1954; Wadell 1991 §3.4.2 and §4.4 (broadside symmetry)",
    validity: "Shielded only; B > S + 2T; ±5% estimated (inter-half-space fringing neglected); exact parallel-plate limit for S ≪ W",
};

/// Inputs for broadside-coupled differential pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BroadsideCoupledInput {
    /// Strip width (mils).
    pub width: f64,
    /// Vertical dielectric gap between the facing surfaces of the two strips (mils).
    pub separation: f64,
    /// Ground-to-ground spacing (mils).
    pub height_total: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
    /// true = shielded (between two ground planes). false is not supported.
    pub shielded: bool,
}

/// Compute differential impedance for a broadside-coupled differential pair.
pub fn calculate(input: &BroadsideCoupledInput) -> Result<DifferentialResult, CalcError> {
    let BroadsideCoupledInput {
        width,
        separation,
        height_total,
        thickness,
        er,
        shielded,
    } = *input;

    validate::positive("width", width)?;
    validate::positive("separation", separation)?;
    validate::positive("height_total", height_total)?;
    validate::non_negative("thickness", thickness)?;
    validate::er(er)?;
    if !shielded {
        return Err(CalcError::Unsupported(
            "unshielded broadside-coupled pair: no validated model is implemented; use shielded = true",
        ));
    }

    let h = (height_total - separation - 2.0 * thickness) / 2.0;
    if h <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "height_total",
            value: height_total,
            expected: "> separation + 2 × thickness (strips must fit between the planes)",
        });
    }

    let zodd = z0_offset(width, separation / 2.0, h, thickness, er)?;
    let zeven = 2.0 * z0_symmetric(width, 2.0 * h + thickness, thickness, er)?;
    let zo = z0_offset(width, separation + thickness + h, h, thickness, er)?;
    types::build(zo, zodd, zeven)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn shielded(
        width: f64,
        separation: f64,
        height_total: f64,
        thickness: f64,
        er: f64,
    ) -> BroadsideCoupledInput {
        BroadsideCoupledInput {
            width,
            separation,
            height_total,
            thickness,
            er,
            shielded: true,
        }
    }

    #[test]
    fn close_wide_strips_reach_parallel_plate_limit() {
        // Audit A06 case: W=10, S=0.1, B=30, T=0.01, εr=4.5. Parallel-plate
        // estimate η₀·S/(W√εr) = 1.776 Ω; the model gives 1.754 Ω.
        let r = calculate(&shielded(10.0, 0.1, 30.0, 0.01, 4.5)).unwrap();
        assert!(r.zdiff.is_finite() && r.zdiff > 0.0);
        let pp = crate::constants::ETA_0 * 0.1 / (10.0 * 4.5_f64.sqrt());
        assert_relative_eq!(r.zdiff, pp, max_relative = 0.02);
    }

    #[test]
    fn typical_geometry() {
        let r = calculate(&shielded(10.0, 5.0, 30.0, 2.0, 4.5)).unwrap();
        assert_relative_eq!(r.zdiff, 46.377, max_relative = 1e-3);
        assert!(r.zeven > r.zodd && r.zo > r.zodd);
    }

    #[test]
    fn wider_strip_lowers_z0() {
        let narrow = calculate(&shielded(5.0, 5.0, 30.0, 2.0, 4.5)).unwrap();
        let wide = calculate(&shielded(20.0, 5.0, 30.0, 2.0, 4.5)).unwrap();
        assert!(narrow.zo > wide.zo);
    }

    #[test]
    fn larger_separation_reduces_coupling() {
        let close = calculate(&shielded(10.0, 2.0, 30.0, 2.0, 4.5)).unwrap();
        let far = calculate(&shielded(10.0, 15.0, 30.0, 2.0, 4.5)).unwrap();
        assert!(far.kb < close.kb);
    }

    #[test]
    fn zdiff_monotonic_in_separation_while_partner_is_nearest() {
        // Increasing S raises Zodd as long as each strip is closer to its partner
        // than to its own plane (S/2 < H, i.e. S < (B − 2T)/2 = 13.6 mil here).
        // Beyond that the strip approaches its ground plane and Zdiff falls again.
        let mut prev = 0.0;
        let mut s = 0.05;
        while s < 13.0 {
            let z = calculate(&shielded(10.0, s, 30.0, 1.4, 4.5)).unwrap().zdiff;
            assert!(z > prev, "s={s}");
            prev = z;
            s *= 1.1;
        }
        let peak = calculate(&shielded(10.0, 13.6, 30.0, 1.4, 4.5))
            .unwrap()
            .zdiff;
        let beyond = calculate(&shielded(10.0, 20.0, 30.0, 1.4, 4.5))
            .unwrap()
            .zdiff;
        assert!(beyond < peak);
    }

    #[test]
    fn strips_must_fit_between_planes() {
        assert!(calculate(&shielded(10.0, 26.0, 30.0, 2.0, 4.5)).is_err());
        assert!(calculate(&shielded(10.0, 25.9, 30.0, 2.0, 4.5)).is_ok());
    }

    #[test]
    fn unshielded_is_rejected_explicitly() {
        let r = calculate(&BroadsideCoupledInput {
            shielded: false,
            ..shielded(10.0, 5.0, 30.0, 2.0, 4.5)
        });
        assert!(matches!(r, Err(CalcError::Unsupported(_))));
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(calculate(&shielded(-1.0, 5.0, 30.0, 2.0, 4.5)).is_err());
        assert!(calculate(&shielded(10.0, 5.0, 30.0, 2.0, 0.5)).is_err());
        assert!(calculate(&shielded(10.0, f64::NAN, 30.0, 2.0, 4.5)).is_err());
    }
}
