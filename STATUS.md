# Implementation Status

192 tests passing across 16 calculator modules and supporting infrastructure.

## Library Modules (`pcb-toolkit`)

| Module | Description | Tests | Notes |
|--------|-------------|------:|-------|
| `impedance::microstrip` | Microstrip (Hammerstad-Jensen 1980) | 3 | Decompiled from Saturn, frequency-dependent Er_eff |
| `impedance::stripline` | Stripline (Cohn / Wadell) | 5 | |
| `impedance::embedded` | Embedded microstrip (Brooks) | 5 | Cover height correction over ground plane |
| `impedance::coplanar` | Coplanar waveguide (Wadell) | 10 | Complete elliptic integral model |
| `impedance::common` | Shared Er_eff, thickness correction | 3 | |
| `differential::edge_coupled_external` | Surface microstrip differential | 5 | |
| `differential::edge_coupled_internal_sym` | Centered stripline differential | 6 | |
| `differential::edge_coupled_internal_asym` | Offset stripline differential | 6 | |
| `differential::edge_coupled_embedded` | Buried microstrip differential | 5 | |
| `differential::broadside_coupled` | Broadside-coupled, shielded/unshielded | 6 | |
| `current` | IPC-2221A + IPC-2152 with modifier charts | 12 | DC resistance, skin depth, voltage drop |
| `fusing` | Onderdonk equation | 6 | Copper melting at 1084.62C |
| `via` | Coaxial model: C, L, Z, f_res | 3 | Goldfarb capacitance (constant 1.41) |
| `inductor` | Planar spiral (Mohan/Wheeler) | 5 | Square, hexagonal, octagonal, circular |
| `crosstalk` | NEXT estimation | 6 | |
| `ohms_law` | V=IR, LED bias, Pi/T-pad attenuators, R/C/L combinations | 12 | 10 sub-calculators |
| `reactance` | Xc, Xl, resonant frequency | 5 | |
| `wavelength` | Signal wavelength in dielectric | 4 | λ, λ/2, λ/4, λ/7, λ/10, λ/20 |
| `ppm` | PPM/Hz conversion, crystal load cap | 5 | |
| `padstack` | Thru-hole sizing, corner-to-corner diagonal | 6 | |
| `spacing` | IPC-2221C lookup table | 11 | 8 device types, >500V linear extrapolation |
| `wire_gauge` | AWG property lookup (44 gauges, 4/0-40) | 5 | |
| `pdn` | PDN impedance: Z_target, C_plane, Xc | 3 | Verified against Saturn Help PDF |
| `thermal` | Junction temperature: T_j = R*P + T_ambient | 4 | |
| `materials` | 45-material substrate database | 7 | Er, Tg, roughness factor |
| `units` | Length, Freq, Capacitance, Inductance, Temperature | 28 | FromStr with unit suffixes |
| `copper` | Copper weight/plating/etch enums | 3 | 9 weights, 7 plating, 3 etch factors |
| `tables` | Interpolation utilities | 4 | |

**Total: 192 tests**

## CLI Commands (`pcb-toolkit-cli`)

All 16 calculator commands are exposed. Every command supports `--json` output.

| Command | Subcommands | Status |
|---------|-------------|--------|
| `impedance` | `microstrip`, `stripline`, `embedded`, `coplanar` | Complete |
| `differential` | `edge-coupled-external`, `edge-coupled-internal-sym`, `edge-coupled-internal-asym`, `edge-coupled-embedded`, `broadside-coupled` | Complete |
| `current` | *(none)* | Complete |
| `fusing` | *(none)* | Complete |
| `via` | *(none)* | Complete |
| `inductor` | *(none)* | Complete |
| `reactance` | *(none)* | Complete |
| `wavelength` | *(none)* | Complete |
| `ohms-law` | `eir`, `led-bias`, `pi-pad`, `t-pad`, `resistors-series`, `resistors-parallel`, `capacitors-series`, `capacitors-parallel`, `inductors-series`, `inductors-parallel` | Complete |
| `ppm` | `hz-to-ppm`, `ppm-to-hz`, `xtal-load` | Complete |
| `padstack` | `thru-hole`, `corner-to-corner` | Complete |
| `spacing` | *(none)* | Complete |
| `wire-gauge` | *(none)* | Complete |
| `pdn` | *(none)* | Complete |
| `thermal` | *(none)* | Complete |
| `crosstalk` | *(none)* | Complete |

## Materials Database

45 substrate materials with verified Er, Tg, and roughness factor values. Data
extracted from Saturn PCB Toolkit v8.44 binary via disassembly of `ComboBox1Change`
at `0x00494dd4`.

Includes: FR-4 variants, Rogers (RO/RT series), Isola, Getek, Arlon, Nelco
(N4000/N7000), Ventec, PCL-FR series, Panasonic Megtron6, Kappa 438, Kapton, Teflon
PTFE, and Air.

## Known Limitations

- **IPC-2152 modifier tables**: The IPC-2152 current capacity calculator has the
  modifier framework implemented but uses simplified piecewise approximations for
  the chart-based modifiers (area, temperature, board thickness). The original
  Saturn implementation uses lookup tables that are partially extracted.
- **Broadside-coupled differential**: Low confidence — no Saturn test vector
  available for validation. Formula implemented from published references.
- **Crosstalk**: Marked as "unsupported" in the original Saturn UI. Our
  implementation uses the standard NEXT estimation formula.
- **Voltage divider**: Present in Saturn's Ohm's Law mode but not yet
  implemented in the library.
- **IPC-2152 solve-for-width**: Reverse mode (given target current, find
  required trace width) not implemented.

## Reverse Engineering

All 19 solver modes in the Saturn PCB Toolkit v8.44 binary have been fully
analyzed. See `PROGRESS.md` for RE details and `docs/notes/` for per-calculator
decompilation notes.
