//! PCB substrate material database.
//!
//! Material data extracted from Saturn PCB Toolkit v8.44 binary.
//! See `docs/notes/15-materials-data.md` and `NOTES.md` for extraction details.

use serde::{Deserialize, Serialize};

/// A PCB substrate material with dielectric properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: &'static str,
    /// Relative permittivity (dielectric constant).
    pub er: f64,
    /// Glass transition temperature (°C), if known.
    pub tg: Option<f64>,
    /// Surface roughness correction factor (0.98 for FR-4 variants, 1.0 for smooth).
    pub roughness_factor: f64,
}

/// Built-in material database.
///
/// First 23 materials have Er values extracted from binary at offset 0x4b9ae8.
/// Roughness factor from ComboBox1Change decompilation: 0.98 for FR-4 family, 1.0 for others.
pub static MATERIALS: &[Material] = &[
    Material { name: "FR-4 STD", er: 4.6, tg: Some(130.0), roughness_factor: 0.98 },
    Material { name: "FR-5", er: 4.3, tg: None, roughness_factor: 0.98 },
    Material { name: "FR406", er: 3.8, tg: None, roughness_factor: 0.98 },
    Material { name: "FR408", er: 3.9, tg: None, roughness_factor: 0.98 },
    Material { name: "Getek ML200C", er: 4.2, tg: None, roughness_factor: 0.98 },
    Material { name: "Getek ML200D", er: 3.78, tg: None, roughness_factor: 0.98 },
    Material { name: "Getek ML200M", er: 2.94, tg: None, roughness_factor: 1.0 },
    Material { name: "Getek RG200D", er: 3.0, tg: None, roughness_factor: 1.0 },
    Material { name: "Isola P95", er: 6.15, tg: None, roughness_factor: 1.0 },
    Material { name: "Isola P96", er: 10.2, tg: None, roughness_factor: 1.0 },
    Material { name: "Isola P26N", er: 3.38, tg: None, roughness_factor: 1.0 },
    Material { name: "RO2800", er: 3.66, tg: None, roughness_factor: 1.0 },
    Material { name: "RO3003", er: 2.5, tg: None, roughness_factor: 1.0 },
    Material { name: "RO3006", er: 2.35, tg: None, roughness_factor: 1.0 },
    Material { name: "RO3010", er: 2.2, tg: None, roughness_factor: 1.0 },
    Material { name: "RO4003", er: 2.1, tg: None, roughness_factor: 1.0 },
    Material { name: "RO4350", er: 4.25, tg: None, roughness_factor: 1.0 },
    Material { name: "RT5500", er: 4.5, tg: None, roughness_factor: 1.0 },
    Material { name: "RT5870", er: 4.1, tg: None, roughness_factor: 1.0 },
    Material { name: "RT5880", er: 3.7, tg: None, roughness_factor: 1.0 },
    Material { name: "RT6002", er: 3.4, tg: None, roughness_factor: 1.0 },
    Material { name: "RT6006", er: 4.15, tg: None, roughness_factor: 1.0 },
    Material { name: "RT6010", er: 4.38, tg: None, roughness_factor: 1.0 },
    // Materials 24-44: Er values not yet extracted from binary (need ComboBox1Change decompilation).
    // For now, users must supply custom Er for these.
    Material { name: "Teflon PTFE", er: 2.1, tg: None, roughness_factor: 1.0 },
    Material { name: "Air", er: 1.0, tg: None, roughness_factor: 1.0 },
];

/// Look up a material by name (case-insensitive).
pub fn lookup(name: &str) -> Option<&'static Material> {
    MATERIALS.iter().find(|m| m.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_fr4() {
        let m = lookup("FR-4 STD").unwrap();
        assert!((m.er - 4.6).abs() < 1e-10);
        assert!((m.roughness_factor - 0.98).abs() < 1e-10);
    }

    #[test]
    fn lookup_case_insensitive() {
        assert!(lookup("fr-4 std").is_some());
        assert!(lookup("AIR").is_some());
    }

    #[test]
    fn lookup_unknown() {
        assert!(lookup("unobtanium").is_none());
    }
}
