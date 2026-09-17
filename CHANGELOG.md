# Changelog

## 0.2.0 — 2026-09-16

Response to the numerical accuracy audit (`pcb-toolkit-audit/AUDIT.md`).
Breaking API and CLI changes are marked **(breaking)**.

### Corrected formulas
- **π-pad attenuator** (A01): series resistor is now `Z·(K²−1)/(2K)`; the
  previous value was the T-pad expression. Verified with an ABCD oracle.
- **Embedded microstrip** (A02, A14): impedance is now `Zo,air/√εeff,embedded`
  with the exponential cover filling applied to εeff. Continuous at zero cover,
  air-cover invariant, monotonic. The differential embedded pair shares the
  external pair's base so zero cover reproduces it exactly.
- **Conductor-backed CPW** (A03): full Ghione-Naldi backed model
  (`εeff = (q + εr·q3)/(q + q3)`, `Zo = 60π/(√εeff·(q + q3))`) with exact AGM
  elliptic integrals; Gupta thickness correction applied instead of ignored.
- **Microstrip** (A04, A13): full Hammerstad-Jensen 1980 closed forms
  (continuous thickness correction) plus Kirschning-Jansen dispersion; the
  frequency input is no longer ignored. `common::effective_width` branch fixed
  to `1/(2π)`.
- **Stripline** (A07): exact Cohn conformal mapping with Wadell thickness;
  wide strips no longer return negative impedance.
- **Asymmetric stripline pair** (A05): real offset model (half-space
  combination); responds to offset at fixed total spacing.
- **Broadside pair** (A06): electric/magnetic-wall symmetry model with the
  correct parallel-plate small-gap limit; unshielded configuration now returns
  `CalcError::Unsupported` **(breaking)**.
- **Symmetric stripline pair**: Cohn 1955 coupled-line modes replace the
  microstrip empirical coupling term.
- **Resistance temperature** (A12): one linear correction from the 20 °C
  resistivity; the IPC-2221A path now honours `ambient_temp`. Results carry
  both `resistance_dc` (ambient) and `resistance_dc_at_rise` **(breaking)**.
- **IPC-2152** (A11): renamed to `calculate_ipc2152_estimate` **(breaking)**,
  marked experimental, modifier tables linearly interpolated (no 20% steps).
- **Copper melting point** (A16): `constants::COPPER_MELTING_POINT_C` is
  1084.62 °C and is the single source for the fusing module.
- **Inductor** (A23): circular spirals use Mohan's current-sheet expression
  with Table II coefficients.
- **Terminated coupling** (A26): rationalised `Kb/(1 + √(1−Kb²))`;
  `kb_terminated` returns `Result` **(breaking)**.

### Validation and contracts
- New `validate` module; every calculator rejects NaN/∞ inputs and returns
  `CalcError::NonFiniteResult` instead of a non-finite number (A09).
- New `CalcError::{NotFinite, NonFiniteResult, Unsupported}` variants.
- Etched trapezoids are validated (`EtchFactor::cross_section_sq_mils` returns
  `Result`) **(breaking)** (A08).
- Unit parsers check finiteness after scaling; Fahrenheit conversion no longer
  overflows; new `Resistance` parser (A10).
- Via: geometry/domain checks, plating used for barrel resistance
  (`resistance_mohm`) (A17).
- Fusing: temperature domain enforced (A24). PDN: negative frequency rejected
  (A25). `tables::interpolate::lerp` returns `Result` **(breaking)** (A27).
- Reactance requires at least one component.
- `DifferentialResult::{kb_db, kb_term_db}` are `Option<f64>` **(breaking)**.

### Provenance and labelling
- New `model` module: every calculator exposes `MODEL: ModelInfo` with name,
  reference, validity range and `ModelStatus`; the CLI prints the caveat for
  non-validated models (A22, A28).
- Materials carry `source` and `note`; Rogers Dk-convention mismatch recorded
  (A18). Spacing table stored in millimetres, categories documented (A19).
- `CopperWeight::thickness_mm` derived from mils (A20).
- `XtalLoadResult` fields renamed to `c_load_f` / `c_external_average_f`
  **(breaking)** (A15). `WireGaugeResult::area_saturn` renamed to
  `area_saturn_display` **(breaking)**.
- README/STATUS rewritten from exercised commands; validation matrix added.

### CLI **(breaking)**
- `pdn`: `--area` → `--area-sq-in`; `--distance` and `--freq` take units.
- `ohms-law`: component lists and impedances accept unit suffixes.
- `current --ambient` accepts `C`/`F` suffixes and is used.
- `impedance embedded` no longer takes `--freq` (quasi-static only).

### Tests
- `crates/pcb-toolkit/tests/accuracy_regressions.rs` (29 audit invariants) and
  `crates/pcb-toolkit-cli/tests/cli.rs` (JSON never `null` for numbers).
