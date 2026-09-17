//! Physical constants used across calculators.

/// Speed of light in vacuum (m/s).
pub const SPEED_OF_LIGHT_MS: f64 = 299_792_458.0;

/// Speed of light in vacuum (in/ns).
///
/// Saturn PCB Toolkit uses the rounded value 11.803; the exact value is
/// 11.802852677. The rounded value is kept for compatibility with Saturn
/// test vectors (0.001% effect).
pub const SPEED_OF_LIGHT_IN_NS: f64 = 11.803;

/// Free-space wave impedance η₀ = μ₀·c (Ω), CODATA 2018.
pub const ETA_0: f64 = 376.730_313_668;

/// Permeability of free space µ₀ (H/m).
pub const MU_0: f64 = 1.256_637_061_435_9e-6;

/// Permittivity of free space ε₀ (F/m).
pub const EPSILON_0: f64 = 8.854_187_817e-12;

/// Copper resistivity at 20 °C (Ω·m). Annealed copper, IACS.
pub const COPPER_RESISTIVITY_OHM_M: f64 = 1.724e-8;

/// Copper resistivity at 20 °C (Ω·cm).
pub const COPPER_RESISTIVITY_OHM_CM: f64 = COPPER_RESISTIVITY_OHM_M * 100.0;

/// Copper resistivity at 20 °C expressed in Ω·mil, for `R = ρ·L/A` with L in
/// mils and A in mil². Equals 1.724e-8 Ω·m × 39 370.08 mil/m = 6.787e-4 Ω·mil,
/// the value used by Saturn PCB Toolkit.
pub const COPPER_RESISTIVITY_OHM_MIL: f64 = COPPER_RESISTIVITY_OHM_M / MIL_TO_M;

/// Copper temperature coefficient of resistance (1/°C) referenced to 20 °C.
pub const COPPER_TEMP_COEFF: f64 = 0.00393;

/// Reference temperature for [`COPPER_RESISTIVITY_OHM_M`] and
/// [`COPPER_TEMP_COEFF`] (°C).
pub const COPPER_RESISTIVITY_REF_C: f64 = 20.0;

/// Copper melting point (°C). RSC / CRC value: 1084.62 °C.
pub const COPPER_MELTING_POINT_C: f64 = 1084.62;

/// 4/π — converts square area to circular mils.
pub const FOUR_OVER_PI: f64 = 1.273_239_544_735_162_8;

/// 1 mil in meters.
pub const MIL_TO_M: f64 = 2.54e-5;

/// 1 inch in centimeters.
pub const INCH_TO_CM: f64 = 2.54;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn melting_point_is_the_accepted_value() {
        assert_relative_eq!(COPPER_MELTING_POINT_C, 1084.62, epsilon = 1e-9);
    }

    #[test]
    fn ohm_mil_resistivity_matches_saturn_constant() {
        assert_relative_eq!(COPPER_RESISTIVITY_OHM_MIL, 6.787e-4, max_relative = 1e-4);
    }

    #[test]
    fn eta0_matches_mu0_times_c() {
        assert_relative_eq!(ETA_0, MU_0 * SPEED_OF_LIGHT_MS, max_relative = 1e-9);
    }
}
