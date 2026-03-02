//! Shared impedance computation helpers.
//!
//! Effective dielectric constant, conductor thickness correction,
//! Kirschning-Jansen frequency dispersion — used by all topology modules.

use crate::constants;

/// Static effective dielectric constant (Hammerstad-Jensen).
///
/// `u` = W/H ratio, `er` = substrate relative permittivity.
pub fn er_eff_static(u: f64, er: f64) -> f64 {
    let f = if u <= 1.0 {
        (1.0 + 12.0 / u).powf(-0.5) + 0.04 * (1.0 - u).powi(2)
    } else {
        (1.0 + 12.0 / u).powf(-0.5)
    };
    (er + 1.0) / 2.0 + (er - 1.0) / 2.0 * f
}

/// Conductor thickness correction — effective width increase due to finite thickness.
///
/// `w` = conductor width (mils), `h` = dielectric height (mils), `t` = conductor thickness (mils).
/// Returns the effective width We (mils).
pub fn effective_width(w: f64, h: f64, t: f64) -> f64 {
    if t <= 0.0 {
        return w;
    }
    let u = w / h;
    let dw = if u >= std::f64::consts::FRAC_PI_2 {
        (t / std::f64::consts::PI) * (1.0 + (2.0 * h / t).ln())
    } else {
        (t / std::f64::consts::PI) * (1.0 + (4.0 * std::f64::consts::PI * w / t).ln())
    };
    w + dw
}

/// Propagation delay from Er_eff (ps/in).
pub fn propagation_delay(er_eff: f64) -> f64 {
    // Tpd = sqrt(Er_eff) / c, where c = 11.803 in/ns = 11803 in/µs
    // Result in ps/in: (sqrt(Er_eff) / 11.803) * 1000
    er_eff.sqrt() / constants::SPEED_OF_LIGHT_IN_NS * 1000.0
}

/// Inductance per unit length from Zo and Tpd (nH/in).
pub fn inductance_per_length(zo: f64, tpd_ps_per_in: f64) -> f64 {
    // Lo = Zo × Tpd, with Tpd in ns/in → Lo in nH/in
    zo * tpd_ps_per_in / 1000.0
}

/// Capacitance per unit length from Zo and Tpd (pF/in).
pub fn capacitance_per_length(zo: f64, tpd_ps_per_in: f64) -> f64 {
    // Co = Tpd / Zo, with Tpd in ns/in → Co in nF/in → ×1000 for pF/in
    (tpd_ps_per_in / 1000.0) / zo * 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn er_eff_fr4_wide_trace() {
        // W/H = 2.0, Er = 4.6 → Er_eff should be roughly 3.5-3.8
        let er_eff = er_eff_static(2.0, 4.6);
        assert!(er_eff > 3.0 && er_eff < 4.6, "er_eff = {er_eff}");
    }

    #[test]
    fn er_eff_narrow_trace() {
        // W/H = 0.5, Er = 4.6 → Er_eff should be lower (more field in air)
        let narrow = er_eff_static(0.5, 4.6);
        let wide = er_eff_static(2.0, 4.6);
        assert!(narrow < wide, "narrow {narrow} should be < wide {wide}");
    }

    #[test]
    fn thickness_correction_increases_width() {
        let w = 5.0; // mils
        let h = 4.0;
        let t = 1.4; // 1oz copper
        let we = effective_width(w, h, t);
        assert!(we > w, "effective width {we} should be > original {w}");
    }
}
