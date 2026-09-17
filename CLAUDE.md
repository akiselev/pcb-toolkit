# pcb-toolkit

Rust workspace for PCB design calculations — impedance, current capacity, via properties,
and more.

## Workspace Structure

* **crates/pcb-toolkit** Core library: all calculator modules, material database, unit conversion
* **crates/pcb-toolkit-cli** Command-line tool wrapping the library

## Architecture

```
pcb-toolkit (core library: calculations, materials, units, tables)
     ↓
pcb-toolkit-cli (binary: CLI interface, output formatting)
```

**Versioning:** Synchronized versions across both crates.

## Design Philosophy

- **Physics first, Saturn compatibility second**: every calculator exposes a `MODEL`
  constant (`pcb_toolkit::ModelInfo`) naming the published model, its validity range and its
  `ModelStatus` (validated / compatibility / experimental). Saturn PCB Toolkit v8.44 help-PDF
  examples are kept as compatibility vectors, but a Saturn match is not treated as evidence of
  accuracy; invariants (continuity, symmetry, limits) and independent evaluations are.
- **Validate everything**: use the `validate` helpers. Reject NaN/∞, check model domains,
  and return `CalcError::NonFiniteResult` rather than a non-finite number. Never clamp.
- **f64 everywhere**: All calculations use IEEE 754 double precision. No arbitrary precision,
  no `num` crate, no `uom` crate. Standard library math functions suffice.
- **Canonical internal units**: Convert at the API boundary, compute internally in canonical
  units (mils for length, Hz for frequency, Farads for capacitance, etc.). See
  `docs/notes/16-rust-design-research.md` for the full table.
- **Minimal dependencies**: `thiserror` + `serde` for the library. `clap` + `anyhow` +
  `serde_json` + `toml` for the CLI. No heavyweight crates.

## Error Handling

* `pcb-toolkit` uses `pcb_toolkit::CalcError` (via `thiserror`)
* `pcb-toolkit-cli` uses `anyhow`

All public calculation functions return `Result<T, CalcError>`. Validate inputs at the
boundary — negative dimensions, out-of-range ratios, unknown materials are all errors,
never silently clamped.

## Reverse Engineering Notes

All reverse engineering documentation lives in `docs/notes/`:
- `NOTES.md` (project root) — master consolidated findings
- `PROGRESS.md` (project root) — what's done, what's not, how to resume
- `docs/notes/00-overview.md` through `15-materials-data.md` — per-calculator notes
- `docs/notes/ghidra-impedance.md` — decompiled impedance calculator (610 lines)
- `docs/notes/16-rust-design-research.md` — crate/dependency decisions

The original binary is at `toolkit/toolkit.exe` (PE32 Delphi/C++ Builder, ~10.4 MB).
Use `ghidra-cli` (project: `saturn-pcb`) for further decompilation if needed.

## Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific calculator
cargo test -p pcb-toolkit impedance
cargo test -p pcb-toolkit wavelength
```

Test vectors from Saturn help PDF examples live alongside their modules as `#[cfg(test)]`
blocks. Use `approx` for float comparison (`assert_relative_eq!`).

## Calculator Modules

| Module                  | Model                                            | Status        |
| ----------------------- | ------------------------------------------------ | ------------- |
| `impedance::microstrip` | Hammerstad-Jensen 1980 + Kirschning-Jansen 1982  | validated     |
| `impedance::stripline`  | Cohn 1954 conformal mapping + Wadell thickness   | validated     |
| `impedance::embedded`   | H-J surface line + exponential cover filling     | compatibility |
| `impedance::coplanar`   | Conductor-backed CPW (Ghione-Naldi) + Gupta T    | validated     |
| `differential::*`       | IPC-2141A (external/embedded); Cohn 1955 (stripline family) | compat./validated |
| `via`                   | Johnson lumped C/L + barrel resistance           | compatibility |
| `current`               | IPC-2221A; IPC-2152-style estimate (experimental)| compat./exp.  |
| `fusing`                | Onderdonk adiabatic                              | compatibility |
| `inductor`              | Mohan 1999 (Wheeler / current sheet)             | validated     |
| `padstack`              | Geometry                                         | validated     |
| `crosstalk`             | NEXT rule of thumb                               | experimental  |
| `ohms_law`              | Exact relations; ABCD-verified attenuators       | validated     |
| `reactance`             | Xc/Xl/f_res                                      | validated     |
| `wavelength`            | λ = c/(f√εeff)                                   | validated     |
| `ppm`                   | PPM↔Hz, XTAL load                                | validated     |
| `spacing`               | IPC-2221C Table 6-1 (mm source values)           | validated     |

See `STATUS.md` for the evidence behind each status and `CHANGELOG.md` for the
0.2.0 audit response.
