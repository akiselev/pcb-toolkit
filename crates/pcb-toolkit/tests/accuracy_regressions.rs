//! Accuracy regressions derived from the 2026-09-16 numerical audit.
//!
//! Each test asserts an exact relation, a physical invariant, a model-domain
//! contract, or agreement with an independently computed reference. Passing
//! them is evidence against the specific defects the audit demonstrated; it
//! is not a substitute for field-solver or measured validation of a model
//! over its whole domain (see each module's `MODEL` constant for that).

use pcb_toolkit::copper::EtchFactor;
use pcb_toolkit::current::{self, CurrentInput, Ipc2152Input};
use pcb_toolkit::differential::{
    broadside_coupled::{self, BroadsideCoupledInput},
    edge_coupled_internal_asym::{self, EdgeCoupledInternalAsymInput},
};
use pcb_toolkit::impedance::{
    coplanar::{self, CoplanarInput},
    embedded::{self, EmbeddedMicrostripInput},
    microstrip::{self, MicrostripInput},
    stripline::{self, StriplineInput},
};
use pcb_toolkit::spacing::{self, DeviceType, SpacingInput};
use pcb_toolkit::units::{Freq, Length, Temperature};

fn close(actual: f64, expected: f64, rel_tol: f64, abs_tol: f64) {
    assert!(
        actual.is_finite() && expected.is_finite(),
        "actual={actual:?}, expected={expected:?}"
    );
    assert!(
        (actual - expected).abs() <= abs_tol.max(rel_tol * expected.abs()),
        "actual={actual:?}, expected={expected:?}"
    );
}

fn ms(width: f64, frequency: f64) -> MicrostripInput {
    MicrostripInput {
        width,
        height: 10.0,
        thickness: 0.1,
        er: 4.6,
        frequency,
    }
}

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

fn cpw(height: f64, er: f64) -> CoplanarInput {
    CoplanarInput {
        width: 10.0,
        gap: 5.0,
        height,
        thickness: 0.0,
        er,
    }
}

fn current_input() -> CurrentInput {
    CurrentInput {
        width: 10.0,
        thickness: 1.4,
        length: 1000.0,
        temperature_rise: 10.0,
        ambient_temp: 20.0,
        frequency: 0.0,
        etch_factor: EtchFactor::None,
        is_internal: false,
    }
}

fn ipc2152(ambient_temp: f64, temperature_rise: f64) -> Ipc2152Input {
    Ipc2152Input {
        width: 100.0,
        thickness: 1.0,
        length: 1000.0,
        temperature_rise,
        ambient_temp,
        frequency: 0.0,
        etch_factor: EtchFactor::None,
        is_internal: false,
        board_thickness_mils: 62.0,
        has_copper_plane: false,
        material_modifier: 1.0,
        user_modifier: 1.0,
    }
}

// A01
#[test]
fn pi_pad_is_matched_and_has_requested_attenuation() {
    for attenuation_db in [1.0, 6.0, 20.0] {
        let pad = pcb_toolkit::ohms_law::pi_pad(attenuation_db, 50.0).unwrap();
        let r = pad.r_series_ohm;
        let y = 1.0 / pad.r_shunt_ohm;
        let a = 1.0 + r * y;
        let b = r;
        let c = 2.0 * y + r * y * y;
        let d = a;
        let z_in = (a * 50.0 + b) / (c * 50.0 + d);
        let s21 = 2.0 / (a + b / 50.0 + c * 50.0 + d);
        close(z_in, 50.0, 1e-10, 1e-10);
        close(-20.0 * s21.log10(), attenuation_db, 1e-10, 1e-10);
    }
}

// A02
#[test]
fn embedded_is_continuous_at_zero_cover() {
    let zero = embedded::calculate(&emb(0.0, 4.6)).unwrap();
    let tiny = embedded::calculate(&emb(1e-6, 4.6)).unwrap();
    close(tiny.zo, zero.zo, 1e-3, 0.0);
}

#[test]
fn adding_air_to_air_does_not_change_impedance() {
    let zero = embedded::calculate(&emb(0.0, 1.0)).unwrap();
    let covered = embedded::calculate(&emb(1.0, 1.0)).unwrap();
    close(covered.zo, zero.zo, 1e-10, 1e-10);
}

#[test]
fn thicker_dielectric_cover_does_not_raise_quasistatic_impedance() {
    let thin = embedded::calculate(&emb(0.1, 4.6)).unwrap();
    let thick = embedded::calculate(&emb(1.0, 4.6)).unwrap();
    assert!(thick.zo <= thin.zo, "thin={}, thick={}", thin.zo, thick.zo);
}

// A04
#[test]
fn microstrip_has_no_spurious_width_branch_jump() {
    let boundary = std::f64::consts::PI * 10.0 / 2.0;
    let below = microstrip::calculate(&ms(boundary - 1e-6, 0.0)).unwrap();
    let above = microstrip::calculate(&ms(boundary + 1e-6, 0.0)).unwrap();
    close(above.zo, below.zo, 1e-5, 1e-6);
}

// A13
#[test]
fn microstrip_nonzero_frequency_is_not_silently_ignored() {
    let dc = microstrip::calculate(&ms(10.0, 0.0)).unwrap();
    let rf = microstrip::calculate(&ms(10.0, 10e9)).unwrap();
    assert!((rf.zo - dc.zo).abs() > 1e-6 || (rf.er_eff - dc.er_eff).abs() > 1e-6);
}

// A03
#[test]
fn cpw_effective_permittivity_is_bounded_by_its_materials() {
    let result = coplanar::calculate(&cpw(2.0, 4.6)).unwrap();
    assert!(result.er_eff >= 1.0 && result.er_eff <= 4.6);
}

#[test]
fn air_filled_grounded_cpw_depends_on_ground_height() {
    let low = coplanar::calculate(&cpw(2.0, 1.0)).unwrap();
    let high = coplanar::calculate(&cpw(100.0, 1.0)).unwrap();
    assert!(low.zo < high.zo - 1.0);
}

#[test]
fn grounded_cpw_matches_independent_zero_thickness_reference() {
    let result = coplanar::calculate(&cpw(5.0, 4.6)).unwrap();
    close(result.zo, 46.0957448064, 1e-9, 0.0);
    close(result.er_eff, 3.34193797552, 1e-9, 0.0);
}

// A05
#[test]
fn asymmetric_stripline_accounts_for_trace_offset() {
    let calc = |height1, height2| {
        edge_coupled_internal_asym::calculate(&EdgeCoupledInternalAsymInput {
            width: 10.0,
            spacing: 5.0,
            height1,
            height2,
            thickness: 1.4,
            er: 4.6,
        })
        .unwrap()
    };
    let centered = calc(10.0, 10.0);
    let offset = calc(1.0, 19.0);
    assert!((centered.zdiff - offset.zdiff).abs() > 1e-3);
    close(calc(1.0, 19.0).zdiff, calc(19.0, 1.0).zdiff, 1e-12, 0.0);
}

// A06
#[test]
fn finite_broadside_gap_cannot_have_zero_impedance() {
    let input = BroadsideCoupledInput {
        width: 10.0,
        separation: 0.1,
        height_total: 30.0,
        thickness: 0.01,
        er: 4.5,
        shielded: true,
    };
    let result = broadside_coupled::calculate(&input).unwrap();
    assert!(result.zdiff.is_finite() && result.zdiff > 0.0);
    // Parallel-plate leading estimate 1.777 Ω.
    close(result.zdiff, 1.777, 0.02, 0.0);
}

// A07
#[test]
fn wide_stripline_is_positive_not_negative() {
    let input = StriplineInput {
        width: 100.0,
        height: 5.0,
        thickness: 1.4,
        er: 4.6,
    };
    let result = stripline::calculate(&input).unwrap();
    assert!(result.zo.is_finite() && result.zo > 0.0);
    assert!(result.co_pf_per_in > 0.0 && result.lo_nh_per_in > 0.0);
}

// A08
#[test]
fn impossible_etched_cross_section_is_rejected() {
    let input = CurrentInput {
        width: 1.0,
        thickness: 1.4,
        etch_factor: EtchFactor::OneToOne,
        ..current_input()
    };
    assert!(current::calculate(&input).is_err());
}

#[test]
fn positive_area_does_not_excuse_negative_etched_top_width() {
    let input = CurrentInput {
        width: 2.0,
        thickness: 1.4,
        etch_factor: EtchFactor::OneToOne,
        ..current_input()
    };
    assert!(current::calculate(&input).is_err());
}

// A09
#[test]
fn nan_dimensions_are_rejected_by_library_boundary() {
    let input = CurrentInput {
        width: f64::NAN,
        ..current_input()
    };
    assert!(current::calculate(&input).is_err());
}

#[test]
fn nan_voltage_is_not_given_a_finite_spacing_recommendation() {
    assert!(
        spacing::spacing(&SpacingInput {
            voltage: f64::NAN,
            device_type: DeviceType::B1
        })
        .is_err()
    );
}

// A11 / A12
#[test]
fn ipc2152_resistance_is_continuous_at_temperature_bucket_boundary() {
    let a = current::calculate_ipc2152_estimate(&ipc2152(20.0, 10.0)).unwrap();
    let b = current::calculate_ipc2152_estimate(&ipc2152(20.000001, 10.0)).unwrap();
    close(b.resistance_dc, a.resistance_dc, 1e-5, 1e-12);
}

#[test]
fn ipc2152_resistance_at_60c_does_not_double_count_temperature() {
    let result = current::calculate_ipc2152_estimate(&ipc2152(60.0, 10.0)).unwrap();
    let expected = 6.787e-4 * (1.0 + 0.00393 * 40.0) * 1000.0 / 100.0;
    close(result.resistance_dc, expected, 1e-4, 1e-12);
}

#[test]
fn infinitesimal_temperature_rise_change_does_not_add_20pct_ampacity() {
    let a = current::calculate_ipc2152_estimate(&ipc2152(20.0, 10.0)).unwrap();
    let b = current::calculate_ipc2152_estimate(&ipc2152(20.0, 10.000001)).unwrap();
    close(b.current_capacity, a.current_capacity, 1e-4, 0.0);
}

#[test]
fn ipc2221_path_uses_its_ambient_temperature() {
    let a = current::calculate(&CurrentInput {
        ambient_temp: 20.0,
        ..current_input()
    })
    .unwrap();
    let b = current::calculate(&CurrentInput {
        ambient_temp: 60.0,
        ..current_input()
    })
    .unwrap();
    close(
        b.resistance_dc / a.resistance_dc,
        1.0 + 0.00393 * 40.0,
        1e-12,
        0.0,
    );
}

// A10
#[test]
fn unit_conversion_overflow_is_rejected() {
    assert!("1e308in".parse::<Length>().is_err());
    assert!("1e308GHz".parse::<Freq>().is_err());
}

#[test]
fn temperature_conversion_does_not_accept_nonfinite_result() {
    let value = "1e308F".parse::<Temperature>().unwrap();
    assert!(value.celsius().is_finite());
}

// A26
#[test]
fn weak_coupling_does_not_disappear_through_cancellation() {
    let actual = pcb_toolkit::differential::types::kb_terminated(1e-9).unwrap();
    close(actual, 5e-10, 1e-6, 1e-20);
}

// A16
#[test]
fn shared_copper_melting_constant_is_correct() {
    close(
        pcb_toolkit::constants::COPPER_MELTING_POINT_C,
        1084.62,
        0.0,
        0.01,
    );
    close(
        pcb_toolkit::fusing::COPPER_MELTING_TEMP_C,
        pcb_toolkit::constants::COPPER_MELTING_POINT_C,
        0.0,
        0.0,
    );
}

// A17
#[test]
fn thin_via_has_positive_parasitics_or_reports_out_of_domain() {
    let input = pcb_toolkit::via::ViaInput {
        hole_diameter_mils: 10.0,
        pad_diameter_mils: 20.0,
        antipad_diameter_mils: 40.0,
        height_mils: 0.5,
        plating_thickness_mils: 0.7,
        er: 4.6,
    };
    if let Ok(result) = pcb_toolkit::via::calculate(&input) {
        assert!(result.inductance_nh.is_finite() && result.inductance_nh > 0.0);
        assert!(result.impedance_ohms.is_finite() && result.impedance_ohms > 0.0);
    }
}

// A27
#[test]
fn interpolation_does_not_panic_on_nan() {
    assert!(pcb_toolkit::tables::interpolate::lerp(&[(0.0, 0.0), (1.0, 1.0)], f64::NAN).is_err());
}

// A19
#[test]
fn spacing_minimum_is_not_undercut() {
    let r = spacing::spacing(&SpacingInput {
        voltage: 10.0,
        device_type: DeviceType::B4,
    })
    .unwrap();
    assert!(r.spacing_mm >= 0.075);
}

// Positive controls.
#[test]
fn ideal_thermal_and_pdn_controls() {
    let thermal = pcb_toolkit::thermal::calculate(&pcb_toolkit::thermal::ThermalInput {
        r_theta_ja: 50.0,
        power_w: 1.0,
        t_ambient_c: 25.0,
    })
    .unwrap();
    close(thermal.t_junction_c, 75.0, 0.0, 1e-12);
    let pdn = pcb_toolkit::pdn::calculate(&pcb_toolkit::pdn::PdnInput {
        v_supply: 5.0,
        i_max: 2.0,
        i_step_pct: 50.0,
        v_ripple_pct: 5.0,
        area_sq_in: 5.0,
        er: 4.6,
        d_mils: 2.0,
        freq_mhz: 1.0,
    })
    .unwrap();
    close(pdn.z_target_ohms, 0.25, 0.0, 1e-12);
}

/// Every calculator result must serialize without a JSON `null` in a
/// non-optional field (i.e. no NaN/inf reaches a result struct).
#[test]
fn results_serialize_without_nonfinite_numbers() {
    let ms = serde_json::to_value(microstrip::calculate(&ms(10.0, 1e9)).unwrap()).unwrap();
    let sl = serde_json::to_value(
        stripline::calculate(&StriplineInput {
            width: 10.0,
            height: 10.0,
            thickness: 1.4,
            er: 4.6,
        })
        .unwrap(),
    )
    .unwrap();
    let cp = serde_json::to_value(coplanar::calculate(&cpw(10.0, 4.6)).unwrap()).unwrap();
    for v in [ms, sl, cp] {
        for (k, x) in v.as_object().unwrap() {
            assert!(x.is_number(), "{k} serialized as {x}");
        }
    }
}
