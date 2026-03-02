# pcb-toolkit

Rust library for PCB design calculations — impedance, current capacity, via
properties, and more.

Part of the [pcb-toolkit](https://github.com/akiselev/pcb-toolkit) workspace.

## Usage

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
println!("Tpd = {:.2} ps/in", result.tpd_ps_per_in);
```

All public functions return `Result<T, CalcError>`. Negative dimensions,
out-of-range dielectric constants, and unknown materials are rejected with
descriptive errors.

## Modules

| Module | Description | Status |
|---|---|---|
| `impedance::microstrip` | Microstrip impedance (Hammerstad-Jensen 1980) | Implemented |
| `impedance::stripline` | Stripline impedance (Cohn / Wadell) | Planned |
| `impedance::embedded` | Embedded microstrip (Brooks) | Planned |
| `impedance::coplanar` | Coplanar waveguide (Wadell) | Planned |
| `differential` | Differential pair impedance (6 topologies) | Types defined |
| `via` | Via parasitics: L, C, Z, resonance, DC R, thermal R | Planned |
| `current` | Conductor current capacity (IPC-2221A / IPC-2152) | Planned |
| `fusing` | Fusing current (Onderdonk equation) | Planned |
| `inductor` | Planar spiral inductors (Mohan/Wheeler) | Planned |
| `padstack` | Pad sizing (TH, BGA, routing) | Planned |
| `crosstalk` | Standalone crosstalk estimation | Planned |
| `ohms_law` | V=IR, LED bias, attenuators, series/parallel | Planned |
| `reactance` | Xc, Xl, resonant frequency | Planned |
| `wavelength` | Signal wavelength in dielectric | Planned |
| `ppm` | PPM/Hz conversion, crystal load capacitor | Planned |
| `spacing` | IPC-2221C minimum conductor spacing | Planned |

## Supporting Infrastructure

- **`materials`** — 25 built-in substrate materials (FR-4, Rogers, Isola, Getek,
  Teflon) with Er, Tg, and roughness correction. Case-insensitive lookup.
- **`copper`** — Copper weight to thickness conversion (0.25 oz through 5 oz),
  plating thickness, etch factor cross-section correction.
- **`units`** — Conversion between user-facing units and canonical internal units
  (mils, Hz, Farads, Henries, Celsius).
- **`constants`** — Physical constants (speed of light, copper resistivity, etc.).
- **`tables`** — Linear interpolation for lookup tables.

## Design

- **f64 everywhere.** No arbitrary-precision or unit-of-measure crates.
- **Canonical internal units.** Mils for length, Hz for frequency. Conversion at
  the API boundary.
- **Minimal dependencies.** `thiserror` for errors, `serde` for serialization.

## License

Licensed under either of [Apache License, Version 2.0](../../LICENSE-APACHE) or
[MIT License](../../LICENSE-MIT) at your option.
