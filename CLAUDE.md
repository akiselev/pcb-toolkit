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

- **Match Saturn output**: Our primary validation target is Saturn PCB Toolkit v8.44. When
  implementing formulas, match its output to within display rounding. Test vectors come from
  the Saturn help PDF and manual testing.
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

| Module                  | Formula Source         | Status                             |
| ----------------------- | ---------------------- | ---------------------------------- |
| `impedance::microstrip` | Hammerstad-Jensen 1980 | Decompiled, test vectors available |
| `impedance::stripline`  | Cohn / Wadell          | Published formulas                 |
| `impedance::embedded`   | Brooks                 | Published formulas                 |
| `impedance::coplanar`   | Wadell                 | Published formulas                 |
| `differential::*`       | Coupled-line theory    | 6 layer types                      |
| `via`                   | Coaxial model          | Partially decompiled               |
| `current`               | IPC-2152 / IPC-2221A   | IPC-2221A formula known            |
| `fusing`                | Onderdonk equation     | Fully documented                   |
| `inductor`              | Mohan/Wheeler modified | Fully documented                   |
| `padstack`              | Geometry               | Fully documented                   |
| `crosstalk`             | NEXT estimation        | Marked unsupported in original     |
| `ohms_law`              | V=IR, attenuators      | Trivial                            |
| `reactance`             | Xc/Xl/f_res            | Trivial                            |
| `wavelength`            | λ = c/(f√Er)           | Decompiled                         |
| `ppm`                   | PPM↔Hz, XTAL caps      | Trivial                            |
| `spacing`               | IPC-2221C lookup       | Need table data                    |
