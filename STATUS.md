# Implementation Status

Test count is whatever `cargo test --workspace` reports (currently 224 unit +
29 audit-regression + 4 CLI integration tests); it is not evidence of physical
accuracy. The evidence column below is.

## Validation matrix

Status meanings (`pcb_toolkit::ModelStatus`): **validated** = exact relation or
published closed form checked against an independent evaluation inside its
stated range; **compatibility** = published approximation kept for Saturn /
IPC-2141 compatibility, internally consistent, accuracy checked only against
the source's own examples; **experimental** = heuristic or partially
reconstructed model, estimate only.

| Module | Model | Status | Evidence | Known limits |
|--------|-------|--------|----------|--------------|
| `impedance::microstrip` | Hammerstad-Jensen 1980 + Kirschning-Jansen 1982 dispersion | validated | H-J closed forms reproduce the older Hammerstad form within 0.5%; continuity and monotonicity sweeps; dispersion range enforced | 0.01 ≤ W/H ≤ 100, εr ≤ 128, T < H; dispersion εr ≤ 20, H/λ₀ ≤ 0.13 |
| `impedance::stripline` | Cohn 1954 conformal mapping + Wadell thickness | validated | Exact K(k)/K(k') via AGM; matches Wheeler closed form within 0.5%; parallel-plate limit; positive for all W/B | T < B; W/B ≲ 400 |
| `impedance::embedded` | H-J surface line + exponential cover filling (Wadell) | compatibility | Continuous at zero cover, air-cover invariant, monotonic, correct deep-burial limit | Quasi-static only; cover εr = substrate εr |
| `impedance::coplanar` | Conductor-backed CPW (Ghione-Naldi) + Gupta thickness | validated | Zero-thickness form agrees with an independent AGM evaluation to 1e-10; εeff bounded by materials; air line depends on ground height | T ≪ S; rejected once the thickness correction exceeds the gap |
| `differential::edge_coupled_external` | IPC-2141A / AN-905 | compatibility | Saturn help p.11 vector reproduced | 0.1 < W/H < 2, 0.2 ≤ S/H ≤ 3; Zeven = Zo²/Zodd approximation |
| `differential::edge_coupled_embedded` | IPC-2141A pair + cover filling | compatibility | Reduces exactly to the external pair at zero cover; air-cover invariant | As above |
| `differential::edge_coupled_internal_sym` | Cohn 1955 coupled stripline + eq. 18/20/22 thickness | validated | Zero-thickness modes agree with an independent AGM evaluation; modes converge to the single strip for wide spacing; continuous at S = 5T | T < B; thickness corrections ±2% |
| `differential::edge_coupled_internal_asym` | Half-space combination of Cohn modes | compatibility | Exact for H1 = H2; symmetric under plane swap; responds to offset at fixed total spacing | Inter-half-space fringing neglected |
| `differential::broadside_coupled` | Electric/magnetic-wall symmetry + Cohn strips | compatibility | Reproduces the parallel-plate limit within 2%; monotonic in separation while the partner is nearest | Shielded only; unshielded returns `Unsupported` |
| `current::calculate` | IPC-2221A power law | compatibility | Formula values reproduced; resistance corrected for temperature once | ±10–20% vs. IPC-2152 data |
| `current::calculate_ipc2152_estimate` | IPC-2221A × Saturn-reconstructed modifiers | experimental | Modifiers linearly interpolated (continuous); no IPC-2152 chart data | Estimate only; not IPC-2152 |
| `via` | Johnson lumped C/L + plated-barrel R | compatibility | Saturn help p.36 vector | h ≥ d_hole; lumped only, not a via transition model |
| `fusing` | Onderdonk adiabatic | compatibility | Saturn help p.16 vector; domain −234 °C < Ta < Tm enforced | Melting onset only |
| `inductor` | Mohan 1999 (modified Wheeler; current sheet for circle) | validated | Saturn help p.30 vector; Table I/II coefficients | ρ ≳ 0.1, s ≤ 3w; free-space DC inductance |
| `crosstalk` | NEXT rule of thumb | experimental | None (does not reproduce Saturn's example) | Order of magnitude only |
| `ohms_law` | Exact relations; attenuators | validated | ABCD-matrix oracle: input match and S21 exact for π and T pads | Ideal elements |
| `reactance`, `wavelength`, `thermal`, `pdn`, `padstack`, `ppm` | Exact relations | validated | Saturn help vectors; algebra checked | Scope stated in each module |
| `spacing` | IPC-2221C Table 6-1 (stored in mm) | validated | All 72 cells match the published table; >500 V slopes | Electrical clearance only |
| `wire_gauge` | AWG table | compatibility | 44 entries consistent with the AWG progression within table rounding | `area_saturn_display` has no physical meaning |
| `materials` | Saturn v8.44 presets | compatibility | Values extracted from the binary; provenance recorded per entry | Not manufacturer data |
| `units`, `copper`, `tables` | Conversions | validated | Overflow rejected after scaling; mm derived from mils; `lerp` returns `Result` | — |

## CLI Commands (`pcb-toolkit-cli`)

All 16 calculator commands are exposed. Every command supports `--json` output.
Text output of any non-validated model ends with a `Model:` caveat line.

| Command | Subcommands |
|---------|-------------|
| `impedance` | `microstrip`, `stripline`, `embedded`, `coplanar` |
| `differential` | `edge-coupled-external`, `edge-coupled-internal-sym`, `edge-coupled-internal-asym`, `edge-coupled-embedded`, `broadside-coupled` (shielded only) |
| `current` | IPC-2221A (the IPC-2152 estimate is library-only) |
| `fusing`, `via`, `inductor`, `reactance`, `wavelength`, `spacing`, `wire-gauge`, `pdn`, `thermal`, `crosstalk` | — |
| `ohms-law` | `eir`, `led-bias`, `pi-pad`, `t-pad`, `resistors-series`, `resistors-parallel`, `capacitors-series`, `capacitors-parallel`, `inductors-series`, `inductors-parallel` |
| `ppm` | `hz-to-ppm`, `ppm-to-hz`, `xtal-load` |
| `padstack` | `thru-hole`, `corner-to-corner` |

## Known Limitations

- **IPC-2152**: only an experimental estimate built on IPC-2221A with
  Saturn-reconstructed modifier curves exists (`calculate_ipc2152_estimate`).
  Real IPC-2152 chart data and a validated interpolation are not implemented.
- **Unshielded broadside-coupled pairs**: no validated closed form; rejected.
- **Embedded microstrip dispersion**: quasi-static only; a non-zero frequency
  is rejected rather than ignored.
- **Crosstalk**: rule-of-thumb estimate, marked experimental in the API and CLI.
- **Coupled-line accuracy**: the external/embedded differential pairs use the
  IPC-2141 empirical coupling term. No field-solver benchmark set exists in the
  repository; the stripline family is exact for zero thickness only.
- **Voltage divider** and **IPC-2152 solve-for-width** are not implemented.

## Reverse Engineering

All 19 solver modes in the Saturn PCB Toolkit v8.44 binary have been analyzed.
See `PROGRESS.md` and `docs/notes/`. Saturn's own microstrip solver is
Hammerstad-Jensen with Kirschning-Jansen dispersion (the decompiled constants
in `docs/notes/stripline-formulas-clean.md` are exactly those models), which
is what `impedance::microstrip` now implements.
