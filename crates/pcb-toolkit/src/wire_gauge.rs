//! AWG wire gauge property lookup — Saturn PCB Toolkit Mode 8.
//!
//! Contains a 44-entry table covering AWG 4/0 (largest) through AWG 40 (smallest).
//! Diameter data is in inches; resistance data is in ohms per 1000 ft.
//! Area is computed in circular mils (diameter_mils²).

use serde::{Deserialize, Serialize};

/// AWG wire gauge sizes from 4/0 (largest) to 40 (smallest).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Awg {
    Awg4_0,
    Awg3_0,
    Awg2_0,
    Awg1_0,
    Awg1,
    Awg2,
    Awg3,
    Awg4,
    Awg5,
    Awg6,
    Awg7,
    Awg8,
    Awg9,
    Awg10,
    Awg11,
    Awg12,
    Awg13,
    Awg14,
    Awg15,
    Awg16,
    Awg17,
    Awg18,
    Awg19,
    Awg20,
    Awg21,
    Awg22,
    Awg23,
    Awg24,
    Awg25,
    Awg26,
    Awg27,
    Awg28,
    Awg29,
    Awg30,
    Awg31,
    Awg32,
    Awg33,
    Awg34,
    Awg35,
    Awg36,
    Awg37,
    Awg38,
    Awg39,
    Awg40,
}

/// Properties computed for a single AWG wire gauge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WireGaugeResult {
    /// Human-readable AWG label: "4/0", "3/0", ..., "40".
    pub awg_label: &'static str,
    /// Diameter in inches.
    pub diameter_in: f64,
    /// Diameter in mils (diameter_in × 1000).
    pub diameter_mils: f64,
    /// DC resistance in ohms per 1000 ft.
    pub resistance_ohm_per_kft: f64,
    /// Cross-sectional area in circular mils (diameter_mils²).
    pub area_circular_mils: f64,
    /// Saturn display area: diameter_mils² / 700.0.
    pub area_saturn: f64,
}

/// Static table entry: (label, diameter_in, resistance_ohm_per_kft).
struct Entry {
    label: &'static str,
    diameter_in: f64,
    resistance_ohm_per_kft: f64,
}

static TABLE: [Entry; 44] = [
    Entry { label: "4/0",  diameter_in: 0.4600, resistance_ohm_per_kft: 0.050 },
    Entry { label: "3/0",  diameter_in: 0.4096, resistance_ohm_per_kft: 0.060 },
    Entry { label: "2/0",  diameter_in: 0.3648, resistance_ohm_per_kft: 0.080 },
    Entry { label: "1/0",  diameter_in: 0.3249, resistance_ohm_per_kft: 0.100 },
    Entry { label: "1",    diameter_in: 0.2893, resistance_ohm_per_kft: 0.120 },
    Entry { label: "2",    diameter_in: 0.2576, resistance_ohm_per_kft: 0.160 },
    Entry { label: "3",    diameter_in: 0.2294, resistance_ohm_per_kft: 0.200 },
    Entry { label: "4",    diameter_in: 0.2043, resistance_ohm_per_kft: 0.250 },
    Entry { label: "5",    diameter_in: 0.1819, resistance_ohm_per_kft: 0.310 },
    Entry { label: "6",    diameter_in: 0.1620, resistance_ohm_per_kft: 0.400 },
    Entry { label: "7",    diameter_in: 0.1443, resistance_ohm_per_kft: 0.500 },
    Entry { label: "8",    diameter_in: 0.1285, resistance_ohm_per_kft: 0.630 },
    Entry { label: "9",    diameter_in: 0.1144, resistance_ohm_per_kft: 0.790 },
    Entry { label: "10",   diameter_in: 0.1019, resistance_ohm_per_kft: 1.000 },
    Entry { label: "11",   diameter_in: 0.0907, resistance_ohm_per_kft: 1.260 },
    Entry { label: "12",   diameter_in: 0.0808, resistance_ohm_per_kft: 1.590 },
    Entry { label: "13",   diameter_in: 0.0720, resistance_ohm_per_kft: 2.000 },
    Entry { label: "14",   diameter_in: 0.0641, resistance_ohm_per_kft: 2.530 },
    Entry { label: "15",   diameter_in: 0.0571, resistance_ohm_per_kft: 3.190 },
    Entry { label: "16",   diameter_in: 0.0508, resistance_ohm_per_kft: 4.020 },
    Entry { label: "17",   diameter_in: 0.0453, resistance_ohm_per_kft: 5.060 },
    Entry { label: "18",   diameter_in: 0.0403, resistance_ohm_per_kft: 6.390 },
    Entry { label: "19",   diameter_in: 0.0359, resistance_ohm_per_kft: 8.050 },
    Entry { label: "20",   diameter_in: 0.0320, resistance_ohm_per_kft: 10.150 },
    Entry { label: "21",   diameter_in: 0.0285, resistance_ohm_per_kft: 12.800 },
    Entry { label: "22",   diameter_in: 0.0254, resistance_ohm_per_kft: 16.140 },
    Entry { label: "23",   diameter_in: 0.0226, resistance_ohm_per_kft: 20.360 },
    Entry { label: "24",   diameter_in: 0.0201, resistance_ohm_per_kft: 25.670 },
    Entry { label: "25",   diameter_in: 0.0179, resistance_ohm_per_kft: 32.370 },
    Entry { label: "26",   diameter_in: 0.0159, resistance_ohm_per_kft: 40.810 },
    Entry { label: "27",   diameter_in: 0.0142, resistance_ohm_per_kft: 51.470 },
    Entry { label: "28",   diameter_in: 0.0126, resistance_ohm_per_kft: 64.900 },
    Entry { label: "29",   diameter_in: 0.0113, resistance_ohm_per_kft: 81.830 },
    Entry { label: "30",   diameter_in: 0.0100, resistance_ohm_per_kft: 103.200 },
    Entry { label: "31",   diameter_in: 0.0089, resistance_ohm_per_kft: 130.100 },
    Entry { label: "32",   diameter_in: 0.0080, resistance_ohm_per_kft: 164.100 },
    Entry { label: "33",   diameter_in: 0.0071, resistance_ohm_per_kft: 206.900 },
    Entry { label: "34",   diameter_in: 0.0063, resistance_ohm_per_kft: 260.900 },
    Entry { label: "35",   diameter_in: 0.0056, resistance_ohm_per_kft: 329.000 },
    Entry { label: "36",   diameter_in: 0.0050, resistance_ohm_per_kft: 414.800 },
    Entry { label: "37",   diameter_in: 0.0045, resistance_ohm_per_kft: 523.100 },
    Entry { label: "38",   diameter_in: 0.0040, resistance_ohm_per_kft: 659.600 },
    Entry { label: "39",   diameter_in: 0.0035, resistance_ohm_per_kft: 831.800 },
    Entry { label: "40",   diameter_in: 0.0031, resistance_ohm_per_kft: 1049.000 },
];

static ALL_AWG: [Awg; 44] = [
    Awg::Awg4_0, Awg::Awg3_0, Awg::Awg2_0, Awg::Awg1_0,
    Awg::Awg1,  Awg::Awg2,  Awg::Awg3,  Awg::Awg4,
    Awg::Awg5,  Awg::Awg6,  Awg::Awg7,  Awg::Awg8,
    Awg::Awg9,  Awg::Awg10, Awg::Awg11, Awg::Awg12,
    Awg::Awg13, Awg::Awg14, Awg::Awg15, Awg::Awg16,
    Awg::Awg17, Awg::Awg18, Awg::Awg19, Awg::Awg20,
    Awg::Awg21, Awg::Awg22, Awg::Awg23, Awg::Awg24,
    Awg::Awg25, Awg::Awg26, Awg::Awg27, Awg::Awg28,
    Awg::Awg29, Awg::Awg30, Awg::Awg31, Awg::Awg32,
    Awg::Awg33, Awg::Awg34, Awg::Awg35, Awg::Awg36,
    Awg::Awg37, Awg::Awg38, Awg::Awg39, Awg::Awg40,
];

impl Awg {
    /// Returns the table index for this gauge (0 = 4/0, 43 = 40).
    fn index(self) -> usize {
        self as usize
    }

    /// Returns all 44 AWG variants in order from largest (4/0) to smallest (40).
    pub fn all() -> &'static [Awg] {
        &ALL_AWG
    }

    /// Returns the `Awg` variant for the given ComboBox-style index, or `None` if out of range.
    pub fn from_index(idx: usize) -> Option<Awg> {
        ALL_AWG.get(idx).copied()
    }
}

/// Look up wire gauge properties for the given AWG size.
///
/// All data originates from the Saturn PCB Toolkit Mode 8 binary table.
/// This function is infallible because all inputs are compile-time enum variants.
pub fn lookup(awg: Awg) -> WireGaugeResult {
    let entry = &TABLE[awg.index()];
    let diameter_mils = entry.diameter_in * 1000.0;
    let area_circular_mils = diameter_mils * diameter_mils;
    WireGaugeResult {
        awg_label: entry.label,
        diameter_in: entry.diameter_in,
        diameter_mils,
        resistance_ohm_per_kft: entry.resistance_ohm_per_kft,
        area_circular_mils,
        area_saturn: area_circular_mils / 700.0,
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // AWG 4/0: diameter=0.4600 in, resistance=0.050 ohm/kft
    // diameter_mils = 460.0, area_circular_mils = 211600.0
    // area_saturn = 211600 / 700 = 302.285714...
    #[test]
    fn awg_4_0_properties() {
        let r = lookup(Awg::Awg4_0);
        assert_eq!(r.awg_label, "4/0");
        assert_relative_eq!(r.diameter_in, 0.4600, epsilon = 1e-10);
        assert_relative_eq!(r.resistance_ohm_per_kft, 0.050, epsilon = 1e-10);
        assert_relative_eq!(r.diameter_mils, 460.0, epsilon = 1e-10);
        assert_relative_eq!(r.area_circular_mils, 211_600.0, epsilon = 1e-6);
        assert_relative_eq!(r.area_saturn, 302.285_714_285, epsilon = 1e-6);
    }

    // AWG 22: diameter=0.0254 in, resistance=16.140 ohm/kft
    #[test]
    fn awg_22_properties() {
        let r = lookup(Awg::Awg22);
        assert_eq!(r.awg_label, "22");
        assert_relative_eq!(r.diameter_in, 0.0254, epsilon = 1e-10);
        assert_relative_eq!(r.resistance_ohm_per_kft, 16.140, epsilon = 1e-10);
        assert_relative_eq!(r.diameter_mils, 25.4, epsilon = 1e-8);
        assert_relative_eq!(r.area_circular_mils, 645.16, epsilon = 1e-6);
    }

    // AWG 40: diameter=0.0031 in, resistance=1049.000 ohm/kft
    #[test]
    fn awg_40_properties() {
        let r = lookup(Awg::Awg40);
        assert_eq!(r.awg_label, "40");
        assert_relative_eq!(r.diameter_in, 0.0031, epsilon = 1e-10);
        assert_relative_eq!(r.resistance_ohm_per_kft, 1049.000, epsilon = 1e-10);
        assert_relative_eq!(r.diameter_mils, 3.1, epsilon = 1e-10);
        assert_relative_eq!(r.area_circular_mils, 9.61, epsilon = 1e-8);
    }

    #[test]
    fn all_has_44_entries() {
        assert_eq!(Awg::all().len(), 44);
    }

    #[test]
    fn from_index_boundary() {
        assert_eq!(Awg::from_index(0), Some(Awg::Awg4_0));
        assert_eq!(Awg::from_index(43), Some(Awg::Awg40));
        assert_eq!(Awg::from_index(44), None);
    }
}
