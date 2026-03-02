# Saturn PCB Toolkit - Broadside Coupled Impedance Solvers (Modes 7 & 8)

## Overview

Modes 7 and 8 in the main dispatcher (`FUN_00403398`) handle the broadside coupled
stripline topologies:

| Mode | Address      | Label (UI)       | Description                           |
|------|-------------|------------------|---------------------------------------|
| 7    | 0x004b8104  | Broad Cpld Shld  | Broadside Coupled Shielded            |
| 8    | 0x00482648  | Broad Cpld NShld | Broadside Coupled Non-Shielded        |

The UI setup function for Mode 8 is `FUN_00492768` (sets `DAT_008d5f88 = 8`).
It is referenced by the string "Broad Cpld NShld" at address `0x0087442f`.

---

## Mode 8 Architecture: FUN_00482648 (Broadside Coupled Non-Shielded)

**Address:** `0x00482648`
**Size:** ~32 KB (3431 lines decompiled)
**Decompilation:** Successful (full decompile via Ghidra)

### Key Finding

Unlike most impedance solvers which embed the impedance formula directly,
`FUN_00482648` is **primarily a wire gauge lookup table function**. The actual
impedance computation is deferred to common pre-computation functions and
`FUN_004e0928`.

### Function Structure

```
0x00482648 - 0x004857a6 : Mode 0 (mils unit mode)
  |- AWG wire gauge lookup table (44 entries, indices 0x00-0x2B)
  |- Math_Ln() call + divide by 700.0
  |- Store to _DAT_008d6498
  |- Display result at param_1 + 0xa28
  |- Call FUN_004e0928 (PreCompute_4)

0x004857a7 - 0x0048e4a3 : Mode 1 (mm unit mode)
  |- AWG wire gauge lookup table (44 entries, indices 0x00-0x2B)
  |- Each entry also has format string application
  |- Math_Ln() call + divide by 700.0
  |- Store to _DAT_008d64a0
  |- Display result at param_1 + 0xa28
  |- Call FUN_004e0928 (PreCompute_4)
```

### Mode Selector

The unit mode is checked at `param_1 + 0x684 + 0x2f0`:
- `== 0`: Mils mode (wire diameters in inches, resistance in ohms/1000ft)
- `== 1`: Metric mode (wire diameters in mm, resistance in metric units)

### Wire Gauge Selector

The AWG wire gauge is selected via `param_1 + 0x598 + 0x184` (a ComboBox/ListBox
virtual call at vtable offset +0x184, returning `ItemIndex`). It dispatches to
44 entries (0x00 through 0x2B), covering AWG 4/0 through AWG 40.

---

## Wire Gauge Lookup Tables

### Table 1: Wire Diameter (DAT_008d6250/008d6254)

Stored as IEEE 754 doubles written to globals `DAT_008d6250` (lo 32 bits) and
`DAT_008d6254` (hi 32 bits).

#### Mils Mode (diameter in inches)

| Index | AWG   | Diameter (in) | Diameter (mils) |
|-------|-------|---------------|-----------------|
| 0     | 4/0   | 0.4600        | 460.0           |
| 1     | 3/0   | 0.4096        | 409.6           |
| 2     | 2/0   | 0.3648        | 364.8           |
| 3     | 1/0   | 0.3249        | 324.9           |
| 4     | 1     | 0.2893        | 289.3           |
| 5     | 2     | 0.2576        | 257.6           |
| 6     | 3     | 0.2294        | 229.4           |
| 7     | 4     | 0.2043        | 204.3           |
| 8     | 5     | 0.1819        | 181.9           |
| 9     | 6     | 0.1620        | 162.0           |
| 10    | 7     | 0.1443        | 144.3           |
| 11    | 8     | 0.1285        | 128.5           |
| 12    | 9     | 0.1144        | 114.4           |
| 13    | 10    | 0.1019        | 101.9           |
| 14    | 11    | 0.0907        | 90.7            |
| 15    | 12    | 0.0808        | 80.8            |
| 16    | 13    | 0.0720        | 72.0            |
| 17    | 14    | 0.0641        | 64.1            |
| 18    | 15    | 0.0571        | 57.1            |
| 19    | 16    | 0.0508        | 50.8            |
| 20    | 17    | 0.0453        | 45.3            |
| 21    | 18    | 0.0403        | 40.3            |
| 22    | 19    | 0.0359        | 35.9            |
| 23    | 20    | 0.0320        | 32.0            |
| 24    | 21    | 0.0285        | 28.5            |
| 25    | 22    | 0.0254        | 25.4            |
| 26    | 23    | 0.0226        | 22.6            |
| 27    | 24    | 0.0201        | 20.1            |
| 28    | 25    | 0.0179        | 17.9            |
| 29    | 26    | 0.0159        | 15.9            |
| 30    | 27    | 0.0142        | 14.2            |
| 31    | 28    | 0.0126        | 12.6            |
| 32    | 29    | 0.0113        | 11.3            |
| 33    | 30    | 0.0100        | 10.0            |
| 34    | 31    | 0.0089        | 8.9             |
| 35    | 32    | 0.0080        | 8.0             |
| 36    | 33    | 0.0071        | 7.1             |
| 37    | 34    | 0.0063        | 6.3             |
| 38    | 35    | 0.0056        | 5.6             |
| 39    | 36    | 0.0050        | 5.0             |
| 40    | 37    | 0.0045        | 4.5             |
| 41    | 38    | 0.0040        | 4.0             |
| 42    | 39    | 0.0035        | 3.5             |
| 43    | 40    | 0.0031        | 3.1             |

All values match standard AWG wire diameters exactly.

#### Metric Mode (diameter in mm)

The metric mode uses the same AWG sizes but stores diameters in mm (e.g., AWG 4/0 =
11.684 mm, AWG 22 = 0.6426 mm, AWG 40 = 0.0787 mm). These are the standard inch
values multiplied by 25.4.

### Table 2: Wire Property (_DAT_008d6aac/_DAT_008d6ab0)

A second double value stored at `_DAT_008d6aac` (lo) and `_DAT_008d6ab0` (hi).

#### Mils Mode Values

| Index | AWG   | Value      |
|-------|-------|------------|
| 0     | 4/0   | 0.050      |
| 1     | 3/0   | 0.060      |
| 2     | 2/0   | 0.080      |
| 3     | 1/0   | 0.100      |
| 4     | 1     | 0.120      |
| 5     | 2     | 0.160      |
| 6     | 3     | 0.200      |
| 7     | 4     | 0.250      |
| 8     | 5     | 0.310      |
| 9     | 6     | 0.400      |
| 10    | 7     | 0.500      |
| 11    | 8     | 0.630      |
| 12    | 9     | 0.790      |
| 13    | 10    | 1.000      |
| 14    | 11    | 1.260      |
| 15    | 12    | 1.590      |
| 16    | 13    | 2.000      |
| 17    | 14    | 2.530      |
| 18    | 15    | 3.190      |
| 19    | 16    | 4.020      |
| 20    | 17    | 5.060      |
| 21    | 18    | 6.390      |
| 22    | 19    | 8.050      |
| 23    | 20    | 10.150     |
| 24    | 21    | 12.800     |
| 25    | 22    | 16.140     |
| 26    | 23    | 20.360     |
| 27    | 24    | 25.670     |
| 28    | 25    | 32.370     |
| 29    | 26    | 40.810     |
| 30    | 27    | 51.470     |
| 31    | 28    | 64.900     |
| 32    | 29    | 81.830     |
| 33    | 30    | 103.200    |
| 34    | 31    | 130.100    |
| 35    | 32    | 164.100    |
| 36    | 33    | 206.900    |
| 37    | 34    | 260.900    |
| ...   | ...   | ...        |
| 43    | 40    | 831.800    |

These values grow approximately with the square of the AWG number, suggesting they
are related to wire resistance. The first few match standard DC resistance in
ohms/1000 ft for annealed copper (AWG 4/0 = 0.0490 rounds to 0.050, AWG 3/0 =
0.0618 rounds to 0.060), but later values diverge from standard tables due to
rounding to preferred values.

#### Metric Mode Values

The metric mode stores a different set of values at the same globals. These range
from 0.1607 (AWG 4/0) to 1079.12 (AWG 40) and appear to be the resistance values
converted to metric units (ohms/km).

---

## Computation

### Step 1: Wire Gauge Lookup

The function reads the wire gauge selector and sets:
- `DAT_008d6250/6254` = wire diameter (double, in inches for mils mode, mm for metric)
- `_DAT_008d6aac/6ab0` = wire resistance/property value (double)

It also writes these values as formatted strings to UI controls:
- `param_1 + 0x894` = diameter display field
- `param_1 + 0x88c` = resistance display field
- `param_1 + 0x59c` = additional display field

### Step 2: Cross-Sectional Area Computation

At the end of each unit mode block, the function calls `FUN_008675ac` (Power function)
with the diameter and exponent 2.0:

```asm
; Mils mode (at 0x004855a0):
FLD    [008d6250]          ; load wire diameter (inches)
FMUL   [0048e4b4]          ; * 1000.0 = diameter in mils
FSTP   [local]             ; store

PUSH   0x40000000, 0x0     ; push exponent = 2.0
PUSH   [local_hi], [local_lo]  ; push diameter_mils
CALL   FUN_008675ac        ; Power(diameter_mils, 2.0)

FDIV   [0048e4b8]          ; divide by 700.0
FSTP   [008d6498]          ; store result
```

This computes: `result = diameter_mils^2 / 700.0`

The result is stored in `_DAT_008d6498` (mils mode) or `_DAT_008d64a0` (metric mode)
and displayed at `param_1 + 0xa28`.

### Step 3: PreCompute_4 (FUN_004e0928)

The function then calls `FUN_004e0928` which reads additional form fields and
computes a combined result:

```c
// FUN_004e0928 decompiled:
val_A = StrToFloat(param_1 + 0x156c);   // stored in _DAT_008d6a94
val_B = StrToFloat(param_1 + 0x157c);   // stored in _DAT_008d6ab4

// Core formula:
_DAT_008d6aa4 = (val_A * _DAT_008d6aac) / 1000.0 * val_B;

// Result displayed at param_1 + 0x1574
```

Where `_DAT_008d6aac` is the wire resistance value from the lookup table.

### Key Constants

| Address      | Value  | Type  | Purpose                                    |
|-------------|--------|-------|--------------------------------------------|
| `0x0048e4b4`| 1000.0 | f32   | Inches to mils conversion                  |
| `0x0048e4b8`| 700.0  | f32   | Divisor for area computation               |
| `0x004e0bbc`| 1000.0 | f32   | Divisor in PreCompute_4 formula             |

---

## Functions Called

| Address      | Name (assigned)      | Purpose                                    |
|-------------|----------------------|---------------------------------------------|
| `0x008675ac`| FUN_008675ac (Power) | Computes `base^exponent` using FYL2X + 2^x  |
| `0x004e0928`| FUN_004e0928         | Common pre-computation: formula application |
| `0x00403638`| FUN_00403638         | Clear/reset result field                    |
| `0x005396c8`| TEdit_SetText        | Write string to UI edit control              |
| `0x00539678`| TEdit_GetText        | Read string from UI edit control             |
| `0x0086f0f4`| StrToFloat           | Parse string to double                       |
| `0x00861e48`| FloatToStr           | Format double to string                      |
| `0x0086ecf0`| StackStringBuilder   | Delphi string helper                         |
| `0x0086ee90`| StringCleanup        | Delphi string reference decrement             |
| `0x0086ef50`| StringValidate       | Check if string is valid/non-empty            |
| `0x0086b424`| ResourceStringLoader | Load Delphi resource string                   |
| `0x0071d964`| Format               | Delphi Format() for number formatting          |
| `0x0085f89f`| IntToStr             | Integer to string conversion                   |

### FUN_008675ac Analysis (Power Function)

Confirmed to be `Power(base, exponent)`:
1. Loads `FLDLN2` (ln(2) constant) at `0x00866fa4`
2. Uses `FYL2X` to compute `y * log2(x)` where y = ln(2), giving `ln(x)`
3. Multiplies by the exponent argument: `ln(x) * exponent`
4. Calls `FUN_00866ed5` to compute `2^(result)` = `2^(exponent * ln(x))` = `x^exponent`
5. Handles edge cases: negative bases, zero, infinity, NaN

This is the standard x87 power implementation: `x^y = 2^(y * log2(x))`.

---

## Mode 7 Comparison: Broadside Coupled Shielded (FUN_004b8104)

For reference, Mode 7 (Broadside Coupled Shielded) at `0x004b8104` has a
fundamentally different structure. It:

1. **Has standard copper thickness lookups** (0.35, 0.70, 1.40, ... 7.00 mils)
   via `param_1 + 0x990` -- the same as other impedance solvers
2. **Has plating thickness lookups** (0.0, 0.70, 1.40, ... 4.20 mils)
   via `param_1 + 0x620`
3. **Reads geometric inputs**:
   - Pad diameter from `param_1 + 0xa6c` -> `_DAT_008d6034`
   - Er from `param_1 + 0xa70` -> `_DAT_008d62a0`
   - Er (via) from `param_1 + 0x1200` -> `_DAT_008d6a20`
4. **Computes cross-sectional area** using three sub-modes (via `PTR__Form6 + 0x494`):
   ```
   Mode 0: area = (diameter - total_thickness) * total_thickness
   Mode 1: area = (diameter - total_thickness/2) * total_thickness
   Mode 2: area = diameter * total_thickness
   ```
5. **Computes capacitance** using:
   ```
   capacitance = area * (4/pi) * sqrt(Er) * ln(...) * Er_via
   ```
   Where `4/pi = 1.2732` is at address `0x004ba940`.
6. **Computes inductance** using:
   ```
   inductance = area * (4/pi)
   ```

Mode 7 is a true via/barrel impedance calculator, while Mode 8 appears to be a
wire gauge property calculator that feeds into a simpler multiplication formula.

---

## UI Layout

### Controls Visible in Mode 8

The UI setup function `FUN_00492768` configures:

| Offset   | Control                          | Visible | Purpose                     |
|----------|----------------------------------|---------|-----------------------------|
| +0x0598  | ComboBox (AWG selector)          | Yes     | Wire gauge selection         |
| +0x0684  | RadioGroup (unit mode)           | Enabled | Mils/mm selector             |
| +0x0588  | Panel                            | Yes     | Main display area            |
| +0x0894  | TEdit (diameter display)         | Yes     | Wire diameter                |
| +0x088c  | TEdit (resistance display)       | Yes     | Wire resistance              |
| +0x059c  | TEdit (additional display)       | Yes     | Secondary property           |
| +0x0a28  | TEdit (area result)              | Yes     | Cross-sectional area result  |
| +0x09bc  | ListView/Grid (result grid)      | Yes     | Results table                |
| +0x09b0  | ListView/Grid (secondary grid)   | Yes     | Secondary results            |
| +0x156c  | TEdit (val_A input)              | Yes     | Input parameter A            |
| +0x157c  | TEdit (val_B input)              | Yes     | Input parameter B            |
| +0x1574  | TEdit (computed result)          | Yes     | Final computed result        |

Many controls from other impedance modes are hidden (SetVisible=0):
`+0x1590`, `+0x14d0`, `+0x14d4`, `+0x14d8`, `+0x11b8`, `+0x14fc`, `+0x998`,
`+0x99c`, `+0xfdc`, `+0xfe0`, and many more.

The TabControl at `param_1 + 0x490` is set to index 8 (the "Broad Cpld NShld" tab).

---

## Global Variables

| Address        | Type   | Purpose                                     |
|----------------|--------|---------------------------------------------|
| `DAT_008d6250` | u32    | Wire diameter double (lo 32 bits)            |
| `DAT_008d6254` | u32    | Wire diameter double (hi 32 bits)            |
| `_DAT_008d6aac`| u32    | Wire resistance double (lo 32 bits)          |
| `_DAT_008d6ab0`| u32    | Wire resistance double (hi 32 bits)          |
| `_DAT_008d6498`| f64    | Computed area: diameter_mils^2 / 700 (mils)  |
| `_DAT_008d64a0`| f64    | Computed area: diameter_mm^2 / 700 (metric)  |
| `_DAT_008d6a94`| f64    | Input val_A from form field 0x156c           |
| `_DAT_008d6ab4`| f64    | Input val_B from form field 0x157c           |
| `_DAT_008d6aa4`| f64    | Result: (val_A * resistance) / 1000 * val_B  |

---

## Formulas Summary

### Wire Area Computation

```
area_result = Power(diameter_mils, 2.0) / 700.0
```

Where `diameter_mils = diameter_inches * 1000.0` for mils mode.

The constant `700.0` is notable -- it does not correspond to a standard unit
conversion. One possibility is that it combines the circular mil area formula
`area_cmil = d_mils^2` with a scaling factor of `1/700` for display purposes
(e.g., converting to kcmil or another unit).

### PreCompute_4 Result

```
result = (val_A * wire_resistance) / 1000.0 * val_B
```

Where:
- `val_A` = user input from `param_1 + 0x156c`
- `val_B` = user input from `param_1 + 0x157c`
- `wire_resistance` = lookup table value from `_DAT_008d6aac`

---

## Notes

1. **Misnamed function?** Despite being dispatched as Mode 8 ("Broad Cpld NShld"),
   the function content is entirely about AWG wire gauge properties. It is possible
   that the "Broadside Coupled Non-Shielded" label was reused or that this mode
   serves as a sub-calculator for a broadside coupled geometry where wire properties
   are needed.

2. **No impedance formula**: Unlike Mode 7 (Broadside Coupled Shielded) which has
   explicit area/capacitance/inductance formulas using constants like `4/pi = 1.2732`,
   Mode 8 has no embedded impedance formula. The computation is a simple lookup +
   power + division + multiplication.

3. **FUN_008675ac is Power(), not Ln()**: The earlier notes identified `FUN_008675ac`
   as `Math_Ln()`, but closer analysis of the calling convention (two double arguments)
   and internal implementation (`FYL2X` + `2^x` = `base^exponent`) confirms it is
   actually `Power(base, exponent)`. When called with a second argument of 1.0, it
   would reduce to `ln(x)` behavior through `FYL2X`, which may explain the earlier
   misidentification. In Mode 8, it is called with exponent 2.0, computing
   `diameter^2`.

4. **44 AWG entries**: The lookup table covers AWG 4/0 through AWG 40, which is the
   full standard range. Each entry sets two double values (diameter and resistance)
   plus displays them in three UI fields.

5. **Common pre-computation**: The function calls `FUN_004e0928` at the end, which
   also runs as a common pre-computation before all mode dispatches. This may create
   the actual impedance output by combining the wire gauge data with other parameters
   read from the form.
