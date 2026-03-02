# Ghidra Decompilation: Solver_ConductorCurrent

Function: `Solver_ConductorCurrent`
Address: `0x004343e4`
Size: 51488 bytes (0x004343e4 - 0x00440d04)
Project: saturn-pcb

Too large for Ghidra's decompiler; analyzed via disassembly.

---

## Function Structure

The function handles 3 IPC modes selected via a combo box at Form6 offset
`+0x454` (accessed through `PTR__Form6_008d5480`):

| Mode | ItemIndex | Description                  |
|------|-----------|------------------------------|
|  0   |     0     | IPC-2152 with modifiers      |
|  1   |     1     | IPC-2152 without modifiers   |
|  2   |     2     | IPC-2221A (legacy)           |

Within each mode, a second combo at offset `+0x494` selects the layer type:

| Index | Layer    |
|-------|----------|
|   0   | External |
|   1   | Internal |
|   2   | Internal (variant, IPC-2152 only) |

A third combo at offset `+0x968` on the form (accessed via `EBX+0x968`)
controls the solve direction:

| Index | Solve Mode                      |
|-------|---------------------------------|
|   0   | Solve for current (given width) |
|   1   | Solve for width (given current) |

---

## Global Variable Map

| Address        | Description                                         | Source              |
|----------------|-----------------------------------------------------|---------------------|
| `0x008d61a8`   | Trace length (user input)                           | EBX+0x604           |
| `0x008d6034`   | Trace width (mils)                                  | text field          |
| `0x008d603c`   | Copper thickness (mils)                             | text field          |
| `0x008d60f4`   | Cross-section area A (sq.mils, calculated)          | computed            |
| `0x008d60f8`   | Cross-section area (high dword)                     |                     |
| `0x008d60fc`   | Current I (Amps, calculated or input)               | computed            |
| `0x008d60c4`   | Temperature rise dT (deg C)                         | copied              |
| `0x008d60c8`   | Temperature rise (high dword)                       |                     |
| `0x008d60cc`   | Ambient temperature (deg C)                         | text field          |
| `0x008d6420`   | Temperature rise (for lookup, from EBX+0x954)       | EBX+0x954           |
| `0x008d6438`   | Plane distance (mils, from EBX+0x960)               | EBX+0x960           |
| `0x008d6458`   | Board thickness (mils, from EBX+0x984)              | EBX+0x984           |
| `0x008d61b8`   | Copper weight (oz, from combo EBX+0x990)            | combo               |
| `0x008d6668`   | Second copper weight (from combo EBX+0x620)         | combo               |
| `0x008d6084`   | k constant (set per mode)                           | constant            |
| `0x008d608c`   | b exponent - dT (set per mode)                      | constant            |
| `0x008d6094`   | c exponent - area (set per mode)                    | constant            |
| `0x008d6428`   | Temperature rise correction factor                  | lookup table        |
| `0x008d6460`   | Board thickness correction factor                   | lookup table        |
| `0x008d6478`   | Material resistivity factor                         | `ComboBox1Change_MaterialSelector` |
| `0x008d6480`   | User correction factor (default 1.0)                | EBX+0x604 init      |
| `0x008d6490`   | Cross-section area chart correction                 | `FUN_004b5184`      |
| `0x008d61b0`   | Temperature-adjusted resistivity                    | computed            |
| `0x008d61c0`   | DC resistance (Ohms)                                | computed            |
| `0x008d62a8`   | Power dissipation (Watts)                           | computed            |
| `0x008d62c8`   | Voltage drop (Volts)                                | computed            |
| `0x008d63e8`   | Skin depth (mils)                                   | computed            |
| `0x008d69b8`   | Base resistivity factor (from ambient temp lookup)  | lookup table        |
| `0x008d6c0c`   | Temperature in Fahrenheit                           | computed            |
| `0x008d65b0`   | 1/c (for reverse solve)                             | computed            |

---

## IPC-2221A Formula (Mode 2)

Address range: `0x0043762a` - `0x00437996`

### Constants

Set at `0x0043762a` (combo `0x9b0` index 0, internal) and `0x004376e1`
(combo `0x9b0` index 1, external):

| Parameter | Internal (index 0) | External (index 1) |
|-----------|-------------------|--------------------|
| k         | 0.024             | 0.048              |
| b (dT)    | 0.44              | 0.44               |
| c (area)  | 0.725             | 0.725              |

Stored as IEEE 754 doubles:
- k_int = `0x3F989374_BC6A7EFA` = 0.024
- k_ext = `0x3FA89374_BC6A7EFA` = 0.048
- b     = `0x3FDC28F5_C28F5C29` = 0.44
- c     = `0x3FE73333_33333333` = 0.725

### Forward Solve (given width, find current)

At `0x00437777`:

```
I = k * pow(dT, b) * pow(A, c)
```

Assembly:
```asm
; pow(dT, 0.44)
PUSH [0x008d6090]    ; b high
PUSH [0x008d608c]    ; b low = 0.44
PUSH [0x008d60c8]    ; dT high
PUSH [0x008d60c4]    ; dT low
CALL 0x008675ac      ; pow(dT, b)
FMUL [0x008d6084]    ; * k
FSTP extended [EBP+...]

; pow(A, 0.725)
PUSH [0x008d6098]    ; c high
PUSH [0x008d6094]    ; c low = 0.725
PUSH [0x008d60f8]    ; area high
PUSH [0x008d60f4]    ; area low
CALL 0x008675ac      ; pow(A, c)

; Final multiply
FLD extended [EBP+...]
FMULP                ; k * dT^b * A^c
FSTP [0x008d60fc]    ; store I
```

### Reverse Solve (given current, find area)

At `0x00436c79` (mode 1) and similar for other modes:

```
inv_c = 1.0 / c
A = pow(I / (k * pow(dT, b)), inv_c)
```

---

## IPC-2152 Without Modifiers (Mode 1)

Address range: `0x004368fa` - `0x0043760f`

Uses the **same** base formula and constants as IPC-2221A:

```
I = k * pow(dT, b) * pow(A, c)
```

| Parameter | Internal | External |
|-----------|----------|----------|
| k         | 0.024    | 0.048    |
| b         | 0.44     | 0.44     |
| c         | 0.725    | 0.725    |

The only difference from IPC-2221A is the UI label and that Mode 1 supports
the reverse solve for both external (0x494=0) and internal (0x494=1) layers.

---

## IPC-2152 With Modifiers (Mode 0)

Address range: `0x004362ff` - `0x004368df`

### Forward Solve (External, combo 0x968 index 0)

At `0x0043644c`:

```
I = k * pow(dT, b) * pow(A, c) * M_area * M_temp * M_board * M_material * M_user
```

Assembly at `0x004364a0`:
```asm
FMUL [0x008d6490]    ; * M_area  (area chart correction)
FMUL [0x008d6428]    ; * M_temp  (temperature rise correction)
FMUL [0x008d6460]    ; * M_board (board thickness correction)
FMUL [0x008d6478]    ; * M_material (material resistivity factor)
FMUL [0x008d6480]    ; * M_user  (user correction, default 1.0)
FSTP [0x008d60fc]    ; store I
```

### Forward Solve (Internal, combo 0x968 index 1)

At `0x0043669c`:

```
I = k * pow(dT, b) * pow(A, c) * M_area * M_temp / plane_dist * M_board * M_material * M_user
```

Assembly at `0x004366f0`:
```asm
FMUL [0x008d6490]    ; * M_area
FMUL [0x008d6428]    ; * M_temp
FDIV [0x008d6438]    ; / plane_dist
FMUL [0x008d6460]    ; * M_board
FMUL [0x008d6478]    ; * M_material
FMUL [0x008d6480]    ; * M_user
FSTP [0x008d60fc]    ; store I
```

### Modifier Variables

| Symbol       | Address        | Source                         |
|--------------|----------------|--------------------------------|
| M_area       | `0x008d6490`   | Piecewise power law (FUN_004b5184) |
| M_temp       | `0x008d6428`   | Lookup table (temperature rise)    |
| M_board      | `0x008d6460`   | Lookup table (board thickness)     |
| M_material   | `0x008d6478`   | Material selector combo            |
| M_user       | `0x008d6480`   | User input field (default 1.0)     |
| plane_dist   | `0x008d6438`   | User input (EBX+0x960)             |

---

## IPC-2152 Modifier Lookup Tables

### Temperature Rise Correction (M_temp)

Stored at `0x008d6428`. Source variable: `0x008d6420` (temperature rise in
deg C, from text field EBX+0x954).

Address range: `0x004349ec` - `0x00434c90`

| dT Range (deg C) | M_temp |
|-------------------|--------|
| dT == 100         | 1.30   |
| 90 < dT <= 100    | 1.20   |
| 80 < dT <= 90     | 1.10   |
| 70 < dT <= 80     | 1.00   |
| 60 < dT <= 70     | 0.95   |
| 50 < dT <= 60     | 0.85   |
| 40 < dT <= 50     | 0.75   |
| 30 < dT <= 40     | 0.67   |
| 20 < dT <= 30     | 0.58   |
| 10 < dT <= 20     | 0.48   |
| dT <= 10           | 0.40   |

### Board Thickness Correction (M_board)

Stored at `0x008d6460`. Source variable: `0x008d6458` (board thickness in
mils, from text field EBX+0x984).

Two tables, selected by combo `EBX+0x9b8`:

**Table 1** (0x9b8 index 0 - No copper plane):

Address range: `0x00434ca1` - `0x00435041`

| Thickness (mils) | M_board |
|-------------------|---------|
| default (init)    | 1.00    |
| t == 100          | 1.20    |
| 90 < t <= 100     | 1.30    |
| 80 < t <= 90      | 1.33    |
| 70 < t <= 80      | 1.37    |
| 60 < t <= 70      | 1.39    |
| 50 < t <= 60      | 1.42    |
| 40 < t <= 50      | 1.45    |
| 30 < t <= 40      | 1.48    |
| 20 < t <= 30      | 1.52    |
| 10 < t <= 20      | 1.55    |
| t <= 10           | 1.59    |

**Table 2** (0x9b8 index 1 - With copper plane):

Address range: `0x00435140` - `0x004353c0`

| Thickness (mils) | M_board |
|-------------------|---------|
| t == 100          | 1.24    |
| 90 < t <= 100     | 1.34    |
| 80 < t <= 90      | 1.37    |
| 70 < t <= 80      | 1.41    |
| 60 < t <= 70      | 1.43    |
| 50 < t <= 60      | 1.46    |
| 40 < t <= 50      | 1.49    |
| 30 < t <= 40      | 1.52    |
| 20 < t <= 30      | 1.56    |
| 10 < t <= 20      | 1.59    |
| t <= 10           | 1.63    |

### Area Chart Correction (M_area)

Computed in `FUN_004b5184` (address `0x004b5184`, called at `0x004362df`).
Piecewise power law based on cross-section area:

```
M_area = multiplier * pow(area, exponent)
```

| Area Range (sq.mils) | Exponent | Multiplier |
|----------------------|----------|------------|
| area <= 20           | -0.145   | 3.0364     |
| 20 < area <= 60      | -0.129   | 2.9143     |
| 60 < area <= 100     | -0.114   | 2.7877     |
| area > 100           | -0.111   | 2.801      |

Constants stored at `0x004b5308`:
- Thresholds: float 20.0 (`0x004b5308`), 60.0 (`0x004b5314`), 100.0 (`0x004b5320`)
- Exponents pushed as immediates: -0.145, -0.129, -0.114, -0.111
- Multipliers as doubles: 3.0364 (`0x004b530c`), 2.9143 (`0x004b5318`),
  2.7877 (`0x004b5324`), 2.801 (`0x004b532c`)

A variant `FUN_004b5334` (`0x004b5334`) uses the same exponents and
multipliers but scales the area input by 1550 (float at `0x004b54d4`):

```
M_area = multiplier * pow(area * 1550, exponent)
```

---

## Copper Weight Combo Box

Combo at `EBX+0x990`, stores double at `0x008d61b8`:

| Index | Copper Weight (oz) |
|-------|--------------------|
|   0   | 0.35               |
|   1   | 0.70               |
|   2   | 1.40               |
|   3   | 2.10               |
|   4   | 2.80               |
|   5   | 3.50               |
|   6   | 4.20               |
|   7   | 5.60               |
|   8   | 7.00               |

---

## Cross-Section Area Calculation

### External layer (0x494 index 0)

Not explicitly shown in the disassembled range; likely a simple
`area = width * thickness` before the function is called.

### Internal layer (0x494 index 1)

At `0x00435711`:

```
area = (width - thickness / 2) * thickness
```

Assembly:
```asm
FLD  [0x008d603c]        ; load thickness
FDIV float [0x00440d0c]  ; / 2.0
FSUBR [0x008d6034]       ; width - (thickness/2)
FMUL [0x008d603c]        ; * thickness
FSTP [0x008d60f4]        ; store area
```

This approximates a trapezoidal etch profile.

### Internal layer (0x494 index 2)

At `0x00435900` (approximate address, same formula structure but uses
`[0x008d6034]` directly for area):

```
area = width * thickness
```

---

## Post-Mode Common Calculations

These calculations run after the current is determined, regardless of mode.

### Celsius to Fahrenheit

At `0x0043833f`:

```
T_F = T_C * 9 / 5 + 32
```

Constants: `0x00440d3c` = 9.0f, `0x00440d40` = 5.0f, `0x00440d44` = 32.0f

### Temperature-Adjusted Resistivity

At `0x0043875c`:

```
rho_adj = (1 + 0.00393 * (T_ambient - 20)) * rho_base
```

Assembly:
```asm
FLD  [0x008d60cc]        ; T_ambient
FSUB float [0x00440d30]  ; - 20
FMUL [0x00440d54]        ; * 0.00393 (copper temp coeff)
FADD float [0x00440d08]  ; + 1.0
FMUL [0x008d69b8]        ; * rho_base (from ambient temp lookup)
FSTP [0x008d61b0]        ; store rho_adj
```

Constants:
- `0x00440d54` = 0.00393 (copper temperature coefficient, per deg C)
- `0x00440d30` = 20.0f (reference temperature)
- `0x00440d08` = 1.0f

### DC Resistance

At `0x00438780`:

```
R_dc = rho_adj * length / area
```

For internal layer with plane (combo 0x968 index 1, at `0x00438994`):

```
R_dc = rho_adj * length / (area / plane_dist)
```

### Power Dissipation

At `0x00438b79`:

```
P = I^2 * R_dc
```

### Voltage Drop

At `0x00438d56`:

```
V = I * R_dc
```

### Skin Depth

At `0x0043919d`:

```
skin_depth_mils = sqrt(rho_copper / (pi * mu_factor * freq_scaled)) * 1e6 * (1/25.4)
```

Assembly:
```asm
; freq_scaled = user_freq * 100000
FMUL float [0x00440d5c]     ; * 100000.0

; mu_factor = 1.256636e-05 (runtime constant at 0x008d6418)
; This is 4*pi*1e-6 (note: NOT mu_0 = 4*pi*1e-7)

; Compute: pi * mu_factor * freq_scaled
FLD  [0x00440d60]            ; pi = 3.14159
FMUL [0x008d6418]            ; * mu_factor (1.256636e-5)
FMUL [0x008d63f0]            ; * freq_scaled

; Compute: rho_copper / (pi * mu * f)
FDIVR [0x00440d68]           ; 1.72e-8 / result

; sqrt
CALL 0x00868834              ; sqrt()

; Convert to mils
FMUL float [0x00440d70]     ; * 1000000.0
FMUL [0x00440d74]            ; * 0.0393700787401575 (= 1/25.4)
FSTP [0x008d63e8]            ; store skin_depth_mils
```

Constants:
- `0x00440d5c` = 100000.0f (frequency scaling)
- `0x00440d60` = 3.14159 (double, pi)
- `0x00440d68` = 1.72e-08 (double, copper resistivity in ohm*m)
- `0x00440d70` = 1000000.0f (m to um)
- `0x00440d74` = 0.0393700787401575 (double, 1/25.4 = mm to inches)

Runtime constant:
- `0x008d6418` = 1.256636e-05 (stored as `0x3EEA5A83_5E0ACE98`)
  This equals 4*pi*1e-6; the factor of 10 vs mu_0 is compensated by
  the frequency scaling of 100000 (user input likely in MHz, so
  MHz * 1e5 is not Hz but an intermediate unit that, combined with the
  modified mu, gives the correct result).

---

## Ambient Temperature Resistivity Lookup

Stored at `0x008d69b8`. Source: `0x008d60cc` (ambient temperature in deg C).

Address range: `0x00438522` - `0x00438704`

| T_ambient Range (deg C) | rho_base (ohm*m related) |
|--------------------------|--------------------------|
| T <= -40                 | 0.000519                 |
| -40 < T <= -20           | 0.000572                 |
| -20 < T <= 0             | 0.000625                 |
| 0 < T <= 20              | 0.0006787                |
| 20 < T <= 40             | 0.000732                 |
| 40 < T <= 60             | 0.000785                 |
| 60 < T <= 80             | 0.000839                 |
| T > 80                   | 0.000839                 |

These values serve as base resistivity that is then adjusted by the
temperature coefficient formula. Note that the last two entries are
identical (0.000839 for both 60-80 and >80 ranges).

---

## FPU Constant Pool

### Main Constants (0x00440d04 - 0x00440dec)

| Address      | Type   | Value       | Purpose                        |
|--------------|--------|-------------|--------------------------------|
| `0x00440d04` | float  | 1200000.0   | (other calculator)             |
| `0x00440d08` | float  | 1.0         | Unity                          |
| `0x00440d0c` | float  | 2.0         | Etch factor divisor            |
| `0x00440d10` | float  | 100.0       | Threshold / percentage         |
| `0x00440d14` | float  | 90.0        | Threshold                      |
| `0x00440d18` | float  | 80.0        | Threshold                      |
| `0x00440d1c` | float  | 70.0        | Threshold                      |
| `0x00440d20` | float  | 60.0        | Threshold                      |
| `0x00440d24` | float  | 50.0        | Threshold                      |
| `0x00440d28` | float  | 40.0        | Threshold                      |
| `0x00440d2c` | float  | 30.0        | Threshold                      |
| `0x00440d30` | float  | 20.0        | Reference temp (deg C)         |
| `0x00440d34` | float  | 10.0        | Threshold                      |
| `0x00440d38` | float  | 0.5         | Half                           |
| `0x00440d3c` | float  | 9.0         | C-to-F numerator               |
| `0x00440d40` | float  | 5.0         | C-to-F denominator             |
| `0x00440d44` | float  | 32.0        | C-to-F offset                  |
| `0x00440d48` | float  | -40.0       | Threshold                      |
| `0x00440d4c` | float  | -20.0       | Threshold                      |
| `0x00440d50` | float  | 0.0         | Zero threshold                 |
| `0x00440d54` | double | 0.00393     | Copper temp coefficient (/deg C)|
| `0x00440d5c` | float  | 100000.0    | Frequency scaling              |
| `0x00440d60` | double | 3.14159     | Pi                             |
| `0x00440d68` | double | 1.72e-08    | Copper resistivity (ohm*m)     |
| `0x00440d70` | float  | 1000000.0   | m to um conversion             |
| `0x00440d74` | double | 0.03937...  | 1/25.4 (mm to inches)          |
| `0x00440d7c` | double | 0.001       | milli prefix                   |
| `0x00440d9c` | double | 0.0254      | inch to mm                     |
| `0x00440da4` | double | 2.54        | inch to cm                     |

### PCB Thickness Constants (0x00440dac - 0x00440dec)

Standard PCB copper thicknesses in mm (doubles):

| Address      | Value (mm) | Value (mils) |
|--------------|------------|--------------|
| `0x00440dac` | 2.286      | 90           |
| `0x00440db4` | 2.032      | 80           |
| `0x00440dbc` | 1.778      | 70           |
| `0x00440dc4` | 1.524      | 60           |
| `0x00440dcc` | 1.270      | 50           |
| `0x00440dd4` | 1.016      | 40           |
| `0x00440ddc` | 0.762      | 30           |
| `0x00440de4` | 0.508      | 20           |
| `0x00440dec` | 0.254      | 10           |

---

## Helper Functions

| Address      | Name/Purpose               | Signature                       |
|--------------|----------------------------|---------------------------------|
| `0x008675ac` | pow(x, y)                  | cdecl, pushes two doubles, returns in ST(0) |
| `0x00868834` | sqrt(x)                    | cdecl, push double, returns in ST(0)        |
| `0x0086f0f4` | StrToFloat                 | Delphi string to FPU            |
| `0x005396c8` | SetText                    | Set text field value            |
| `0x00539678` | GetText                    | Get text field value            |
| `0x00861e48` | FloatToStr (formatted)     | Double to formatted string      |
| `0x0071d964` | String concatenation       | Delphi string concat            |
| `0x004b5184` | ComputeAreaCorrectionFwd   | Area chart correction (external)|
| `0x004b5334` | ComputeAreaCorrectionScaled| Area chart correction (internal, *1550) |
| `0x00440df8` | FormatNumber               | Number formatting helper        |

---

## Summary of All Formulas

### IPC-2221A / IPC-2152 Base (all modes)

```
Forward:  I = k * dT^b * A^c
Reverse:  A = (I / (k * dT^b))^(1/c)
```

| Constant | Internal | External |
|----------|----------|----------|
| k        | 0.024    | 0.048    |
| b        | 0.44     | 0.44     |
| c        | 0.725    | 0.725    |

### IPC-2152 With Modifiers (Mode 0 only)

```
External: I = k * dT^b * A^c * M_area * M_temp * M_board * M_material * M_user
Internal: I = k * dT^b * A^c * M_area * M_temp / plane_dist * M_board * M_material * M_user
```

### DC Resistance

```
rho_adj = (1 + 0.00393 * (T_ambient - 20)) * rho_base
R_dc    = rho_adj * length / area
```

### Power and Voltage

```
P = I^2 * R_dc
V = I * R_dc
```

### Skin Depth

```
delta = sqrt(rho_copper / (pi * mu_factor * f_scaled)) * 1e6 / 25.4   [mils]
```

Where:
- rho_copper = 1.72e-08 ohm*m
- mu_factor = 1.256636e-05 (4*pi*1e-6)
- f_scaled = user_input * 100000

### Celsius to Fahrenheit

```
T_F = T_C * 9 / 5 + 32
```
