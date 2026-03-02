# Ghidra Analysis: Mode 12 -- PDN Impedance Calculator

## Identification

| Property | Value |
|----------|-------|
| Mode | 12 (`DAT_008d5f88 == 0x0C`) |
| Solver function | `FUN_00408b68` (address `0x00408b68`) |
| Menu handler | `PDNImpedance1Click` at `0x004b14d0` |
| UI tab label | "PDN Calculator" |
| Decompilation | Full success (~15K chars, both sub-modes) |

## Function Overview

The PDN Impedance calculator computes three outputs:

1. **Target PDN Impedance** (Z_target) -- the maximum allowable impedance of
   the power delivery network to keep voltage ripple within specification.
2. **Total Plane Capacitance** (C_plane) -- the parallel-plate capacitance
   between adjacent power/ground planes.
3. **Capacitive Reactance** (Xc) -- the impedance of the plane capacitance
   at a given frequency (optional; skipped when DC checkbox is checked).

The function dispatches on a sub-mode selector at `param_1 + 0x684` (ComboBox
ItemIndex at offset `+0x2f0`):

- **Sub-mode 0** (ItemIndex == 0): Imperial units (area in sq.in, distance in mils)
- **Sub-mode 1** (ItemIndex == 1): Metric units (area in mm^2, distance in mm)

Both sub-modes compute identical target impedance. The plane capacitance
formula differs only in unit-conversion constants. The Xc formula is
identical in both sub-modes.

## Structure

```
FUN_00408b68(param_1, param_2):
    if sub_mode == 0:          // Imperial
        compute Z_target
        compute C_plane (imperial)
        if !DC_checkbox:
            compute Xc
    if sub_mode == 1:          // Metric
        compute Z_target       // identical formula
        compute C_plane (metric, with optional micron conversion)
        if !DC_checkbox:
            compute Xc         // identical formula
```

The function shares the same global variables for intermediate results
regardless of sub-mode. At most one sub-mode branch executes per call.

---

## Formula 1: Target PDN Impedance

### Inputs

| UI Label | Offset | Global | Description |
|----------|--------|--------|-------------|
| Voltage Rail (VDC) | `param_1+0xe70` | `_DAT_008d63b8` | DC supply voltage |
| Maximum Current (A) | `param_1+0xe54` | `_DAT_008d63c0` | Maximum load current |
| Transient Percentage (%) | `param_1+0xe5c` | `_DAT_008d63c8` | Percentage of current that is transient (clamped > 0, divided by 100) |
| Maximum AC Ripple (%) | `param_1+0xe40` | `_DAT_008d63a8` | Allowable voltage ripple percentage (clamped > 0, divided by 100) |

### Output

| UI Label | Offset | Global |
|----------|--------|--------|
| Target PDN Impedance (Ohms) | `param_1+0xe2c` | `_DAT_008d63a0` |

### Formula

```
V_ripple   = V_supply * (V_ripple_pct / 100)
I_transient = I_max * (I_step_pct / 100)

Z_target = V_ripple / I_transient
         = (V_supply * V_ripple_pct) / (I_max * I_step_pct)
```

In the decompiled code:

```c
_DAT_008d63c8 = read(param_1+0xe5c) / 100.0;   // I_step_pct / 100
_DAT_008d63a8 = read(param_1+0xe40) / 100.0;   // V_ripple_pct / 100
_DAT_008d63a0 = (_DAT_008d63b8 * _DAT_008d63a8) / (_DAT_008d63c0 * _DAT_008d63c8);
```

### Validation

From the Saturn Help PDF screenshot (page 28):
- V_supply = 5 VDC, I_max = 2 A, I_step = 50%, V_ripple = 5%
- Z_target = (5 * 0.05) / (2 * 0.50) = 0.25 / 1.0 = **0.2500 Ohms** (matches)

### Input Validation

Both percentage fields are clamped: if the read value is <= 0, a message
dialog is shown (`FUN_005dff68`), the field is reset, and `FUN_00403398`
(main dispatcher) is re-invoked. The check uses `DAT_0040a0b8` (float 0.0)
as the comparison threshold.

---

## Formula 2: Total Plane Capacitance

The plane capacitance uses the standard parallel-plate capacitor formula:

```
C = epsilon_0 * epsilon_r * A / d
```

expressed with unit-specific constants.

### Inputs

| UI Label | Offset | Global | Units (Imperial) | Units (Metric) |
|----------|--------|--------|------------------|----------------|
| Area of Plane | `param_1+0x67c` | `_DAT_008d6054` | sq.in | mm^2 |
| Er (dielectric constant) | `param_1+0xe7c` | `_DAT_008d6004` | dimensionless | dimensionless |
| Distance Between Planes | `param_1+0xe80` | `_DAT_008d600c` | mils | mm |

### Output

| UI Label | Offset | Global |
|----------|--------|--------|
| Total Plane Capacitance (pF) | `param_1+0xe84` | `_DAT_008d6014` |

### Sub-mode 0 (Imperial)

```
C_pF = 0.225 * Er * A_sqin / (d_mils / 1000)
```

Derivation:
```
d_inches = d_mils / 1000
C_pF = 0.225 * Er * A_sqin / d_inches
```

The constant 0.225 (at `0x0040a0c8`) is the well-known PCB parallel-plate
constant: `epsilon_0` expressed in pF with area in square inches and
distance in inches. The exact theoretical value is 0.22486 (from
`8.854e-12 F/m * 39.3701 in/m * 1e12 pF/F / 39.3701`), rounded to 0.225.

In the decompiled code:
```c
_DAT_008d6004 = read(param_1+0xe7c);                      // Er
_DAT_008d600c = read(param_1+0xe80) / 1000.0;             // d_mils -> d_inches
_DAT_008d6014 = 0.225 * _DAT_008d6054 * _DAT_008d6004 / _DAT_008d600c;
```

### Sub-mode 1 (Metric)

```
C_pF = 0.225 * Er * A_mm2 * 0.00155 / (d_mm / 25.4)
     = 0.008858 * Er * A_mm2 / d_mm
```

Derivation:
```
A_sqin = A_mm2 / 25.4^2 = A_mm2 * 0.00155      (1/645.16 = 0.001550)
d_inches = d_mm / 25.4
C_pF = 0.225 * Er * A_sqin / d_inches
     = 0.225 * Er * (A_mm2 * 0.00155) / (d_mm / 25.4)
```

The constants `0.00155` (at `0x0040a0e4`) and `25.4` (at `0x0040a0ec`)
convert mm^2 to in^2 and mm to inches respectively, reducing the metric
formula to the same imperial one internally.

The effective metric constant `0.008858` closely matches `epsilon_0` in
pF/mm units: `8.854e-12 * 1e15 = 8.854e-3 pF/mm` (0.05% difference due
to the 0.225 rounding).

In the decompiled code:
```c
_DAT_008d6004 = read(param_1+0xe7c);                      // Er
_DAT_008d600c = read(param_1+0xe80);                       // d_mm (used directly)
_DAT_008d6014 = (0.225 * _DAT_008d6054 * _DAT_008d6004 * 0.00155)
              / (_DAT_008d600c / 25.4);
```

#### Metric Micron Checkbox (Form6)

In sub-mode 1, there are two conditional branches controlled by a checkbox
on Form6 (`PTR__Form6_008d5480 + 0x490`, accessed via virtual call at
offset `+0x184`). This is the same "mm vs microns" toggle seen in other
calculators (padstack, impedance).

When the checkbox is checked (`AL == 1`):

1. **Er field**: `_DAT_008d6004 = read(param_1+0xe7c) / 1e6`
2. **Distance field**: `_DAT_008d600c = read(param_1+0xe80) / 1000`

The `/1000` on distance converts microns to mm. The `/1e6` on the Er
field is unexpected for a dimensionless dielectric constant; this may
indicate the field is repurposed in micron mode, or it may be an artifact
of shared code that handles other parameters (e.g., permittivity in
nF/m scaled to pF/mm). In practice, this branch is unlikely to be
triggered for standard PDN calculations.

### Validation

From the Saturn Help PDF screenshot (page 28):
- A = 5 sq.in, Er = 4.6, d = 2 mils
- C = 0.225 * 4.6 * 5.0 / (2/1000) = 5.175 / 0.002 = **2587.50 pF** (matches)

### Input Validation

Area is validated against `DAT_0040a0c0` (float 1.0): if area < 1.0, a
message is shown, the field is reset, and the main dispatcher is re-invoked.

---

## Formula 3: Capacitive Reactance (Xc)

### Inputs

| UI Label | Offset | Global | Description |
|----------|--------|--------|-------------|
| Frequency (MHz) | `param_1+0xe9c` | `_DAT_008d602c` | Frequency on the planes |
| DC checkbox | `param_1+0xeb0` | -- | If checked, Xc is not computed |
| (C_plane from Formula 2) | -- | `_DAT_008d6014` | Used internally |

### Output

| UI Label | Offset | Global |
|----------|--------|--------|
| Capacitive Reactance (Ohms) | `param_1+0xea4` | `_DAT_008d6024` |

### Formula

```
Xc = 1 / (2 * pi * f_Hz * C_F)
```

Where `C_F = C_pF / 1e12` and `f_Hz = f_MHz * 1e6`.

In the implementation:

```
Xc = 1.0 / ((C_pF / 1e12) * f_MHz * 1e6 * 6.28318)
   = 1.0 / (C_F * f_Hz * 2*pi)
```

In the decompiled code (both sub-modes, FPU stack trace):
```asm
FLD  [008d602c]          ; ST0 = f_MHz
FMUL [0040a0d0]          ; ST0 = f_MHz * 1e6          (= f_Hz)
FMUL [0040a0d4]          ; ST0 = f_Hz * 6.28318       (= f_Hz * 2*pi)
FLD  [008d6014]          ; ST1 = omega*f, ST0 = C_pF
FDIV [0040a0dc]          ; ST0 = C_pF / 1e12          (= C_F)
FMULP                    ; ST0 = C_F * f_Hz * 2*pi
FDIVR [0040a0c0]         ; ST0 = 1.0 / ST0            (= Xc)
FSTP [008d6024]          ; store result
```

### DC Checkbox

The checkbox at `param_1+0xeb0` is read via a virtual method call at
offset `+0x184` (likely `TCheckBox.Checked`). If the return value is
zero (unchecked), the frequency is read and Xc is computed. If non-zero
(checked, DC mode), the entire Xc block is skipped.

### Validation

From the Saturn Help PDF screenshot (page 28):
- C_plane = 2587.50 pF, f = 1 MHz
- Xc = 1 / (2*pi * 1e6 * 2587.50e-12) = 1 / (1.626e-5) = **61.5092 Ohms** (matches)

---

## Constants Table

| Address | Size | Value | Identifier | Usage |
|---------|------|-------|------------|-------|
| `0x0040a0b8` | float (4B) | 0.0 | `DAT_0040a0b8` | Zero comparison threshold for percentage inputs |
| `0x0040a0bc` | float (4B) | 100.0 | `DAT_0040a0bc` | Percentage-to-fraction divisor (% / 100) |
| `0x0040a0c0` | float (4B) | 1.0 | `DAT_0040a0c0` | Unity; used in FDIVR for reciprocal; also area minimum check |
| `0x0040a0c4` | float (4B) | 1000.0 | `DAT_0040a0c4` | mils-to-inches divisor; microns-to-mm divisor |
| `0x0040a0c8` | double (8B) | 0.225 | `DAT_0040a0c8` | Parallel-plate constant (epsilon_0 in pF, imperial units) |
| `0x0040a0d0` | float (4B) | 1000000.0 | `DAT_0040a0d0` | MHz-to-Hz multiplier (1e6) |
| `0x0040a0d4` | double (8B) | 6.28318 | `DAT_0040a0d4` | 2*pi |
| `0x0040a0dc` | double (8B) | 1e12 | `DAT_0040a0dc` | pF-to-F divisor |
| `0x0040a0e4` | double (8B) | 0.00155 | `DAT_0040a0e4` | 1/25.4^2 = 1/645.16; mm^2-to-in^2 conversion |
| `0x0040a0ec` | double (8B) | 25.4 | `DAT_0040a0ec` | mm-to-inches divisor |

---

## Global Variables (Intermediate Results)

| Address | Type | Name | Description |
|---------|------|------|-------------|
| `0x008d63b8` | double | `V_supply` | Supply voltage (V) |
| `0x008d63c0` | double | `I_max` | Maximum current (A) |
| `0x008d63c8` | double | `I_step_frac` | Transient percentage as fraction |
| `0x008d63a8` | double | `V_ripple_frac` | AC ripple percentage as fraction |
| `0x008d63a0` | double | `Z_target` | Target PDN impedance (Ohms) |
| `0x008d6054` | double | `area` | Plane area (sq.in or mm^2) |
| `0x008d6004` | double | `Er` | Dielectric constant |
| `0x008d600c` | double | `distance` | Plane separation (inches after conversion) |
| `0x008d6014` | double | `C_plane` | Total plane capacitance (pF) |
| `0x008d602c` | double | `frequency` | Frequency (MHz) |
| `0x008d6024` | double | `Xc` | Capacitive reactance (Ohms) |

---

## UI Field Map

### Input Fields

| Offset | UI Label (from Help PDF) | Type | Units |
|--------|--------------------------|------|-------|
| `param_1+0xe70` | Voltage Rail | TEdit | VDC |
| `param_1+0xe54` | Maximum Current | TEdit | Amps |
| `param_1+0xe5c` | Transient Percentage | TEdit | % |
| `param_1+0xe40` | Maximum AC Ripple (Supply Noise Margin) | TEdit | % |
| `param_1+0x67c` | Area of Plane | TEdit | Sq.In (imperial) / mm^2 (metric) |
| `param_1+0xe7c` | Er (dielectric constant) | TEdit | dimensionless |
| `param_1+0xe80` | Distance Between Planes | TEdit | mils (imperial) / mm (metric) |
| `param_1+0xe9c` | Frequency | TEdit | MHz |

### Output Fields

| Offset | UI Label (from Help PDF) | Type | Units |
|--------|--------------------------|------|-------|
| `param_1+0xe2c` | Target PDN Impedance | TEdit | Ohms |
| `param_1+0xe84` | Total Plane Capacitance | TEdit | pF |
| `param_1+0xea4` | Capacitive Reactance | TEdit | Ohms |

### Control Fields

| Offset | UI Element | Purpose |
|--------|------------|---------|
| `param_1+0x684` | TComboBox (sub-mode) | Imperial (0) vs Metric (1) unit selector |
| `param_1+0xeb0` | TCheckBox (DC) | When checked, skip Xc calculation |
| `param_1+0xc44` | TEdit (format) | Decimal format string for number formatting |
| `Form6+0x490` | TCheckBox | mm vs microns toggle (sub-mode 1 only) |

---

## Menu Handler: PDNImpedance1Click (0x004b14d0)

The menu handler performs standard UI initialization:

1. Sets `DAT_008d5f88 = 12` (mode selector)
2. Clears/hides many UI panels not used by this calculator
3. Makes visible the relevant panels:
   - `param_1+0x684` -- sub-mode ComboBox (Imperial/Metric)
   - `param_1+0x67c` -- Area of Plane field
   - `param_1+0x668` -- (visible, unknown label)
   - `param_1+0x8f8` -- (visible, unknown label)
   - `param_1+0x66c`, `0x670`, `0x678` -- (visible, likely section labels)
4. Populates the sub-mode ComboBox (`param_1+0x9b0`) with two items
   (likely "Imperial" and "Metric")
5. Populates the unit label ComboBox (`param_1+0x9bc`) with two items
6. Sets `param_1+0x9bc` ItemIndex to 0
7. Reads a registry key to restore previous settings
8. Calls the main dispatcher `FUN_00403398` to trigger initial calculation

---

## Output Formatting

After each calculation result is stored to a global variable, the function:

1. Calls `FUN_00861e48` to format the double value as a string (pushes
   the value onto the FPU stack and converts to text)
2. Calls `FUN_005396c8` (TEdit.SetText) to display the result
3. Reads the format string from `param_1+0xc44` via `FUN_00539678`
   (TEdit.GetText)
4. Calls `FUN_0086ef50` (TryStrToFloat) to check if a custom format exists
5. If a valid format is found, calls `FUN_0085f89f` and `FUN_0071d964`
   (string replacement/formatting) to re-format the output with the
   specified precision

---

## Summary Formula Table

| Output | Formula | Notes |
|--------|---------|-------|
| Z_target | `(V_supply * V_ripple_pct/100) / (I_max * I_step_pct/100)` | Standard PDN target impedance |
| C_plane (imperial) | `0.225 * Er * A_sqin / (d_mils / 1000)` | Parallel-plate capacitance, result in pF |
| C_plane (metric) | `0.225 * 0.00155 * 25.4 * Er * A_mm2 / d_mm` | Same formula with metric conversion |
| Xc | `1 / (2*pi * f_MHz * 1e6 * C_pF * 1e-12)` | Capacitive reactance, skipped if DC checkbox |

### Simplified Forms

```
Z_target = V_ripple / I_transient

C_plane = epsilon_0_eff * Er * A / d
  where epsilon_0_eff = 0.225 pF (imperial, A in in^2, d in inches)
  or    epsilon_0_eff = 0.008858 pF/mm (metric, A in mm^2, d in mm)

Xc = 1 / (2*pi*f*C)       (standard capacitive reactance)
```

---

## Test Vectors (from Saturn Help PDF, page 28)

### Test Case 1: PDN Calculator defaults

**Inputs:**
- Voltage Rail: 5 VDC
- Maximum Current: 2 A
- Transient Percentage: 50%
- Maximum AC Ripple: 5%
- Area of Plane: 5 sq.in
- Er: 4.6 (FR-4 STD)
- Distance Between Planes: 2 mils
- Frequency: 1 MHz
- Units: Imperial

**Expected Outputs:**
- Target PDN Impedance: **0.2500 Ohms**
- Total Plane Capacitance: **2587.50 pF**
- Capacitive Reactance: **61.5092 Ohms**

**Verification:**
```
Z = (5 * 0.05) / (2 * 0.50) = 0.25 / 1.0 = 0.2500            OK
C = 0.225 * 4.6 * 5 / 0.002 = 2587.50                          OK
Xc = 1 / (2*pi * 1e6 * 2587.5e-12) = 61.5092                   OK
```

---

## Confidence Assessment

| Item | Confidence | Notes |
|------|------------|-------|
| Z_target formula | **HIGH** | Verified against screenshot, standard formula |
| C_plane imperial | **HIGH** | Verified against screenshot, well-known constant |
| C_plane metric | **HIGH** | Conversion constants verified (0.00155 = 1/25.4^2, 25.4 = mm/in) |
| Xc formula | **HIGH** | Verified against screenshot, standard Xc formula |
| UI field mapping | **HIGH** | All inputs/outputs match help PDF screenshot |
| DC checkbox behavior | **HIGH** | Code path clearly skips Xc when checked |
| Form6 micron checkbox | **MEDIUM** | Pattern matches other calculators; /1e6 on Er is unusual |
| Sub-mode ComboBox items | **MEDIUM** | Inferred as Imperial/Metric from unit handling |
