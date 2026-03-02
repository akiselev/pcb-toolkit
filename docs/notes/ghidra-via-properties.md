# Ghidra Analysis: Solver_ViaProperties (Mode 16)

## Function Overview

- **Address**: `0x00427090` to `0x00434282`
- **Size**: ~53,746 bytes (0xD1F2) - too large for Ghidra decompiler
- **Dispatched when**: `DAT_008d5f88 == 16`
- **Menu handler**: `ViaProperties1Click` at `0x0048e4c4`
- **Analysis method**: Disassembly in chunks (300-500 instructions at a time)

## Function Structure

The function has two major sections controlled by `[EBX + 0x684] + 0x2f0`:
1. **Standard Via Properties** (value == 0): `0x00427090` to `0x0042bee4`
2. **Differential Via / Stub Length** (value == 1): `0x0042bee4` to `0x00434273`

### Sub-mode (Via Type) Dispatch

The standard via section dispatches on `[EBX + 0x9b0] + 0x2f0` (radio group):
- **0** = 2-Layer via
- **1** = Multi-Layer via (enables capacitance/impedance calculations)
- **2** = Microvia

### Pre-computation Functions

Called at `0x00427eb5`-`0x00427ecb` before main calculations:
- `FUN_00471678` at `0x00427eb9`: Reads **temp_rise** from field `[EBX + 0x630]` -> `DAT_008d60c4`
  - Max: 200°C, Min: 1°C (0 treated as no temp rise)
  - Also computes Fahrenheit: `DAT_008d610c = 1.8 * temp_rise`
- `FUN_00471e7c` at `0x00427ec2`: Reads **ambient_temp** from field `[EBX + 0x648]` -> `DAT_008d60d4`
  - Max: 200°C, Min: -80°C
  - Also computes Fahrenheit: `DAT_008d62b0 = 1.8 * ambient_temp + 32.0`
- `FUN_00471a08` at `0x00427ecb`: Computes **via_temperature** = ambient + rise
  - `DAT_008d60cc = DAT_008d60d4 + DAT_008d60c4`
  - Also: `DAT_008d6104 = 1.8 * via_temperature + 32.0`

### IPC-2221A Constants (set at `0x00427ed0`)

```
DAT_008d6084 = 0.024     (k_internal coefficient)
DAT_008d608c = 0.44      (temperature exponent)
DAT_008d6094 = 0.725     (area exponent)
```

---

## Input Variables (UI Field Mapping)

| Global Variable | Field Offset | Description | Unit Conversion |
|----------------|-------------|-------------|-----------------|
| `DAT_008d60e4` | `[EBX + 0x6e4]` | Via Hole Diameter | Raw input (mils or mm) |
| `DAT_008d6168` | `[EBX + 0x6e8]` | Internal Pad Diameter | Divided by 1000 (mils -> inches) |
| `DAT_008d6170` | `[EBX + 0x6ec]` | Ref Plane Opening Diameter | Divided by 1000 (mils -> inches) |
| `DAT_008d6178` | `[EBX + 0x6f0]` | Via Height | Divided by 1000 (mils -> inches) |
| `DAT_008d6668` | `[EBX + 0x6f8]` | Via Plating Thickness | Raw input |
| `DAT_008d6054` | `[EBX + 0x67c]` | Dielectric Constant (Er) | Raw input |
| `DAT_008d60c4` | `[EBX + 0x630]` | Temp Rise (°C) | Raw input |
| `DAT_008d60d4` | `[EBX + 0x648]` | Ambient Temp (°C) | Raw input |
| `DAT_008d60cc` | (computed) | Via Temperature (°C) | = ambient + rise |
| `DAT_008d6420` | `[EBX + 0x6f0]` | Via Height (second read) | Raw input |
| `DAT_008d69b0` | `[form + 0x46c]` | Via Current (Amps) | From input or computed |

### Output Field Offsets

| Output | Field Offset | Variable |
|--------|-------------|----------|
| Via Cross Section | `[EBX + 0x6f4]` | `DAT_008d60f4` |
| Via Capacitance | `[EBX + 0x700]` | `DAT_008d6180` |
| Via Inductance | `[EBX + 0x704]` | `DAT_008d6188` |
| Via Impedance | `[EBX + 0x708]` (step response) | `DAT_008d61a0` / `DAT_008d6198` |
| Via Current | `[EBX + 0x6fc]` | `DAT_008d60fc` |
| Via DC Resistance | `[EBX + 0x718]` | `DAT_008d61c0` |
| Via Power Dissipation | `[EBX + 0x71c]` | `DAT_008d62a8` |
| Aspect Ratio | `[EBX + 0x11b8]` | `DAT_008d69a8` |
| Via Thermal Resistance | (computed) | `DAT_008d6670` |

---

## Formulas

### 1. Via Barrel Cross-Section Area (sq.mils)

**Address**: `0x004283db` - `0x00428467`

```
d_outer = d_hole + 2 * T_plating
A_barrel = (pi/4) * (d_outer^2 - d_hole^2)
```

Implementation:
```
area_outer = pow(d_outer, 2.0) * pi / 4.0    ; pi = 3.14159 at 0x004342d0
area_inner = pow(d_hole, 2.0) * pi / 4.0
A_barrel = area_outer - area_inner
```

Stored at `DAT_008d60f4`. Note: `pow()` function is at `0x008675ac`.

### 2. Via Capacitance (pF) — Multi-Layer Only

**Address**: `0x00429497` - `0x004294c3`

**Condition**: `[EBX + 0x9b0] + 0x2f0 == 1` (multi-layer mode)

```
C_via = 1.41 * Er * h * D_pad / (D_anti - D_pad)
```

Where:
- `1.41` = Goldfarb empirical constant (at `0x004342dc`)
- `Er` = dielectric constant (`DAT_008d6054`)
- `h` = via height in inches (`DAT_008d6178`)
- `D_pad` = internal pad diameter in inches (`DAT_008d6168`)
- `D_anti` = antipad (ref plane opening) diameter in inches (`DAT_008d6170`)

Result stored at `DAT_008d6180`.

**Negative check**: If `C_via < 0.0`, it is clamped to 0.0 (address `0x004294c3`).

**2-Layer mode**: Capacitance is forced to 1.0 pF (address `0x004294f7`).

### 3. Via Inductance (nH)

**Address**: `0x00429509` - `0x0042954b`

```
L_via = 5.08 * h * (ln(4 * h / d) + 1)
```

Where:
- `5.08` = constant (at `0x004342e8`)
- `h` = via height in inches (`DAT_008d6178`)
- `d` = via hole diameter / 1000 (mils to inches) (`DAT_008d60e4` / `0x00434290`)
- `4.0` at `0x004342d8` (float)
- `1.0` at `0x00434288` (float)
- `ln()` function at `0x00867350` (uses FLDLN2 + FYL2X)

Implementation:
```
arg = 4.0 * h / (d_hole / 1000.0)
L_via = (ln(arg) + 1.0) * 5.08 * h
```

Result stored at `DAT_008d6188`.

**Negative check**: If `L_via < 0.0`, it is clamped to 0.0 (address `0x0042954b`).

### 4. Via Impedance (Ohms) — Multi-Layer Only

**Address**: `0x004295b0` - `0x004295d0`

```
Z_via = sqrt(L_nH / (C_pF * 0.001))
```

Where:
- `L_nH` = inductance in nH (`DAT_008d6188`)
- `C_pF` = capacitance in pF (`DAT_008d6180`)
- `0.001` converts pF to nF so that nH/nF = H/F -> sqrt gives Ohms
- `sqrt()` function at `0x00868834` (uses FSQRT)

Result stored at `DAT_008d61a0`.

### 5. Resonant Frequency (MHz) — Multi-Layer Only

**Address**: `0x00429570` - `0x004295aa`

```
f_res = 1 / (2*pi * sqrt(L_H * C_F)) / 1e6
```

Where:
- `L_H = L_nH * 1e-9` (nH to Henries, constant `0x004342f0`)
- `C_F = C_pF * 1e-12` (pF to Farads, constant `0x004342f8`)
- `2*pi = 6.28318` (at `0x00434300`)
- `1e6` at `0x00434308` (float)

Implementation:
```
temp = L_nH * 1e-9 * C_pF * 1e-12
f_Hz = 1.0 / (sqrt(temp) * 6.28318)
f_MHz = f_Hz / 1e6
```

Result stored at `DAT_008d6190`.

### 6. Step Response T10-90% (ps)

**Address**: `0x004295d6` - `0x004295f0`

```
T_step = 2.2 * C_pF * Z_via / 2.0
```

Where:
- `2.2` at `0x00434314` (double)
- `C_pF` = via capacitance
- `Z_via` = via impedance (or 50 Ohm line impedance / 2)
- `2.0` at `0x00434294` (float)

Result: `T_step = 1.1 * C_pF * Z_via` in picoseconds.

Stored at `DAT_008d6198`.

### 7. Aspect Ratio

**Address**: `0x004290a3` - `0x00429104`

```
aspect_ratio = via_height_raw / via_hole_dia_raw
```

Read from fields `[EBX + 0x6f0]` and `[EBX + 0x6e4]`, divided directly.

Result stored at `DAT_008d69a8`. If aspect ratio exceeds the limit (`DAT_008d69b0`),
the aspect ratio label turns red (font color 0xFF = red) and a warning icon is shown.

### 8. Copper Resistivity Lookup Table

**Address**: `0x00429ecf` - `0x0042a0b1`

The base copper resistivity (`rho_base`) is selected from a temperature-indexed table
based on via temperature `DAT_008d60cc`:

| Temperature Range | rho_base (Ohm*mil) |
|-------------------|---------------------|
| T <= -40°C        | 0.0005190000        |
| -40 < T <= -20°C  | 0.0005720000        |
| -20 < T <=   0°C  | 0.0006250000        |
|   0 < T <=  20°C  | 0.0006787000        |
|  20 < T <=  40°C  | 0.0007320000        |
|  40 < T <=  60°C  | 0.0007850000        |
|  60 < T <=  80°C  | 0.0008390000        |
|       T >  80°C   | 0.0008390000 (last value used) |

Note: 0.0006787 at 20°C is consistent with copper bulk resistivity of
1.724e-6 Ohm*cm converted to Ohm*mil units (1.724e-6 * 2.54 / 0.00254 ~ 0.000679).

### 9. Temperature-Adjusted Resistivity

**Address**: `0x0042a0b1` - `0x0042a0d5`

```
rho_adjusted = rho_base * (1 + 0.00393 * (T_via - 20.0))
```

Where:
- `rho_base` = from lookup table above
- `0.00393` = temperature coefficient of copper resistance at `0x00434324`
- `20.0` at `0x004342c8` (float) - reference temperature
- `T_via` = via temperature in °C (`DAT_008d60cc`)

Result stored at `DAT_008d61b0`.

### 10. Via DC Resistance (Ohms)

**Address**: `0x0042a0e2` - `0x0042a111`

```
R_dc = rho_adjusted * via_height / A_barrel
```

Where:
- `rho_adjusted` = temperature-corrected resistivity (`DAT_008d61b0`)
- `via_height` = re-read from UI field `[EBX + 0x6f0]`
- `A_barrel` = barrel cross-section area (from formula 1)

Result stored at `DAT_008d61c0`.

### 11. Power Dissipation (Watts)

**Address**: `0x0042a2f0` - `0x0042a302`

```
P_watts = I^2 * R_dc
```

Where:
- `I` = via current (`DAT_008d60fc`)
- `R_dc` = DC resistance (`DAT_008d61c0`)

Result stored at `DAT_008d62a8`.

### 12. Power Dissipation (dBm)

**Address**: `0x0042a4cd` - `0x0042a4ed`

```
P_dBm = 10 * log10(P_watts / 0.001)
```

Where:
- `log10()` function at `0x008673c0`
- `0.001` at `0x0043430c`
- `10` at `0x004342cc` (float)

Result stored at `DAT_008d60b4`.

### 13. Via Voltage Drop (mV)

**Address**: `0x0042ac60` - `0x0042ac72`

```
V_drop_mV = I * R_dc * 1000
```

Where:
- `I` = via current (`DAT_008d60fc`)
- `R_dc` = DC resistance (`DAT_008d61c0`)
- `1000.0` at `0x00434290` (float)

Result stored at `DAT_008d60dc`.

### 14. Current Density J (A/m^2)

**Address**: `0x0042ae3d` - `0x0042ae5b`

```
J = I / (A_barrel * 6.4516e-10)
```

Implementation:
```
J = I / ((A_barrel / 1000) * 0.00064516) / 10000
```

Where:
- `0.00064516` = (0.0254)^2, conversion from sq.mils to sq.mm at `0x00434338`
- `10000` at `0x00434340` (float) - converts A/mm^2 to A/cm^2...
- Effective: 1 sq.mil = 6.4516e-10 m^2, so `J = I / (A_sqmil * 6.4516e-10)` A/m^2

Result stored at `DAT_008d6c1c`.

### 15. Via Thermal Resistance (°C/W)

**Address**: `0x0042a6b8` - `0x0042a6d6`

```
R_thermal = via_height * 1e6 / (A_barrel * 10.008)
```

Where:
- `10.008` at `0x0043432c` (double) — related to copper thermal conductivity in
  appropriate unit system (Cu thermal conductivity ~ 401 W/(m·K), converted to
  mil-based system)
- `A_barrel` = cross-section area (sq.mils)
- `via_height` in inches (divided by 1000 earlier)

Implementation:
```
R_thermal = via_height / ((A_barrel/1000) * 10.008) * 1000
```

Result stored at `DAT_008d6670`.

Per-via thermal resistance: `R_per_via = R_thermal / via_count` (at `0x0042aa7a`).

### 16. Via Current Capacity (Amps) — IPC-2221A

**Address**: `0x00428674` (2-layer), `0x004288de` (multi-layer), `0x00428b24` (microvia)

The current capacity uses the IPC-2221A internal conductor formula with modifiers:

```
I = 0.024 * dT^0.44 * A_barrel^0.725 * M_area * M_plating * M_roughness
```

Where:
- `0.024` = internal conductor coefficient (`DAT_008d6084`)
- `0.44` = temperature exponent (`DAT_008d608c`)
- `0.725` = area exponent (`DAT_008d6094`)
- `dT` = temperature rise (`DAT_008d60c4`)
- `A_barrel` = barrel cross-section area
- `M_area` = area correction modifier from `FUN_004b5184` (`DAT_008d6490`)
- `M_plating` = plating thickness modifier (`DAT_008d6428`)
- `M_roughness` = copper roughness factor (`DAT_008d6478`, typically 0.98 for FR-4)

**Mode-specific behavior**:
- **2-Layer**: Uses all three modifiers: `M_area * M_plating * M_roughness`
- **Multi-Layer**: Omits plating modifier: `M_area * M_roughness`
- **Microvia**: Base formula only (no modifiers)

Result stored at `DAT_008d60fc`.

### Plating Thickness Modifier Table

**Address**: `0x004280da` - `0x0042837e`

Based on via height (`DAT_008d6420`), a plating modifier is selected:

| Via Height Range | Modifier |
|-----------------|----------|
| Height > 100 mil | 1.30 |
| 90 < H <= 100   | 1.20 |
| 80 < H <=  90   | 1.10 |
| 70 < H <=  80   | 1.00 |
| 60 < H <=  70   | 0.95 |
| 50 < H <=  60   | 0.85 |
| 40 < H <=  50   | 0.75 |
| 30 < H <=  40   | 0.67 |
| 20 < H <=  30   | 0.58 |
| 10 < H <=  20   | 0.48 |
| H <= 10 mil     | 0.40 |

Stored at `DAT_008d6428`.

### Area Correction Modifier (FUN_004b5184)

**Address**: `0x004b5184`

Based on the via barrel cross-section area, applies a correction factor:

| Area Range (sq.mils) | Factor Constant |
|-----------------------|-----------------|
| area <= 20            | Uses `DAT_004b530c` |
| 20 < area <= 60       | 2.9143 (`DAT_004b5318`) |
| 60 < area <= 100      | 2.7877 (`DAT_004b5324`) |
| area > 100            | 2.801 (`DAT_004b532c`) |

The modifier is computed as: `M_area = pow(area, factor_constant)` (approximate —
the decompiler output is partially garbled due to FPU stack issues).

Stored at `DAT_008d6490`.

---

## Differential Via / Stub Length Section

**Address**: `0x0042bee4` - `0x00434273`

**Condition**: `[EBX + 0x684] + 0x2f0 == 1` (differential via tab selected)

This section implements the Bert Simonovich method for differential via modeling.

### Differential Via Inputs

| Variable | Description |
|----------|-------------|
| `DAT_008d60e4` | Drill Hole Diameter |
| `DAT_008d6168` | Via Spacing (center-to-center) |
| `DAT_008d6170` | Ref Plane Opening (H or W) |
| `DAT_008d6178` | Via Height/Length |
| `DAT_008d6054` | Dielectric Constant (Er) |
| `DAT_008d6668` | Via Spacing or plating (context-dependent) |
| `DAT_008d6420` | Baud Rate |
| `DAT_008d69e8` | Anisotropy |

### Effective Dielectric Constant

**Address**: `0x0042b385` - `0x0042b3a9`

```
Er_eff = (Er * (1 + anisotropy/100) + Er) / 2
       = Er * (2 + anisotropy/100) / 2
       = Er * (1 + anisotropy/200)
```

Where:
- `anisotropy` is divided by `100.0` (float at `0x004342a8`)
- Result stored at `DAT_008d69f0`

### Via Barrel Cross-Section (Differential)

**Address**: `0x0043023f` - `0x004302cb`

Same formula as standard via:
```
d_outer = D_drill + 2 * T_plating
A_barrel = pi/4 * (d_outer^2 - D_drill^2)
```

### Odd-Mode Impedance (Partial Trace)

**Address**: `0x0042b3c3` - `0x0042b478`

Complex formula involving ratios of via dimensions:
```
factor1 = 60.0 / sqrt(Er_eff)          ; 60/sqrt at 0x0042b3c3
ratio1 = D_ref / D_hole                ; D_ref / D_hole
ratio2 = (H1 + H2) / (2 * D_hole)     ; sum of openings / (2 * hole dia)
Z_odd = factor1 * ln(ratio1 + ratio2) * ... (complex expression)
```

### Differential Impedance

**Address**: `0x0042b643`

```
Z_diff = 2 * Z_odd
```

Where `Z_odd` is the odd-mode impedance (`DAT_008d69f8`). Constant `2.0` at `0x00434294`.

Result stored at `DAT_008d6a00`.

### Insertion Loss

**Address**: `0x0042ba8f` - `0x0042baa4`

```
insertion_loss = some_factor / 0.84 * 1000
```

Where `0.84` is at `0x00434344` (double). The exact formula involves the via
resistance and impedance but the full trace was not completed.

### Maximum Stub Length

**Address**: `0x0042bc9e` - `0x0042bd19`

```
fill_ratio = (Z_total - Z_diff) / (Z_total + Z_diff)
stub_correction = (1.0 - fill_ratio) * 20.0
max_stub = 2.0 * stub_correction
```

---

## Key Constants Table

| Address | Type | Value | Purpose |
|---------|------|-------|---------|
| `0x00434284` | float | 250.0 | Threshold |
| `0x00434288` | float | 1.0 | Unity constant |
| `0x0043428c` | float | 500.0 | Threshold |
| `0x00434290` | float | 1000.0 | mil-to-inch divisor |
| `0x00434294` | float | 2.0 | General multiplier |
| `0x00434298` | float | 3.0 | General multiplier |
| `0x0043429c` | double | 0.1 | Threshold |
| `0x004342a4` | float | 350.0 | Max Er threshold |
| `0x004342a8` | float | 100.0 | Percentage divisor |
| `0x004342b0` | float | 80.0 | Temperature threshold |
| `0x004342b4` | float | 70.0 | Temperature threshold |
| `0x004342b8` | float | 60.0 | Temperature threshold |
| `0x004342bc` | float | 50.0 | Temperature threshold |
| `0x004342c0` | float | 40.0 | Temperature/height threshold |
| `0x004342c4` | float | 30.0 | Height threshold |
| `0x004342c8` | float | 20.0 | Reference temperature |
| `0x004342cc` | float | 10.0 | Height threshold / dBm multiplier |
| `0x004342d0` | double | 3.14159 | pi |
| `0x004342d8` | float | 4.0 | Geometry constant |
| `0x004342dc` | double | 1.41 | Goldfarb via capacitance constant |
| `0x004342e4` | float | 0.0 | Zero / boundary |
| `0x004342e8` | double | 5.08 | Via inductance constant (nH/inch) |
| `0x004342f0` | double | 1e-9 | nH to H conversion |
| `0x004342f8` | double | 1e-12 | pF to F conversion |
| `0x00434300` | double | 6.28318 | 2*pi |
| `0x00434308` | float | 1e6 | Hz to MHz conversion |
| `0x0043430c` | double | 0.001 | mW conversion / pF to nF |
| `0x00434314` | double | 2.2 | Step response time constant |
| `0x0043431c` | float | -40.0 | Minimum temperature |
| `0x00434320` | float | -20.0 | Temperature threshold |
| `0x00434324` | double | 0.00393 | Cu temperature coefficient (alpha) |
| `0x0043432c` | double | 10.008 | Thermal conductivity factor |
| `0x00434334` | float | 999.0 | Threshold |
| `0x00434338` | double | 0.00064516 | sq.mil to sq.mm conversion |
| `0x00434340` | float | 10000.0 | Unit conversion |
| `0x00434344` | double | 0.84 | Insertion loss factor |
| `0x0043434c` | double | 6.35 | Threshold (250 mils in mm) |
| `0x00434354` | double | 0.0254 | inch/mil conversion |
| `0x00434364` | double | 25.4 | mm/inch conversion |
| `0x0043436c` | double | 0.051 | Small offset (2 mils in inches) |
| `0x00434374` | float | 51.0 | Threshold |
| `0x00434378` | double | 0.127 | 5 mils in mm |
| `0x00434380` | double | 0.0025 | Threshold |
| `0x004343d8` | float | 1550.0 | Frequency constant? |

### Via Spacing Table (Differential)

| Address | Value (mm) | Approx. (mils) |
|---------|-----------|-----------------|
| `0x00434388` | 2.540 | 100 |
| `0x00434390` | 2.286 | 90 |
| `0x00434398` | 2.032 | 80 |
| `0x004343a0` | 1.778 | 70 |
| `0x004343a8` | 1.524 | 60 |
| `0x004343b0` | 1.270 | 50 |
| `0x004343b8` | 1.016 | 40 |
| `0x004343c0` | 0.762 | 30 |

---

## Math Function Addresses

| Address | Function | Implementation |
|---------|----------|----------------|
| `0x008675ac` | `pow(x, y)` | General power function (x^y) |
| `0x00867350` | `ln(x)` | Natural log via FLDLN2 + FYL2X |
| `0x008673c0` | `log10(x)` | Common logarithm |
| `0x00868834` | `sqrt(x)` | Square root via FSQRT |

---

## Notes on 4/pi (1.2732) and Speed of Light

Cross-reference analysis shows:
- **4/pi (0x004BA940)** is only referenced from `FUN_004b8104` (Solver_FusingCurrent, mode 7).
  It is **NOT used** in the via properties calculator.
- **Speed of light (0x004BFF64)** is only referenced from `FUN_004bf410` (Solver_Wavelength, mode 17).
  It is **NOT used** in the via properties calculator.

The via capacitance formula uses `1.41` (Goldfarb constant) directly, not derived from 4/pi.
The NOTES.md entry referencing 4/pi for via capacitance was based on the fusing current
function, not the via calculator itself.

---

## Summary of Via Properties Output Formulas

### Standard Via Mode

| Output | Formula | Units |
|--------|---------|-------|
| Cross Section | `A = pi/4 * ((d+2t)^2 - d^2)` | sq.mils |
| Capacitance (multi-layer) | `C = 1.41 * Er * h * D_pad / (D_anti - D_pad)` | pF |
| Inductance | `L = 5.08 * h * (ln(4h/d) + 1)` | nH |
| Impedance (multi-layer) | `Z = sqrt(L_nH / C_nF)` | Ohms |
| Resonant Freq (multi-layer) | `f = 1/(2pi*sqrt(LC)) / 1e6` | MHz |
| Step Response | `T = 2.2 * C_pF * Z / 2` | ps |
| DC Resistance | `R = rho(T) * h / A` | Ohms |
| Power | `P = I^2 * R` | Watts |
| Power (dBm) | `P_dBm = 10*log10(P/0.001)` | dBm |
| Voltage Drop | `V = I * R * 1000` | mV |
| Current Density | `J = I / (A * 6.4516e-10)` | A/m^2 |
| Thermal Resistance | `R_th = h*1e6 / (A * 10.008)` | °C/W |
| Current (IPC-2221A) | `I = 0.024 * dT^0.44 * A^0.725 * modifiers` | Amps |
| Aspect Ratio | `AR = h / d` | dimensionless |

### Notes
- All dimensions in formulas above are in inches unless noted otherwise
- The via hole diameter and via height are read from input in mils and divided by 1000
- Temperature-corrected resistivity: `rho(T) = rho_base(T_range) * (1 + 0.00393*(T-20))`
- The function uses the IPC-2221A **internal** conductor formula (k=0.024), not external (k=0.048)
