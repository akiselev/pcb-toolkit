//! Copper weight to thickness conversion and etched cross-section geometry.

use serde::{Deserialize, Serialize};

use crate::{CalcError, validate};

/// Millimetres per mil.
const MM_PER_MIL: f64 = 0.0254;

/// Standard copper weight options (oz/ft²).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CopperWeight {
    /// 0.25 oz/ft²
    Oz025,
    /// 0.5 oz/ft²
    Oz05,
    /// 1 oz/ft²
    Oz1,
    /// 1.5 oz/ft²
    Oz15,
    /// 2 oz/ft²
    Oz2,
    /// 2.5 oz/ft²
    Oz25,
    /// 3 oz/ft²
    Oz3,
    /// 4 oz/ft²
    Oz4,
    /// 5 oz/ft²
    Oz5,
}

impl CopperWeight {
    /// Nominal copper thickness in mils. This is the single canonical value;
    /// every other unit is derived from it.
    ///
    /// Uses the Saturn / industry rule of 1.4 mil per oz/ft² (the IPC-4562
    /// nominal is 1.37 mil = 34.8 µm; the difference is inside the ±10%
    /// foil tolerance).
    pub fn thickness_mils(self) -> f64 {
        match self {
            Self::Oz025 => 0.35,
            Self::Oz05 => 0.70,
            Self::Oz1 => 1.40,
            Self::Oz15 => 2.10,
            Self::Oz2 => 2.80,
            Self::Oz25 => 3.50,
            Self::Oz3 => 4.20,
            Self::Oz4 => 5.60,
            Self::Oz5 => 7.00,
        }
    }

    /// Copper thickness in mm, derived from [`thickness_mils`](Self::thickness_mils)
    /// so that both accessors describe the same physical thickness.
    pub fn thickness_mm(self) -> f64 {
        self.thickness_mils() * MM_PER_MIL
    }

    /// Parse from a string like "1oz", "0.5oz", "2.5oz".
    pub fn from_str_oz(s: &str) -> Result<Self, CalcError> {
        match s.trim().to_lowercase().trim_end_matches("oz").trim() {
            "0.25" => Ok(Self::Oz025),
            "0.5" => Ok(Self::Oz05),
            "1" => Ok(Self::Oz1),
            "1.5" => Ok(Self::Oz15),
            "2" => Ok(Self::Oz2),
            "2.5" => Ok(Self::Oz25),
            "3" => Ok(Self::Oz3),
            "4" => Ok(Self::Oz4),
            "5" => Ok(Self::Oz5),
            _ => Err(CalcError::UnknownCopperWeight(s.to_string())),
        }
    }
}

/// Plating thickness options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatingThickness {
    Bare,
    Oz05,
    Oz1,
    Oz15,
    Oz2,
    Oz25,
    Oz3,
}

impl PlatingThickness {
    /// Plating thickness in mils (same 1.4 mil/oz convention as [`CopperWeight`]).
    pub fn thickness_mils(self) -> f64 {
        match self {
            Self::Bare => 0.0,
            Self::Oz05 => 0.70,
            Self::Oz1 => 1.40,
            Self::Oz15 => 2.10,
            Self::Oz2 => 2.80,
            Self::Oz25 => 3.50,
            Self::Oz3 => 4.20,
        }
    }

    /// Plating thickness in mm, derived from the mil value.
    pub fn thickness_mm(self) -> f64 {
        self.thickness_mils() * MM_PER_MIL
    }
}

/// Etch factor affecting conductor cross-section geometry.
///
/// The etched trace is modelled as an isosceles trapezoid whose base is the
/// nominal width `W` and whose top is narrowed by the etch profile. The top
/// width must remain strictly positive: a trapezoid whose top would be zero
/// or negative is not a manufacturable conductor and is rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EtchFactor {
    /// Rectangular cross-section (no etch compensation).
    None,
    /// 1:1 etch — trapezoid with top = W − 2T.
    OneToOne,
    /// 2:1 etch — trapezoid with top = W − T.
    TwoToOne,
}

impl EtchFactor {
    /// Width of the top of the trapezoid (mils) for base width `width` and
    /// thickness `thickness`.
    pub fn top_width_mils(self, width: f64, thickness: f64) -> f64 {
        match self {
            Self::None => width,
            Self::OneToOne => width - 2.0 * thickness,
            Self::TwoToOne => width - thickness,
        }
    }

    /// Cross-sectional area in square mils given base width `W` and
    /// thickness `T` (both in mils).
    ///
    /// # Errors
    /// Rejects non-finite or non-positive `width`/`thickness`, and any
    /// etch profile whose top width would not be strictly positive.
    pub fn cross_section_sq_mils(self, width: f64, thickness: f64) -> Result<f64, CalcError> {
        validate::positive("width", width)?;
        validate::positive("thickness", thickness)?;
        let top = self.top_width_mils(width, thickness);
        if top <= 0.0 {
            return Err(CalcError::OutOfRange {
                name: "etched top width",
                value: top,
                expected: "> 0 (width too small for this thickness and etch factor)",
            });
        }
        validate::finite_result("cross_section", (width + top) * thickness / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn copper_weight_values() {
        assert_relative_eq!(CopperWeight::Oz1.thickness_mils(), 1.40, epsilon = 1e-12);
        // mm is derived from mils, not a separately rounded table (audit A20).
        assert_relative_eq!(
            CopperWeight::Oz1.thickness_mm(),
            1.40 * 0.0254,
            epsilon = 1e-12
        );
        assert_relative_eq!(
            CopperWeight::Oz2.thickness_mm(),
            2.80 * 0.0254,
            epsilon = 1e-12
        );
    }

    #[test]
    fn mil_and_mm_accessors_agree_for_every_weight() {
        for w in [
            CopperWeight::Oz025,
            CopperWeight::Oz05,
            CopperWeight::Oz1,
            CopperWeight::Oz15,
            CopperWeight::Oz2,
            CopperWeight::Oz25,
            CopperWeight::Oz3,
            CopperWeight::Oz4,
            CopperWeight::Oz5,
        ] {
            assert_relative_eq!(
                w.thickness_mm() / 0.0254,
                w.thickness_mils(),
                max_relative = 1e-12
            );
        }
    }

    #[test]
    fn etch_factor_rectangular() {
        let area = EtchFactor::None.cross_section_sq_mils(10.0, 1.4).unwrap();
        assert_relative_eq!(area, 14.0, epsilon = 1e-10);
    }

    #[test]
    fn etch_two_to_one_area() {
        // W=10, T=2.10 → (10 + 7.9)·2.1/2 = 18.795
        let area = EtchFactor::TwoToOne
            .cross_section_sq_mils(10.0, 2.10)
            .unwrap();
        assert_relative_eq!(area, 18.795, epsilon = 1e-9);
    }

    #[test]
    fn impossible_trapezoids_are_rejected() {
        // Negative algebraic area.
        assert!(
            EtchFactor::OneToOne
                .cross_section_sq_mils(1.0, 1.4)
                .is_err()
        );
        // Positive algebraic area but negative top width.
        assert!(
            EtchFactor::OneToOne
                .cross_section_sq_mils(2.0, 1.4)
                .is_err()
        );
        // Zero top width (triangle) is not a supported limit.
        assert!(
            EtchFactor::OneToOne
                .cross_section_sq_mils(2.8, 1.4)
                .is_err()
        );
        assert!(
            EtchFactor::TwoToOne
                .cross_section_sq_mils(1.4, 1.4)
                .is_err()
        );
        // Just above the limit is fine.
        assert!(
            EtchFactor::OneToOne
                .cross_section_sq_mils(2.81, 1.4)
                .is_ok()
        );
        // Non-finite inputs.
        assert!(
            EtchFactor::None
                .cross_section_sq_mils(f64::NAN, 1.4)
                .is_err()
        );
    }

    #[test]
    fn parse_copper_weight() {
        assert_eq!(CopperWeight::from_str_oz("1oz").unwrap(), CopperWeight::Oz1);
        assert_eq!(
            CopperWeight::from_str_oz("0.5oz").unwrap(),
            CopperWeight::Oz05
        );
        assert_eq!(
            CopperWeight::from_str_oz("2.5").unwrap(),
            CopperWeight::Oz25
        );
    }
}
