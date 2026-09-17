# pcb-toolkit

A Rust library and command-line tool for PCB design calculations — impedance,
current capacity, via properties, and more.

Every calculator states which published model it implements, its validity
range, and its validation status (`validated`, `compatibility`, or
`experimental`) through a `MODEL` constant; the CLI prints that caveat for
anything that is not `validated`. Compatibility with
[Saturn PCB Toolkit](https://saturnpcb.com/pcb_toolkit/) v8.44 is checked where
its help PDF gives examples, but "matches Saturn" is tracked separately from
"physically validated" — see [STATUS.md](STATUS.md) for the per-calculator
matrix.

## Workspace

| Crate             | Description                                                   |
| ----------------- | ------------------------------------------------------------- |
| `pcb-toolkit`     | Core library: calculators, material database, unit conversion |
| `pcb-toolkit-cli` | Command-line interface wrapping the library                   |

## Building

Requires Rust 1.85+ (edition 2024).

```
cargo build --workspace
```

## CLI

The CLI binary is called `pcb-toolkit`. All commands support `--json` for
machine-readable output. Lengths, frequencies, capacitances, inductances,
resistances and temperatures accept unit suffixes (`10mil`, `0.254mm`, `1GHz`,
`10nF`, `4.7k`, `77F`); a bare number means the canonical unit named in the
option's help text (mil, Hz, F, H, Ohm, °C). Percentages, voltages, currents
and areas are plain numbers whose unit is named in the option's help.

### Commands

```
pcb-toolkit <COMMAND> [OPTIONS]

impedance      Transmission line impedance (microstrip, stripline, embedded, coplanar)
differential   Differential pair impedance (5 topologies)
current        Conductor current capacity (IPC-2221A)
fusing         Fusing current (Onderdonk equation)
via            Via lumped parasitics and barrel resistance
inductor       Planar spiral inductor
reactance      Capacitive/inductive reactance and resonant frequency
wavelength     Wavelength in a dielectric
ohms-law       V=IR, LED bias, attenuators, series/parallel R/C/L
ppm            PPM/Hz conversion, crystal load capacitance
padstack       Padstack geometry (thru-hole, corner-to-corner)
spacing        Conductor spacing (IPC-2221C)
wire-gauge     AWG wire gauge properties
pdn            PDN impedance calculator
thermal        Thermal management (junction temperature)
crosstalk      Crosstalk estimation (NEXT) — experimental rule of thumb
```

### Examples

Outputs below are copied from the built binary.

```
$ pcb-toolkit impedance microstrip -w 10 --height 5 --er 4.6

Microstrip Impedance
────────────────────
  Zo      = 44.8322 Ω
  Er_eff  = 3.3075
  Tpd     = 154.0839 ps/in
  Lo      = 6.9079 nH/in
  Co      = 3.4369 pF/in
```

```
$ pcb-toolkit differential edge-coupled-external -w 10 --spacing 10 --height 5 -t 1.4mil --er 4.6

Edge-Coupled External Differential
──────────────────────────────────
  Zdiff    = 76.3503 Ω
  Zo       = 41.0649 Ω
  Zodd     = 38.1751 Ω
  Zeven    = 44.1735 Ω
  Kb       = 0.072841
  Kb       = -22.7525 dB
  Kb_term  = 0.036469
  Kb_term  = -28.7616 dB

  Model: IPC-2141A surface microstrip differential pair [compatibility] — IPC-2141A §4; National Semiconductor AN-905; Saturn PCB Toolkit help p.11. Range: 0.1 < W/H < 2.0, 1 < εr < 15, 0.2 ≤ S/H ≤ 3.0; single-ended ±5%, coupling term empirical (Zeven from Zo²/Zodd)
```

```
$ pcb-toolkit pdn --voltage 5 --current 2 --i-step 50 --v-ripple 5 --area-sq-in 5 --er 4.6 --distance 2mil --freq 1MHz

PDN Impedance
─────────────
  Z target   = 0.250000 Ω
  C plane    = 2587.5000 pF
  Xc         = 61.509157 Ω
```

```
$ pcb-toolkit --json impedance microstrip -w 10 --height 5 --er 4.6

{
  "zo": 44.832236725855715,
  "er_eff": 3.307496138618307,
  "tpd_ps_per_in": 154.08390124056288,
  "lo_nh_per_in": 6.9079259360602885,
  "co_pf_per_in": 3.436899706404775
}
```

## Library

Add the dependency:

```toml
[dependencies]
pcb-toolkit = "0.2"
```

```rust
use pcb_toolkit::impedance::microstrip::{self, MicrostripInput};

let result = microstrip::calculate(&MicrostripInput {
    width: 10.0,       // mils
    height: 5.0,       // mils
    thickness: 1.4,    // mils (1 oz copper)
    er: 4.6,           // FR-4
    frequency: 0.0,    // Hz (0 = quasi-static; > 0 applies Kirschning-Jansen dispersion)
}).unwrap();

println!("Zo = {:.2} Ohms", result.zo);
println!("Er_eff = {:.4}", result.er_eff);
println!("{}", microstrip::MODEL.caveat());
```

All calculation functions return `Result<T, CalcError>`. Inputs are validated
at the boundary: non-finite values, negative dimensions, out-of-range dielectric
constants, impossible etched cross-sections, and geometries outside a model's
domain are rejected with descriptive errors, and a formula that would produce a
non-finite number returns `CalcError::NonFiniteResult` instead of `NaN`. Unit
conversion helpers (`units::to_mils` and friends) and the copper thickness
accessors are plain functions; the `FromStr` parsers reject overflow.

## Materials Presets

45 built-in substrate presets with dielectric constant (Er), glass transition
temperature (Tg), and Saturn's surface roughness factor. These reproduce the
Saturn PCB Toolkit v8.44 material table and carry a `source` field saying so;
they are compatibility presets, not manufacturer data (a scalar Er with no test
method, frequency or construction). Entries whose Saturn value mixes Dk
conventions (e.g. RO4003/RO4350) carry a `note`. Use the laminate datasheet
for an impedance-controlled design.

```rust
use pcb_toolkit::materials;

let fr4 = materials::lookup("FR-4 STD").unwrap();
assert_eq!(fr4.er, 4.6);
assert_eq!(fr4.tg, Some(130.0));
```

Lookup is case-insensitive. Custom Er values can be passed directly to calculator
functions.

## Design Decisions

- **f64 everywhere.** IEEE 754 double precision for all calculations. No
  arbitrary-precision or unit-of-measure crates.
- **Canonical internal units.** Mils for length, Hz for frequency, Farads for
  capacitance. Conversion happens at the API boundary via the `units` module.
- **Minimal dependencies.** `thiserror` + `serde` for the library. `clap` +
  `anyhow` + `serde_json` + `toml` for the CLI.

## Testing

```
cargo test --workspace
```

Test a specific calculator:

```
cargo test -p pcb-toolkit impedance
```

Float comparisons use the `approx` crate (`assert_relative_eq!`). Three layers
of tests exist:

- unit tests next to each module: exact relations, physical invariants
  (continuity, symmetry, air-line invariance, monotonicity), independent
  reference evaluations (AGM elliptic integrals, ABCD circuit oracle), and
  Saturn help-PDF compatibility vectors;
- `crates/pcb-toolkit/tests/accuracy_regressions.rs`: the defects demonstrated
  by the 2026-09 numerical audit, asserted as invariants;
- `crates/pcb-toolkit-cli/tests/cli.rs`: end-to-end checks that JSON output
  never carries `null` for a number and that invalid input fails.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
