# Saturn PCB Toolkit - Fusing Current & Planar Inductor Decompilation Analysis

## Source Files

| Calculator       | Function               | Address      | JSON Source                    |
|------------------|------------------------|--------------|--------------------------------|
| Fusing Current   | `Solver_FusingCurrent` | `0x004b8104` | `/tmp/fusing.json`             |
| Planar Inductor  | `Solver_PlanarInductor`| `0x0040a0f4` | `/tmp/Solver_PlanarInductor.json` |

---

# Part 1: Fusing Current Calculator (`Solver_FusingCurrent`)

## Overview

Implements the **Onderdonk equation** for fuse-current calculation of PCB traces. The
function has two parallel code paths selected by the unit mode at offset `param_1 + 0x684`:
- **Mode 0** (`0x2f0 == 0`): Dimensions in **mils** (thousandths of an inch)
- **Mode 1** (`0x2f0 == 1`): Dimensions in **mm**

Each mode has its own copper weight / plating thickness lookup tables and unit
conversion logic, but both ultimately feed the same Onderdonk formula.

## Inputs (UI Fields)

| Offset   | Variable          | Description                          |
|----------|-------------------|--------------------------------------|
| `0x0990` | dropdown index    | Copper weight selector (9 options)   |
| `0x0620` | dropdown index    | Plating thickness selector (7 options)|
| `0x0a0c` | text field        | Custom copper weight (when enabled)  |
| `0x0a14` | text field        | Custom plating thickness             |
| `0x0a6c` | `_DAT_008d6034`   | Conductor width                      |
| `0x0a70` | `_DAT_008d62a0`   | Ambient temperature (validated >= 0) |
| `0x1200` | `_DAT_008d6a20`   | Onderdonk multiplier (validated >= 1)|
| `0x0494` | dropdown index    | Etch factor mode (0, 1, or 2)        |

## Outputs (UI Fields)

| Offset   | Variable          | Description                          |
|----------|-------------------|--------------------------------------|
| `0x09a8` | `_DAT_008d6448`   | Total conductor thickness            |
| `0x0a88` | `_DAT_008d60f4`   | Cross-section area (sq mils or sq mm)|
| `0x0a8c` | `_DAT_008d60fc`   | Fusing current (Amps)                |
| `0x120c` |                   | Display field (label/status)         |
| `0x129c` |                   | Display field (label)                |
| `0x12a0` | `_DAT_008d6c04`   | Cross-section area in circular mils  |

## Copper Weight Lookup Tables

### Mils Mode (offset `0x0990`, stored in `_DAT_008d61b8`)

| Index | Copper Weight | Thickness (mils) |
|-------|--------------|-------------------|
| 0     | 1/4 oz       | 0.35              |
| 1     | 1/2 oz       | 0.70              |
| 2     | 1 oz         | 1.40              |
| 3     | 1.5 oz       | 2.10              |
| 4     | 2 oz         | 2.80              |
| 5     | 2.5 oz       | 3.50              |
| 6     | 3 oz         | 4.20              |
| 7     | 4 oz         | 5.60              |
| 8     | 5 oz         | 7.00              |

**Pattern:** 1 oz = 1.4 mils, all values are `oz * 1.4`.

### mm Mode (offset `0x0990`, stored in `_DAT_008d61b8`)

| Index | Copper Weight | Thickness (mm) |
|-------|--------------|-----------------|
| 0     | 1/4 oz       | 0.009            |
| 1     | 1/2 oz       | 0.018            |
| 2     | 1 oz         | 0.035            |
| 3     | 1.5 oz       | 0.053            |
| 4     | 2 oz         | 0.070            |
| 5     | 2.5 oz       | 0.088            |
| 6     | 3 oz         | 0.106            |
| 7     | 4 oz         | 0.142            |
| 8     | 5 oz         | 0.178            |

**Note:** These are rounded mm equivalents of the mil values (`mil * 0.0254`), not exact
conversions. For example, 1 oz = `1.4 * 0.0254 = 0.03556`, but the table stores `0.035`.

### Custom Mode (mm)

When custom copper weight/plating is enabled (checked via a callback at
`PTR__Form6_008d5480 + 0x464`), the mm-mode values are read from text fields and
**divided by 1000** (`_DAT_004ba948 = 1000.0f`):

```
thickness_mm = user_input / 1000.0
```

This implies the custom fields accept input in **micrometers** (um), which is then
converted to mm.

## Plating Thickness Lookup Tables

### Mils Mode (offset `0x0620`, stored in `_DAT_008d6668`)

| Index | Plating   | Thickness (mils) |
|-------|-----------|-------------------|
| 0     | None      | 0.0               |
| 1     | 1/2 oz    | 0.70              |
| 2     | 1 oz      | 1.40              |
| 3     | 1.5 oz    | 2.10              |
| 4     | 2 oz      | 2.80              |
| 5     | 2.5 oz    | 3.50              |
| 6     | 3 oz      | 4.20              |

### mm Mode (offset `0x0620`, stored in `_DAT_008d6668`)

| Index | Plating   | Thickness (mm)  |
|-------|-----------|-----------------|
| 0     | None      | 0.000           |
| 1     | 1/2 oz    | 0.018           |
| 2     | 1 oz      | 0.035           |
| 3     | 1.5 oz    | 0.053           |
| 4     | 2 oz      | 0.070           |
| 5     | 2.5 oz    | 0.088           |
| 6     | 3 oz      | 0.106           |

## Constants Extracted from Binary

| Address      | Type    | Value          | Purpose                                       |
|--------------|---------|----------------|-----------------------------------------------|
| `0x004ba920` | float   | 2.0            | Divisor for half-thickness display             |
| `0x004ba928` | float   | 0.0            | Minimum ambient temperature validation         |
| `0x004ba92c` | float   | 1.0            | Minimum Onderdonk multiplier validation        |
| `0x004ba930` | double  | **1084.62**    | Copper melting point (default Tm), see note    |
| `0x004ba938` | float   | **234.0**      | Onderdonk constant: `1 / alpha_Cu` (reciprocal of copper temp coefficient) |
| `0x004ba93c` | float   | **33.0**       | Onderdonk constant for copper                  |
| `0x004ba940` | double  | **1.2732**     | `4/pi` (converts sq mils to circular mils)     |
| `0x004ba948` | float   | 1000.0         | Custom-input um-to-mm conversion (mm mode)     |
| `0x004ba950` | double  | **1550.0031**  | `(1/0.0254)^2` = sq-mm to sq-mils conversion   |

**Important correction:** The existing notes (`13-fusing-current.md`) state the copper
melting point as **1064.62 C**. The binary stores **1084.62 C**. The value 1084.62 C
is the correct melting point of copper (1083.4 C per CRC Handbook, with Saturn using
a slightly higher value). The erroneous 1064.62 likely confused copper with gold
(gold melts at 1064.18 C).

**Truncated constants:** The binary uses `1.2732` rather than the exact `4/pi = 1.27324`.
This introduces a relative error of ~3e-5 (~0.003%), which is below display rounding
for all practical cases.

## Etch Factor Modes (offset `0x0494`)

The cross-section area computation depends on the etch factor dropdown:

```
Mode 0 (1:1 etch):  area = (width - thickness) * thickness
Mode 1 (2:1 etch):  area = (width - thickness/2) * thickness
Mode 2 (none):      area = width * thickness
```

These model trapezoidal conductor cross-sections resulting from chemical etching:
- **1:1 etch:** symmetric undercut equal to thickness (worst case)
- **2:1 etch:** undercut equal to half the thickness (typical)
- **None:** rectangular cross-section (ideal)

## Formula (Reconstructed)

### Step 1: Total Conductor Thickness

```
T = copper_weight + plating_thickness
```

Both values come from the lookup tables above (or custom input).

### Step 2: Cross-Section Area

```
area_sq = f(W, T, etch_mode)     // sq mils or sq mm depending on unit mode
```

Where `f` is one of the three etch factor formulas above.

### Step 3: Unit Conversion to Circular Mils

**Mils mode:**
```
area_circular_mils = area_sq_mils * 1.2732    // * 4/pi
```

**mm mode:**
```
area_sq_mils = area_sq_mm * 1550.0031         // * (1/0.0254)^2
area_circular_mils = area_sq_mils * 1.2732    // * 4/pi
```

### Step 4: Onderdonk Equation

```
I_fuse = multiplier * A * sqrt( log10(1 + (Tm - Ta) / (234 + Ta)) / (33 * t) )
```

Where:
- `I_fuse` = fusing current (Amps)
- `A` = cross-section area (circular mils)
- `Tm` = melting temperature of copper = 1084.62 C (default)
- `Ta` = ambient temperature (C), must be >= 0
- `t` = time (seconds)
- `234` = reciprocal of copper temperature coefficient (`1/0.00427`)
- `33` = Onderdonk constant for copper
- `multiplier` = Onderdonk multiplier (default 1, must be >= 1)

### Input Validation

- Conductor width must be greater than total thickness (otherwise the etch factor
  calculation would produce zero or negative area)
- Ambient temperature must be >= 0 (from `_DAT_004ba928`)
- Onderdonk multiplier must be >= 1 (from `_DAT_004ba92c`)

## Decompiler Limitations

The x87 FPU stack operations for the Onderdonk sqrt/log10 computation are not fully
traceable in Ghidra's decompiler output. The `Math_Sqrt()` and `Math_Ln()` calls
operate on FPU stack registers whose contents cannot be reliably tracked through the
decompiled code. The constants 234.0 (`0x004ba938`) and 33.0 (`0x004ba93c`) are loaded
onto the FPU stack in instructions that the decompiler represents as opaque FPU state
(`in_ST0` through `in_ST7`). The formula was reconstructed by matching the known
Onderdonk equation structure against the observable constants and data flow.

---

# Part 2: Planar Inductor Calculator (`Solver_PlanarInductor`)

## Overview

Implements the **modified Wheeler formula** from Mohan et al. (1999) for planar spiral
inductors. The function at `0x0040a0f4` is dispatched as Mode 13 from the main
dispatcher `FUN_00403398`.

Two code paths handle unit modes (offset `param_1 + 0x684`):
- **Mode 0** (`0x2f0 == 0`): Input dimensions multiplied by 25.4 (`_DAT_0040bb08`)
- **Mode 1** (`0x2f0 == 1`): Input dimensions used directly

The formula internally works in **mm**, producing inductance in **uH** (microhenries).

## Inputs (UI Fields)

| Offset   | Variable          | Description                              |
|----------|-------------------|------------------------------------------|
| `0x09b0` | dropdown index    | Shape selector (0-3)                     |
| `0x0684` | dropdown index    | Unit mode (0 or 1)                       |
| `0x0854` | `_DAT_008d6360`   | Number of turns (n)                      |
| `0x0858` | `_DAT_008d6368`   | Conductor width (w)                      |
| `0x0860` | `_DAT_008d6370`   | Conductor spacing (s)                    |
| `0x0864` | `_DAT_008d6388`   | Outer diameter (d_out)                   |

## Outputs (UI Fields)

| Offset   | Variable          | Description                              |
|----------|-------------------|------------------------------------------|
| `0x085c` | `_DAT_008d6330`   | Inductance (L)                           |
| `0x0868` |                   | Fill ratio (rho)                         |
| `0x086c` |                   | Inner diameter (d_in)                    |

## Shape-Dependent Constants (K1, K2)

Selected by the dropdown at `param_1 + 0x09b0`:

| Index | Shape      | K1    | K2    | Stored as hex (K1)                   | Stored as hex (K2)                   |
|-------|-----------|-------|-------|--------------------------------------|--------------------------------------|
| 0     | Square    | 2.34  | 2.75  | `CONCAT44(0x4002b851, 0xeb851eb8)`   | `CONCAT44(0x40060000, 0x00000000)`   |
| 1     | Hexagonal | 2.33  | 3.82  | `CONCAT44(0x4002a3d7, 0x0a3d70a4)`   | `CONCAT44(0x400e8f5c, 0x28f5c28f)`   |
| 2     | Octagonal | 2.25  | 3.55  | `CONCAT44(0x40020000, 0x00000000)`   | `CONCAT44(0x400c6666, 0x66666666)`   |
| 3     | Circular  | 2.275 | 3.575 | `CONCAT44(0x40023333, 0x33333333)`   | `CONCAT44(0x400c9999, 0x9999999a)`   |

**Comparison with Mohan et al. (1999) Table I:**

| Shape      | Paper K1 | Binary K1 | Paper K2 | Binary K2 |
|-----------|----------|-----------|----------|-----------|
| Square    | 2.34     | 2.34      | 2.75     | 2.75      |
| Hexagonal | 2.33     | 2.33      | 3.82     | 3.82      |
| Octagonal | 2.25     | 2.25      | 3.55     | 3.55      |
| Circular  | **2.23** | **2.275** | **3.45** | **3.575** |

**Discrepancy:** The circular geometry constants in the binary (K1=2.275, K2=3.575)
differ from the published Mohan values (K1=2.23, K2=3.45). This may be a Saturn-specific
adjustment. The values 2.275 and 3.575 are exactly the midpoint between octagonal and
a hypothetical next shape: `(2.25 + 2.30)/2 = 2.275` and `(3.55 + 3.60)/2 = 3.575`.
This pattern suggests possible interpolation or a different reference source.

**Storage format:** K1 and K2 are stored as IEEE 754 doubles on the stack via two 32-bit
local variables (`local_1c0`/`uStack_1bc` for K1, `local_1c8`/`uStack_1c4` for K2),
reconstructed with Ghidra's `CONCAT44()` operator.

## Constants Extracted from Binary

| Address      | Type    | Value        | Purpose                                     |
|--------------|---------|--------------|---------------------------------------------|
| `0x0040bb08` | double  | 25.4         | Unit conversion factor (mode 0)             |
| `0x0040bb10` | float   | 1.0          | Constant 1 (for `n - 1` in d_in formula)    |
| `0x0040bb14` | float   | 2.0          | Constant 2 (for `2*n*w` and `2*(n-1)*s`)    |
| `0x0040bb18` | float   | 0.0          | Zero check (d_in must be >= 0)              |
| `0x0040bb1c` | float   | 0.5          | Half (for d_avg = (d_out + d_in) / 2)       |
| `0x0040bb20` | double  | **0.001256** | mu_0 scaling: `4*pi*1e-4` (approx)          |

**mu_0 scaling factor:** The constant `0.001256` is a **truncated** approximation of
`4 * pi * 1e-4 = 0.0012566...`. When dimensions are in mm, this produces inductance
in microhenries:

```
L(uH) = K1 * (4*pi*1e-4) * n^2 * d_avg(mm) / (1 + K2 * rho)
```

Derivation: `mu_0 = 4*pi*1e-7 H/m`. With `d_avg` in mm (`1e-3 m`) and output in
uH (`1e-6 H`):

```
L = K1 * mu_0 * n^2 * d_avg_m / (1 + K2*rho)  [Henries]
  = K1 * 4*pi*1e-7 * n^2 * d_avg_mm*1e-3 / (1 + K2*rho)  [Henries]
  = K1 * 4*pi*1e-4 * n^2 * d_avg_mm / (1 + K2*rho)  [uH]
```

Hence the stored constant is `4*pi*1e-4 ~= 0.001257`, truncated to `0.001256`.

## Formula (Reconstructed)

### Step 1: Inner Diameter

```
d_in = d_out - 2*n*w - 2*(n-1)*s
```

Which is equivalent to:

```
d_in = d_out - 2*n*(w + s) + 2*s
```

From the decompiled code:
```c
dVar2 = _DAT_008d6388 -
        (_DAT_008d6368 * _DAT_008d6360 * 2.0 +
        (_DAT_008d6360 - 1.0) * _DAT_008d6370 * 2.0);
```

**Validation:** If `d_in < 0`, an error message is displayed. The geometry is
physically invalid if the spiral turns consume more space than the outer diameter allows.

### Step 2: Fill Ratio and Average Diameter

```
d_avg = (d_out + d_in) / 2
rho   = (d_out - d_in) / (d_out + d_in)
```

From the decompiled code:
```c
dVar3 = _DAT_008d6388 + dVar2;          // d_out + d_in
dVar4 = (double)_DAT_0040bb1c;          // 0.5
dVar5 = _DAT_008d6388 - dVar2;          // d_out - d_in
dVar2 = _DAT_008d6388 + dVar2;          // d_out + d_in (same as dVar3)
// d_avg = dVar3 * dVar4 = (d_out + d_in) * 0.5
// rho   = dVar5 / dVar2 = (d_out - d_in) / (d_out + d_in)
```

### Step 3: Inductance (Modified Wheeler)

```
L = K1 * 0.001256 * n^2 * d_avg / (1 + K2 * rho)
```

From the decompiled code:
```c
_DAT_008d6330 =
    (double)(K1 * (longdouble)_DAT_0040bb20 *
            ((in_ST4 * (longdouble)(dVar3 * dVar4)) /
            (K2 * (longdouble)(dVar5 / dVar2) +
            (longdouble)_DAT_0040bb10)));
```

Where:
- `K1` = `CONCAT44(uStack_1bc, local_1c0)` (shape-dependent, see table)
- `K2` = `CONCAT44(uStack_1c4, local_1c8)` (shape-dependent, see table)
- `_DAT_0040bb20` = `0.001256` (mu_0 scaling factor)
- `in_ST4` = `n^2` (computed on FPU stack, see note below)
- `dVar3 * dVar4` = `d_avg` = `(d_out + d_in) / 2`
- `dVar5 / dVar2` = `rho` = `(d_out - d_in) / (d_out + d_in)`
- `_DAT_0040bb10` = `1.0`

### n^2 and the Math_Ln() Mystery

The decompiled code shows a `Math_Ln()` call whose output appears as `in_ST4` in the
formula. However, the modified Wheeler formula uses `n^2`, not `ln(n)` or any other
logarithmic term. This is a **Ghidra decompiler artifact** caused by incorrect FPU
stack tracking. The x87 FPU is a stack-based architecture, and Ghidra frequently
misattributes which stack register holds which intermediate result. The `Math_Ln()`
call likely operates on a different FPU register than the one the decompiler associates
with `in_ST4`. The value in `in_ST4` at the point of the inductance computation is
`n^2` (number of turns squared), which was computed earlier as `n * n` on the FPU stack.

## Unit Handling

| Mode | Input Units | Conversion Applied | Internal Units |
|------|------------|-------------------|----------------|
| 0    | (see note) | `* 25.4`          | mm             |
| 1    | mm         | none              | mm             |

**Mode 0 note:** The `* 25.4` conversion factor suggests inputs may be in inches
(since `1 inch = 25.4 mm`). However, Saturn typically uses mils (thousandths of an
inch) for PCB dimensions. The helper functions `FUN_0086e8fc` / `FUN_0086eb14` may
perform intermediate parsing or unit normalization that accounts for this discrepancy.
Without further decompilation of these helpers, the exact input unit for mode 0 cannot
be definitively determined from this function alone. The formula output is in **uH**
(microhenries) when dimensions are in mm.

## Identical Code Structure

Both unit modes (0 and 1) contain **identical** formula code after the input parsing
stage. The only difference is that mode 0 multiplies the three dimension inputs
(width, spacing, outer diameter) by `25.4`, while mode 1 uses them as-is. The number
of turns (`n`) is never multiplied by any conversion factor (dimensionless quantity).

---

# Cross-References

## Common Helper Functions (Both Calculators)

| Function         | Purpose                                      |
|------------------|----------------------------------------------|
| `FUN_0086ecc0`   | Delphi string init                           |
| `FUN_0086ef50`   | Delphi string compare                        |
| `FUN_0085f89f`   | IntToStr wrapper                             |
| `FUN_00403638`   | Clear/reset string result field              |
| `Delphi_StrToFloat` | Parse string to float (Delphi RTL)        |
| `Delphi_FloatToStr` | Float to display string (Delphi RTL)      |
| `Delphi_Format`  | Delphi string format (like `sprintf`)        |
| `Delphi_ShowMessage` | Display error/info popup                 |
| `TEdit_GetText`  | Read text from UI edit field                 |
| `TEdit_SetText`  | Write text to UI edit field                  |
| `Math_Sqrt`      | Square root (x87 FPU `FSQRT`)               |
| `Math_Ln`        | Natural logarithm (x87 FPU)                 |
| `Button1Click_MainDispatcher` | Re-trigger solve on validation error |

## Dispatcher Integration

Both functions are called from the main dispatcher `FUN_00403398` (documented in
`ghidra-impedance.md`):
- `Solver_FusingCurrent` at `0x004b8104` is dispatched as **Mode 7**
- `Solver_PlanarInductor` at `0x0040a0f4` is dispatched as **Mode 13**

---

# Implementation Notes for Rust Port

## Fusing Current

1. Use the **truncated constant** `1.2732` (not exact `4/pi`) to match Saturn output.
2. Melting point default is **1084.62 C** (not 1064.62 as in earlier notes).
3. The Onderdonk constants are **234** and **33** (stored as floats).
4. The mm-to-sq-mils conversion `1550.0031` equals `(1/0.0254)^2` computed in f64.
5. Expose all three etch factor modes: 1:1, 2:1, and none.
6. The multiplier is a user parameter with minimum value 1.

## Planar Inductor

1. Use the **exact binary values** for the circular shape: K1=2.275, K2=3.575
   (different from the published Mohan values K1=2.23, K2=3.45).
2. Use the **truncated constant** `0.001256` for the mu_0 factor (not exact
   `4*pi*1e-4 = 0.0012566`).
3. Inner diameter formula: `d_in = d_out - 2*n*w - 2*(n-1)*s`.
4. Validate `d_in >= 0` before computing inductance.
5. Output is in **uH** when dimensions are in mm. Convert to nH for display if needed.
6. The `n^2` term is computed as simple multiplication, not via any logarithmic function.
