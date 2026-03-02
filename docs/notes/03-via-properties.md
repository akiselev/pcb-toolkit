# Via Properties / Via Current Calculator

## Overview
Calculates via electrical properties: capacitance, inductance, impedance,
DC resistance, current capacity, thermal resistance, and more.

## Via Types (Layer Set)
- **2 Layer** - simple through-hole via (capacitance forced to 1.0 pF)
- **Multi Layer** - via through multiple layers (enables capacitance/impedance calcs)
- **Microvia** - small laser-drilled via (base formula only, no modifiers)

## Inputs
- **Via Hole Diameter** - finished hole diameter (mils/mm)
- **Internal Pad Diameter** - inner layer pad diameter (mils/mm, divided by 1000 → inches)
- **Ref Plane Opening Diameter** - anti-pad opening (mils/mm, divided by 1000 → inches)
- **Via Height** - length from start to end layer (mils/mm, divided by 1000 → inches)
- **Via Plating Thickness** - copper plating thickness (mils/mm)
- **Er** - dielectric constant
- **Temp Rise** - maximum allowed temperature rise (1-200°C)
- **Ambient Temp** - ambient temperature (-80 to 200°C)

## Formulas (verified from decompilation)

### 1. Via Barrel Cross-Section Area (sq.mils)
```
d_outer = d_hole + 2 * T_plating
A_barrel = (pi/4) * (d_outer^2 - d_hole^2)
```

### 2. Via Capacitance (pF) — Multi-Layer Only
```
C_via = 1.41 * Er * h * D_pad / (D_anti - D_pad)
```
Where:
- `1.41` = Goldfarb empirical constant (at 0x004342dc)
- `h` = via height in inches
- `D_pad` = internal pad diameter in inches
- `D_anti` = antipad (ref plane opening) diameter in inches
- 2-Layer mode: forced to 1.0 pF
- Clamped to 0.0 if negative

### 3. Via Inductance (nH)
```
L_via = 5.08 * h * (ln(4*h/d) + 1)
```
Where:
- `5.08` = constant at 0x004342e8
- `h` = via height in inches
- `d` = via hole diameter in inches (d_hole / 1000)
- Clamped to 0.0 if negative

### 4. Via Impedance (Ohms) — Multi-Layer Only
```
Z_via = sqrt(L_nH / (C_pF * 0.001))
```
The 0.001 converts pF→nF so that nH/nF → H/F → sqrt gives Ohms.

### 5. Resonant Frequency (MHz) — Multi-Layer Only
```
f_res = 1 / (2*pi * sqrt(L_H * C_F)) / 1e6
```
Where `L_H = L_nH * 1e-9`, `C_F = C_pF * 1e-12`.

### 6. Step Response T10-90% (ps)
```
T_step = 2.2 * C_pF * Z_via / 2.0 = 1.1 * C_pF * Z_via
```

### 7. DC Resistance (Ohms)
```
R_dc = rho_adjusted * via_height / A_barrel
```
Where `rho_adjusted = rho_base * (1 + 0.00393 * (T_via - 20.0))`.

### 8. Copper Resistivity Lookup Table
| Temperature Range | rho_base (Ohm*mil) |
|-------------------|---------------------|
| T <= -40°C        | 0.0005190000        |
| -40 < T <= -20°C  | 0.0005720000        |
| -20 < T <=   0°C  | 0.0006250000        |
|   0 < T <=  20°C  | 0.0006787000        |
|  20 < T <=  40°C  | 0.0007320000        |
|  40 < T <=  60°C  | 0.0007850000        |
|  60 < T <=  80°C  | 0.0008390000        |
|       T >  80°C   | 0.0008390000 (last) |

### 9. Current Capacity (Amps) — IPC-2221A Internal
```
I = 0.024 * dT^0.44 * A_barrel^0.725 * modifiers
```
Mode-specific modifiers:
- **2-Layer**: `M_area * M_plating * M_roughness`
- **Multi-Layer**: `M_area * M_roughness` (no plating modifier)
- **Microvia**: no modifiers (base formula only)

### 10. Plating Thickness Modifier Table
| Via Height Range | Modifier |
|-----------------|----------|
| H > 100 mil     | 1.30     |
| 90 < H <= 100   | 1.20     |
| 80 < H <=  90   | 1.10     |
| 70 < H <=  80   | 1.00     |
| 60 < H <=  70   | 0.95     |
| 50 < H <=  60   | 0.85     |
| 40 < H <=  50   | 0.75     |
| 30 < H <=  40   | 0.67     |
| 20 < H <=  30   | 0.58     |
| 10 < H <=  20   | 0.48     |
| H <= 10 mil     | 0.40     |

### 11. Other Outputs
- **Power Dissipation**: `P = I^2 * R_dc` (Watts), `P_dBm = 10*log10(P/0.001)`
- **Voltage Drop**: `V = I * R_dc * 1000` (mV)
- **Current Density**: `J = I / (A_barrel * 6.4516e-10)` (A/m²)
- **Thermal Resistance**: `R_th = via_height * 1e6 / (A_barrel * 10.008)` (°C/W)
- **Aspect Ratio**: `AR = via_height / via_hole_diameter`

## Differential Vias / Via Stub Length (Tab 1)

Implements Bert Simonovich's method for differential via modeling.

### Inputs
- Drill Hole Diameter
- Ref Plane Opening H / W
- Via Spacing (center to center)
- Anisotropy
- Baud Rate

### Formulas (from decompilation)
```
Er_eff = Er * (1 + anisotropy/200)
Z_odd = 60/sqrt(Er_eff) * ln(D_ref/D_hole + ...) * ...   (complex expression)
Z_diff = 2 * Z_odd
```

### Outputs
- Differential impedance
- Effective dielectric constant
- Odd mode impedance
- Insertion loss estimate (uses factor 0.84)
- Maximum Stub Length

### References
- Bert Simonovich - "Method of Modeling Differential Vias"
- "Dispelling Via Stub Anxieties"
- Dankov et al. - "Two-Resonator Method for Characterization of Dielectric Substrate Anisotropy"

## Binary Constants (from decompilation)
| Address | Value | Purpose |
|---------|-------|---------|
| 0x004342dc | 1.41 | Goldfarb via capacitance constant |
| 0x004342e8 | 5.08 | Via inductance constant (nH/inch) |
| 0x00434324 | 0.00393 | Cu temperature coefficient |
| 0x0043432c | 10.008 | Thermal conductivity factor |
| 0x004342d0 | 3.14159 | pi |
| 0x00434300 | 6.28318 | 2*pi |
| 0x00434314 | 2.2 | Step response time constant |
| 0x00434344 | 0.84 | Insertion loss factor |

Full decompilation details in `ghidra-via-properties.md`
