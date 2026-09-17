//! Embedded (buried) microstrip impedance calculator.
//!
//! An embedded microstrip is a surface microstrip covered by a dielectric
//! layer of the same material. The cover moves part of the fringing field
//! from air into dielectric, raising the effective permittivity towards εr
//! and lowering the impedance.
//!
//! Model (Wadell, *Transmission Line Design Handbook*, §3.5.4; also the
//! IPC-D-317A / Brooks form):
//!
//! ```text
//! εeff,emb = εr − (εr − εeff,surface) · exp(−2·cover / height)
//! Zo,emb   = Zo,air / √εeff,emb          with  Zo,air = Zo,surface · √εeff,surface
//! ```
//!
//! Because the impedance is obtained from the air-filled impedance and the
//! embedded permittivity, the model is exactly continuous at zero cover,
//! invariant to adding an air cover to an air line, monotonic in cover
//! thickness, and reaches the homogeneous limit `Zo,air/√εr` for deep burial.

use crate::impedance::{microstrip, types::ImpedanceResult};
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Embedded microstrip: Hammerstad-Jensen surface line + exponential cover filling (Wadell)",
    status: ModelStatus::Compatibility,
    reference: "Wadell 1991 §3.5.4; Brooks, Signal Integrity Issues and PCB Design",
    validity: "Same range as the surface microstrip; cover of the same εr as the substrate; quasi-static only (frequency must be 0). Estimated ±3%",
};

/// Inputs for embedded microstrip impedance calculation. All dimensions in mils.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmbeddedMicrostripInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Dielectric height — distance from trace to ground plane (mils).
    pub height: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate (and cover) relative permittivity.
    pub er: f64,
    /// Cover height — dielectric above the trace (mils). 0 = surface microstrip.
    pub cover_height: f64,
    /// Frequency (Hz). Only 0 (quasi-static) is supported; the dispersion
    /// model is defined for open microstrip and is not applied here.
    pub frequency: f64,
}

/// Effective permittivity of the covered line given the surface value.
pub fn er_eff_embedded(er: f64, er_eff_surface: f64, cover_height: f64, height: f64) -> f64 {
    er - (er - er_eff_surface) * (-2.0 * cover_height / height).exp()
}

/// Impedance of the covered line from the surface line quantities.
pub fn zo_embedded(zo_surface: f64, er_eff_surface: f64, er_eff_embedded: f64) -> f64 {
    zo_surface * (er_eff_surface / er_eff_embedded).sqrt()
}

/// Compute embedded microstrip impedance and derived quantities.
pub fn calculate(input: &EmbeddedMicrostripInput) -> Result<ImpedanceResult, CalcError> {
    let EmbeddedMicrostripInput {
        width,
        height,
        thickness,
        er,
        cover_height,
        frequency,
    } = *input;

    validate::non_negative("cover_height", cover_height)?;
    validate::finite("frequency", frequency)?;
    if frequency != 0.0 {
        return Err(CalcError::Unsupported(
            "embedded microstrip is quasi-static only; pass frequency = 0",
        ));
    }

    let surface = microstrip::calculate(&microstrip::MicrostripInput {
        width,
        height,
        thickness,
        er,
        frequency: 0.0,
    })?;

    let er_eff = er_eff_embedded(er, surface.er_eff, cover_height, height);
    let zo = zo_embedded(surface.zo, surface.er_eff, er_eff);
    microstrip::finish(zo, er_eff)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn emb(cover_height: f64, er: f64) -> EmbeddedMicrostripInput {
        EmbeddedMicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er,
            cover_height,
            frequency: 0.0,
        }
    }

    fn surface(er: f64) -> ImpedanceResult {
        microstrip::calculate(&microstrip::MicrostripInput {
            width: 10.0,
            height: 5.0,
            thickness: 1.4,
            er,
            frequency: 0.0,
        })
        .unwrap()
    }

    #[test]
    fn zero_cover_equals_surface() {
        let s = surface(4.6);
        let e = calculate(&emb(0.0, 4.6)).unwrap();
        assert_relative_eq!(e.zo, s.zo, max_relative = 1e-12);
        assert_relative_eq!(e.er_eff, s.er_eff, max_relative = 1e-12);
    }

    #[test]
    fn continuous_at_zero_cover() {
        let zero = calculate(&emb(0.0, 4.6)).unwrap();
        let tiny = calculate(&emb(1e-6, 4.6)).unwrap();
        assert_relative_eq!(tiny.zo, zero.zo, max_relative = 1e-5);
    }

    #[test]
    fn air_cover_on_air_line_changes_nothing() {
        let zero = calculate(&emb(0.0, 1.0)).unwrap();
        let covered = calculate(&emb(1.0, 1.0)).unwrap();
        assert_relative_eq!(covered.zo, zero.zo, max_relative = 1e-12);
        assert_relative_eq!(covered.er_eff, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn impedance_decreases_monotonically_with_cover() {
        let mut prev = calculate(&emb(0.0, 4.6)).unwrap().zo;
        for c in [0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 50.0] {
            let z = calculate(&emb(c, 4.6)).unwrap().zo;
            assert!(z <= prev, "cover {c}: {z} > {prev}");
            prev = z;
        }
    }

    #[test]
    fn deep_burial_reaches_homogeneous_limit() {
        let s = surface(4.6);
        let deep = calculate(&emb(50.0, 4.6)).unwrap();
        let zo_air = s.zo * s.er_eff.sqrt();
        assert_relative_eq!(deep.er_eff, 4.6, max_relative = 1e-6);
        assert_relative_eq!(deep.zo, zo_air / 4.6_f64.sqrt(), max_relative = 1e-6);
    }

    #[test]
    fn independent_evaluation() {
        // Reconstructed from the H-J surface values (44.832 Ω, εeff 3.3075):
        let r = calculate(&emb(1.0, 4.6)).unwrap();
        assert_relative_eq!(r.zo, 42.196, max_relative = 1e-3);
        assert_relative_eq!(r.er_eff, 3.7336, max_relative = 1e-3);
    }

    #[test]
    fn rejects_negative_cover_and_nonzero_frequency() {
        assert!(calculate(&emb(-1.0, 4.6)).is_err());
        let mut i = emb(1.0, 4.6);
        i.frequency = 1e9;
        assert!(matches!(calculate(&i), Err(CalcError::Unsupported(_))));
    }
}
