//! Unit conversion at the API boundary.
//!
//! All internal computation uses canonical units (mils for length, Hz for frequency, etc.).
//! These types and functions convert user-facing values to/from internal representation.
//!
//! The newtype wrappers ([`Length`], [`Freq`], etc.) implement [`FromStr`] to parse
//! strings like `"0.254mm"` or `"1GHz"`, converting to canonical units on construction.

use std::fmt;
use std::str::FromStr;

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

// ── Parse error ────────────────────────────────────────────────────

/// Error returned when parsing a unit-annotated value from a string.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum UnitParseError {
    #[error("invalid number in '{0}'")]
    InvalidNumber(String),

    #[error("unknown unit suffix in '{0}'")]
    UnknownSuffix(String),

    #[error("value is not finite")]
    NotFinite,
}

// ── Parsing helper ─────────────────────────────────────────────────

/// Split `"3.14mm"` into `("3.14", "mm")`.
///
/// Handles leading/trailing whitespace, negative signs, and scientific notation
/// (`1e3`, `1.5E-6`). The suffix portion is trimmed of leading whitespace so
/// that `"100 mil"` (quoted on the shell) also works.
fn split_number_suffix(s: &str) -> (&str, &str) {
    let s = s.trim();
    let bytes = s.as_bytes();
    let mut i = 0;

    // Optional leading sign.
    if i < bytes.len() && (bytes[i] == b'-' || bytes[i] == b'+') {
        i += 1;
    }

    // Integer part.
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }

    // Optional fractional part.
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }

    // Optional exponent — only consume 'e'/'E' if followed by digit or sign+digit.
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        let mut j = i + 1;
        if j < bytes.len() && (bytes[j] == b'-' || bytes[j] == b'+') {
            j += 1;
        }
        if j < bytes.len() && bytes[j].is_ascii_digit() {
            i = j;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
        }
    }

    (&s[..i], s[i..].trim_start())
}

// ── Newtype wrappers ───────────────────────────────────────────────

/// A length value stored in canonical mils.
///
/// Parses strings like `"10mil"`, `"0.254mm"`, `"0.01in"`, `"254um"`.
/// Bare numbers (no suffix) are interpreted as mils.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length(pub f64);

impl Length {
    pub fn mils(self) -> f64 {
        self.0
    }
}

impl FromStr for Length {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, suffix) = split_number_suffix(s);
        let value: f64 = num
            .parse()
            .map_err(|_| UnitParseError::InvalidNumber(s.to_string()))?;
        if !value.is_finite() {
            return Err(UnitParseError::NotFinite);
        }
        let unit = match suffix.to_lowercase().as_str() {
            "" | "mil" | "mils" => LengthUnit::Mils,
            "mm" => LengthUnit::Mm,
            "in" | "inch" | "inches" => LengthUnit::Inches,
            "um" | "µm" => LengthUnit::Um,
            _ => return Err(UnitParseError::UnknownSuffix(s.to_string())),
        };
        Ok(Length(to_mils(value, unit)))
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}mil", self.0)
    }
}

/// A frequency value stored in canonical Hz.
///
/// Parses strings like `"1GHz"`, `"100MHz"`, `"50kHz"`, `"1000"`.
/// Bare numbers (no suffix) are interpreted as Hz.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Freq(pub f64);

impl Freq {
    pub fn hz(self) -> f64 {
        self.0
    }
}

impl FromStr for Freq {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, suffix) = split_number_suffix(s);
        let value: f64 = num
            .parse()
            .map_err(|_| UnitParseError::InvalidNumber(s.to_string()))?;
        if !value.is_finite() {
            return Err(UnitParseError::NotFinite);
        }
        let unit = match suffix.to_lowercase().as_str() {
            "" | "hz" => FreqUnit::Hz,
            "khz" => FreqUnit::KHz,
            "mhz" => FreqUnit::MHz,
            "ghz" => FreqUnit::GHz,
            _ => return Err(UnitParseError::UnknownSuffix(s.to_string())),
        };
        Ok(Freq(to_hz(value, unit)))
    }
}

impl fmt::Display for Freq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}Hz", self.0)
    }
}

/// A capacitance value stored in canonical Farads.
///
/// Parses strings like `"100pF"`, `"10nF"`, `"1uF"`, `"1µF"`.
/// Bare numbers (no suffix) are interpreted as Farads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capacitance(pub f64);

impl Capacitance {
    pub fn farads(self) -> f64 {
        self.0
    }
}

impl FromStr for Capacitance {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, suffix) = split_number_suffix(s);
        let value: f64 = num
            .parse()
            .map_err(|_| UnitParseError::InvalidNumber(s.to_string()))?;
        if !value.is_finite() {
            return Err(UnitParseError::NotFinite);
        }
        let norm = suffix.replace('µ', "u").to_lowercase();
        let unit = match norm.as_str() {
            "" | "f" => CapUnit::F,
            "uf" => CapUnit::UF,
            "nf" => CapUnit::NF,
            "pf" => CapUnit::PF,
            _ => return Err(UnitParseError::UnknownSuffix(s.to_string())),
        };
        Ok(Capacitance(to_farads(value, unit)))
    }
}

impl fmt::Display for Capacitance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}F", self.0)
    }
}

/// An inductance value stored in canonical Henries.
///
/// Parses strings like `"10nH"`, `"1uH"`, `"1µH"`, `"100mH"`.
/// Bare numbers (no suffix) are interpreted as Henries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inductance(pub f64);

impl Inductance {
    pub fn henries(self) -> f64 {
        self.0
    }
}

impl FromStr for Inductance {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, suffix) = split_number_suffix(s);
        let value: f64 = num
            .parse()
            .map_err(|_| UnitParseError::InvalidNumber(s.to_string()))?;
        if !value.is_finite() {
            return Err(UnitParseError::NotFinite);
        }
        let norm = suffix.replace('µ', "u").to_lowercase();
        let unit = match norm.as_str() {
            "" | "h" => IndUnit::H,
            "mh" => IndUnit::MH,
            "uh" => IndUnit::UH,
            "nh" => IndUnit::NH,
            _ => return Err(UnitParseError::UnknownSuffix(s.to_string())),
        };
        Ok(Inductance(to_henries(value, unit)))
    }
}

impl fmt::Display for Inductance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}H", self.0)
    }
}

/// A temperature value stored in canonical Celsius.
///
/// Parses strings like `"25C"`, `"77F"`, `"25°C"`, `"25degC"`.
/// Bare numbers (no suffix) are interpreted as Celsius.
/// Suffix matching is **case-sensitive** (`C`/`F` uppercase only).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature(pub f64);

impl Temperature {
    pub fn celsius(self) -> f64 {
        self.0
    }
}

impl FromStr for Temperature {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, suffix) = split_number_suffix(s);
        let value: f64 = num
            .parse()
            .map_err(|_| UnitParseError::InvalidNumber(s.to_string()))?;
        if !value.is_finite() {
            return Err(UnitParseError::NotFinite);
        }
        let unit = match suffix {
            "" | "C" | "°C" | "degC" => TempUnit::Celsius,
            "F" | "°F" | "degF" => TempUnit::Fahrenheit,
            _ => return Err(UnitParseError::UnknownSuffix(s.to_string())),
        };
        Ok(Temperature(to_celsius(value, unit)))
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°C", self.0)
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

    // ── split_number_suffix ────────────────────────────────────────

    #[test]
    fn split_bare_number() {
        assert_eq!(split_number_suffix("100"), ("100", ""));
    }

    #[test]
    fn split_with_suffix() {
        assert_eq!(split_number_suffix("0.254mm"), ("0.254", "mm"));
    }

    #[test]
    fn split_scientific() {
        assert_eq!(split_number_suffix("1e3mm"), ("1e3", "mm"));
        assert_eq!(split_number_suffix("1.5E-6nH"), ("1.5E-6", "nH"));
    }

    #[test]
    fn split_negative() {
        assert_eq!(split_number_suffix("-5mil"), ("-5", "mil"));
    }

    #[test]
    fn split_whitespace() {
        assert_eq!(split_number_suffix("  100mil  "), ("100", "mil"));
        // Space between number and suffix (quoted on shell).
        assert_eq!(split_number_suffix("100 mil"), ("100", "mil"));
    }

    // ── Length parsing ─────────────────────────────────────────────

    #[test]
    fn parse_length_bare() {
        let l: Length = "100".parse().unwrap();
        assert!((l.mils() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn parse_length_mil() {
        let l: Length = "100mil".parse().unwrap();
        assert!((l.mils() - 100.0).abs() < 1e-10);
        let l2: Length = "100mils".parse().unwrap();
        assert!((l2.mils() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn parse_length_mm() {
        let l: Length = "0.254mm".parse().unwrap();
        assert!((l.mils() - 10.0).abs() < 1e-6);
    }

    #[test]
    fn parse_length_inches() {
        let l: Length = "0.1in".parse().unwrap();
        assert!((l.mils() - 100.0).abs() < 1e-10);
        let l2: Length = "0.1inch".parse().unwrap();
        assert!((l2.mils() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn parse_length_um() {
        let l: Length = "25.4um".parse().unwrap();
        assert!((l.mils() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn parse_length_um_unicode() {
        let l: Length = "25.4µm".parse().unwrap();
        assert!((l.mils() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn parse_length_scientific() {
        let l: Length = "1e3mil".parse().unwrap();
        assert!((l.mils() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn parse_length_negative() {
        let l: Length = "-5mil".parse().unwrap();
        assert!((l.mils() - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn parse_length_case_insensitive() {
        let l: Length = "10MIL".parse().unwrap();
        assert!((l.mils() - 10.0).abs() < 1e-10);
        let l2: Length = "1MM".parse().unwrap();
        assert!((l2.mils() - to_mils(1.0, LengthUnit::Mm)).abs() < 1e-10);
    }

    #[test]
    fn parse_length_errors() {
        assert!("".parse::<Length>().is_err());
        assert!("mm".parse::<Length>().is_err());
        assert!("100ft".parse::<Length>().is_err());
        assert!("abc".parse::<Length>().is_err());
    }

    // ── Freq parsing ───────────────────────────────────────────────

    #[test]
    fn parse_freq_bare() {
        let f: Freq = "1000000".parse().unwrap();
        assert!((f.hz() - 1_000_000.0).abs() < 1.0);
    }

    #[test]
    fn parse_freq_mhz() {
        let f: Freq = "100MHz".parse().unwrap();
        assert!((f.hz() - 100e6).abs() < 1.0);
    }

    #[test]
    fn parse_freq_ghz() {
        let f: Freq = "2.4GHz".parse().unwrap();
        assert!((f.hz() - 2.4e9).abs() < 1.0);
    }

    #[test]
    fn parse_freq_khz() {
        let f: Freq = "50kHz".parse().unwrap();
        assert!((f.hz() - 50e3).abs() < 1.0);
    }

    #[test]
    fn parse_freq_case_insensitive() {
        let f: Freq = "1ghz".parse().unwrap();
        assert!((f.hz() - 1e9).abs() < 1.0);
    }

    #[test]
    fn parse_freq_errors() {
        assert!("".parse::<Freq>().is_err());
        assert!("Hz".parse::<Freq>().is_err());
        assert!("100rpm".parse::<Freq>().is_err());
    }

    // ── Capacitance parsing ────────────────────────────────────────

    #[test]
    fn parse_cap_pf() {
        let c: Capacitance = "100pF".parse().unwrap();
        assert!((c.farads() - 100e-12).abs() < 1e-20);
    }

    #[test]
    fn parse_cap_uf_unicode() {
        let c: Capacitance = "10µF".parse().unwrap();
        assert!((c.farads() - 10e-6).abs() < 1e-14);
    }

    // ── Inductance parsing ─────────────────────────────────────────

    #[test]
    fn parse_ind_nh() {
        let i: Inductance = "10nH".parse().unwrap();
        assert!((i.henries() - 10e-9).abs() < 1e-18);
    }

    #[test]
    fn parse_ind_uh_unicode() {
        let i: Inductance = "4.7µH".parse().unwrap();
        assert!((i.henries() - 4.7e-6).abs() < 1e-14);
    }

    // ── Temperature parsing ────────────────────────────────────────

    #[test]
    fn parse_temp_celsius() {
        let t: Temperature = "25C".parse().unwrap();
        assert!((t.celsius() - 25.0).abs() < 1e-10);
        let t2: Temperature = "25°C".parse().unwrap();
        assert!((t2.celsius() - 25.0).abs() < 1e-10);
        let t3: Temperature = "25degC".parse().unwrap();
        assert!((t3.celsius() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn parse_temp_fahrenheit() {
        let t: Temperature = "77F".parse().unwrap();
        assert!((t.celsius() - 25.0).abs() < 0.01);
        let t2: Temperature = "77°F".parse().unwrap();
        assert!((t2.celsius() - 25.0).abs() < 0.01);
    }

    #[test]
    fn parse_temp_bare_is_celsius() {
        let t: Temperature = "100".parse().unwrap();
        assert!((t.celsius() - 100.0).abs() < 1e-10);
    }

    // ── Display ────────────────────────────────────────────────────

    #[test]
    fn display_length() {
        assert_eq!(format!("{}", Length(100.0)), "100mil");
    }

    #[test]
    fn display_freq() {
        assert_eq!(format!("{}", Freq(1e9)), "1000000000Hz");
    }

    #[test]
    fn display_temp() {
        assert_eq!(format!("{}", Temperature(25.0)), "25°C");
    }
}
