//! PCB substrate material presets.
//!
//! # Provenance
//!
//! Every entry reproduces the material table embedded in Saturn PCB Toolkit
//! v8.44 (extracted from the binary; see `docs/notes/15-materials-data.md`).
//! The values are **compatibility presets**, not manufacturer engineering
//! data: each `er` is a single scalar with no stated test method,
//! frequency, resin content, glass style or tolerance, and Saturn does not
//! distinguish between "process" and "design" dielectric constants. For an
//! impedance-controlled design use the laminate datasheet value for the
//! construction, frequency and Dk definition your fabricator applies.
//!
//! Known conventions mismatches are recorded in each entry's `note`.

use serde::{Deserialize, Serialize};

/// Where a preset's numbers come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialSource {
    /// Value as listed by Saturn PCB Toolkit v8.44 (compatibility preset).
    SaturnV844,
}

/// A PCB substrate material with dielectric properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: &'static str,
    /// Relative permittivity (dielectric constant) as listed by the source.
    pub er: f64,
    /// Glass transition temperature (°C), if listed.
    pub tg: Option<f64>,
    /// Surface roughness correction factor used by Saturn (0.98 for FR-4
    /// family, 1.0 for others). Saturn-specific, not a measured quantity.
    pub roughness_factor: f64,
    /// Provenance of the numbers.
    pub source: MaterialSource,
    /// Caveats about the entry (product identity, Dk convention).
    pub note: Option<&'static str>,
}

const S: MaterialSource = MaterialSource::SaturnV844;

/// Built-in material presets (Saturn PCB Toolkit v8.44 compatibility values).
pub static MATERIALS: &[Material] = &[
    Material {
        name: "FR-4 STD",
        er: 4.60,
        tg: Some(130.0),
        roughness_factor: 0.98,
        source: S,
        note: Some(
            "Generic FR-4; real laminates range roughly 4.2–4.8 depending on resin content and frequency",
        ),
    },
    Material {
        name: "FR-5",
        er: 4.30,
        tg: Some(170.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "FR406",
        er: 4.60,
        tg: Some(170.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "FR408",
        er: 3.80,
        tg: Some(180.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "Getek ML200C",
        er: 3.80,
        tg: Some(175.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "Getek ML200D",
        er: 3.90,
        tg: Some(175.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "Getek ML200M",
        er: 3.80,
        tg: Some(175.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "Getek RG200D",
        er: 4.20,
        tg: Some(175.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "Isola P95",
        er: 3.78,
        tg: Some(260.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Isola P96",
        er: 3.78,
        tg: Some(260.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Isola P26N",
        er: 3.90,
        tg: Some(250.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RO2800",
        er: 2.94,
        tg: None,
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RO3003",
        er: 3.00,
        tg: None,
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RO3006",
        er: 6.15,
        tg: None,
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RO3010",
        er: 10.20,
        tg: None,
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RO4003",
        er: 3.38,
        tg: Some(280.0),
        roughness_factor: 1.0,
        source: S,
        note: Some(
            "Saturn value 3.38 matches Rogers RO4003C *process* Dk (10 GHz); the RO4003C *design* Dk is 3.55",
        ),
    },
    Material {
        name: "RO4350",
        er: 3.66,
        tg: Some(280.0),
        roughness_factor: 1.0,
        source: S,
        note: Some(
            "Saturn value 3.66 matches Rogers RO4350B *design* Dk; the RO4350B *process* Dk (10 GHz) is 3.48",
        ),
    },
    Material {
        name: "RT5500",
        er: 2.50,
        tg: Some(260.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RT5870",
        er: 2.35,
        tg: Some(260.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RT5880",
        er: 2.20,
        tg: Some(260.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RT6002",
        er: 2.94,
        tg: None,
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RT6006",
        er: 6.15,
        tg: None,
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "RT6010",
        er: 10.20,
        tg: None,
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Teflon PTFE",
        er: 2.10,
        tg: Some(240.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Arlon 25N",
        er: 3.38,
        tg: Some(260.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Arlon 33N",
        er: 4.25,
        tg: Some(250.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Arlon 85N",
        er: 4.20,
        tg: Some(250.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "PCL-FR-226",
        er: 4.50,
        tg: Some(140.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "PCL-FR-240",
        er: 4.50,
        tg: Some(140.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "PCL-FR-370",
        er: 4.50,
        tg: Some(175.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "PCL-FR-370HR",
        er: 4.60,
        tg: Some(180.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "N4000-7 EF",
        er: 4.10,
        tg: Some(165.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "N4000-13",
        er: 3.70,
        tg: Some(210.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "N4000-13SI",
        er: 3.40,
        tg: Some(210.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "N4000-13 EP",
        er: 3.70,
        tg: Some(210.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "N4000-13 EPSI",
        er: 3.40,
        tg: Some(210.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "N4000-29",
        er: 4.50,
        tg: Some(185.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "N7000-1",
        er: 3.90,
        tg: Some(260.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Ventec VT-47",
        er: 4.60,
        tg: Some(180.0),
        roughness_factor: 0.98,
        source: S,
        note: None,
    },
    Material {
        name: "Ventec VT-901",
        er: 4.15,
        tg: Some(250.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Ventec VT-90H",
        er: 4.15,
        tg: Some(250.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Megtron6",
        er: 3.40,
        tg: Some(185.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Kappa 438",
        er: 4.38,
        tg: Some(280.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Kapton",
        er: 3.40,
        tg: Some(400.0),
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
    Material {
        name: "Air",
        er: 1.00,
        tg: None,
        roughness_factor: 1.0,
        source: S,
        note: None,
    },
];

/// Look up a material by name (case-insensitive).
pub fn lookup(name: &str) -> Option<&'static Material> {
    MATERIALS.iter().find(|m| m.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_count() {
        assert_eq!(MATERIALS.len(), 45);
    }

    #[test]
    fn lookup_fr4() {
        let m = lookup("FR-4 STD").unwrap();
        assert!((m.er - 4.6).abs() < 1e-10);
        assert_eq!(m.tg, Some(130.0));
        assert!((m.roughness_factor - 0.98).abs() < 1e-10);
    }

    #[test]
    fn lookup_ro4350() {
        let m = lookup("RO4350").unwrap();
        assert!((m.er - 3.66).abs() < 1e-10);
        assert_eq!(m.tg, Some(280.0));
        assert!((m.roughness_factor - 1.0).abs() < 1e-10);
    }

    #[test]
    fn lookup_kapton() {
        let m = lookup("Kapton").unwrap();
        assert!((m.er - 3.40).abs() < 1e-10);
        assert_eq!(m.tg, Some(400.0));
    }

    #[test]
    fn lookup_air() {
        let m = lookup("Air").unwrap();
        assert!((m.er - 1.0).abs() < 1e-10);
        assert_eq!(m.tg, None);
    }

    #[test]
    fn lookup_case_insensitive() {
        assert!(lookup("fr-4 std").is_some());
        assert!(lookup("AIR").is_some());
        assert!(lookup("megtron6").is_some());
    }

    #[test]
    fn lookup_unknown() {
        assert!(lookup("unobtanium").is_none());
    }

    #[test]
    fn every_preset_is_physical_and_sourced() {
        for m in MATERIALS {
            assert!(m.er >= 1.0 && m.er.is_finite(), "{}", m.name);
            assert!(
                m.roughness_factor > 0.0 && m.roughness_factor <= 1.0,
                "{}",
                m.name
            );
            assert_eq!(m.source, MaterialSource::SaturnV844);
        }
        assert!(lookup("RO4003").unwrap().note.is_some());
        assert!(lookup("RO4350").unwrap().note.is_some());
    }
}
