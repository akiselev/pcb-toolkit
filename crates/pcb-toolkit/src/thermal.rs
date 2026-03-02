//! Thermal management calculator.
//!
//! Computes junction temperature using the thermal resistance model:
//! T_junction = R_theta_ja × P_dissipated + T_ambient

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// Inputs for thermal management calculation.
pub struct ThermalInput {
    /// Thermal resistance, junction-to-ambient (°C/W).
    pub r_theta_ja: f64,
    /// Power dissipation (W).
    pub power_w: f64,
    /// Ambient temperature (°C).
    pub t_ambient_c: f64,
}

/// Result of a thermal management calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThermalResult {
    /// Junction temperature (°C).
    pub t_junction_c: f64,
    /// Junction temperature (°F).
    pub t_junction_f: f64,
}

/// Calculate junction temperature.
pub fn calculate(input: &ThermalInput) -> Result<ThermalResult, CalcError> {
    if input.r_theta_ja <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "r_theta_ja",
            value: input.r_theta_ja,
            expected: "> 0",
        });
    }
    if input.power_w < 0.0 {
        return Err(CalcError::OutOfRange {
            name: "power_w",
            value: input.power_w,
            expected: ">= 0",
        });
    }

    let t_junction_c = input.r_theta_ja * input.power_w + input.t_ambient_c;
    let t_junction_f = 1.8 * t_junction_c + 32.0;

    Ok(ThermalResult { t_junction_c, t_junction_f })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn basic_junction_temp() {
        let result = calculate(&ThermalInput {
            r_theta_ja: 50.0,
            power_w: 1.0,
            t_ambient_c: 25.0,
        })
        .unwrap();

        assert_relative_eq!(result.t_junction_c, 75.0, epsilon = 1e-10);
        assert_relative_eq!(result.t_junction_f, 167.0, epsilon = 1e-10);
    }

    #[test]
    fn high_thermal_resistance() {
        let result = calculate(&ThermalInput {
            r_theta_ja: 100.0,
            power_w: 0.5,
            t_ambient_c: 25.0,
        })
        .unwrap();

        assert_relative_eq!(result.t_junction_c, 75.0, epsilon = 1e-10);
    }

    #[test]
    fn zero_power() {
        let result = calculate(&ThermalInput {
            r_theta_ja: 50.0,
            power_w: 0.0,
            t_ambient_c: 25.0,
        })
        .unwrap();

        assert_relative_eq!(result.t_junction_c, 25.0, epsilon = 1e-10);
    }

    #[test]
    fn invalid_r_theta() {
        assert!(calculate(&ThermalInput {
            r_theta_ja: -1.0,
            power_w: 1.0,
            t_ambient_c: 25.0,
        })
        .is_err());
    }
}
