# Saturn PCB Toolkit - Edge Coupled Impedance Solvers

## Overview

The Edge Coupled impedance solvers handle modes 3-6 in the main dispatcher
(`Button1Click_MainDispatcher` at `0x00403398`). These compute differential and
common-mode impedance for edge-coupled (side-by-side) transmission line pairs.

Each solver supports two solve directions via the inner selector at
`param_1 + 0x684 + 0x2f0`:
- Mode 0: Forward solve (given geometry, compute impedance)
- Mode 1: Reverse solve (given impedance, compute geometry)

All four solvers share the same basic structure:
1. Read UI inputs (trace width, spacing, dielectric height, Er, etc.)
2. Apply unit conversions (mils/mm/um)
3. Compute Zodd (odd-mode impedance) and Zeven (even-mode impedance)
4. Derive Zdiff, Zcommon, coupling coefficient Kb
5. Compute propagation delay
6. Display results and validate formatting

---

## Identified RTL / Math Functions

| Address      | Function         | Notes                                    |
|--------------|------------------|------------------------------------------|
| `0x008675ac` | `Math_Ln`        | Natural logarithm (FYL2X-based)          |
| `0x008673c0` | `Math_Sqrt`      | Square root                              |
| `0x00868870` | `Math_Exp`       | Exponential e^x (f2xm1 + fscale)        |
| `0x00867350` | `Math_Pow`       | General power x^y (FYL2X + 2^x)         |
| `0x00866d60` | `Math_Cosh`      | Hyperbolic cosine (e^x + e^-x)/2         |
| `0x00866e5c` | `Math_Exp2`      | Exponential variant (calls 0x00866ed5)   |
| `0x006f61a0` | `Math_Atan`      | Arctangent (FPATAN instruction)          |
| `0x00866d20` | `Math_Cos`       | Cosine (FCOS instruction)                |
| `0x008687f4` | `Math_Sin`       | Sine (FSIN instruction)                  |
| `0x00868834` | `Math_Abs`       | Absolute value (FABS-like)               |
| `0x00861e48` | `Format_Float`   | Format float for display                 |
| `0x0086ecf0` | `FPU_GetResult`  | Get FPU result from stack                |
| `0x0086ee90` | `FPU_Cleanup`    | Clean up FPU stack entry                 |
| `0x0086ecc0` | `FPU_Prepare`    | Prepare FPU format string                |
| `0x0086f0f4` | `StrToFloat`     | String to float conversion               |
| `0x0086ef50` | `FPU_Validate`   | Validate float string                    |
| `0x00403638` | `ClearResult`    | Clear result variable                    |
| `0x005396c8` | `TEdit_SetText`  | Set TEdit control text                   |
| `0x00539678` | `TEdit_GetText`  | Get TEdit control text                   |
| `0x005dff68` | `ShowMessage`    | Display error message                    |
| `0x0071d964` | `Format_String`  | Format string with params                |

---

## Mode 3: Edge Coupled External (`Solver_EdgeCoupledExternal`)

**Address:** `0x00422ddc` (16,980 bytes)

### Constants

| Address      | Value    | Type   | Identified Meaning                      |
|--------------|----------|--------|-----------------------------------------|
| `0x00427030` | 32.0     | f32    | Fahrenheit offset (temp conversion)     |
| `0x00427034` | 0.5556   | f64    | 5/9 (Celsius conversion factor)         |
| `0x0042703c` | 0.001    | f64    | mm to meters                            |
| `0x00427044` | 25.4     | f64    | mm per inch                             |
| `0x0042704c` | 1000.0   | f32    | mils per inch / unit scaling            |
| `0x00427050` | 20.0     | f32    | Display scaling                         |
| `0x00427054` | 10.0     | f32    | Display scaling                         |
| `0x00427058` | 1.0      | f32    | Unity                                   |
| `0x0042705c` | 1.0e-6   | f64    | Micro prefix                            |
| `0x00427064` | 1.0e6    | f32    | Mega prefix                             |
| `0x00427068` | 1.0e9    | f32    | Giga prefix                             |
| `0x0042706c` | 57.2958  | ext10  | 180/pi (radians to degrees)             |
| `0x00427078` | 0.01745  | f64    | pi/180 (degrees to radians)             |
| `0x00427080` | 57.2958  | f64    | 180/pi (duplicate)                      |
| `0x00427088` | 1.8      | f64    | Fahrenheit per Celsius degree           |

### Global Variables Used

| Address        | Meaning (reconstructed)                      |
|----------------|----------------------------------------------|
| `_DAT_008d6340`| Input: Trace width (W) in mils               |
| `_DAT_008d6338`| Trace width converted to mm                  |
| `_DAT_008d6328`| Input: Dielectric height (H) in raw units    |
| `_DAT_008d6320`| Dielectric height in mm                       |
| `_DAT_008d64d0`| Zodd in mils (H * 1000)                      |
| `_DAT_008d64e0`| Zeven (H * 0.001 * 25.4 * 1000)              |
| `_DAT_008d6628`| Input: Er (relative permittivity)             |
| `_DAT_008d6148`| Input: frequency                              |
| `_DAT_008d6630`| ln(Er) - natural log of Er                    |
| `_DAT_008d6650`| Er_eff input                                  |
| `_DAT_008d6648`| Er_eff_coupled = Er_eff + Er - frequency      |
| `_DAT_008d6150`| Propagation delay (scaled)                    |
| `_DAT_008d6658`| Coupling spacing factor                       |
| `_DAT_008d6660`| Coupling coefficient = spacing * ln(Er)       |
| `_DAT_008d66a8`| Skin depth input A                            |
| `_DAT_008d66b0`| Skin depth input B                            |
| `_DAT_008d66b8`| Skin effect result = abs(A*B)                 |
| `_DAT_008d66c0`| Propagation delay = 57.2958 * angle           |
| `_DAT_008d6700`| Attenuation coefficient A                     |
| `_DAT_008d6708`| Attenuation coefficient B                     |
| `_DAT_008d66f0`| Conductor loss = cos(phi) * A                 |
| `_DAT_008d66f8`| Dielectric loss = sin(phi) * A                |

### Forward Solve (mode == 0) Flow

```
// Read inputs
W = ReadUI(param_1 + 0x800)                 // trace width
W_mm = (W - 32.0) * 0.5556                  // convert (Fahrenheit -> Celsius pattern)

H = ReadUI(param_1 + 0x7a8)                 // dielectric height
H_mm = H * 0.001 * 25.4                     // convert to mm

// Compute Zodd and Zeven
Zodd  = H / 1000.0                           // impedance in some unit
Zeven = H * 0.001 * 25.4 * 1000.0           // impedance scaled

// Read Er and frequency
Er       = ReadUI(param_1 + 0xb24)
freq     = ReadUI(param_1 + 0x1520)
ln_Er    = ln(Er)

// Er effective for coupled lines
Er_eff_in = ReadUI(param_1 + 0xb34)
Er_eff    = Er_eff_in + Er - freq

// Propagation delay with unit scaling
prop_delay = ln(something) / 1000.0
if prop_delay < 1.0:
    if prop_delay >= 0.001:
        prop_delay = prop_delay * 1000.0     // convert to ns/in or ps/in
    elif prop_delay >= 1e-6:
        prop_delay = prop_delay * 1e6        // convert to ps/m
    else:
        prop_delay = prop_delay * 1e9        // convert to fs scale

// Coupling coefficient
coupling_spacing = ReadUI(param_1 + 0xb44)
coupling_coeff   = coupling_spacing * ln_Er

// Skin effect (checkbox at param_1 + 0xcf0)
if checkbox_unchecked:
    skin_A = ReadUI(param_1 + 0xcb0)
    skin_B = ReadUI(param_1 + 0xcb4)
    skin_effect = abs(skin_A * skin_B)                    // Math_Abs
    conductor_loss = 57.2958 * angle                      // DAT_0042706c * arctan result

if checkbox_checked:
    atten_A = ReadUI(param_1 + 0xcb0)
    atten_B = ReadUI(param_1 + 0xcb4)
    conductor_loss = cos(phi) * atten_A                   // Math_Cos
    dielectric_loss = sin(phi) * atten_A                  // Math_Sin

// Surface roughness (checkbox at param_1 + 0xcec)
if roughness_unchecked:
    roughness_input = ReadUI(param_1 + 0xcd8)
    roughness_result = roughness_input  // direct display

if roughness_checked:
    roughness_input = ReadUI(param_1 + 0xcd8)
    roughness_result = roughness_input  // with correction
```

### Reverse Solve (mode == 1) Flow

The reverse solve reads impedance values and back-calculates geometry.
The key formula inversions are:

```
H = ReadUI(param_1 + 0x7a8)
H_mm = (H / 25.4) * 1000.0                  // reverse conversion

W = ReadUI(param_1 + 0x800)
W_mm = 1.8 * W + 32.0                       // 0x427088 * W + 0x427030

Zodd_rev  = H * 1000.0
Zeven_rev = H / 25.4
```

---

## Mode 4: Edge Coupled Internal Symmetric (`Solver_EdgeCoupledIntSym`)

**Address:** `0x004435f8` (116,071 bytes -- Ghidra decompiler fails due to size)

Analysis performed via disassembly of key math sections.

### Constants

| Address      | Value       | Type   | Identified Meaning                          |
|--------------|-------------|--------|---------------------------------------------|
| `0x0045fb60` | 1.0         | f32    | Unity                                       |
| `0x0045fb64` | 1000.0      | f32    | Mils per inch                               |
| `0x0045fb68` | 25.4        | f64    | mm per inch                                 |
| `0x0045fb70` | 10.0        | f32    | Unit conversion                             |
| `0x0045fb74` | 12.0        | f32    | Hammerstad constant (12*H/W term)           |
| `0x0045fb78` | 2.0         | f32    | Factor of 2 in impedance formulas           |
| `0x0045fb7c` | 0.04        | f64    | Thickness correction factor                 |
| `0x0045fb84` | 11.8        | f64    | Related to eta_0/(2*pi) = 60/pi ~ 19.1     |
| `0x0045fb8c` | 1.41        | f64    | sqrt(2) approximation                       |
| `0x0045fb94` | 87.0        | f32    | Z0 for stripline (87 * ln(...) formula)     |
| `0x0045fb98` | 5.98        | f64    | Stripline geometry constant                 |
| `0x0045fba0` | 0.8         | f64    | Coupling correction exponent                |
| `0x0045fba8` | -0.96       | f64    | Coupling correction coefficient             |
| `0x0045fbb0` | 0.48        | f64    | Coupling correction offset                  |
| `0x0045fbb8` | 20.0        | f32    | Scaling factor                              |
| `0x0045fbbc` | 60.0        | f32    | eta_0/2pi = 60 ohms (stripline Z0 base)     |
| `0x0045fbc0` | 2.1049      | f64    | Geometry correction constant                |
| `0x0045fbc8` | 4.0         | f32    | Factor in ln(4*...) terms                   |
| `0x0045fbcc` | -2.9        | f64    | Coupling coefficient                        |
| `0x0045fbd4` | 0.374       | f64    | Coupling geometry factor                    |
| `0x0045fbdc` | 0.35        | f64    | Default copper thickness (mils)             |
| `0x0045fbe4` | 1.023       | f64    | Impedance correction factor                 |
| `0x0045fbec` | 1.0235      | f64    | Impedance correction factor (alternate)     |
| `0x0045fbf4` | 0.5008      | f64    | Half-factor with correction                 |
| `0x0045fbfc` | 1.1564      | f64    | Geometry correction                         |
| `0x0045fc04` | 0.4749      | f64    | Coupling exponent                           |

### Copper Thickness Lookup (same as stripline modes)

| Index | Value (mils) | Equivalent         |
|-------|-------------|---------------------|
| 0     | 0.35        | 1/4 oz copper       |
| 1     | 0.70        | 1/2 oz copper       |
| 2     | 1.40        | 1 oz copper         |
| 3     | 2.10        | 1.5 oz copper       |
| 4     | 2.80        | 2 oz copper         |
| 5     | 3.50        | 2.5 oz copper       |
| 6     | 4.20        | 3 oz copper         |
| 7     | 5.60        | 4 oz copper         |
| 8     | 7.00        | 5 oz copper         |

### Reconstructed Formulas (from disassembly at `0x00443d80`-`0x004452c9`)

The solver computes Zodd and Zeven for edge-coupled striplines using the
Cohn/Garg-Bahl model. The math was traced from x87 FPU instruction sequences.

#### Unit Conversion

All geometry inputs undergo unit conversion:
```
val_mm = val_input / 1000.0 * 25.4 / 10.0
```
This converts from 0.1-mil units to mm (via: `/ 1000 * 25.4 / 10`).

Applied to: trace width (`g_6064`), spacing (`g_603c`), height (`g_6034`),
copper thickness (`g_6044`).

#### Er_effective Calculation

```
// g_6128 = Er_eff (result stored to UI at param_1 + 0xbf0)
ratio = W / H                               // g_6064 / g_6034

if ratio >= 1.0:
    // Wide trace: Hammerstad-Jensen formula, W/H >= 1
    Er_eff = ln(12.0 * ratio + 1.0) * (Er - 1.0) / 2.0 + (Er + 1.0) / 2.0

else:
    // Narrow trace: W/H < 1, includes (1-W/H)^2 correction
    temp = ln(12.0 * ratio + 1.0) * (Er - 1.0) / 2.0 + (Er + 1.0) / 2.0
    correction = (1.0 - H/W) * 0.04          // 0x0045fb7c
    Er_eff = temp + correction
```

The above is stored to `g_6128` (address `0x008d6128`).

#### Impedance Z0 (Uncoupled)

```
// g_6138 = Z0 uncoupled stripline impedance
Z0 = 11.8 / abs(Er_eff + 1.41) * (t/H) / 2.0 * 1000.0
```
Where `11.8` and `1.41` are from the stripline impedance formula:
```
Z0 = (60 / sqrt(Er_eff)) * ln(4*H / (pi * We))
```
The constant `11.8` derives from `60 / (2*pi) * 4 = 11.8` (approximately).

#### Coupling Coefficient (Er_eff for coupled modes)

```
// g_604c = Er_eff for even/odd mode coupling
coupling_factor = abs(Er_eff + 1.41) / 87.0
                  * pow(5.98 * W / (0.8 * H + spacing), exponent)
```

The `87.0` constant (`0x0045fb94`) corresponds to the well-known stripline formula:
```
Z0 = 87 / sqrt(Er + 1.41) * ln(5.98 * H / (0.8 * W + T))
```

#### Even/Odd Mode Coupling

```
// Coupling factor Cf
Cf = exp(-0.96 * S/W) * 0.48 - 1.0          // using c_a8, c_b0, c_60
Kb_coupling = 2.0 * Cf * coupling_factor

// g_605c = differential coupling adjustment
// Stored as: (1.0 - exp(-0.96 * S/W) * 0.48) * 2.0 * coupling_factor
```

#### Zodd and Zeven

```
// g_5f9c = Z0 / 2 (half-impedance reference)
Z_half = coupling_coeff / 2.0

// g_5fa4 = coupling parameter from ln ratio
coupling_param = ln(something) / Z_half

// g_5f8c = Kb (backward coupling coefficient)
//   Kb = (Zodd + 2*coupling_param*Z_half/Er_eff) / (Zodd - Zeven)
//        ... simplified to:
Kb = (Zeven + 2.0 * coupling_param * Z_half / Er_coupled) / (Zeven - Z_half)

// g_5f94 = Kb (forward coupling coefficient)
Kb_fwd = (Zeven - Z_half) / (Zeven + Z_half)

// g_5fac = Zdiff (differential impedance)
Zdiff = sqrt(Kb / 1.0) * 20.0               // c_b8 = 20.0
```

### Identified Output Variables

| Global Address   | UI Control Offset | Physical Meaning          |
|-----------------|-------------------|---------------------------|
| `g_6128`        | `0xbf0`           | Er_effective              |
| `g_6138`        | next              | Z0 (uncoupled impedance)  |
| `g_604c`        | next              | Er_eff coupled            |
| `g_605c`        | next              | Coupling factor           |
| `g_61e0`        | next              | H/W ratio                 |
| `g_61e8`        | next              | S/W ratio                 |
| `g_5f9c`        | next              | Z0/2                      |
| `g_5fa4`        | next              | Coupling parameter        |
| `g_5f8c`        | next              | Kb (coupling coefficient) |
| `g_5f94`        | next              | Kb forward                |
| `g_5fac`        | next              | Zdiff                     |

---

## Mode 5: Edge Coupled Internal Asymmetric (`Solver_EdgeCoupledIntAsym`)

**Address:** `0x004066b0` (6,842 bytes)

### Constants

| Address      | Value       | Type   | Identified Meaning                         |
|--------------|-------------|--------|--------------------------------------------|
| `0x0040816c` | 0.001       | f64    | mm to meters                               |
| `0x00408174` | 25400.0     | f32    | Unit conversion (mils to um, 25.4 * 1000)  |
| `0x00408178` | 9128.0      | f32    | Speed-related constant                     |
| `0x0040817c` | 0.6208      | f64    | Impedance formula coefficient              |
| `0x00408184` | 1000.0      | f32    | Mils per inch                              |
| `0x00408188` | 6.28318     | f64    | 2*pi                                       |
| `0x00408190` | 3.14159     | f64    | pi                                         |
| `0x00408198` | 25.4        | f64    | mm per inch                                |
| `0x004081a0` | 645.16      | f64    | ~ 20/pi * 100 = impedance scale            |
| `0x004081a8` | 1550.0      | f32    | Frequency/wavelength constant              |

### Reconstructed Formulas

The asymmetric solver handles two dielectric heights (H1, H2) and computes
impedance for non-symmetric stripline geometries.

#### Mode Selection

The solver has an inner mode selector at `param_1 + 0xaac + 0x2f0`:
- Mode 0 or 1: Standard geometry input (read H1, H2, Er)
- Mode 2: Impedance-first input (read Z, compute geometry)

#### Forward Solve (mode 0, inner modes 0/1)

```
// Read inputs
H1 = ReadUI(param_1 + 0xad4) * 0.001        // upper dielectric * DAT_0040816c
H2 = ReadUI(param_1 + 0xad8) * 0.001        // lower dielectric * DAT_0040816c
Er = ReadUI(param_1 + 0xacc)                  // relative permittivity

// Compute geometric ratios
ratio = H1 / H2 * Er
product = H1 * H2

// Impedance formula
Z_factor = (product / 25400.0) * 9128.0 * 0.6208 * 1000.0
//        = (H1*H2 / DAT_00408174) * DAT_00408178 * DAT_0040817c * (double)DAT_00408184

// Natural log term
ln_term = ln(H1 * H2)

// Combined impedance
Z0 = H1 * H2 * ln_term                       // g_65f8
```

Display result to `param_1 + 0xae4` (impedance) and `param_1 + 0xaec` (secondary).

#### Inner Mode 2 (Impedance-first)

```
Er = ReadUI(param_1 + 0xacc)
W  = ReadUI(param_1 + 0xad4)
H  = ReadUI(param_1 + 0xad8)

// Uses FUN_00866e5c (Math_Exp2) for exponential computation
// Z = W * 0.001 * exp2(...) * 1000.0
Z_result = W * DAT_0040816c * exp_result * (double)DAT_00408184

spacing = Z_result - W
```

#### Reverse Solve (mode 1)

For reverse solve, additional unit handling is required:
```
// Check units checkbox at PTR__Form6_008d5480 + 0x490
if units_mm:
    H1 = ReadUI(param_1 + 0xad4) / DAT_00408198         // divide by 25.4
if units_mil:
    H1 = ReadUI(param_1 + 0xad4) / (1000.0 * 25.4)

Er = ReadUI(param_1 + 0xacc) * DAT_004081a0              // * 645.16

// Impedance ratio computation
ratio = H1 / H2 * Er
Z_factor = product / 25400.0 * 9128.0 * 0.6208 * 1000.0

// Coupling computation using ln and pi
ln_coupled = ln(something)
Z_coupled = (H1 * H2 * ln_coupled) / 1550.0              // DAT_004081a8

// Differential computation
coupling = ln(val1) * pi - ln(val2) * pi                  // DAT_00408190
```

The `pi` constant (`DAT_00408190 = 3.14159`) appears in the coupling
coefficient calculation, consistent with the stripline formula:
```
Z0 = (eta_0 / (2 * sqrt(Er))) * (1/pi) * ln(...)
```
where `eta_0 = 376.73 ohms = 120*pi`.

---

## Mode 6: Edge Coupled Embedded (`Solver_EdgeCoupledEmbedded`)

**Address:** `0x00403f40` (9,840 bytes)

This is the most complex of the four solvers, implementing the embedded
(buried) edge-coupled microstrip model.

### Constants

| Address      | Value       | Type   | Identified Meaning                          |
|--------------|-------------|--------|---------------------------------------------|
| `0x004065b0` | 1000.0      | f32    | Mils per inch                               |
| `0x004065b4` | 25.4        | f64    | mm per inch                                 |
| `0x004065bc` | 1.0         | f32    | Unity                                       |
| `0x004065c0` | 6.517       | f64    | Geometry constant (Wheeler/IPC approx)      |
| `0x004065c8` | 10.8731     | f64    | Geometry constant                           |
| `0x004065d0` | 3.14159     | f64    | pi                                          |
| `0x004065d8` | 2.0         | f32    | Factor of 2                                 |
| `0x004065dc` | 2704.0      | f32    | ~ (60*ln(10))^2 frequency const             |
| `0x004065e0` | 0.432       | f64    | Coupling geometry exponent                  |
| `0x004065e8` | 49.0        | f32    | Impedance base (modified Wheeler)           |
| `0x004065ec` | 18.1        | f64    | Geometry factor                             |
| `0x004065f4` | 18.7        | f64    | Geometry factor                             |
| `0x004065fc` | 0.9         | f64    | Correction factor                           |
| `0x00406604` | 3.0         | f32    | Factor                                      |
| `0x00406608` | 0.564       | f64    | Hammerstad coupling coefficient             |
| `0x00406610` | -1.0        | f32    | Sign inversion                              |
| `0x00406614` | 10.0        | f32    | Scaling                                     |
| `0x00406618` | 30.666      | f64    | eta_0/(4*pi) ~ 30 ohms (free space)        |
| `0x00406620` | 0.28318     | f64    | Fractional part of pi-related constant      |
| `0x00406628` | 6.0         | f32    | Factor                                      |
| `0x0040662c` | 4.0         | f32    | Factor in ln(4*...) terms                   |
| `0x00406630` | 376.9908    | f64    | **eta_0 = 120*pi** (free-space impedance)   |
| `0x00406638` | 6.28318     | f64    | **2*pi**                                    |
| `0x00406640` | 0.0157      | f64    | ~ pi/200 (percent to radians)               |
| `0x00406648` | 0.525       | f64    | Coupling coefficient A                      |
| `0x00406650` | 0.6315      | f64    | Coupling coefficient B                      |
| `0x00406658` | 0.27488     | f64    | Coupling coefficient C                      |
| `0x00406660` | -8.7513     | f64    | Coupling coefficient D (negative)           |
| `0x00406668` | 0.065683    | f64    | Coupling coefficient E                      |
| `0x00406670` | -0.03442    | f64    | Coupling coefficient F (negative)           |
| `0x00406678` | 0.33622     | f64    | Coupling coefficient G                      |
| `0x00406680` | 38.7        | f64    | Impedance scaling constant                  |
| `0x00406688` | -4.6        | f64    | Impedance offset                            |
| `0x00406690` | 0.0363      | f64    | Correction factor                           |
| `0x00406698` | 15.916      | f64    | Impedance formula constant                  |
| `0x004066a0` | 2.751       | f64    | Embedded correction factor                  |
| `0x004066a8` | 0.1844      | f64    | Embedded correction factor                  |

### Copper Thickness Lookup

**Mils mode** (standard selection):

| Index | Value (mils) | Copper Weight |
|-------|-------------|---------------|
| 0     | 0.35        | 1/4 oz        |
| 1     | 0.70        | 1/2 oz        |
| 2     | 1.40        | 1 oz          |
| 3     | 2.10        | 1.5 oz        |
| 4     | 2.80        | 2 oz          |
| 5     | 3.50        | 2.5 oz        |
| 6     | 4.20        | 3 oz          |
| 7     | 5.60        | 4 oz          |
| 8     | 7.00        | 5 oz          |

**mm mode** (metric selection, 1 mil = 0.0254 mm):

| Index | Value (mm)  |
|-------|-------------|
| 0     | 0.009       |
| 1     | 0.018       |
| 2     | 0.035       |
| 3     | 0.053       |
| 4     | 0.070       |
| 5     | 0.088       |
| 6     | 0.106       |
| 7     | 0.142       |
| 8     | 0.178       |

**Plating thickness** follows a similar pattern (stored at `_DAT_008d6668`).

### Reconstructed Formulas

The embedded coupled microstrip uses a modified IPC-2141/Wheeler model with
coupling corrections from Kirschning-Jansen.

#### Total Thickness and Height Setup

```
copper_t = _DAT_008d61b8                     // copper thickness (from lookup)
plating  = _DAT_008d6668                     // plating thickness (from lookup)
total_t  = plating + copper_t                 // _DAT_008d603c (total conductor height)

// Read dielectric parameters
H_raw    = ReadUI(param_1 + 0xd8c)           // dielectric height
H_mm     = (H_raw / 1000.0) * 25.4           // convert to mm

cover_H  = _DAT_008d603c / 1000.0 * 25.4     // total height in mm

Er       = ReadUI(param_1 + 0x67c)           // dielectric constant
if Er <= 1.0:
    ShowMessage("Er must be > 1.0")

// Additional geometry inputs
embed_H  = ReadUI(param_1 + 0xd90)           // embedded height
spacing  = ReadUI(param_1 + 0xdbc)           // trace spacing
```

#### Er Effective for Embedded Microstrip

```
// Intermediate ratios
S_over_H = cover_H / embed_H                 // g_6720
W_over_H = H_mm / embed_H                    // g_6730

// Logarithmic terms used throughout
ln1 = ln(S_over_H)
exp1 = exp(ln1)                              // = S_over_H (identity through ln+exp)
ln2 = ln(W_over_H)

// Er effective base
pow_result = pow(something)                  // Math_Pow for fractional exponents

g_6728 = (exp1 * S_over_H) / pi + W_over_H

// cos-based correction for embedded geometry
cos_result = cos(something)                  // Math_Cos -> FUN_00866d60 (cosh used)

g_6738 = ((g_6728 - W_over_H) * (1.0 / Er + 1.0)) / 2.0 + W_over_H
```

#### Impedance Computation

The core impedance formula uses multiple correction terms:

```
// Base impedance components
ln3 = ln(W_over_H)
ln4 = ln(S_over_H)
ln5 = ln(Er)

pow1 = pow(ln3)
pow2 = pow(ln4)

g_6740 = embed_H / 49.0 + 1.0 + something / 18.7
//        (from DAT_004065e8 = 49.0, DAT_004065f4 = 18.7)

g_6748 = exp(ln5) * 0.564                   // coupling via DAT_00406608
g_6768 = g_6740 * g_6748 * (-1.0)           // sign inversion via DAT_00406610

// Er_effective for coupled line
g_6760 = (Er + 1.0) / 2.0 + ((Er - 1.0) / 2.0) * exp(ln_correction)

// Impedance correction factors
g_6800 = exp(ln_a) * (-1.0)                 // via DAT_00406610
g_67f8 = exp(ln_b) * (-1.0)

// Free-space impedance terms using eta_0
// eta_0 = 376.9908 ohms (DAT_00406630)
// 2*pi = 6.28318 (DAT_00406638)
g_67e8 = (exp(ln_c) * 376.9908) / 6.28318   // = Z0_freespace_term
g_67f0 = (exp(ln_d) * 376.9908) / 6.28318

g_67e0 = g_67f0 / exp(something)             // ratio correction
```

#### Coupling Coefficient Polynomial

The coupling between traces uses a polynomial fit with 6 coefficients:

```
// Coupling polynomial: f(u) = (A*u^2 + B)*W_over_H + C - D*u
// where u = ln(S_over_H) related parameter
A = 0.525     // DAT_00406648
B = 0.6315    // DAT_00406650
C = 0.27488   // DAT_00406658
D = -8.7513   // DAT_00406660
E = 0.065683  // DAT_00406668
F = -0.03442  // DAT_00406670

g_6780 = ((pow_a * A + B) * W_over_H + C) - pow_b * E

// Additional coupling terms
g_6788 = (1.0 - pow_c) * 0.33622            // DAT_00406678
g_6790 = pow_d * 0.0363 * (1.0 - pow_e)     // DAT_00406690
g_6798 = (1.0 - pow_f) * 2.751 + 1.0        // DAT_004066a0
```

#### Final Impedance Assembly

```
// Combine coupling effects
g_67a0 = pow_g * g_6788 * g_6780            // composite coupling factor

// Corrected Er_effective
g_67c0 = Er - (Er - g_6778) / (1.0 + g_67a0)
// This is the IPC-2141 embedded correction:
// Er_eff_embedded = Er - (Er - Er_eff_surface) / (1 + correction)

// Final differential impedance
g_6808 = ((g_67c0 - 1.0) / (g_6778 - 1.0)) * exp(ln_ratio) * g_67e0
// Displayed at param_1 + 0xdc4
```

### Physical Interpretation

The constants identify this as implementing the **IPC-2141** embedded microstrip
coupled-line model, with corrections from:

1. **Hammerstad-Jensen** (1980): Base Er_effective formula
   - Constants: 12.0, 0.04 (thickness correction)

2. **Kirschning-Jansen** coupling model:
   - Coupling coefficients: 0.525, 0.6315, 0.27488, -8.7513, 0.065683, -0.03442
   - These match published Kirschning-Jansen polynomial fits

3. **Wheeler-Schneider** impedance:
   - Free-space impedance: eta_0 = 376.99 = 120*pi
   - Base constants: 6.517, 10.873, 49.0, 18.1, 18.7, 0.564, 30.666

4. **IPC-2141** embedded correction:
   - `Er_eff_embed = Er - (Er - Er_eff) / (1 + f(geometry))`
   - Correction factor `f` is the polynomial coupling term

---

## Cross-Mode Comparison

### Common Patterns

All four modes share:
1. **UI pattern**: Read from TEdit -> convert string to float -> compute -> format -> write to TEdit
2. **Unit conversion**: `value / 1000.0 * 25.4` (mils to mm) or similar
3. **Bidirectional solve**: mode 0 = forward, mode 1 = reverse
4. **Er validation**: Er must be > 1.0 (show error otherwise)

### Key Output Fields

| UI Control Offset | Physical Quantity        | Modes  |
|-------------------|--------------------------|--------|
| `0x7fc` / `0x800` | Trace Width              | 3      |
| `0x7a4` / `0x7a8` | Dielectric Height        | 3      |
| `0x9a8`           | Propagation Delay        | 3, 6   |
| `0xa30` / `0xa34` | Zodd / Zeven             | 3      |
| `0xb20`           | ln(Er)                   | 3      |
| `0xb24` / `0xb34` | Er / Er_eff              | 3      |
| `0xb38`           | Er_eff coupled           | 3      |
| `0xb44` / `0xb4c` | Coupling spacing/coeff   | 3      |
| `0xbf0`           | Er_effective             | 4, 6   |
| `0xcb0` / `0xcb4` | Skin effect inputs       | 3      |
| `0xcb8` / `0xcbc` | Attenuation results      | 3      |
| `0xcd8` / `0xcdc` | Roughness input/result   | 3      |
| `0xd8c`           | Dielectric height        | 6      |
| `0xd90`           | Embedded height          | 6      |
| `0xd9c`           | Impedance result         | 6      |
| `0xdbc`           | Trace spacing            | 6      |
| `0xdc4`           | Diff impedance result    | 6      |
| `0xad4` / `0xad8` | H1 / H2 (asymmetric)    | 5      |
| `0xacc`           | Er                       | 5      |
| `0xae4` / `0xaec` | Impedance results        | 5      |
| `0xe24`           | Secondary display        | 4, 6   |

### Formula Summary Table

| Mode | Topology                  | Base Formula                                      | Reference            |
|------|---------------------------|---------------------------------------------------|----------------------|
| 3    | Edge Coupled External     | Microstrip coupled-line (ln-based, Hammerstad)    | Hammerstad-Jensen 80 |
| 4    | Edge Coupled Int Sym      | Stripline coupled `Z0=87/sqrt(Er+1.41)*ln(5.98H/(0.8W+T))` | Cohn/Wadell |
| 5    | Edge Coupled Int Asym     | Asymmetric stripline `Z0 = f(H1,H2,Er)` with pi  | Cohn/Shelton         |
| 6    | Edge Coupled Embedded     | Embedded microstrip with IPC-2141 correction      | IPC-2141/Kirschning  |

---

## Notable Constants Cross-Reference

| Constant      | Mode 3         | Mode 4         | Mode 6         | Physical Meaning      |
|---------------|----------------|----------------|----------------|-----------------------|
| eta_0         | --             | --             | 376.991        | Free-space impedance  |
| 2*pi          | --             | --             | 6.28318        | 2*pi                  |
| pi            | --             | --             | 3.14159        | pi                    |
| 25.4          | 0x427044       | 0x45fb68       | 0x4065b4       | mm per inch           |
| 1000.0        | 0x42704c       | 0x45fb64       | 0x4065b0       | mils per inch         |
| 1.0           | 0x427058       | 0x45fb60       | 0x4065bc       | unity                 |
| 0.001         | 0x42703c       | --             | --             | mm to m               |
| 12.0          | --             | 0x45fb74       | --             | Hammerstad 12*H/W     |
| 87.0          | --             | 0x45fb94       | --             | Stripline Z0 base     |
| 0.04          | --             | 0x45fb7c       | --             | Thickness correction  |
| 60.0          | --             | 0x45fbbc       | --             | eta_0/(2*pi)          |

---

## References

The formulas implemented in these solvers are consistent with:

1. E. Hammerstad and O. Jensen, "Accurate Models for Microstrip Computer-Aided Design,"
   IEEE MTT-S Int. Microwave Symp. Dig., 1980, pp. 407-409.

2. S.B. Cohn, "Characteristic Impedance of Shielded-Strip Transmission Line,"
   IRE Trans. Microwave Theory Tech., vol. MTT-2, July 1954.

3. R. Garg and I.J. Bahl, "Characteristics of Coupled Microstriplines,"
   IEEE Trans. Microwave Theory Tech., vol. MTT-27, No. 7, July 1979.

4. M. Kirschning and R.H. Jansen, "Accurate Wide-Range Design Equations for the
   Frequency-Dependent Characteristic of Parallel Coupled Microstrip Lines,"
   IEEE Trans. Microwave Theory Tech., vol. MTT-32, No. 1, Jan 1984.

5. IPC-2141A, "Design Guide for High-Speed Controlled Impedance Circuit Boards," 2004.

6. B.C. Wadell, "Transmission Line Design Handbook," Artech House, 1991.
