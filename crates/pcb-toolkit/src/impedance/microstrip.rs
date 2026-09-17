//! Surface microstrip impedance calculator.
//!
//! Quasi-static model: Hammerstad & Jensen, "Accurate Models for Microstrip
//! Computer-Aided Design", IEEE MTT-S Digest, 1980 (free-space impedance,
//! effective permittivity and conductor-thickness corrections).
//!
//! Dispersion: Kirschning & Jansen, "Accurate model for effective dielectric
//! constant of microstrip with validity up to millimetre-wave frequencies",
//! Electronics Letters 18(6), 1982, with the Hammerstad-Jensen impedance
//! dispersion form.
//!
//! These are the same models the Saturn PCB Toolkit binary implements
//! (see `docs/notes/stripline-formulas-clean.md`, Steps 3–9).

use crate::impedance::{common, types::ImpedanceResult};
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, constants, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Hammerstad-Jensen 1980 microstrip + Kirschning-Jansen 1982 dispersion",
    status: ModelStatus::Validated,
    reference: "Hammerstad & Jensen, IEEE MTT-S 1980; Kirschning & Jansen, Electron. Lett. 1982",
    validity: "0.01 ≤ W/H ≤ 100, 1 ≤ εr ≤ 128, T < H (Zo ±0.03%, εeff ±0.2% quasi-static); dispersion for 0.1 ≤ W/H ≤ 100, εr ≤ 20, H/λ₀ ≤ 0.13 (±0.6%)",
};

/// Inputs for microstrip impedance calculation. All dimensions in mils.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MicrostripInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Dielectric height — distance from trace to ground plane (mils).
    pub height: f64,
    /// Conductor thickness (mils). 0 selects the zero-thickness model.
    pub thickness: f64,
    /// Substrate relative permittivity (e.g., 4.6 for FR-4).
    pub er: f64,
    /// Frequency (Hz). 0 = quasi-static; > 0 applies Kirschning-Jansen dispersion.
    pub frequency: f64,
}

/// Compute microstrip characteristic impedance and derived quantities.
pub fn calculate(input: &MicrostripInput) -> Result<ImpedanceResult, CalcError> {
    let MicrostripInput {
        width,
        height,
        thickness,
        er,
        frequency,
    } = *input;

    validate::positive("width", width)?;
    validate::positive("height", height)?;
    validate::non_negative("thickness", thickness)?;
    validate::in_range("er", er, 1.0, 128.0, "1.0 ..= 128.0")?;
    validate::non_negative("frequency", frequency)?;

    let u = width / height;
    validate::in_range(
        "W/H",
        u,
        0.01,
        100.0,
        "0.01 ..= 100 (Hammerstad-Jensen validity range)",
    )?;
    if thickness >= height {
        return Err(CalcError::OutOfRange {
            name: "thickness",
            value: thickness,
            expected: "< height",
        });
    }

    let (u1, ur) = common::hj_thickness(u, thickness / height, er);
    let er_eff_ur = common::hj_er_eff(ur, er);
    // Thickness lowers εeff slightly (Hammerstad-Jensen 1980, eq. 10).
    let er_eff_static = er_eff_ur * (common::hj_z01(u1) / common::hj_z01(ur)).powi(2);
    let zo_static = common::hj_z01(ur) / er_eff_ur.sqrt();

    let (zo, er_eff) = if frequency > 0.0 {
        validate::in_range(
            "er",
            er,
            1.0,
            20.0,
            "≤ 20 for the Kirschning-Jansen dispersion model",
        )?;
        validate::in_range(
            "W/H",
            u,
            0.1,
            100.0,
            "≥ 0.1 for the Kirschning-Jansen dispersion model",
        )?;
        let h_over_lambda0 =
            frequency * height * constants::MIL_TO_M / constants::SPEED_OF_LIGHT_MS;
        if h_over_lambda0 > 0.13 {
            return Err(CalcError::OutOfRange {
                name: "frequency",
                value: frequency,
                expected: "H/λ₀ ≤ 0.13 (Kirschning-Jansen validity limit)",
            });
        }
        let er_eff_f = common::kirschning_jansen_er_eff(er, er_eff_static, ur, height, frequency);
        (
            common::z0_dispersion(zo_static, er_eff_static, er_eff_f),
            er_eff_f,
        )
    } else {
        (zo_static, er_eff_static)
    };

    finish(zo, er_eff)
}

/// Build the result struct from Zo and Er_eff with finite checks.
pub(crate) fn finish(zo: f64, er_eff: f64) -> Result<ImpedanceResult, CalcError> {
    let zo = validate::finite_result("zo", zo)?;
    let er_eff = validate::finite_result("er_eff", er_eff)?;
    if zo <= 0.0 {
        return Err(CalcError::NonFiniteResult {
            name: "zo (non-positive)",
        });
    }
    let tpd = common::propagation_delay(er_eff);
    Ok(ImpedanceResult {
        zo,
        er_eff,
        tpd_ps_per_in: validate::finite_result("tpd", tpd)?,
        lo_nh_per_in: validate::finite_result("lo", common::inductance_per_length(zo, tpd))?,
        co_pf_per_in: validate::finite_result("co", common::capacitance_per_length(zo, tpd))?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn ms(width: f64, height: f64, thickness: f64, er: f64, frequency: f64) -> MicrostripInput {
        MicrostripInput {
            width,
            height,
            thickness,
            er,
            frequency,
        }
    }

    #[test]
    fn basic_microstrip() {
        let result = calculate(&ms(10.0, 5.0, 1.4, 4.6, 0.0)).unwrap();
        // Independent evaluation of the H-J closed forms: 44.832 Ω, εeff 3.3075.
        assert_relative_eq!(result.zo, 44.832, max_relative = 1e-3);
        assert_relative_eq!(result.er_eff, 3.3075, max_relative = 1e-3);
        assert!(
            result.tpd_ps_per_in > 0.0 && result.lo_nh_per_in > 0.0 && result.co_pf_per_in > 0.0
        );
    }

    #[test]
    fn zero_thickness_matches_hammerstad_1975_within_half_percent() {
        // The older Hammerstad/Wheeler closed form (60/√εeff·ln(8h/w + w/4h)) agrees
        // with H-J 1980 to <0.5% for ordinary geometries.
        let r = calculate(&ms(10.0, 5.0, 0.0, 4.6, 0.0)).unwrap();
        assert_relative_eq!(r.zo, 47.911, max_relative = 5e-3);
        let r = calculate(&ms(5.0, 5.0, 0.0, 2.2, 0.0)).unwrap();
        assert_relative_eq!(r.zo, 95.265, max_relative = 5e-3);
    }

    #[test]
    fn narrow_trace_higher_impedance() {
        let narrow = calculate(&ms(3.0, 5.0, 1.4, 4.6, 0.0)).unwrap();
        let wide = calculate(&ms(20.0, 5.0, 1.4, 4.6, 0.0)).unwrap();
        assert!(narrow.zo > wide.zo);
    }

    #[test]
    fn impedance_is_continuous_and_monotonic_in_width() {
        // Sweep W/H from 0.05 to 20 in fine steps; Zo must fall monotonically with
        // no step larger than the local slope allows (audit A04).
        let mut prev = f64::INFINITY;
        let mut w = 0.5;
        while w <= 200.0 {
            let z = calculate(&ms(w, 10.0, 1.4, 4.6, 0.0)).unwrap().zo;
            assert!(z < prev, "Zo not monotonic at w={w}: {z} >= {prev}");
            assert!(
                prev.is_infinite() || prev - z < 0.05 * prev,
                "step too large at w={w}: {prev} -> {z}"
            );
            prev = z;
            w *= 1.01;
        }
    }

    #[test]
    fn no_branch_jump_at_old_pi_over_two_boundary() {
        let boundary = std::f64::consts::PI * 10.0 / 2.0;
        let below = calculate(&ms(boundary - 1e-6, 10.0, 0.1, 4.6, 0.0)).unwrap();
        let above = calculate(&ms(boundary + 1e-6, 10.0, 0.1, 4.6, 0.0)).unwrap();
        assert_relative_eq!(above.zo, below.zo, max_relative = 1e-5);
    }

    #[test]
    fn thickness_lowers_impedance() {
        let thin = calculate(&ms(10.0, 5.0, 0.0, 4.6, 0.0)).unwrap();
        let thick = calculate(&ms(10.0, 5.0, 1.4, 4.6, 0.0)).unwrap();
        assert!(thick.zo < thin.zo);
    }

    #[test]
    fn air_line_has_er_eff_one() {
        let r = calculate(&ms(10.0, 5.0, 1.4, 1.0, 0.0)).unwrap();
        assert_relative_eq!(r.er_eff, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn frequency_is_not_ignored() {
        let dc = calculate(&ms(10.0, 10.0, 1.4, 4.6, 0.0)).unwrap();
        let rf = calculate(&ms(10.0, 10.0, 1.4, 4.6, 10e9)).unwrap();
        assert!(
            rf.er_eff > dc.er_eff + 1e-3,
            "dc {} rf {}",
            dc.er_eff,
            rf.er_eff
        );
        assert!(rf.er_eff < 4.6);
        assert!((rf.zo - dc.zo).abs() > 1e-3);
        // Independent evaluation: εeff(10 GHz) = 3.1688.
        assert_relative_eq!(rf.er_eff, 3.1688, max_relative = 1e-3);
        // Low frequency reduces to the static result.
        let lf = calculate(&ms(10.0, 10.0, 1.4, 4.6, 1e6)).unwrap();
        assert_relative_eq!(lf.zo, dc.zo, max_relative = 1e-6);
    }

    #[test]
    fn dispersion_range_is_enforced() {
        // H/λ₀ = 0.13 at f = 0.13·c/h; h = 62 mil → 24.7 GHz.
        assert!(calculate(&ms(10.0, 62.0, 1.4, 4.6, 30e9)).is_err());
        assert!(calculate(&ms(10.0, 62.0, 1.4, 4.6, 20e9)).is_ok());
        assert!(calculate(&ms(10.0, 10.0, 1.4, 25.0, 1e9)).is_err());
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(calculate(&ms(-1.0, 5.0, 1.4, 4.6, 0.0)).is_err());
        assert!(calculate(&ms(f64::NAN, 5.0, 1.4, 4.6, 0.0)).is_err());
        assert!(calculate(&ms(10.0, 5.0, 5.0, 4.6, 0.0)).is_err());
        assert!(calculate(&ms(10.0, 5.0, 1.4, 0.9, 0.0)).is_err());
        assert!(calculate(&ms(10.0, 5.0, 1.4, 4.6, -1.0)).is_err());
        assert!(calculate(&ms(0.05, 10.0, 0.0, 4.6, 0.0)).is_err()); // W/H < 0.01
    }
}
