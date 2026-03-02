# pcb-toolkit

A Rust library and command-line tool for PCB design calculations — impedance,
current capacity, via properties, and more.

Validated against [Saturn PCB Toolkit](https://saturnpcb.com/pcb_toolkit/) v8.44
output where possible.

## Status

This project is under active development. The core architecture (units, materials
database, error handling) is in place. Calculator modules are being implemented
incrementally — see the [module status table](#calculator-modules) below.

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

## Usage

### CLI

The CLI binary is called `pcb-toolkit`. All subcommands support `--json` for
machine-readable output.

```
pcb-toolkit impedance microstrip -w 10 --height 5 --er 4.6
```

```
Microstrip Impedance
────────────────────
  Zo      = 44.3599 Ω
  Er_eff  = 3.5172
  Tpd     = 158.8927 ps/in
  Lo      = 7.0485 nH/in
  Co      = 3.5819 pF/in
```

JSON output:

```
pcb-toolkit impedance microstrip -w 10 --height 5 --er 4.6 --json
```

### Library

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

## Calculator Modules

| Module                  | Description                                         | Status        |
| ----------------------- | --------------------------------------------------- | ------------- |
| `impedance::microstrip` | Microstrip impedance (Hammerstad-Jensen 1980)       | Implemented   |
| `impedance::stripline`  | Stripline impedance (Cohn / Wadell)                 | Planned       |
| `impedance::embedded`   | Embedded microstrip (Brooks)                        | Planned       |
| `impedance::coplanar`   | Coplanar waveguide (Wadell)                         | Planned       |
| `differential`          | Differential pair impedance (6 topologies)          | Types defined |
| `via`                   | Via parasitics: L, C, Z, resonance, DC R, thermal R | Planned       |
| `current`               | Conductor current capacity (IPC-2221A / IPC-2152)   | Planned       |
| `fusing`                | Fusing current (Onderdonk equation)                 | Planned       |
| `inductor`              | Planar spiral inductors (Mohan/Wheeler)             | Planned       |
| `padstack`              | Pad sizing (TH, BGA, routing — 7 sub-calculators)   | Planned       |
| `crosstalk`             | Standalone crosstalk estimation                     | Planned       |
| `ohms_law`              | V=IR, LED bias, attenuators, series/parallel        | Planned       |
| `reactance`             | Xc, Xl, resonant frequency                          | Planned       |
| `wavelength`            | Signal wavelength in dielectric                     | Planned       |
| `ppm`                   | PPM/Hz conversion, crystal load capacitor           | Planned       |
| `spacing`               | IPC-2221C minimum conductor spacing                 | Planned       |

## Materials Database

25 built-in substrate materials with dielectric constant (Er), glass transition
temperature, and surface roughness correction factor. Includes FR-4 variants,
Rogers, Isola, Getek, and Teflon.

```rust
use pcb_toolkit::materials;

let fr4 = materials::lookup("FR-4 STD").unwrap();
assert_eq!(fr4.er, 4.6);
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

Float comparisons use the `approx` crate (`assert_relative_eq!`).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
