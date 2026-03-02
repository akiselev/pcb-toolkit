# Ghidra Analysis: Mode 15 — Thermal Management Solver

## Overview

**Function**: `Solver_ThermalManagement` at `0x004081b0`
**Mode**: `DAT_008d5f88 == 15` (0x0f)
**Menu handler**: `ThermalManagement1Click` at `0x004b1fe4`
**Resource string**: `DAT_00877f68`

This is the **Thermal Management / Junction Temperature Calculator**. It computes
junction temperature using the standard thermal resistance model:

```
T_junction = R_theta * P_dissipated + T_ambient
```

The calculator provides **two independent computation channels** (A and B), each
computing a junction temperature from a thermal resistance (in C/W) and a power
dissipation (in Watts). Both channels share the same ambient temperature.

The formula is the textbook thermal management equation used in electronics design.

---

## Function Structure

### Main Solver: `FUN_004081b0`

```c
void Solver_ThermalManagement(int param_1, undefined4 param_2)
// param_1 = TForm1 self pointer
// param_2 = passed through to pre-computation
```

**Two sub-modes** selected by `*(int *)(*(int *)(param_1 + 0x684) + 0x2f0)`:
- **Mode 0**: Standard decimal display (format: `"%.3f C"`)
- **Mode 1**: Same computations, alternate number formatting (uses comma `,` separator
  or scientific notation via `Delphi_Format`)

Both sub-modes compute the same formulas with the same input/output fields. The only
difference is the output string formatting.

### Call Chain

```
ThermalManagement1Click (0x004b1fe4)
  sets DAT_008d5f88 = 15
  configures UI visibility
  calls Button1Click_MainDispatcher (0x00403398)
    calls common pre-compute chain:
      FUN_00471678  -- reads temperature rise input
      FUN_00471e7c  -- reads ambient temperature input (THE KEY PRE-COMPUTE)
      FUN_00471a08  -- computes combined temperature + Fahrenheit display
      FUN_004e0928  -- reads additional form fields (for other calculators)
    calls Solver_ThermalManagement (0x004081b0)
```

The "PreCompute_3" referenced in the earlier (incorrect) analysis is `FUN_00471e7c`.
This function reads the ambient temperature from the UI and stores it in `DAT_008d60d4`.
The solver then uses this global as the additive offset in the junction temperature formula.

---

## Core Formula

For each computation channel:

```
T_junction = R_theta_ja * P_dissipated + T_ambient
```

Where:
- `R_theta_ja` = thermal resistance, junction-to-ambient (C/W)
- `P_dissipated` = power dissipation (Watts)
- `T_ambient` = ambient temperature (C), from `DAT_008d60d4`
- `T_junction` = resulting junction/case temperature (C)

### Disassembly Proof (Channel A, Mode 0)

```asm
; At 0x004082a3:
FLD     double ptr [0x008d63e0]      ; load input_A2 (power, W)
FMUL    double ptr [0x008d63d0]      ; multiply by input_A1 (R_theta, C/W)
FADD    double ptr [0x008d60d4]      ; add ambient temperature (C)
FSTP    double ptr [0x008d63d8]      ; store result (T_junction, C)
```

### Disassembly Proof (Channel B, Mode 0)

```asm
; At 0x004084cf:
FLD     double ptr [0x008d6400]      ; load input_B2 (power, W)
FMUL    double ptr [0x008d6408]      ; multiply by input_B1 (R_theta, C/W)
FADD    double ptr [0x008d60d4]      ; add ambient temperature (C)
FSTP    double ptr [0x008d63f8]      ; store result (T_junction, C)
```

Both channels use the identical formula: `result = input2 * input1 + ambient`.

### Output Format

Results are formatted using `"%.3f C"` (format string at `0x8713f4`), displaying
the junction temperature to 3 decimal places in degrees Celsius. The degree symbol
(0xB0) appears in the actual binary: `"%.3f \xB0C"`.

---

## Pre-Computation Functions

### FUN_00471678 — Read Temperature Rise

**Address**: `0x00471678`
**Purpose**: Reads the temperature rise input from `param_1 + 0x630`, validates it,
and stores it in `DAT_008d60c4`.

```
Pseudocode:
  temp_rise = StrToFloat(TEdit_GetText(param_1 + 0x630))
  DAT_008d60c4 = temp_rise

  if temp_rise > 200.0:
      show_warning("Maximum temperature rise can't be greater than 200C")
      TEdit_SetText(param_1 + 0x630, "200")
      DAT_008d60c4 = 200.0

  if temp_rise < 0.0:     // Note: < 0, not <= 0
      show_warning("Minimum temperature rise can't be less than 0C")
      TEdit_SetText(param_1 + 0x630, "200")   // resets to max (bug or design?)
      DAT_008d60c4 = 1.0                       // sets to 1.0 internally

  if temp_rise == 0.0:    // exact zero
      TEdit_SetText(param_1 + 0x62c, "N/A")   // output set to N/A
      DAT_008d60c4 = 0.0
  else:
      TrackBar_SetPosition(param_1 + 0x638, round(temp_rise))
      DAT_008d610c = 1.8 * temp_rise           // rise in Fahrenheit (delta)
      TEdit_SetText(param_1 + 0x62c, formatted_result)
```

**Constants**:
| Address | Type | Value | Purpose |
|---------|------|-------|---------|
| `0x004719f8` | float32 | 200.0 | Maximum temperature rise (C) |
| `0x004719fc` | float32 | 0.0 | Minimum temperature rise check |
| `0x00471a00` | float64 | 1.8 | C-to-F delta conversion factor |

**Globals**:
| Address | Variable | Description |
|---------|----------|-------------|
| `DAT_008d60c4` | temp_rise_C | Temperature rise in Celsius |
| `DAT_008d610c` | temp_rise_F | Temperature rise in Fahrenheit (delta: 1.8 * C) |

### FUN_00471e7c — Read Ambient Temperature (PreCompute_3)

**Address**: `0x00471e7c`
**Purpose**: Reads the ambient temperature from `param_1 + 0x648`, validates it,
computes the combined temperature, and converts to Fahrenheit.

This is the critical pre-computation function that sets `DAT_008d60d4` (the ambient
temperature used as the additive offset in the main solver formula).

```
Pseudocode:
  ambient_C = StrToFloat(TEdit_GetText(param_1 + 0x648))
  DAT_008d60d4 = ambient_C

  if ambient_C > 200.0:
      show_warning("Maximum ambient temperature can't be greater than 200C")
      TEdit_SetText(param_1 + 0x648, "200")
      DAT_008d60d4 = 200.0

  TrackBar_SetPosition(param_1 + 0x650, round(ambient_C))

  // Check combined temperature (ambient + rise)
  combined_C = DAT_008d60cc   // set by FUN_00471a08: ambient + rise
  if combined_C < -80.0:
      show_warning("Minimum ambient temperature can't be less than -80")
      TEdit_SetText(param_1 + 0x648, "-80")
      DAT_008d60d4 = 0.0      // reset to 0

  // Convert ambient to Fahrenheit for display
  DAT_008d62b0 = 1.8 * DAT_008d60d4 + 32.0
  TEdit_SetText(param_1 + 0x644, formatted_fahrenheit)
```

**Constants**:
| Address | Type | Value | Purpose |
|---------|------|-------|---------|
| `0x004721c4` | float32 | 200.0 | Maximum ambient temperature (C) |
| `0x004721c8` | float32 | -80.0 | Minimum combined temperature (C) |
| `0x004721cc` | float64 | 1.8 | C-to-F conversion multiplier |
| `0x004721d4` | float32 | 32.0 | C-to-F conversion offset |

**Globals**:
| Address | Variable | Description |
|---------|----------|-------------|
| `DAT_008d60d4` | ambient_C | Ambient temperature in Celsius |
| `DAT_008d60cc` | combined_C | Combined temperature (ambient + rise) |
| `DAT_008d62b0` | ambient_F | Ambient temperature in Fahrenheit |

### FUN_00471a08 — Compute Combined Temperature

**Address**: `0x00471a08`
**Purpose**: Computes combined temperature (ambient + rise) and its Fahrenheit
equivalent. Also writes the combined temperature to a display label.

```
Pseudocode:
  // Compute combined temperature
  DAT_008d60cc = DAT_008d60d4 + DAT_008d60c4   // ambient + rise
  format_and_display(DAT_008d60cc, "%0.1fC")     // e.g., "125.0C"

  // Convert combined temperature to Fahrenheit
  DAT_008d6104 = 1.8 * DAT_008d60cc + 32.0
  format_and_display(DAT_008d6104, "%0.1fF")     // e.g., "257.0F"

  // If on the right tab (tab index 2 or 16), update the display label
  tab_index = TabControl_GetCurSel(param_1 + 0x490)
  if tab_index == 2 || tab_index == 0x10:
      TEdit_SetText(param_1 + 0x99c, formatted_combined_temp)
```

**Constants**:
| Address | Type | Value | Purpose |
|---------|------|-------|---------|
| `0x00471e70` | float64 | 1.8 | C-to-F multiplier |
| `0x00471e78` | float32 | 32.0 | C-to-F offset |

**Globals**:
| Address | Variable | Description |
|---------|----------|-------------|
| `DAT_008d60cc` | combined_C | ambient_C + temp_rise_C |
| `DAT_008d6104` | combined_F | 1.8 * combined_C + 32.0 |

### FUN_004e0928 — PreCompute_4 (Additional Fields)

**Address**: `0x004e0928`
**Purpose**: Reads additional form fields for other calculators (not directly used
by the Thermal Management solver). Computes:

```
result = (input1 * DAT_008d6aac / 1000.0) * input2
```

Where `DAT_008d6aac` is a runtime variable set by other computations. This function
is part of the common pre-compute chain but its output is not consumed by Mode 15.

---

## Sub-Mode Details

### Mode 0 (Standard Display)

```c
// Step 1: Pre-compute ambient temperature
FUN_00471e7c(param_1, param_2);

// Step 2: Display combined temp label
TEdit_SetText(param_1 + 0x99c, formatted_combined_temp);

// Step 3: Channel A — Junction Temperature #1
R_theta_A  = StrToFloat(TEdit_GetText(param_1 + 0x90c));  // DAT_008d63d0
P_diss_A   = StrToFloat(TEdit_GetText(param_1 + 0x910));  // DAT_008d63e0
T_junct_A  = P_diss_A * R_theta_A + DAT_008d60d4;         // DAT_008d63d8
TEdit_SetText(param_1 + 0x914, format("%.3f C", T_junct_A));

// Apply custom number formatting if format string is non-empty
format_str = TEdit_GetText(param_1 + 0xc44);
if (format_str != ""):
    TEdit_SetText(param_1 + 0x914, Delphi_Format(format_str, T_junct_A));

// Step 4: Channel B — Junction Temperature #2
R_theta_B  = StrToFloat(TEdit_GetText(param_1 + 0x92c));  // DAT_008d6408
P_diss_B   = StrToFloat(TEdit_GetText(param_1 + 0x940));  // DAT_008d6400
T_junct_B  = P_diss_B * R_theta_B + DAT_008d60d4;         // DAT_008d63f8
TEdit_SetText(param_1 + 0x938, format("%.3f C", T_junct_B));

// Apply custom number formatting if format string is non-empty
if (format_str != ""):
    TEdit_SetText(param_1 + 0x938, Delphi_Format(format_str, T_junct_B));
```

### Mode 1 (Alternate Formatting)

Identical computation logic to Mode 0. The only difference is in the custom number
formatting path — Mode 1 uses a different variant of the `Delphi_Format` call that
references `DAT_00871343` (comma `,` separator) for locale-specific formatting.

---

## UI Field Map

### Thermal Management Input/Output Fields

| Offset   | Direction | Description |
|----------|-----------|-------------|
| `+0x0630` | Input | Temperature Rise (C) |
| `+0x062c` | Output | Temperature Rise display (formatted, or "N/A" if zero) |
| `+0x0638` | Display | Temperature Rise trackbar/slider |
| `+0x0644` | Output | Ambient Temperature in Fahrenheit |
| `+0x0648` | Input | Ambient Temperature (C) |
| `+0x0650` | Display | Ambient Temperature trackbar/slider |
| `+0x0684` | Selector | Sub-mode selector (0=standard, 1=alternate format) |
| `+0x090c` | Input | Channel A: Thermal Resistance R_theta (C/W) |
| `+0x0910` | Input | Channel A: Power Dissipation (W) |
| `+0x0914` | Output | Channel A: Junction Temperature (C) |
| `+0x092c` | Input | Channel B: Thermal Resistance R_theta (C/W) |
| `+0x0938` | Output | Channel B: Junction Temperature (C) |
| `+0x0940` | Input | Channel B: Power Dissipation (W) |
| `+0x099c` | Output | Combined Temperature label (ambient + rise, with C/F) |
| `+0x0c44` | Input | Number format string (controls decimal formatting) |

### Controls Set by ThermalManagement1Click

The menu handler at `0x004b1fe4` configures extensive UI visibility:
- Shows: `+0x648` (ambient input), `+0x64c`, `+0x640`, `+0x644` (ambient F output)
- Shows: `+0x900` (enabled), `+0x63c` panel
- Hides: Many other calculator-specific panels (`+0x5d8`, `+0x4b4`, `+0x79c`, etc.)
- Sets tab control `+0x490` to tab index 15

---

## Global Variables

| Address | Type | Description |
|---------|------|-------------|
| `DAT_008d5f88` | int | Global mode selector (15 for Thermal Management) |
| `DAT_008d60c4` | double | Temperature rise (C), set by FUN_00471678 |
| `DAT_008d60cc` | double | Combined temperature: ambient + rise (C) |
| `DAT_008d60d4` | double | **Ambient temperature (C)** — the key pre-computed offset |
| `DAT_008d610c` | double | Temperature rise in Fahrenheit (delta: 1.8 * rise_C) |
| `DAT_008d6104` | double | Combined temperature in Fahrenheit (1.8 * combined_C + 32) |
| `DAT_008d62b0` | double | Ambient temperature in Fahrenheit (1.8 * ambient_C + 32) |
| `DAT_008d63d0` | double | Channel A: R_theta input (C/W) |
| `DAT_008d63d8` | double | Channel A: T_junction result (C) |
| `DAT_008d63e0` | double | Channel A: P_dissipated input (W) |
| `DAT_008d63f8` | double | Channel B: T_junction result (C) |
| `DAT_008d6400` | double | Channel B: P_dissipated input (W) |
| `DAT_008d6408` | double | Channel B: R_theta input (C/W) |

---

## Constants Table

| Address | Type | Value | Purpose |
|---------|------|-------|---------|
| `0x004719f8` | float32 | 200.0 | Max temperature rise (C) |
| `0x004719fc` | float32 | 0.0 | Min temperature rise check |
| `0x00471a00` | float64 | 1.8 | C-to-F delta conversion |
| `0x00471e70` | float64 | 1.8 | C-to-F multiplier (combined temp) |
| `0x00471e78` | float32 | 32.0 | C-to-F offset (combined temp) |
| `0x004721c4` | float32 | 200.0 | Max ambient temperature (C) |
| `0x004721c8` | float32 | -80.0 | Min combined temperature (C) |
| `0x004721cc` | float64 | 1.8 | C-to-F multiplier (ambient) |
| `0x004721d4` | float32 | 32.0 | C-to-F offset (ambient) |

---

## String Table

| Address | String | Context |
|---------|--------|---------|
| `0x87130a` | `"N/A"` | Displayed when temperature rise is zero |
| `0x8713f4` | `"%.3f C"` | Junction temperature output format (with degree symbol 0xB0) |
| `0x8735fa` | `"Maximum temperature rise can't be greater than 200C"` | Validation error |
| `0x87362e` | `"200"` | Clamp display value |
| `0x873632` | `"Minimum temperature rise can't be less than 0C"` | Validation error |
| `0x873661` | `"%0.1f"` | Generic float format |
| `0x873667` | `"%0.1fC"` | Combined temperature Celsius format |
| `0x87366e` | `"%0.1fF"` | Combined temperature Fahrenheit format |
| `0x8736ac` | `"Minimum ambient temperature can't be less than -80"` | Validation error |
| `0x8736e0` | `"-80"` | Min ambient clamp display value |

---

## Corrections to Earlier Analysis

The earlier analysis in `ghidra-ppm-reactance-mode15.md` (lines 362-570) incorrectly
identified Mode 15 as "PDN Impedance Calculator". The corrections are:

1. **Identity**: Mode 15 is the **Thermal Management** calculator, not PDN Impedance.
   PDN Impedance is Mode 12 (handler `PDNImpedance1Click` at `0x004b14d0`, solver at
   `0x00408b68`).

2. **The "precomputed offset" is ambient temperature**: `DAT_008d60d4` is not some
   opaque pre-computed base impedance. It is simply the ambient temperature in degrees
   Celsius, read from the UI by `FUN_00471e7c`.

3. **The formula is T_j = R_theta * P + T_ambient**: The `y = m*x + b` pattern
   identified in the earlier analysis is correct structurally, but the physical
   interpretation is thermal, not electrical:
   - `m` (input_A2) = power dissipation (Watts)
   - `x` (input_A1) = thermal resistance (C/W)
   - `b` (offset) = ambient temperature (C)
   - `y` (output) = junction temperature (C)

4. **The two computation channels** are for two independent device calculations
   (e.g., two components on the same board), not DC/AC or two frequency points.

5. **The sub-mode selector** controls number formatting, not unit/topology selection.

---

## Validation Ranges

| Parameter | Min | Max | Units |
|-----------|-----|-----|-------|
| Temperature Rise | 0 (exclusive; 0 shows "N/A") | 200 | C |
| Ambient Temperature | -80 (checked on combined temp) | 200 | C |
| Combined Temperature | -80 | (not explicitly capped) | C |

---

## Implementation Notes

For the Rust implementation:

```rust
/// Compute junction temperature from thermal resistance model.
///
/// T_junction = R_theta_ja * P_dissipated + T_ambient
///
/// # Arguments
/// * `r_theta` - Thermal resistance, junction-to-ambient (C/W)
/// * `power` - Power dissipation (W)
/// * `t_ambient` - Ambient temperature (C)
///
/// # Returns
/// Junction temperature in degrees Celsius.
pub fn junction_temperature(r_theta: f64, power: f64, t_ambient: f64) -> f64 {
    r_theta * power + t_ambient
}

/// Convert Celsius to Fahrenheit.
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    1.8 * celsius + 32.0
}

/// Convert a Celsius temperature delta to Fahrenheit delta.
/// (No +32 offset for deltas.)
pub fn celsius_delta_to_fahrenheit(delta_c: f64) -> f64 {
    1.8 * delta_c
}
```

Validation:
- Temperature rise: must be in (0, 200] C (0 is a special "N/A" case)
- Ambient temperature: must be in [-80, 200] C (lower bound checked on combined temp)
- Thermal resistance: no explicit validation in the solver (positive values expected)
- Power: no explicit validation in the solver (positive values expected)

---

## Confidence Assessment

| Aspect | Confidence | Notes |
|--------|------------|-------|
| Calculator identity | **HIGH** | Menu handler name `ThermalManagement1Click` confirmed |
| Core formula | **HIGH** | `T_j = R * P + T_amb` confirmed by disassembly at 0x4082a3, 0x4084cf |
| PreCompute_3 = ambient temp | **HIGH** | `FUN_00471e7c` reads from `+0x648`, clamps to [-80,200], stores to `DAT_008d60d4` |
| C-to-F conversion | **HIGH** | Constants 1.8 and 32.0 confirmed at multiple addresses |
| Two-channel structure | **HIGH** | Both channels use identical formula with different input/output fields |
| Sub-mode = formatting | **MEDIUM** | Mode 0 vs 1 differ only in number format path; may also affect units display |
| Field assignments | **MEDIUM** | Input fields identified by read order; labels not directly confirmed from DFM |
