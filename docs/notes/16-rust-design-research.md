# Rust Implementation Design Research

## Crate & Dependency Decisions

### Numeric Type: `f64`

Plain `f64`. No arbitrary precision needed. PCB manufacturing tolerances are
±0.5 mil trace width, ±2% impedance — we have ~15 digits of precision and need
~3. The original Saturn tool uses C++ `double` (also f64-equivalent). Every
formula in the decompiled code operates on 8-byte IEEE 754 doubles.

Standard library provides everything: `sqrt`, `ln`, `log10`, `sin`, `cos`,
`exp`, `powf`, `abs`, `PI`, `FRAC_1_PI`, etc. No `num` crate needed.

For float comparison in tests: `approx` crate (`assert_relative_eq!`).

### Unit Handling: Roll Our Own (skip `uom`)

**Why not `uom`:**
- Drastically increases compile times (heavy generics)
- Error messages become unreadable
- Formulas become verbose: `Length::new::<mil>(5.0)` vs `5.0`
- Our formulas are well-defined — we know exactly what units go where
- The original tool converts at the boundary, does math in canonical units

**Our approach:**
- Accept inputs with an explicit `Units` enum (`Mils`, `Mm`, `Inches`, `Um`)
- Convert to canonical internal units at the API boundary
- All internal math in **mils** for length (matching the original tool's primary mode)
- Convert to requested output units on the way out
- Frequency always in Hz internally, accept MHz/GHz/kHz at boundary
- Capacitance in Farads internally, display as pF/nF/µF
- Inductance in Henries internally, display as nH/µH/mH

Simple conversion functions, no generics overhead:
```rust
pub enum LengthUnit { Mils, Mm, Inches, Um }

/// Convert from user units to mils (internal canonical unit)
pub fn to_mils(value: f64, unit: LengthUnit) -> f64 {
    match unit {
        LengthUnit::Mils => value,
        LengthUnit::Mm => value / 0.0254,     // 1 mil = 0.0254 mm
        LengthUnit::Inches => value * 1000.0,
        LengthUnit::Um => value / 25.4,        // 1 mil = 25.4 µm
    }
}
```

### CLI: `clap` (derive)

Standard choice. Subcommand per calculator:
```
pcb-toolkit impedance microstrip --width 5 --height 4 --er 4.5
pcb-toolkit current --width 10 --copper 1oz --rise 20
pcb-toolkit wavelength --freq 1GHz --er-eff 3.28
```

### Serialization: `serde` + `toml` + `serde_json`

- **TOML** for material database and config (human-editable)
- **JSON** for test vectors and machine output (`--json` flag)
- Both via `serde` derive

### Error Handling: `thiserror` (lib) + `anyhow` (CLI)

Library crate exposes typed errors:
```rust
#[derive(Debug, thiserror::Error)]
pub enum CalcError {
    #[error("W/H ratio {0:.3} outside valid range [0.1, 3.0]")]
    InvalidWHRatio(f64),
    #[error("negative dimension: {name} = {value}")]
    NegativeDimension { name: &'static str, value: f64 },
    #[error("unknown material: {0}")]
    UnknownMaterial(String),
}
```

CLI wraps with `anyhow` for context.

### Lookup Tables / Interpolation: Hand-rolled

The IPC tables are small:
- IPC-2221C spacing: 8 device types × 10 voltage ranges = 80 entries
- IPC-2152 current: a few hundred entries at most
- Copper weight tables: 9 entries

Simple linear interpolation in ~20 lines of code. No crate needed:
```rust
fn lerp_table(table: &[(f64, f64)], x: f64) -> f64 {
    // binary search + linear interpolation
}
```

### Testing: `approx` + JSON test vectors

```toml
[dev-dependencies]
approx = "0.5"
```

Test vectors extracted from Saturn PCB Toolkit help PDF examples, stored in
`tests/vectors/`. Each calculator gets a JSON file of known input→output pairs.

---

## Dependency Summary

```toml
# pcb-toolkit (library crate)
[dependencies]
thiserror = "2"
serde = { version = "1", features = ["derive"] }

# pcb-toolkit-cli (binary crate)
[dependencies]
pcb-toolkit = { path = "../pcb-toolkit" }
clap = { version = "4", features = ["derive"] }
anyhow = "1"
serde_json = "1"
toml = "0.8"

[dev-dependencies]
approx = "0.5"
serde_json = "1"
```

Minimal dependency footprint. No `uom`, no `num`, no `lookup-tables` crate.

---

## Project Structure

```
pcb-toolkit/              # workspace root
├── Cargo.toml            # workspace definition
├── pcb-toolkit/          # library crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── error.rs
│       ├── units.rs            # unit conversion (LengthUnit, FreqUnit, etc.)
│       ├── constants.rs        # physical constants (c, µ₀, ε₀, etc.)
│       ├── materials.rs        # material database (Er, Tg, roughness)
│       ├── copper.rs           # copper weight/thickness tables
│       ├── impedance/
│       │   ├── mod.rs
│       │   ├── microstrip.rs
│       │   ├── stripline.rs
│       │   ├── embedded.rs
│       │   ├── coplanar.rs
│       │   ├── common.rs       # shared: Er_eff, thickness correction, dispersion
│       │   └── types.rs        # ImpedanceResult, Topology enum
│       ├── differential/
│       │   ├── mod.rs
│       │   ├── edge_coupled.rs
│       │   ├── broadside.rs
│       │   └── types.rs
│       ├── via.rs              # via L, C, Z, thermal
│       ├── current.rs          # IPC-2152, IPC-2221A conductor current
│       ├── fusing.rs           # Onderdonk fusing current
│       ├── inductor.rs         # planar spiral inductor (Mohan/Wheeler)
│       ├── padstack.rs         # pad sizing (TH, BGA, routing)
│       ├── crosstalk.rs        # standalone crosstalk
│       ├── ohms_law.rs         # E-I-R, LED bias, series/parallel, attenuators
│       ├── reactance.rs        # Xc, Xl, f_resonant
│       ├── wavelength.rs       # λ = c / (f √Er_eff)
│       ├── ppm.rs              # PPM ↔ Hz, XTAL load caps
│       ├── spacing.rs          # IPC-2221C conductor spacing lookup
│       └── tables/
│           ├── mod.rs
│           ├── ipc2152.rs      # current capacity lookup data
│           ├── ipc2221c.rs     # spacing lookup data
│           └── interpolate.rs  # generic lerp/table lookup
├── pcb-toolkit-cli/      # binary crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── impedance.rs
│       │   ├── current.rs
│       │   ├── ... (one per calculator)
│       │   └── common.rs       # shared CLI args (units, copper weight)
│       └── output.rs           # formatting, --json support
├── data/
│   └── materials.toml          # material database
└── tests/
    └── vectors/
        ├── impedance.json
        ├── current.json
        └── ... (one per calculator)
```

---

## Calculator Module Inventory

| Module | Formulas | Complexity | Notes |
|--------|----------|------------|-------|
| `impedance/microstrip` | Hammerstad-Jensen, thickness correction, K-J dispersion | Medium | Well-decompiled, good test vectors |
| `impedance/stripline` | Cohn model | Medium | Not decompiled, use published formulas |
| `impedance/embedded` | Brooks embedded microstrip | Medium | Reference in binary |
| `impedance/coplanar` | CPW formulas (Wadell) | Medium | |
| `impedance/common` | Er_eff(f), We(W,T,H) | Low | Shared across all topologies |
| `differential/*` | Coupled-line Zodd/Zeven, Kb | High | 6 layer types, crosstalk coupling |
| `via` | Coax model L/C/Z, thermal R | Medium | 4/π constant confirmed |
| `current` | IPC-2152 + IPC-2221A, skin depth | High | Lookup tables + empirical formulas |
| `fusing` | Onderdonk equation | Low | Simple, well-understood |
| `inductor` | Mohan/Wheeler modified | Low | 4 geometry constants |
| `padstack` | Pure geometry | Low | 7 sub-calculators, all arithmetic |
| `crosstalk` | NEXT estimation | Low | Marked "unsupported" in original |
| `ohms_law` | V=IR, attenuator networks | Low | 7 sub-modes, basic algebra |
| `reactance` | Xc, Xl, f_res | Low | Trivial formulas |
| `wavelength` | λ = c/(f√Er) | Low | Trivial |
| `ppm` | PPM↔Hz, XTAL caps | Low | Trivial |
| `spacing` | IPC-2221C lookup | Low | Pure table lookup |

---

## Internal Canonical Units

All internal computation uses these canonical units to match the original tool:

| Quantity | Internal Unit | Rationale |
|----------|--------------|-----------|
| Length | mils | Original tool primary mode, most formulas written in mils |
| Frequency | Hz | SI base, convert MHz/GHz at boundary |
| Capacitance | Farads | SI base, display as pF |
| Inductance | Henries | SI base, display as nH |
| Resistance | Ohms | Already SI |
| Temperature | °C | PCB industry standard |
| Current | Amps | SI base |
| Time | seconds | SI base |
| Copper weight | oz/ft² | Industry standard, map to thickness via table |
| Impedance | Ohms | Already SI |
| Voltage | Volts | Already SI |

**Exception**: Some formulas (Hammerstad-Jensen) use mixed units internally
(e.g., mils for geometry but m/s for speed of light). The original does this
too — we replicate it for accuracy and match Saturn's output exactly.

---

## Key Physical Constants (from binary, verified)

```rust
pub const SPEED_OF_LIGHT_MS: f64 = 299_792_458.0;      // m/s
pub const SPEED_OF_LIGHT_IN_NS: f64 = 11.803;           // in/ns (approx)
pub const MU_0: f64 = 1.2566370614359e-6;               // H/m (4π×10⁻⁷)
pub const EPSILON_0: f64 = 8.854187817e-12;              // F/m
pub const COPPER_RESISTIVITY: f64 = 1.724e-6;            // Ω·cm at 20°C
pub const COPPER_TEMP_COEFF: f64 = 0.00393;              // /°C
pub const COPPER_MELTING_POINT: f64 = 1064.62;           // °C
pub const FOUR_OVER_PI: f64 = 1.2732395447351628;        // 4/π
pub const MIL_TO_M: f64 = 2.54e-5;                      // 1 mil in meters
pub const INCH_TO_CM: f64 = 2.54;
```

---

## Open Questions

1. **IPC-2152 table data**: Not yet extracted from binary. Need to either:
   - Continue Ghidra decompilation of conductor current solver
   - Use published IPC-2152 data (if legally permissible)
   - Implement IPC-2221A only initially (simpler, formula-based)

2. **IPC-2221C spacing table**: Same situation — need full lookup data.

3. **Remaining 21 material Er/Tg values**: ComboBox1Change not fully decompiled.
   Could supplement with published datasheets for known materials.

4. **Stripline/differential formulas**: Several solver functions too large for
   Ghidra. Use published academic formulas (Wadell, Cohn) and validate against
   Saturn output.

5. **Etch factor geometry**: The trapezoidal cross-section calculation is
   documented but should be validated with test vectors.

6. **Skin depth at frequency**: Need to verify Saturn's skin depth formula matches
   standard `δ = √(ρ/(πfµ₀))`.
