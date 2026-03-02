//! Unit conversion at the API boundary.
//!
//! All internal computation uses canonical units (mils for length, Hz for frequency, etc.).
//! These types and functions convert user-facing values to/from internal representation.

use serde::{Deserialize, Serialize};

/// Length units accepted at the API boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LengthUnit {
    Mils,
    Mm,
    Inches,
    #[serde(rename = "um")]
    Um,
}

/// Frequency units accepted at the API boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FreqUnit {
    Hz,
    #[serde(rename = "kHz")]
    KHz,
    MHz,
    GHz,
}

/// Capacitance units for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapUnit {
    F,
    #[serde(rename = "uF")]
    UF,
    #[serde(rename = "nF")]
    NF,
    #[serde(rename = "pF")]
    PF,
}

/// Inductance units for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndUnit {
    H,
    #[serde(rename = "mH")]
    MH,
    #[serde(rename = "uH")]
    UH,
    #[serde(rename = "nH")]
    NH,
}

/// Resistance units for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResUnit {
    #[serde(rename = "mOhm")]
    MOhm,
    Ohm,
    #[serde(rename = "kOhm")]
    KOhm,
    #[serde(rename = "MOhm")]
    MOhmMega,
}

/// Temperature units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
}

// ── Length conversions ───────────────────────────────────────────────

/// Convert from user units to mils (internal canonical unit).
pub fn to_mils(value: f64, unit: LengthUnit) -> f64 {
    match unit {
        LengthUnit::Mils => value,
        LengthUnit::Mm => value / 0.0254,
        LengthUnit::Inches => value * 1000.0,
        LengthUnit::Um => value / 25.4,
    }
}

/// Convert from mils to user units.
pub fn from_mils(value: f64, unit: LengthUnit) -> f64 {
    match unit {
        LengthUnit::Mils => value,
        LengthUnit::Mm => value * 0.0254,
        LengthUnit::Inches => value / 1000.0,
        LengthUnit::Um => value * 25.4,
    }
}

// ── Frequency conversions ───────────────────────────────────────────

/// Convert from user units to Hz (internal canonical unit).
pub fn to_hz(value: f64, unit: FreqUnit) -> f64 {
    match unit {
        FreqUnit::Hz => value,
        FreqUnit::KHz => value * 1e3,
        FreqUnit::MHz => value * 1e6,
        FreqUnit::GHz => value * 1e9,
    }
}

/// Convert from Hz to user units.
pub fn from_hz(value: f64, unit: FreqUnit) -> f64 {
    match unit {
        FreqUnit::Hz => value,
        FreqUnit::KHz => value / 1e3,
        FreqUnit::MHz => value / 1e6,
        FreqUnit::GHz => value / 1e9,
    }
}

// ── Capacitance conversions ─────────────────────────────────────────

/// Convert from user units to Farads (internal canonical unit).
pub fn to_farads(value: f64, unit: CapUnit) -> f64 {
    match unit {
        CapUnit::F => value,
        CapUnit::UF => value * 1e-6,
        CapUnit::NF => value * 1e-9,
        CapUnit::PF => value * 1e-12,
    }
}

/// Convert from Farads to user units.
pub fn from_farads(value: f64, unit: CapUnit) -> f64 {
    match unit {
        CapUnit::F => value,
        CapUnit::UF => value / 1e-6,
        CapUnit::NF => value / 1e-9,
        CapUnit::PF => value / 1e-12,
    }
}

// ── Inductance conversions ──────────────────────────────────────────

/// Convert from user units to Henries (internal canonical unit).
pub fn to_henries(value: f64, unit: IndUnit) -> f64 {
    match unit {
        IndUnit::H => value,
        IndUnit::MH => value * 1e-3,
        IndUnit::UH => value * 1e-6,
        IndUnit::NH => value * 1e-9,
    }
}

/// Convert from Henries to user units.
pub fn from_henries(value: f64, unit: IndUnit) -> f64 {
    match unit {
        IndUnit::H => value,
        IndUnit::MH => value / 1e-3,
        IndUnit::UH => value / 1e-6,
        IndUnit::NH => value / 1e-9,
    }
}

// ── Temperature conversions ─────────────────────────────────────────

/// Convert to Celsius (internal canonical unit).
pub fn to_celsius(value: f64, unit: TempUnit) -> f64 {
    match unit {
        TempUnit::Celsius => value,
        TempUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
    }
}

/// Convert from Celsius to user units.
pub fn from_celsius(value: f64, unit: TempUnit) -> f64 {
    match unit {
        TempUnit::Celsius => value,
        TempUnit::Fahrenheit => value * 9.0 / 5.0 + 32.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_roundtrip() {
        let mils = 100.0;
        for unit in [LengthUnit::Mils, LengthUnit::Mm, LengthUnit::Inches, LengthUnit::Um] {
            let converted = from_mils(mils, unit);
            let back = to_mils(converted, unit);
            assert!((back - mils).abs() < 1e-10, "roundtrip failed for {unit:?}");
        }
    }

    #[test]
    fn known_conversions() {
        // 1 mil = 0.0254 mm
        assert!((to_mils(0.0254, LengthUnit::Mm) - 1.0).abs() < 1e-10);
        // 1 inch = 1000 mils
        assert!((to_mils(1.0, LengthUnit::Inches) - 1000.0).abs() < 1e-10);
        // 25.4 µm = 1 mil
        assert!((to_mils(25.4, LengthUnit::Um) - 1.0).abs() < 1e-10);
        // 1 GHz = 1e9 Hz
        assert!((to_hz(1.0, FreqUnit::GHz) - 1e9).abs() < 1.0);
        // 32°F = 0°C
        assert!((to_celsius(32.0, TempUnit::Fahrenheit)).abs() < 1e-10);
    }
}
