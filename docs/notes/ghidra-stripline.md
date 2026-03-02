# Saturn PCB Toolkit - Stripline Impedance Solver (FUN_0040bc00)

## Overview

The stripline impedance solver at `FUN_0040bc00` is the Mode 1 handler in the main
dispatcher (`FUN_00403398`). It is a very large function (~28KB, from `0x0040bc00` to
`0x00422bd7`) that could not be decompiled by Ghidra due to its size. This analysis
is based on manual disassembly and FPU instruction tracing.

The function implements the **Cohn stripline impedance model** with corrections for
finite strip thickness, and computes impedance (Z0), propagation delay, capacitance,
inductance, skin resistance, and wavelength results.

---

## Sub-functions Called

### Called functions (all calls from cross-reference analysis)

| Address    | Function       | Purpose                                    |
|------------|----------------|--------------------------------------------|
| 0x00403398 | FUN_00403398   | Main dispatcher (recursive call for refresh)|
| 0x00403638 | FUN_00403638   | Clear/reset string result field             |
| 0x00511ed4 | FUN_00511ed4   | TEdit.SetEnabled (sets Enabled property)    |
| 0x0053957c | FUN_0053957c   | TControl.SetVisible                         |
| 0x00539678 | FUN_00539678   | TEdit.GetText (Delphi RTL)                  |
| 0x005396c8 | FUN_005396c8   | TEdit.SetText (Delphi RTL)                  |
| 0x005dff68 | FUN_005dff68   | ShowMessage (Delphi RTL)                    |
| 0x0071d964 | FUN_0071d964   | Format (Delphi RTL)                         |
| 0x0085f89f | FUN_0085f89f   | IntToStr wrapper                            |
| 0x00861e48 | FUN_00861e48   | FloatToStr (Delphi RTL)                     |
| 0x00866d60 | FUN_00866d60   | **cosh()** (hyperbolic cosine)              |
| 0x00866e5c | FUN_00866e5c   | **exp() with overflow check**               |
| 0x00866fe4 | FUN_00866fe4   | **abs()** (floating-point absolute value, nop in this build) |
| 0x00867350 | FUN_00867350   | **pow() / power function**                  |
| 0x008675ac | FUN_008675ac   | **ln() / natural logarithm**                |
| 0x008687f4 | FUN_008687f4   | **sin()** (sine function)                   |
| 0x00868834 | FUN_00868834   | **sqrt()** (square root)                    |
| 0x00868870 | FUN_00868870   | **exp()** (exponential function)            |
| 0x0086b424 | FUN_0086b424   | ResourceString loader (Delphi RTL)          |
| 0x0086ecc0 | FUN_0086ecc0   | String init (Delphi RTL)                    |
| 0x0086ecf0 | FUN_0086ecf0   | Stack string builder (Delphi RTL)           |
| 0x0086ee20 | FUN_0086ee20   | String concat + format (Delphi RTL)         |
| 0x0086ee90 | FUN_0086ee90   | String cleanup (Delphi RTL)                 |
| 0x0086ef50 | FUN_0086ef50   | String compare (Delphi RTL)                 |
| 0x0086f0f4 | FUN_0086f0f4   | StrToFloat (Delphi RTL)                     |

### Math Function Decompiled Code

#### FUN_00866d60 - cosh() (hyperbolic cosine)
```c
void __stdcall cosh(undefined4 param_1, undefined4 param_2)
{
  ushort uVar1;
  longdouble lVar2;
  longdouble lVar3;

  lVar2 = -(longdouble)1;
  uVar1 = param_2._2_2_ & 0x7fff;
  lVar3 = (longdouble)(double)CONCAT26(param_2._2_2_, CONCAT24((ushort)param_2, param_1));
  if (uVar1 < 0x4086) {
    if (uVar1 < 0x3f20) {
      fscale(lVar3 * lVar3, lVar2);  // x^2 / 2 for small x
      return;
    }
  }
  else if ((0x4086 < uVar1) || (0x33cd < (ushort)param_2)) {
    FUN_00866a00();  // overflow handler
    return;
  }
  FUN_00866ed5();  // compute exp(x)
  fscale((longdouble)1 / lVar3 + lVar3, lVar2);  // (exp(x) + exp(-x)) / 2
  return;
}
```

#### FUN_008687f4 - sin()
```c
void __stdcall sin_func(double param_1)
{
  if ((param_1._6_2_ & 0x7ff0) < 0x4340) {
    fsin((longdouble)param_1);  // x87 FSIN instruction
  }
  else {
    FUN_00866a00();  // overflow/error for very large args
  }
  return;
}
```

#### FUN_00868870 - exp()
```c
void __stdcall exp_func(double param_1)
{
  ushort uVar1;
  longdouble lVar2;

  uVar1 = param_1._6_2_ * 2;
  if (uVar1 < 0x8081) {
    if (uVar1 < 0x7f8c) {
      if (0x7bbf < uVar1) {
        lVar2 = (longdouble)fscale(ABS((longdouble)param_1), (longdouble)1);
        f2xm1((longdouble)1.4426950408889634 * lVar2);  // 1/ln(2) * x
      }
    }
    else {
      FUN_00866ed5();  // standard exp path
    }
  }
  return;
}
```

#### FUN_00866fe4 - abs() (no-op in this build)
```c
void __stdcall abs_noop(void)
{
  return;
}
```
Note: This is effectively `fabs()` -- the FPU likely already has the value on the stack
and the `FABS` instruction is used inline rather than in this function.

---

## Constants Table

All floating-point constants referenced from the `0x00422b*` - `0x00422d*` range.
The "type" column indicates whether the instruction loads it as a 4-byte float or 8-byte double.

| Address      | Type   | Value          | Identified As                                |
|--------------|--------|----------------|----------------------------------------------|
| 0x00422bd8   | float  | 1.0            | Unity constant                               |
| 0x00422bdc   | float  | 0.0            | Zero constant                                |
| 0x00422be0   | float  | 1000.0         | mil-to-inch (or mm-to-m) conversion          |
| 0x00422be4   | double | 25.4           | mil-to-mm conversion                         |
| 0x00422bec   | double | 6.517          | Cohn model constant (strip width scaling)    |
| 0x00422bf4   | double | 10.8731272     | 4*e (4 * Euler's number = 10.87313)          |
| 0x00422bfc   | double | 3.14159        | pi                                           |
| 0x00422c04   | float  | 2.0            | Constant 2                                   |
| 0x00422c08   | float  | 2704.0         | Unknown constant                             |
| 0x00422c0c   | double | 0.432          | Cohn fringe correction constant              |
| 0x00422c14   | float  | 49.0           | Unknown constant                             |
| 0x00422c18   | double | 18.1           | Cohn model constant                          |
| 0x00422c20   | double | 18.7           | Cohn model constant                          |
| 0x00422c28   | double | 0.9            | Cohn thickness correction                    |
| 0x00422c30   | float  | 3.0            | Constant 3                                   |
| 0x00422c34   | double | 0.564          | Cohn model constant (~ln(2)*0.814)           |
| 0x00422c3c   | float  | -1.0           | Negation constant                            |
| 0x00422c40   | float  | 10.0           | Constant 10                                  |
| 0x00422c44   | double | 30.666         | Stripline impedance constant (60/sqrt(Er) for Er~3.83)|
| 0x00422c4c   | double | 0.28318        | ~0.9/pi = 0.28648 (Cohn correction term)     |
| 0x00422c54   | float  | 6.0            | Constant 6                                   |
| 0x00422c58   | float  | 4.0            | Constant 4                                   |
| 0x00422c5c   | double | 376.9908       | ~120*pi = impedance of free space (ohms)     |
| 0x00422c64   | double | 6.28318        | 2*pi                                         |
| 0x00422c6c   | double | 0.0157         | Loss tangent or skin depth related           |
| 0x00422c74   | double | 0.525          | Cohn fringe field coefficient                |
| 0x00422c7c   | double | 0.6315         | Cohn coupling coefficient                    |
| 0x00422c84   | double | 0.27488        | Cohn coupling coefficient                    |
| 0x00422c8c   | double | -8.7513        | Cohn log-domain correction                   |
| 0x00422c94   | double | 0.065683       | Cohn correction factor                       |
| 0x00422c9c   | double | -0.03442       | Cohn correction factor                       |
| 0x00422ca4   | double | 0.33622        | Cohn correction factor                       |
| 0x00422cac   | double | 38.7           | ~120*pi / pi (stripline characteristic)      |
| 0x00422cb4   | double | -4.6           | Cohn log-domain coefficient                  |
| 0x00422cbc   | double | 0.0363         | Cohn correction                              |
| 0x00422cc4   | double | 15.916         | ~50/pi (stripline Z0 base)                   |
| 0x00422ccc   | double | 2.751          | Cohn correction coefficient                  |
| 0x00422cd4   | double | 0.1844         | Cohn correction coefficient                  |
| 0x00422cdc   | double | 1.1            | Thickness ratio correction                   |
| 0x00422ce4   | float  | 8.0            | Constant 8                                   |
| 0x00422ce8   | float  | 14.0           | Constant 14                                  |
| 0x00422cec   | float  | 11.0           | Constant 11                                  |
| 0x00422cf0   | float  | 16.0           | Constant 16                                  |
| 0x00422cf4   | double | 11.8           | Propagation delay constant                   |
| 0x00422cfc   | double | 0.1            | Convergence threshold / min ratio             |
| 0x00422d04   | float  | 15.0           | Max range check value                        |
| 0x00422d08   | double | -1.55          | exp() argument coefficient                   |
| 0x00422d10   | float  | 60.0           | Impedance of free space / 2pi factor         |
| 0x00422d14   | double | 0.8            | Thickness correction coefficient             |
| 0x00422d1c   | double | 5.98           | Cohn thickness correction (~6)               |
| 0x00422d24   | double | 0.08457        | ~1/(4*pi) = magnetic permeability related    |
| 0x00422d2c   | float  | 5.0            | Range check min value                        |
| 0x00422d30   | float  | 40.0           | Range check value                            |
| 0x00422d34   | float  | 90.0           | Range check max value                        |
| 0x00422d38   | double | 1.9            | Unknown correction                           |
| 0x00422d40   | double | 1.017          | Near-unity correction factor                 |
| 0x00422d48   | float  | 12.0           | Constant 12                                  |
| 0x00422d4c   | double | 0.35           | W/(W1+W2) ratio threshold                    |
| 0x00422d54   | double | 1.023          | Correction factor ~1                         |
| 0x00422d5c   | double | 1.0235         | Correction factor ~1                         |
| 0x00422d64   | double | 0.5008         | ~0.5 correction                              |
| 0x00422d6c   | double | 1.1564         | Correction factor                            |
| 0x00422d74   | double | 0.4749         | Correction factor                            |
| 0x00422d7c   | double | 2.71828        | e (Euler's number, Math.E)                   |
| 0x00422d84   | double | 376.991        | 120*pi (free space impedance eta_0, ohms)    |
| 0x00422d8c   | double | 6.27           | ~2*pi                                        |
| 0x00422d94   | float  | 0.5            | Constant 0.5                                 |
| 0x00422d98   | double | 0.102101675    | ~1/(2*pi*sqrt(Er)) related                   |
| 0x00422da0   | double | 0.079577538... | 1/(4*pi) = 0.07958 (permeability scaling)    |
| 0x00422da8   | double | 0.31831015...  | 1/pi = 0.31831 (inverse pi)                  |
| 0x00422db0   | float  | 80.0           | Unknown constant                             |
| 0x00422dbc   | double | 188.4954       | 60*pi = 188.496 (stripline Z0 factor)        |
| 0x00422dc4   | double | 2.54           | Conversion: 0.1 inch to mm                   |
| 0x00422dcc   | double | 0.1269         | Unknown correction                           |
| 0x00422dd4   | double | 0.3811         | Unknown correction                           |

---

## Global Variables

### Input Variables (read from form fields)

| Global Addr  | Form Field (EBX+offset) | Variable Name    | Description                         |
|--------------|-------------------------|------------------|-------------------------------------|
| 0x008d61b8   | EBX+0xa0c               | H1               | Dielectric height above strip (mils)|
| 0x008d6668   | EBX+0xa14               | H2               | Dielectric height below strip (mils)|
| 0x008d6054   | EBX+0x67c               | Er               | Dielectric constant (relative perm) |
| 0x008d6034   | EBX+0x694               | W                | Trace/strip width (mils)            |
| 0x008d6064   | EBX+0x698               | T                | Trace/strip thickness (mils)        |
| 0x008d67c8   | EBX+0xe14               | Rho              | Conductor resistivity (from field)  |
| 0x008d603c   | (computed: H1+H2)       | B                | Total ground plane spacing (mils)   |

### Computed Intermediate Variables

| Global Addr  | Variable Name      | Description                                          |
|--------------|--------------------|------------------------------------------------------|
| 0x008d61e0   | W_over_T           | W/T ratio                                            |
| 0x008d61e8   | (unnamed)          | Some ratio related to conductor spacing              |
| 0x008d6720   | b_norm             | B (normalized, in mm) / H (ground spacing / thickness)|
| 0x008d6730   | w_norm             | W (normalized, in mm) / W/B ratio                    |
| 0x008d6728   | W_eff_1            | Effective width (first correction)                   |
| 0x008d6738   | W_eff_2            | Effective width (second correction, Cohn model)      |
| 0x008d6740   | Cf_factor          | Fringe capacitance factor                            |
| 0x008d6748   | Er_correction      | Er-dependent correction factor                       |
| 0x008d6768   | Z0_uncorrected     | Impedance before final corrections                   |
| 0x008d6760   | geom_factor        | Geometric factor for loss calculation                |
| 0x008d67e8   | Z_num              | Numerator of impedance expression                    |
| 0x008d67f0   | Z_den              | Denominator of impedance expression                  |
| 0x008d67e0   | Z_ratio            | Z_num/Z_den intermediate                             |
| 0x008d6778   | loss_factor        | Loss-related geometric factor                        |
| 0x008d6780   | A_coeff            | Polynomial coefficient A                             |
| 0x008d6788   | B_coeff            | Polynomial coefficient B                             |
| 0x008d67b8   | C_coeff            | Polynomial coefficient C                             |
| 0x008d6790   | D_coeff            | Polynomial coefficient D                             |
| 0x008d6798   | E_coeff            | Polynomial coefficient E                             |
| 0x008d67a0   | F_coeff            | Combined correction factor                           |
| 0x008d67c0   | Z0_final           | Final characteristic impedance (ohms)                |
| 0x008d604c   | Z0_output          | Output impedance value (post-rounding)               |
| 0x008d6288   | skin_depth         | Skin depth or skin resistance value                  |
| 0x008d61c8   | C_per_length       | Capacitance per unit length                          |
| 0x008d6488   | L_per_length       | Inductance per unit length                           |

### Mode Selector

| Form Field (EBX+offset) | Purpose                                              |
|--------------------------|------------------------------------------------------|
| EBX+0x9b0               | Solve mode combo box (ItemIndex at +0x2f0)           |
| EBX+0x684               | Unit system selector                                 |
| EBX+0x990               | Sub-mode / configuration combo box                   |
| EBX+0x620               | Sub-mode / configuration combo box                   |
| EBX+0xc44               | Result display field (read-only status text)         |

---

## Function Structure

The function is organized as follows:

```
0x0040bc00 - 0x0040be87  : Prologue, initial field reads (units, first field group)
0x0040be88 - 0x0040c413  : Read all input fields (Er, W, T, H1, H2, Rho)
                            Validate inputs (Er >= 1.0, W > 0, T > 0)
                            Compute basic ratios (W/T, W/H, etc.)
                            Display W/T and W/H ratios

0x0040c414               : CMP [mode], 0  => Mode 0: Solve for Z0
0x0040c41b - 0x0040e388  :   MODE 0: Solve for impedance Z0
    0x0040c421 - 0x0040c689  : Unit conversions (mils->mm), compute B=H1+H2
    0x0040c689 - 0x0040ce38  : *** CORE IMPEDANCE CALCULATION (Cohn model) ***
    0x0040ce38 - 0x0040cfe0  : Display Z0, format output
    0x0040cfe1 - 0x0040d209  : Propagation delay, effective Er, capacitance
    0x0040d209 - 0x0040d7db  : Inductance, skin resistance, display all results
    0x0040d7db - 0x0040e388  : Range validation, display bounds, secondary results

0x0040e38e               : CMP [mode], 1  => Mode 1: Solve for Width (W)
0x0040e395 - 0x0040f818  :   MODE 1: Iterative solver for trace width
    0x0040e395 - 0x0040e618  : Read target Z0, read other params
    0x0040e619 - 0x0040e704  : Compute effective parameters for iteration
    0x0040e704 - 0x0040f818  : Newton-Raphson iteration, display results

0x0040f81e               : CMP [mode], 2  => Mode 2: Solve for Height (H)
0x0040f825 - 0x00410d41  :   MODE 2: Iterative solver for dielectric height

0x00410d47               : CMP [mode], 3  => Mode 3: Solve for Spacing (B)
0x00410d4e - 0x004118be  :   MODE 3: Solver for ground plane spacing

0x004118bf               : CMP [mode], 0  => Additional Mode 0 block
0x004118bf - 0x00412210  :   DIFFERENTIAL STRIPLINE COMPUTATION
    (Coupled stripline: computes Zodd, Zeven, Zdiff)

0x00412211               : CMP [mode], 1
0x00412218 - 0x004128af  :   Differential mode 1

0x004128b0               : CMP [mode], 2
0x004128b7 - 0x00412e63  :   Differential mode 2

0x00412e64               : CMP [mode], 4
0x00412e6b - 0x00413c26  :   Differential mode 4

0x00413c27               : CMP [mode], 5
0x00413c2e - 0x00414ccc  :   Differential mode 5

0x00414ccd - 0x00422bd7  : Alternative entry (unit system != 0)
                            Same computation with different unit scaling
```

---

## Core Impedance Formula (Mode 0: Solve for Z0)

### Reconstructed from FPU instruction trace at 0x0040c647 - 0x0040ce1c

The core impedance computation implements a modified Cohn model for symmetric stripline.
Below is the pseudocode reconstructed from the x87 FPU instruction stream.

#### Step 1: Unit Conversion (mils to mm)

```
W_mm  = [008d6034] / 1000.0 * 25.4     ; trace width in mm
T_mm  = [008d6064] / 1000.0 * 25.4     ; trace thickness in mm
B_mm  = [008d603c] / 1000.0 * 25.4     ; ground spacing in mm (H1+H2)
```
Stored at: `[EBP-0x10F0]`, `[EBP-0x10F8]`, `[EBP-0x1100]`

#### Step 2: Read Resistivity and Normalize

```
Rho = StrToFloat(field[0xe14]) / 1000.0   ; stored at [008d67c8]
b   = B_mm / T_mm                          ; stored at [008d6720]
w   = W_mm / T_mm                          ; stored at [008d6730]
```

#### Step 3: Effective Width (Cohn model first correction)

```
; At 0x0040c70a:
temp1 = exp(ln(6.517 * w))                ; = (6.517 * w)
temp2 = ln(...)                            ; logarithmic term
W_eff_1 = (ln(10.8731272 / b) * temp2 + 1.0) ^ b / pi + w
```

More precisely, from the instruction trace:
```
; 0x0040c70a - 0x0040c78e:
x1 = ln(6.517 * w)          ; [008675ac] call with 6.517*w
x2 = exp(x1)                ; [00868870] call - this is just 6.517*w recovered
x3 = ln(10.8731272 / b)     ; second ln call
W_eff_1 = (x3 * x2 + 1.0) ^ b / pi + w
```

Actual reconstructed formula:
```
W_eff_1 = ln(10.8731272/b) * exp(ln(6.517*w)) + 1.0
W_eff_1 = W_eff_1^(b) / pi + w        ; [008d6728]
```

#### Step 4: Cohn Thickness Correction

```
; 0x0040c794 - 0x0040c7e2:
cosh_term = cosh(ln(Er - 1.0))         ; = (Er - 1.0) since cosh(ln(x)) = (x+1/x)/2
; More accurately: 1/cosh(ln(Er-1)) + 1
correction = 1.0 / cosh(ln(Er - 1.0)) + 1.0
W_eff_2 = (W_eff_1 - w) * correction / 2.0 + w     ; [008d6738]
```

#### Step 5: Fringe Capacitance Factor (Cf)

```
; 0x0040c813 - 0x0040c922:
; Series of ln() and pow() calls computing Cf
; Three terms with W_eff_2 variations

term_a = ln(W_eff_2) + ln(W_eff_2)/2704.0     ; combined ln terms
term_b = (term_a + ln(W_eff_2) + 0.432)
term_c = term_b ^ (W_eff_2)                    ; pow(term_b, W_eff_2)
result = term_c / 49.0 + 1.0                   ; stored in extended precision

; Second correction with divisions by 18.1 and 18.7:
sub_term = W_eff_2 / 18.1
ln_sub = ln(sub_term) + 1.0
pow_sub = ln_sub ^ (...)
Cf = pow_sub / 18.7 + first_extended_result     ; [008d6740]
```

#### Step 6: Er Correction

```
; 0x0040c928 - 0x0040c968:
Er_corr = ln((Er - 0.9) / (Er + 3.0)) * 0.564     ; [008d6748]
Z0_uncorrected = Cf * Er_corr * (-1.0)              ; [008d6768]
```

#### Step 7: Geometric Factor for Loss

```
; 0x0040c980 - 0x0040c9d2:
geom = ln(10.0/W_eff_2 + 1.0) * (Er-1.0)/2.0 + (Er+1.0)/2.0    ; [008d6760]
```

#### Step 8: Impedance Calculation (Z0)

The core impedance formula using constants 30.666, 376.9908, 6.28318:
```
; 0x0040c9d8 - 0x0040cbec:
; Two parallel impedance terms computed with ln() and pow()

; First term (for W_eff_1):
Z_arg1 = 30.666 / W_eff_1                   ; [00422c44]
z1 = ln(Z_arg1) * (-1.0)                    ; [008d6800]

; Second term (for W_eff_2):
Z_arg2 = 30.666 / W_eff_2                   ; [00422c44]
z2 = ln(Z_arg2) * (-1.0)                    ; [008d67f8]

; Correction terms with 0.28318 and 6.0:
; For W_eff_1:
corr1_a = ln(W_eff_1) * 0.28318 + 6.0
corr1_b = 4.0 / W_eff_1
corr1_c = ln(corr1_b + 1.0)
corr1 = pow(corr1_a + corr1_c) * 376.9908 / 6.28318   ; [008d67e8]

; For W_eff_2 (same structure):
corr2_a = ln(W_eff_2) * 0.28318 + 6.0
corr2_b = 4.0 / W_eff_2
corr2_c = ln(corr2_b + 1.0)
corr2 = pow(corr2_a + corr2_c) * 376.9908 / 6.28318   ; [008d67f0]

; Final ratio:
Z_ratio = ln(W_eff_2) / corr2              ; [008d67e0]
Z_frac  = corr1 / corr2                    ; stored in local

; Loss and coupling:
loss_geom = ln(Er) * geom                  ; [008d6778]
```

#### Step 9: Surface Roughness & Coupling Corrections

```
; 0x0040cbf2 - 0x0040cddc:
; Complex polynomial correction factors

skin_factor = 0.0157 * Rho * T_mm + 1.0        ; [EBP-0x1180]

; Polynomial with coefficients:
; A = 0.525*ln(w) + 0.6315 + 0.27488
A_coeff = ln(w) * 0.525 + 0.6315               ; first two terms
A_coeff = A_coeff * w + 0.27488                 ; [EBP-0x1754] extended

; B = exp((-8.7513 * w)) * 0.065683
B_term = ln(-8.7513 * w)
B_coeff = exp(B_term) * 0.065683
A_coeff = A_coeff - B_coeff                     ; [008d6780]

; C = (1.0 - exp(-0.03442 * Er)) * 0.33622
C_coeff = exp(ln(-0.03442 * Er))
C_coeff = (1.0 - C_coeff) * 0.33622             ; [008d6788]

; D = ln(Rho*T_mm / 38.7) * (-1.0)
D_coeff = ln(Rho * T_mm / 38.7) * (-1.0)        ; [008d67b8]

; E = exp(ln(-4.6 * w)) * 0.0363
; F = (1.0 - exp(-4.6*w)) * E_factor
E_sub = ln(-4.6 * w)
E_exp = exp(E_sub)
E_coeff = (1.0 - E_exp) * ... * 0.0363          ; [008d6790]

; Composite correction:
; G = (Er / 15.916)
; H = ln(G) * (-1.0)
; I = (1.0 - exp(H)) * 2.751 + 1.0
G = Er / 15.916                                  ; [EBP-0x1188]
H = ln(G) * (-1.0)
I = (1.0 - exp(H)) * 2.751 + 1.0                ; [008d6798]

; Final Z0 correction compound:
; J = E_coeff * I + 0.1844
; K = ln(Rho * T_mm)
; L = exp(K) * C_coeff * A_coeff
J = D_coeff * E_coeff + 0.1844
K = ln(Rho * T_mm)
correction_factor = exp(K) * C_coeff * A_coeff   ; [008d67a0]

; Final impedance:
; Z0 = Er - loss_geom
; Z0 = (1.0 + correction_factor) / Z0 - Er
Z0_final = Er - (Er - loss_geom) / (1.0 + correction_factor)  ; [008d67c0]
```

#### Step 10: Final Z0 Output

```
; 0x0040ce38:
Z0_display = FloatToStr(Z0_final)
; Written to result TEdit at EBX+0x99c
```

---

## Propagation Delay Computation (at 0x0040cfe1)

After Z0 is computed, several derived quantities are calculated:

```
; Step A: Ratio computations
H_over_T = B_mm / T_mm                          ; b/t ratio

; Step B: Effective dielectric constant related
; 0x0040d014 - 0x0040d09f:
W_pi = W_mm * pi                                ; W * 3.14159
correction_1p1 = 1.1 * B_mm * pi               ; 1.1 * B * pi
denom = B_mm                                    ; denominator

effective_term = ln(...)
eff_combine = sqrt(ln(...) + effective_term)
eff_result = (10.8731272 / b) * pow(eff_combine)
eff_width = eff_result * (Z0_final + 1.0) * (2.0 * Z0_final / (Z0_final))
eff_width = eff_width + W_mm                    ; [EBP-0x1118]

; Step C: Capacitance per unit length
; 0x0040d0bb - 0x0040d1c0:
; Involves constants 8, 14, 11, 16 for polynomial approximation:
C_term1 = (8.0/Z0_final + 14.0) / 11.0
C_term2 = 4.0 * T_mm / eff_width
C_combined = C_term1 * C_term2                   ; [EBP-0x1120]

T_over_eff = T_mm / eff_width                   ; [EBP-0x11A4]

; More ln() and sqrt() calls for propagation delay:
pd_term1 = ln(T_over_eff) * 16.0               ; extended precision
pd_term2 = (14.0 * Z0_final + 8.0) / (11.0 * Z0_final)
; Combined with (Z0_final+1)/(2*Z0_final) factor
; Final propagation delay: sqrt(Er_eff) * 85.0 (ps/inch) approx

; Step D: Conductor loss via skin depth
; 0x0040d1cd - 0x0040d209:
loss_ratio = loss_geom / Z0_final
skin_term = ln(loss_ratio)
skin_res = exp(skin_term) * Z_ratio
skin_factor = (Z0_final - 1.0) * (loss_geom - 1.0) / (Z0_final - 1.0)
conductor_loss = skin_res * skin_factor          ; [008d604c]
```

---

## Mode 1: Solve for Width (Iterative)

At `0x0040e395`, the function enters the "solve for width" mode. This reads a target
impedance Z0 from the form and iteratively finds the trace width W that produces it.

### Input Reading (0x0040e395 - 0x0040e618)
```
; Read target Z0 from field, read H1, H2, Er, T
; Validate that width fields are reasonable
; Set initial width guess from field or default
```

### Core Iteration Formula (0x0040e619 - 0x0040e704)

The key formulas used in the iterative solve:

```
; At 0x0040e674:
; exp(-1.55 * W_guess / T) factor:
exp_term = exp(-1.55 * W_guess / T)             ; [00866e5c] call
delta_W = (1.0 - exp_term) * Er                 ; effective width correction [008d64e8]

; Effective thickness via sqrt:
; At 0x0040e6b2:
T_eff = sqrt(...) / 60.0                        ; [extended precision local]

; Cohn thickness-corrected width:
; At 0x0040e6c6:
; 0.8 * W + B/1000
; 5.98 * T / (combined denominator)
; pow(result, T_eff)
W_corrected = pow((0.8*W + B/1000.0) / (5.98*T), T_eff)   ; [008d604c]
```

### Skin Depth Computation (0x0040e8db)
```
; sqrt() call at 0x0040e8db:
skin = sqrt(...) * 0.08457 * 1000.0             ; [008d6288]
```

### Convergence Check

The solver checks for convergence against `0.1` (at `0x00422cfc`) and iterates
with range bounds `[5, 15]` (at `0x00422d2c`, `0x00422d04`), using `FUN_00511ed4`
to enable/disable UI elements when out of range.

---

## Mode 2: Solve for Height (Iterative)

At `0x0040f825`, the function enters "solve for height" mode. This uses a similar
iterative approach to find the dielectric height (H) given target Z0, W, T, and Er.

---

## Mode 3: Solve for Spacing (at 0x00410d47)

Solves for ground plane spacing B given impedance and other parameters.

---

## Differential/Coupled Stripline Section (at 0x004118bf)

Starting at `0x004118bf`, there is a large additional block that handles the
**coupled/differential stripline** computation. This block has its own mode
sub-dispatch (modes 0-5) and computes:

### Input Processing
```
; 0x004118cc:
B_total = W1 + W2 + gap                          ; [EDI] = total width
W_diff  = W1 - W2                                ; [EBP-0x11F4] = width difference
abs_diff = abs(W_diff)                            ; via FUN_00866fe4
```

### Key Computations (from 0x00411909)
```
; Half-width computation:
W_half = (B_total - abs_diff) / 2.0               ; [008d6520]

; Coupling coefficient:
k_coupling = gap / B_total - W_half/B_total        ; [008d6530]

; Asymmetry factor:
asym = 1.0 - gap/B_total                           ; [008d65a8]
```

### Impedance Ratio Selection (0x00411966)
```
; If W < gap: ratio = gap/W
; Else:       ratio = W/gap
; Stored at [008d6558]
```

### Odd/Even Mode Impedance (0x00411a33 - 0x00411c5e)

Uses a polynomial approximation with the ratio:
```
; Polynomial terms:
; ln(ratio) * 1.023 - (1.0235 * ratio_stored + 0.5008)
; + ln(ratio) * 1.1564
; - ln(ratio) * 0.4749
; * W (the trace width)
; Result: effective coupled width [008d6528]

; sin() and cosh() calls for coupled impedance:
sin_term = sin(pi * W_half / B_total)              ; [008d6538]
cosh_term_a = cosh(-1.0 * pi * ratio / (2*B_total));
cosh_term_b = cosh(-1.0 * pi * ratio / (2*B_total));

; Coupled mode factor:
coupled = sin_term * (cosh_a + cosh_b) / (cosh_a - cosh_b)  ; [008d6540]

; Multiplied by geometric term:
Z_coupled = sin_term * coupled                    ; [008d62d0]

; Free-space impedance factor:
; sqrt(6.28318) / (376.9908 / 6.28318)
Z_base = sqrt(2*pi) * 376.9908 / (2*pi)          ; [008d6548]

; Combined:
; pow(ln(ratio) - 1.0) * combined_term
; Z_odd_even = Z_base * pow_term                   ; [008d604c]
```

### Effective Er for Coupled Lines (0x00411e29 onward)

Additional computation block for effective dielectric constant in coupled stripline:
```
; Check ratio threshold: (W1+W2)/W < 0.35?
; If so, use simplified model

; Polynomial Er correction:
; Terms with (1 - ratio^2), ratio*ln(ratio), (2*ratio - 1)*(1 - ratio^2)
; Using constants 1.023, 1.0235, 0.5008, 1.1564, 0.4749

; Coupling via sin/cosh formulas:
sin_arg = Er * effective_width / pi               ; [008d6568]
; Asymmetry corrections...
```

---

## Key Formulas Summary (Cohn Stripline Model)

### Impedance (Z0) for Symmetric Stripline

The toolkit implements the **Cohn closed-form** stripline impedance formula with
finite-thickness corrections. The essential formula structure is:

```
Z0 = (60/sqrt(Er)) * ln(4*B / (pi*d*W_eff))
```

Where:
- `B` = ground plane spacing (H1 + H2)
- `W_eff` = effective strip width (with thickness correction)
- `Er` = relative dielectric constant
- `d` = effective width coefficient

The thickness correction modifies W to W_eff:
```
W_eff = W + (delta_W)
delta_W = (T/pi) * (1 + ln(4*pi*W/T))     ; for W/T > 0.5 (wide strip)
delta_W = (T/pi) * (1 + ln(2*B/T))         ; for W/T <= 0.5 (narrow strip)
```

The constant `30.666 ~= 60/sqrt(Er)` for Er ~= 3.83 (a common FR-4 value suggests
this is actually `60/sqrt(Er)` computed at runtime), and `376.991 = 120*pi` is the
impedance of free space.

### Propagation Delay
```
t_pd = sqrt(Er_eff) * 85.0   ps/inch  (approximately)
```
Where `Er_eff` is the effective dielectric constant for the stripline geometry.

### Capacitance and Inductance Per Unit Length
```
C = 1 / (Z0 * v_p)     ; where v_p = c / sqrt(Er_eff)
L = Z0 / v_p
```

### Skin Resistance
```
R_skin = sqrt(pi * f * mu_0 * rho) / W_eff
```
The constant `0.08457 ~= 1/(4*pi)^(0.5)` relates to the skin depth formula,
and `11.8` relates to the propagation delay normalization.

---

## Solve Mode Summary

| Mode | Solve For      | Method              | Address Range            |
|------|----------------|---------------------|--------------------------|
| 0    | Z0 (impedance) | Direct calculation  | 0x0040c41b - 0x0040e388 |
| 1    | W (width)      | Iterative/Newton    | 0x0040e395 - 0x0040f818 |
| 2    | H (height)     | Iterative/Newton    | 0x0040f825 - 0x00410d41 |
| 3    | B (spacing)    | Iterative/Newton    | 0x00410d4e - 0x004118be |
| 4    | (differential) | Direct/Iterative    | 0x00412e6b - 0x00413c26 |
| 5    | (differential) | Direct/Iterative    | 0x00413c2e - 0x00414ccc |

The iterative solvers use convergence threshold `0.1` and validate that computed
results fall within ranges (e.g., width between 5-15 mils for certain checks,
height between 40-90 mils for others).

---

## Notes on Implementation

1. **Unit handling**: When the unit combo box (`EBX+0x684`) has ItemIndex == 1
   (metric mode), the function jumps to `0x00414ccd` where inputs are divided
   by 1000.0 early (converting um to mm) rather than using the mil-based path.

2. **Validation**: Extensive validation is performed on all inputs:
   - Er must be >= 1.0 (compared against float 1.0 at 0x00422bd8)
   - W and T must be > 0.0 (compared against float 0.0 at 0x00422bdc)
   - Various computed ratios are checked for sanity

3. **Error messages**: When validation fails, `ShowMessage()` (FUN_005dff68) is
   called with a resource string loaded via `FUN_0086b424`.

4. **Result display**: Multiple result fields are updated using `FloatToStr` and
   `Format` calls, with the formatted string set via `TEdit.SetText`.

5. **The function recurses**: It calls `FUN_00403398` (the dispatcher) internally,
   likely to refresh dependent calculations when inputs change.

6. **Function size**: At ~28KB with ~30,000+ instructions, this is one of the
   largest functions in the binary. The monolithic structure (no extracted
   sub-functions for the math) is typical of Delphi event handler code where
   everything was written in a single Button.OnClick procedure.
