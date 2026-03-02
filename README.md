# pcb-toolkit

A Rust library and command-line tool for PCB design calculations — impedance,
current capacity, via properties, and more.

Validated against [Saturn PCB Toolkit](https://saturnpcb.com/pcb_toolkit/) v8.44
output where possible.

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
machine-readable output. Dimensional inputs accept units: `10mil`, `0.254mm`,
`1GHz`, `10nF`, etc.

### Commands

```
pcb-toolkit <COMMAND> [OPTIONS]

impedance      Transmission line impedance (microstrip, stripline, embedded, coplanar)
differential   Differential pair impedance (5 topologies)
current        Conductor current capacity (IPC-2221A)
fusing         Fusing current (Onderdonk equation)
via            Via impedance and parasitic properties
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
crosstalk      Crosstalk estimation (NEXT)
```

### Examples

```
$ pcb-toolkit impedance microstrip -w 10 --height 5 --er 4.6

Microstrip Impedance
────────────────────
  Zo      = 44.3599 Ω
  Er_eff  = 3.5172
  Tpd     = 158.8927 ps/in
  Lo      = 7.0485 nH/in
  Co      = 3.5819 pF/in
```

```
$ pcb-toolkit differential edge-coupled-external -w 10 --spacing 10 --height 5 -t 1.4mil --er 4.6

Edge-Coupled External Differential
───────────────────────────────────
  Zdiff    = 76.3503 Ω
  Zo       = 41.0649 Ω
  Zodd     = 38.1751 Ω
  Zeven    = 44.1735 Ω
  Kb       = 0.072841
  Kb       = -22.7525 dB
  Kb_term  = 0.036469
  Kb_term  = -28.7616 dB
```

```
$ pcb-toolkit pdn --voltage 5 --current 2 --i-step 50 --v-ripple 5 --area 5 --er 4.6 --distance 2 --freq 1

PDN Impedance
─────────────
  Z target   = 0.250000 Ω
  C plane    = 2587.5000 pF
  Xc         = 61.509157 Ω
```

```
$ pcb-toolkit --json impedance microstrip -w 10 --height 5 --er 4.6

{
  "zo": 44.359895838626485,
  "er_eff": 3.5171650243068555,
  "tpd_ps_per_in": 158.89270763232736,
  "lo_nh_per_in": 7.048463960087373,
  "co_pf_per_in": 3.581899926238581
}
```

## Library

Add the dependency:

```toml
[dependencies]
pcb-toolkit = "0.1"
```

```rust
use pcb_toolkit::impedance::microstrip::{self, MicrostripInput};

let result = microstrip::calculate(&MicrostripInput {
    width: 10.0,       // mils
    height: 5.0,       // mils
    thickness: 1.4,    // mils (1 oz copper)
    er: 4.6,           // FR-4
    frequency: 0.0,    // Hz (0 = static)
}).unwrap();

println!("Zo = {:.2} Ohms", result.zo);
println!("Er_eff = {:.4}", result.er_eff);
```

All public functions return `Result<T, CalcError>`. Inputs are validated at the
boundary — negative dimensions, out-of-range dielectric constants, and unknown
materials are rejected with descriptive errors.

## Materials Database

45 built-in substrate materials with dielectric constant (Er), glass transition
temperature (Tg), and surface roughness correction factor. Includes FR-4
variants, Rogers, Isola, Getek, Arlon, Nelco, Ventec, Panasonic, and Teflon.

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

Float comparisons use the `approx` crate (`assert_relative_eq!`). Test vectors
sourced from the Saturn PCB Toolkit help PDF and manual testing against v8.44.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
