//! Shared quasi-static microstrip expressions.
//!
//! * Hammerstad & Jensen (1980) closed forms for the free-space impedance,
//!   static effective permittivity and conductor-thickness corrections.
//! * Kirschning & Jansen (1982) dispersion of the effective permittivity.
//! * Unit helpers for propagation delay and per-length L / C.

use crate::constants;

/// Free-space (εr = 1) characteristic impedance of a zero-thickness
/// microstrip with normalised width `u = W/H` (Hammerstad-Jensen 1980, eq. 1).
///
/// Stated accuracy: better than 0.01% for u ≤ 1 and 0.03% for u ≤ 1000.
pub fn hj_z01(u: f64) -> f64 {
    let f_u = 6.0 + (2.0 * std::f64::consts::PI - 6.0) * (-((30.666 / u).powf(0.7528))).exp();
    constants::ETA_0 / (2.0 * std::f64::consts::PI)
        * (f_u / u + (1.0 + (2.0 / u).powi(2)).sqrt()).ln()
}

/// Static effective permittivity of a zero-thickness microstrip
/// (Hammerstad-Jensen 1980, eq. 3). Accuracy better than 0.2% for
/// εr ≤ 128 and 0.01 ≤ u ≤ 100.
pub fn hj_er_eff(u: f64, er: f64) -> f64 {
    let a = 1.0
        + ((u.powi(4) + (u / 52.0).powi(2)) / (u.powi(4) + 0.432)).ln() / 49.0
        + (1.0 + (u / 18.1).powi(3)).ln() / 18.7;
    let b = 0.564 * ((er - 0.9) / (er + 3.0)).powf(0.053);
    (er + 1.0) / 2.0 + (er - 1.0) / 2.0 * (1.0 + 10.0 / u).powf(-a * b)
}

/// Hammerstad-Jensen conductor-thickness corrections.
///
/// Returns `(u1, ur)`: the effective normalised widths to use for the
/// homogeneous (free-space) impedance and for the mixed-dielectric
/// permittivity respectively. `t_over_h` is the strip thickness divided by
/// the substrate height. Both corrections are continuous in `u`.
pub fn hj_thickness(u: f64, t_over_h: f64, er: f64) -> (f64, f64) {
    if t_over_h <= 0.0 {
        return (u, u);
    }
    let coth = 1.0 / (6.517 * u).sqrt().tanh();
    let du1 = (t_over_h / std::f64::consts::PI)
        * (1.0 + 4.0 * std::f64::consts::E / (t_over_h * coth * coth)).ln();
    let dur = 0.5 * (1.0 + 1.0 / (er - 1.0).sqrt().cosh()) * du1;
    (u + du1, u + dur)
}

/// Kirschning-Jansen (1982) frequency-dependent effective permittivity.
///
/// `er_eff_static` is the quasi-static value, `u = W/H`, `h_mils` the
/// substrate height and `f_hz` the frequency. Stated accuracy: better than
/// 0.6% for 1 ≤ εr ≤ 20, 0.1 ≤ u ≤ 100 and h/λ₀ ≤ 0.13.
pub fn kirschning_jansen_er_eff(
    er: f64,
    er_eff_static: f64,
    u: f64,
    h_mils: f64,
    f_hz: f64,
) -> f64 {
    // The published fit uses f in GHz and h in cm.
    let fh = (f_hz / 1e9) * (h_mils * constants::MIL_TO_M * 100.0);
    let p1 = 0.27488 + (0.6315 + 0.525 / (1.0 + 0.157 * fh).powi(20)) * u
        - 0.065683 * (-8.7513 * u).exp();
    let p2 = 0.33622 * (1.0 - (-0.03442 * er).exp());
    let p3 = 0.0363 * (-4.6 * u).exp() * (1.0 - (-((fh / 3.87).powf(4.97))).exp());
    let p4 = 1.0 + 2.751 * (1.0 - (-((er / 15.916).powi(8))).exp());
    let p = p1 * p2 * ((0.1844 + p3 * p4) * 10.0 * fh).powf(1.5763);
    er - (er - er_eff_static) / (1.0 + p)
}

/// Frequency-dependent characteristic impedance from the dispersive
/// effective permittivity (Hammerstad-Jensen 1980 form):
/// `Z(f) = Z(0)·√(εeff(0)/εeff(f))·(εeff(f) − 1)/(εeff(0) − 1)`.
///
/// For a homogeneous line (εeff(0) = εr = 1) the impedance is dispersion-free.
pub fn z0_dispersion(z0_static: f64, er_eff_static: f64, er_eff_f: f64) -> f64 {
    if er_eff_static - 1.0 <= 1e-12 {
        return z0_static;
    }
    z0_static * (er_eff_static / er_eff_f).sqrt() * (er_eff_f - 1.0) / (er_eff_static - 1.0)
}

/// Static effective dielectric constant of a zero-thickness microstrip.
///
/// This is the Hammerstad-Jensen 1980 expression (see [`hj_er_eff`]); kept
/// under this name for callers that only need a velocity estimate.
pub fn er_eff_static(u: f64, er: f64) -> f64 {
    hj_er_eff(u, er)
}

/// Wheeler/Schneider conductor-thickness correction: effective width
/// increase due to finite thickness.
///
/// `w` = conductor width, `h` = dielectric height, `t` = conductor thickness
/// (all in the same unit). The two logarithmic branches meet at
/// `w/h = 1/(2π)`, which is where the branch is switched, so the returned
/// width is continuous in `w`. Applicable for `t < h` and `t < w/2`.
///
/// [`hj_thickness`] is the preferred correction for impedance work; this
/// helper is retained for callers that need a plain effective width.
pub fn effective_width(w: f64, h: f64, t: f64) -> f64 {
    if t <= 0.0 {
        return w;
    }
    let u = w / h;
    let dw = if u >= 1.0 / (2.0 * std::f64::consts::PI) {
        (t / std::f64::consts::PI) * (1.0 + (2.0 * h / t).ln())
    } else {
        (t / std::f64::consts::PI) * (1.0 + (4.0 * std::f64::consts::PI * w / t).ln())
    };
    w + dw
}

/// Propagation delay from Er_eff (ps/in).
pub fn propagation_delay(er_eff: f64) -> f64 {
    er_eff.sqrt() / constants::SPEED_OF_LIGHT_IN_NS * 1000.0
}

/// Inductance per unit length from Zo and Tpd (nH/in).
pub fn inductance_per_length(zo: f64, tpd_ps_per_in: f64) -> f64 {
    zo * tpd_ps_per_in / 1000.0
}

/// Capacitance per unit length from Zo and Tpd (pF/in).
pub fn capacitance_per_length(zo: f64, tpd_ps_per_in: f64) -> f64 {
    tpd_ps_per_in / zo
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn er_eff_fr4_wide_trace() {
        let er_eff = er_eff_static(2.0, 4.6);
        assert!(er_eff > 3.0 && er_eff < 4.6, "er_eff = {er_eff}");
    }

    #[test]
    fn er_eff_narrow_trace() {
        let narrow = er_eff_static(0.5, 4.6);
        let wide = er_eff_static(2.0, 4.6);
        assert!(narrow < wide, "narrow {narrow} should be < wide {wide}");
    }

    #[test]
    fn er_eff_bounded_by_materials() {
        for u in [0.01, 0.1, 1.0, 10.0, 100.0] {
            for er in [1.0, 2.2, 4.6, 10.2, 128.0] {
                let e = hj_er_eff(u, er);
                assert!(e >= 1.0 - 1e-12 && e <= er + 1e-12, "u={u} er={er} e={e}");
            }
        }
    }

    #[test]
    fn hj_z01_wide_limit_is_parallel_plate() {
        // Z → η₀ h / w for very wide strips.
        let u = 1000.0;
        assert_relative_eq!(hj_z01(u), constants::ETA_0 / u, max_relative = 0.01);
    }

    #[test]
    fn thickness_correction_increases_width() {
        let we = effective_width(5.0, 4.0, 1.4);
        assert!(we > 5.0);
        let (u1, ur) = hj_thickness(1.0, 0.1, 4.6);
        assert!(u1 > 1.0 && ur > 1.0 && ur < u1);
    }

    #[test]
    fn effective_width_is_continuous_at_branch() {
        let h = 10.0;
        let t = 1.4;
        let w0 = h / (2.0 * std::f64::consts::PI);
        let below = effective_width(w0 - 1e-9, h, t);
        let above = effective_width(w0 + 1e-9, h, t);
        assert!((above - below).abs() < 1e-6, "below={below} above={above}");
    }

    #[test]
    fn dispersion_raises_er_eff_towards_er() {
        let e0 = hj_er_eff(1.0, 4.6);
        let e10 = kirschning_jansen_er_eff(4.6, e0, 1.0, 10.0, 10e9);
        assert!(e10 > e0 && e10 < 4.6, "e0={e0} e10={e10}");
        let e0_1 = kirschning_jansen_er_eff(4.6, e0, 1.0, 10.0, 1e6);
        assert_relative_eq!(e0_1, e0, max_relative = 1e-6);
    }
}
