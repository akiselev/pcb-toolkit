# pcb-toolkit-cli

Command-line interface for PCB design calculations. Wraps the
[pcb-toolkit](https://crates.io/crates/pcb-toolkit) library.

Part of the [pcb-toolkit](https://github.com/akiselev/pcb-toolkit) workspace.

## Installation

```
cargo install pcb-toolkit-cli
```

The binary is called `pcb-toolkit`.

## Usage

### Microstrip Impedance

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

Options:

| Flag | Description | Default |
|---|---|---|
| `-w`, `--width` | Conductor width (mils) | Required |
| `--height` | Dielectric height (mils) | Required |
| `-t`, `--thickness` | Conductor thickness (mils) | 1.4 (1 oz copper) |
| `--er` | Substrate relative permittivity | 4.6 (FR-4) |
| `-f`, `--freq-mhz` | Frequency for dispersion correction (MHz) | 0 (static) |

### JSON Output

All subcommands support the `--json` flag for machine-readable output:

```
pcb-toolkit impedance microstrip -w 10 --height 5 --er 4.6 --json
```

```json
{
  "zo": 44.359895838626485,
  "er_eff": 3.5171650243068555,
  "tpd_ps_per_in": 158.89270763232736,
  "lo_nh_per_in": 7.048463960087373,
  "co_pf_per_in": 3.581899926238581
}
```

### Help

```
pcb-toolkit --help
pcb-toolkit impedance --help
pcb-toolkit impedance microstrip --help
```

## Available Subcommands

| Subcommand | Description | Status |
|---|---|---|
| `impedance microstrip` | Microstrip impedance calculation | Available |

Additional subcommands will be added as the underlying library modules are
implemented.

## License

Licensed under either of [Apache License, Version 2.0](../../LICENSE-APACHE) or
[MIT License](../../LICENSE-MIT) at your option.
