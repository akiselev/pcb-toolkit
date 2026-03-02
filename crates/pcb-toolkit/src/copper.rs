//! Copper weight to thickness conversion tables.

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// Standard copper weight options (oz/ft²).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    /// Copper thickness in mils.
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

    /// Copper thickness in mm.
    pub fn thickness_mm(self) -> f64 {
        match self {
            Self::Oz025 => 0.009,
            Self::Oz05 => 0.018,
            Self::Oz1 => 0.035,
            Self::Oz15 => 0.053,
            Self::Oz2 => 0.070,
            Self::Oz25 => 0.088,
            Self::Oz3 => 0.106,
            Self::Oz4 => 0.142,
            Self::Oz5 => 0.178,
        }
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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    /// Plating thickness in mils.
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
}

/// Etch factor affecting conductor cross-section geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EtchFactor {
    /// Rectangular cross-section (no etch compensation).
    None,
    /// 1:1 etch — trapezoid with top = W - 2T.
    OneToOne,
    /// 2:1 etch — trapezoid with top = W - T.
    TwoToOne,
}

impl EtchFactor {
    /// Cross-sectional area in square mils given width W and thickness T (both in mils).
    pub fn cross_section_sq_mils(self, width: f64, thickness: f64) -> f64 {
        match self {
            Self::None => width * thickness,
            Self::OneToOne => (width + (width - 2.0 * thickness)) * thickness / 2.0,
            Self::TwoToOne => (width + (width - thickness)) * thickness / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copper_weight_values() {
        assert!((CopperWeight::Oz1.thickness_mils() - 1.40).abs() < 1e-10);
        assert!((CopperWeight::Oz1.thickness_mm() - 0.035).abs() < 1e-10);
    }

    #[test]
    fn etch_factor_rectangular() {
        // 10 mil wide, 1.4 mil thick, no etch → 14 sq mils
        let area = EtchFactor::None.cross_section_sq_mils(10.0, 1.4);
        assert!((area - 14.0).abs() < 1e-10);
    }

    #[test]
    fn parse_copper_weight() {
        assert_eq!(CopperWeight::from_str_oz("1oz").unwrap(), CopperWeight::Oz1);
        assert_eq!(CopperWeight::from_str_oz("0.5oz").unwrap(), CopperWeight::Oz05);
        assert_eq!(CopperWeight::from_str_oz("2.5").unwrap(), CopperWeight::Oz25);
    }
}
