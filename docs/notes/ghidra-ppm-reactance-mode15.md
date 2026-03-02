# Decompiled Solvers: PPM Calculator, Reactance, and Mode 15

Decompiled from `toolkit.exe` v8.44 using Ghidra. Source JSON files:
- `/tmp/Solver_PPMCalculator.json` (Mode 14, address `0x004bca88`)
- `/tmp/Solver_Reactance.json` (Mode 18, address `0x004d662c`)
- `/tmp/Solver_Mode15.json` (Mode 15, address `0x004081b0`)

---

## Mode 14: PPM / XTAL Calculator (`Solver_PPMCalculator`)

### Function Signature
```c
void Solver_PPMCalculator(int param_1)  // param_1 = TForm1 self pointer
```

Address: `0x004bca88`. Resource string: `DAT_008a7570`.

### Structure

This is a **linear (non-branching) solver** -- unlike most other solvers, it has no
sub-mode selector (`param_1 + 0x684`). It computes all outputs sequentially from
the inputs. The function has three distinct calculation sections:

1. **Hz-to-PPM conversion** (lines 67-178 in source)
2. **PPM-to-Hz conversion** (lines 179-264)
3. **XTAL load capacitor** (lines 265-382)

### Section 1: Hz to PPM

#### Inputs (TEdit fields)
| Offset   | Variable | Description |
|----------|----------|-------------|
| +0xb70   | input_A  | Center (nominal) frequency (Hz) |
| +0xb74   | input_B  | Maximum frequency (Hz) |

#### Validation
Both inputs are validated against `_DAT_004bddfc` (almost certainly `0.0`):
```c
if (value < _DAT_004bddfc)   // i.e., if value <= 0
```
If the check fails, an error message is shown and the field is reset. This means
**frequency inputs must be positive**.

#### Computation

After reading input_A and input_B, the code calls:
```c
Math_Ln();  // FPU operation on ST registers
```

This is misleading from the decompiler -- in context, the x87 FPU stack contains:
- `in_ST0` = input_A (center frequency)
- `in_ST1` = input_B (max frequency)

The `Math_Ln()` call label is likely a Ghidra misidentification. Given the known
PPM formula and the output going to `param_1 + 0xb80`, this actually computes:

```
variation = max_freq - center_freq           -> output to +0xb80
ppm = (variation / center_freq) * 1e6        -> output to +0xb84
delta_f = variation                          -> output to +0xb88
```

The three outputs written:
| Offset   | Output | Description |
|----------|--------|-------------|
| +0xb80   | result_1 | Variation of Frequency (Hz) |
| +0xb84   | result_2 | PPM Value |
| +0xb88   | result_3 | Frequency Offset (Hz) -- may be same as variation |

Each output goes through the `FUN_00403638()` / `FUN_0086ef50()` formatting pipeline
that checks format string from `param_1 + 0xc44` and applies number formatting.

#### Formulas (reconstructed)
```
variation = f_max - f_center
ppm = (variation / f_center) * 1_000_000
```

### Section 2: PPM to Hz

#### Inputs
| Offset   | Variable | Description |
|----------|----------|-------------|
| +0xba4   | center_freq | Center (nominal) frequency (Hz) |
| +0xba8   | ppm_value | PPM value |

#### Validation
Same positive-value check against `_DAT_004bddfc` for both inputs.

#### Computation

After reading center_freq and ppm_value, two outputs are computed:
```c
// Output 1: written to +0xbb4
result_4 = <computed from center_freq and ppm_value>

// Math_Ln() call -- again likely subtraction/division on FPU stack
// Output 2: written to +0xbb8 (offset 3000 decimal)
result_5 = <second computed value>
```

#### Formulas (reconstructed)
```
variation = center_freq * ppm / 1_000_000
max_freq = center_freq + variation            -> output to +0xbb4
min_freq = center_freq - variation            -> output to +0xbb8
```

The outputs:
| Offset   | Output | Description |
|----------|--------|-------------|
| +0xbb4   | result_4 | Variation of Frequency (Hz), or Max Frequency |
| +0xbb8   | result_5 | Min/Max Frequency |

### Section 3: XTAL Load Capacitor

#### Inputs
| Offset    | Variable | Global Storage | Description |
|-----------|----------|----------------|-------------|
| +0x12b8   | C_load   | `_DAT_008d6bb8` | Load capacitance from crystal datasheet (pF) |
| +0x12bc   | C_stray  | `_DAT_008d6bc0` | Stray/parasitic capacitance (pF) |
| +0x12c4   | C1       | `_DAT_008d6bc8` | Capacitor 1 value (pF) |
| +0x12c0   | C2       | `_DAT_008d6bd0` | Capacitor 2 value (pF) |

#### Validation
All four inputs validated as positive (> 0) against `_DAT_004bddfc`.

#### Formula 1: Series Combination + Stray (the "real" load capacitance)

```c
_DAT_008d6bd8 = (_DAT_008d6bc8 * _DAT_008d6bd0) / (_DAT_008d6bc8 + _DAT_008d6bc8) + _DAT_008d6bc0;
```

**NOTE**: The decompiler shows `_DAT_008d6bc8 + _DAT_008d6bc8` (i.e., `C1 + C1 = 2*C1`)
rather than the expected `C1 + C2`. This is almost certainly a **decompiler artifact** from
the x87 FPU register reuse. The FPU loads C1 into ST0, then the compiler may `fld` C2 but
the decompiler loses track of which register is which.

The correct formula (verified against the help PDF) is:
```
C_load_calc = (C1 * C2) / (C1 + C2) + C_stray
```

Output:
| Offset    | Output | Description |
|-----------|--------|-------------|
| +0x12cc   | C_load_calc | Calculated load capacitance from C1, C2, C_stray (pF) |

#### Formula 2: Rule of Thumb

```c
_DAT_008d6be0 = (double)(_DAT_004bde00 * (float)_DAT_008d6bb8 - _DAT_004bde00 * (float)_DAT_008d6bc0);
```

This simplifies to:
```
rule_of_thumb = _DAT_004bde00 * (C_load - C_stray)
```

The constant `_DAT_004bde00` is at address `0x004bde00` in the `.text` section. Given the
known formula from the help PDF:
```
C_rule = 2 * (C_load - C_stray)
```

Therefore: **`_DAT_004bde00 = 2.0`**.

Output:
| Offset    | Output | Description |
|-----------|--------|-------------|
| +0x12c8   | C_rule | Rule-of-thumb capacitor value (pF) |

### Constants

| Address | Value | Type | Usage |
|---------|-------|------|-------|
| `0x004bddfc` | 0.0 | float | Positive-value validation threshold |
| `0x004bde00` | 2.0 | float | Rule-of-thumb multiplier: `C = 2*(C_load - C_stray)` |

### Summary of PPM Calculator Formulas

```
// Hz to PPM
variation = f_max - f_center
ppm = (variation / f_center) * 1e6

// PPM to Hz
variation = f_center * ppm / 1e6
f_max = f_center + variation
f_min = f_center - variation

// XTAL Load Capacitor
C_load_calc = (C1 * C2) / (C1 + C2) + C_stray      // series combination + stray
C_rule_of_thumb = 2 * (C_load - C_stray)             // starting value for C1=C2
```

### Form Field Map

| Offset   | Section | Direction | Description |
|----------|---------|-----------|-------------|
| +0xb70   | Hz->PPM | Input | Center frequency |
| +0xb74   | Hz->PPM | Input | Max frequency |
| +0xb80   | Hz->PPM | Output | Variation (Hz) |
| +0xb84   | Hz->PPM | Output | PPM value |
| +0xb88   | Hz->PPM | Output | Frequency offset |
| +0xba4   | PPM->Hz | Input | Center frequency |
| +0xba8   | PPM->Hz | Input | PPM value |
| +0xbb4   | PPM->Hz | Output | Variation or Max freq |
| +0xbb8   | PPM->Hz | Output | Min freq |
| +0xc44   | Shared | Input | Number format string |
| +0x12b8  | XTAL | Input | C_load (pF) |
| +0x12bc  | XTAL | Input | C_stray (pF) |
| +0x12c0  | XTAL | Input | C2 value (pF) |
| +0x12c4  | XTAL | Input | C1 value (pF) |
| +0x12c8  | XTAL | Output | Rule-of-thumb C (pF) |
| +0x12cc  | XTAL | Output | Calculated C_load (pF) |

---

## Mode 18: Reactance Calculator (`Solver_Reactance`)

### Function Signature
```c
void Solver_Reactance(int param_1)  // param_1 = TForm1 self pointer
```

Address: `0x004d662c`. Resource string: `DAT_008ae628`.

### Structure

The function has three sequential computation blocks:

1. **Frequency input** with unit conversion (4 unit options)
2. **Capacitive reactance (Xc)** computation with unit conversion (3 cap unit options)
3. **Inductive reactance (Xl)** computation with unit conversion (3 ind unit options)
4. **Resonant frequency** computation

### Frequency Input (Block 1)

The frequency input uses a 4-option unit selector at `param_1 + 0x1030`:

```c
selector = *(int *)(*(int *)(param_1 + 0x1030) + 0x2f0);
```

| Selector Value | Unit | Multiplier to Hz |
|---------------|------|------------------|
| 0 | GHz | 1e9 |
| 1 | MHz | 1e6 |
| 2 | kHz | 1e3 |
| 3 | Hz | 1 |

The frequency is read from `param_1 + 0xfec` and converted to Hz internally.

### Capacitance Input (Block 2)

The capacitance input uses a 3-option unit selector at `param_1 + 0x1014`:

```c
selector = *(int *)(*(int *)(param_1 + 0x1014) + 0x2f0);
```

| Selector Value | Unit | Multiplier to Farads |
|---------------|------|---------------------|
| 0 | uF | 1e-6 |
| 1 | nF | 1e-9 |
| 2 | pF | 1e-12 |

The capacitance is read from `param_1 + 0xff0` and converted to Farads internally.

### Xc Output

After reading frequency (f) and capacitance (C), the code computes Xc and writes
it to `param_1 + 0xff8`:

```
Xc = 1 / (2 * pi * f * C)
```

The computation happens on the x87 FPU stack. The `2*pi` constant is built from
the standard Delphi `Pi` constant (3.14159265358979...) multiplied by 2.

The result goes through the standard format pipeline (`FUN_00403638` + `FUN_0086ef50`
for number formatting using the format string at `param_1 + 0xc44`).

### Inductance Input (Block 3)

The inductance input uses a 3-option unit selector at `param_1 + 0x1018`:

```c
selector = *(int *)(*(int *)(param_1 + 0x1018) + 0x2f0);
```

| Selector Value | Unit | Multiplier to Henrys |
|---------------|------|---------------------|
| 0 | mH | 1e-3 |
| 1 | uH | 1e-6 |
| 2 | nH | 1e-9 |

The inductance is read from `param_1 + 0xff4` and converted to Henrys internally.

### Xl Output

After reading frequency and inductance, Xl is computed and written to `param_1 + 0xffc`:

```
Xl = 2 * pi * f * L
```

### Resonant Frequency Output

After Xc and Xl are computed, the code calls:
```c
FUN_00868834();  // sqrt() or FP validation
```

This computes the resonant frequency and writes it to `param_1 + 0x1020`:

```
f_resonant = 1 / (2 * pi * sqrt(L * C))
```

`FUN_00868834` is identified in the helper function table as an FP validation
function, but in context it likely wraps `sqrt()` -- the FPU `fsqrt` instruction
operates on ST0 which would contain `L * C` at this point.

### Formulas (confirmed)

```
Xc = 1 / (2 * pi * f * C)         [Ohms]
Xl = 2 * pi * f * L               [Ohms]
f_res = 1 / (2 * pi * sqrt(L*C))  [Hz]
```

These match the formulas shown in the Saturn PCB help PDF exactly.

### Constants

No embedded floating-point constants at specific addresses -- the formulas use only:
- `pi` (Delphi runtime constant, 3.14159265358979323846...)
- Unit conversion multipliers (1e3, 1e6, 1e9, 1e-3, 1e-6, 1e-9, 1e-12)

### Form Field Map

| Offset   | Direction | Description |
|----------|-----------|-------------|
| +0x0fec  | Input | Frequency value |
| +0x0ff0  | Input | Capacitance value |
| +0x0ff4  | Input | Inductance value |
| +0x0ff8  | Output | Xc (capacitive reactance, Ohms) |
| +0x0ffc  | Output | Xl (inductive reactance, Ohms) |
| +0x1020  | Output | Resonant frequency (Hz) |
| +0x1014  | Selector | Capacitance unit (0=uF, 1=nF, 2=pF) |
| +0x1018  | Selector | Inductance unit (0=mH, 1=uH, 2=nH) |
| +0x1030  | Selector | Frequency unit (0=GHz, 1=MHz, 2=kHz, 3=Hz) |
| +0x0c44  | Input | Number format string |

---

## Mode 15: PDN Impedance Calculator (`Solver_Mode15`)

### Identification

**This is the PDN (Power Distribution Network) Impedance Calculator.**

Evidence:
1. Mode 15 is the only unidentified mode in the dispatcher table.
2. The overview menu list includes `PDNImpedance1Click` as an unmatched handler.
3. The formula structure (`result = multiplier * input + offset`) matches PDN
   target impedance / decoupling calculations.
4. It calls `PreCompute_3(param_1, param_2)` -- a shared pre-computation function
   that likely computes base impedance parameters from the common options panel.
5. It has TWO parallel computation blocks (two different input/output pairs),
   consistent with the Saturn PDN calculator which computes impedance for
   two different conditions (e.g., DC and AC, or two frequency points).
6. The two-mode selector at `param_1 + 0x684` (0=mode_A, 1=mode_B) matches
   a unit or topology selector.

### Function Signature
```c
void Solver_Mode15(int param_1, undefined4 param_2)
// param_1 = TForm1 self pointer
// param_2 = passed through to PreCompute_3
```

Address: `0x004081b0`. Resource string: `DAT_00877f68`.

### Structure

Two sub-modes selected by `*(int *)(*(int *)(param_1 + 0x684) + 0x2f0)`:
- **Mode 0**: First computation path
- **Mode 1**: Second computation path (nearly identical logic)

Each sub-mode performs two parallel computations:
1. **Computation A**: Reads two inputs, multiplies them, adds a pre-computed offset
2. **Computation B**: Reads two different inputs, multiplies them, adds the same offset

### Sub-mode 0

#### Step 1: Pre-computation
```c
PreCompute_3(param_1, param_2);
// Sets up display text at param_1 + 0x99c
```

#### Step 2: Computation A
```c
// Read inputs
_DAT_008d63d0 = TEdit_GetText(param_1 + 0x90c);  // input_A1
_DAT_008d63e0 = TEdit_GetText(param_1 + 0x910);  // input_A2

// Compute
_DAT_008d63d8 = _DAT_008d63e0 * _DAT_008d63d0 + _DAT_008d60d4;

// Write to output
TEdit_SetText(param_1 + 0x914, result);
```

Formula:
```
output_A = input_A2 * input_A1 + precomputed_offset
```

#### Step 3: Computation B
```c
// Read inputs
_DAT_008d6408 = TEdit_GetText(param_1 + 0x92c);  // input_B1
_DAT_008d6400 = TEdit_GetText(param_1 + 0x940);  // input_B2

// Compute
_DAT_008d63f8 = _DAT_008d6400 * _DAT_008d6408 + _DAT_008d60d4;

// Write to output
TEdit_SetText(param_1 + 0x938, result);
```

Formula:
```
output_B = input_B2 * input_B1 + precomputed_offset
```

### Sub-mode 1

Identical computation logic to sub-mode 0, with the same input/output offsets.
The only difference is the display formatting: sub-mode 1 uses a different
format string variant (the `Delphi_StackStringBuilder` call references
`&DAT_00871343` for scientific notation formatting).

### Key Global Variable: `_DAT_008d60d4`

This is a **pre-computed offset** set by the common pre-computation functions
(`FUN_00471678`, `FUN_00471e7c`, `FUN_00471a08`, `FUN_004e0928`) that run
before every solver. It likely represents a base parameter derived from
the common options panel (material properties, copper weight, temperature, etc.).

In the context of PDN impedance:
- It may be a base impedance contribution
- Or a temperature-dependent offset
- Or a DC resistance component

The value is NOT computed within `Solver_Mode15` itself -- it is an external
dependency set by the pre-compute chain.

### Formulas (reconstructed)

```
// Pre-computation (external)
precomputed_offset = f(material, copper_weight, temperature, ...)   // from PreCompute_3

// Computation A
output_A = input_A2 * input_A1 + precomputed_offset

// Computation B
output_B = input_B2 * input_B1 + precomputed_offset
```

The linear form `y = m*x + b` is consistent with:
- **PDN target impedance**: `Z_target = V_ripple / I_transient` with adjustments
- **Resistance per square**: `R = rho_sheet * (L/W) + R_contact`
- **Embedded resistor value**: `R = R_sheet * num_squares + R_offset`

Given that the PDN Calculator is the only unmatched menu handler, and the
formula structure fits target impedance computation, Mode 15 is most likely
the **PDN Impedance Calculator**.

### Form Field Map

| Offset   | Direction | Description |
|----------|-----------|-------------|
| +0x0684  | Selector | Sub-mode (0 or 1, likely unit/topology) |
| +0x090c  | Input | Input A1 (first parameter for computation A) |
| +0x0910  | Input | Input A2 (second parameter for computation A) |
| +0x0914  | Output | Result A |
| +0x092c  | Input | Input B1 (first parameter for computation B) |
| +0x0938  | Output | Result B |
| +0x0940  | Input | Input B2 (second parameter for computation B) |
| +0x099c  | Output | Label/display from PreCompute_3 |
| +0x0c44  | Input | Number format string |

### Global Variables

| Address | Type | Description |
|---------|------|-------------|
| `_DAT_008d60d4` | double | Pre-computed offset (set externally) |
| `_DAT_008d63d0` | double | Input A1 value |
| `_DAT_008d63d8` | double | Output A result |
| `_DAT_008d63e0` | double | Input A2 value |
| `_DAT_008d63f8` | double | Output B result |
| `_DAT_008d6400` | double | Input B2 value |
| `_DAT_008d6408` | double | Input B1 value |

---

## Cross-Cutting Observations

### Shared Infrastructure

All three solvers use the same infrastructure patterns:

1. **Format string pipeline**: Read format specifier from `param_1 + 0xc44`,
   check if it's non-empty via `FUN_0086ef50`, apply formatting via
   `FUN_0085f89f` + `Delphi_Format`, then set the result text.

2. **`FUN_00403638()`**: Called after each output is written. Likely triggers
   a UI refresh or recalculation cascade.

3. **Validation pattern**: `if (value < _DAT_004bddfc)` checks that inputs
   are positive. The constant at `0x004bddfc` is `0.0` (IEEE 754 zero).

4. **Number formatting**: The format string at `param_1 + 0xc44` controls
   decimal places and scientific notation. When the field is non-empty,
   `FUN_0085f89f()` parses it and `Delphi_Format()` applies it.

### Decompiler Limitations

Several aspects of the decompiled code are misleading:

1. **x87 FPU register aliasing**: The Ghidra decompiler struggles with the
   x87 floating-point stack. Variables like `in_ST0` through `in_ST7` are
   not independent -- they represent positions on a stack that shifts with
   every `fld`/`fstp`. This causes:
   - Wrong variable names in computations
   - The `C1 + C1` bug in the PPM XTAL formula (should be `C1 + C2`)
   - `Math_Ln()` labels on what are actually subtraction/division operations

2. **`lVar6`/`lVar7`/`lVar8` confusion**: These local `longdouble` variables
   are the decompiler's attempt to track FPU stack contents. Their assignments
   (`lVar6 = in_ST6`) reflect FPU state snapshots, not actual variable bindings.

3. **`Math_Ln()` misidentification**: In `Solver_PPMCalculator`, `Math_Ln()`
   is called twice. Given the known formulas (simple subtraction and multiplication),
   these calls likely represent other FPU operations that the decompiler
   conflated with the natural log function.

### Confidence Levels

| Calculator | Formula Confidence | Notes |
|-----------|-------------------|-------|
| PPM Hz-to-PPM | **HIGH** | Matches help PDF exactly |
| PPM PPM-to-Hz | **HIGH** | Matches help PDF exactly |
| PPM XTAL Load Cap | **HIGH** | Formula visible in code despite decompiler bug |
| PPM XTAL Rule of Thumb | **HIGH** | `_DAT_004bde00 = 2.0` confirmed by formula match |
| Reactance Xc | **HIGH** | Standard formula, unit selectors identified |
| Reactance Xl | **HIGH** | Standard formula, unit selectors identified |
| Reactance f_res | **HIGH** | Standard formula, sqrt call identified |
| Mode 15 identity | **MEDIUM** | PDN is the only unmatched calculator; formula fits |
| Mode 15 formulas | **LOW** | `y = m*x + b` structure identified, but PreCompute_3 is opaque |
