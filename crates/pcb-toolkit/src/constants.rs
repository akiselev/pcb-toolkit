//! Physical constants used across calculators.

/// Speed of light in vacuum (m/s).
pub const SPEED_OF_LIGHT_MS: f64 = 299_792_458.0;

/// Speed of light in vacuum (in/ns) — convenience for transmission line calcs.
/// Exact: 299_792_458 m/s × (1 in / 0.0254 m) × (1 s / 1e9 ns) = 11.80285...
pub const SPEED_OF_LIGHT_IN_NS: f64 = 11.803;

/// Permeability of free space µ₀ (H/m).
pub const MU_0: f64 = 1.256_637_061_435_9e-6;

/// Permittivity of free space ε₀ (F/m).
pub const EPSILON_0: f64 = 8.854_187_817e-12;

/// Copper resistivity at 20°C (Ω·cm).
pub const COPPER_RESISTIVITY_OHM_CM: f64 = 1.724e-6;

/// Copper temperature coefficient (1/°C).
pub const COPPER_TEMP_COEFF: f64 = 0.00393;

/// Copper melting point (°C) — used in Onderdonk fusing equation.
pub const COPPER_MELTING_POINT_C: f64 = 1064.62;

/// 4/π — geometric correction for coaxial/via capacitance.
pub const FOUR_OVER_PI: f64 = 1.273_239_544_735_162_8;

/// 1 mil in meters.
pub const MIL_TO_M: f64 = 2.54e-5;

/// 1 inch in centimeters.
pub const INCH_TO_CM: f64 = 2.54;

/// Kirschning-Jansen dispersion constant a.
pub const KJ_DISPERSION_A: f64 = 0.457;

/// Kirschning-Jansen dispersion constant b.
pub const KJ_DISPERSION_B: f64 = 0.67;
