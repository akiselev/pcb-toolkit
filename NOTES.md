# Saturn PCB Toolkit - Reverse Engineering Master Notes

## Binary Info
- **File**: `toolkit/toolkit.exe`, PE32 Windows GUI, Intel i386, ~10.4 MB
- **Compiler**: Embarcadero C++ Builder / RAD Studio (Delphi VCL) - NOT .NET
- **Version**: 8.44 (Jan 5, 2026)
- **Ghidra analysis**: 14,689 functions (when successfully analyzed)

## Architecture
- Single main form `TForm1` with tabbed interface
- Additional forms: TForm2, TForm4, TForm5, TForm6, TForm8, TForm9, TForm10, TForm11, TForm12, TForm100
- Event-driven: `ButtonNClick` handlers trigger calculations
- Global mode selector at `DAT_008d5f88` (int) dispatches to 19 solver functions
- Main dispatch function: `FUN_00403398` at address `0x00403398`

## Detailed Notes
See `docs/notes/` for per-calculator documentation (00-overview.md through 15-materials-data.md).
See `docs/notes/ghidra-impedance.md` for decompiled impedance calculator code.

---

## Handler Function Addresses (Verified from Delphi Published Method Table)

### Main Calculation Handlers
| Handler | Address | Purpose |
|---------|---------|---------|
| ComboBox1Change | 0x00494dd4 | Material Er/Tg selector (sets roughness factor 0.98 or 1.0) |
| ComboBox1Change | 0x004f4900 | Material selector (2nd form) |
| Button3Click | 0x004a7dc0 | Solve button (main calc form) |
| Button3Click | 0x004f6ce0 | Solve button (2nd form) |
| Button3Click | 0x00507fb0 | Solve button (3rd form) |
| Button1Click | 0x00403398 | **Main dispatcher** - reads mode and calls solver |
| Button2Click | 0x004715dc | Button2 handler |
| RadioGroup2Click | 0x004b32c4 | Circuit type selector |
| RadioGroup5Click | 0x004b3998 | Another radio group |
| ConductorSpacing1Click | 0x004940CC | Conductor spacing tab handler |

### Solver Functions (Dispatched by FUN_00403398 based on DAT_008d5f88)
| Mode | Address | Calculator |
|------|---------|-----------|
| 0 | FUN_00440e34 | **Microstrip** (decompiled - see ghidra-impedance.md) |
| 1 | FUN_0040bc00 | Stripline (too large to decompile) |
| 2 | FUN_004343e4 | Differential Layer (too large) |
| 3 | FUN_00422ddc | Edge Coupled External |
| 4 | FUN_004435f8 | Edge Coupled Internal Symmetric |
| 5 | FUN_004066b0 | Edge Coupled Internal Asymmetric |
| 6 | FUN_00403f40 | Edge Coupled Embedded |
| 7 | FUN_004b8104 | Broadside Coupled Shielded (decompiled) |
| 8 | FUN_00482648 | Broadside Coupled Non-Shielded |
| 9 | FUN_00498e84 | (Microstrip Embed?) |
| 10 | FUN_004c41d8 | (Stripline Asymmetric?) |
| 11 | FUN_0045fc54 | (Dual Stripline?) |
| 12 | FUN_00408b68 | Coplanar Waveguide |
| 13 | FUN_0040a0f4 | Unknown |
| 14 | FUN_004bca88 | Unknown |
| 15 | FUN_004081b0 | Unknown |
| 16 | FUN_00427090 | Er Effective display (too large) |
| 17 | FUN_004bf410 | **Wavelength Calculator** (decompiled) |
| 18 | FUN_004d662c | Unknown |

### Common Pre-computation Functions (called before every solver)
- FUN_00471678
- FUN_00471e7c
- FUN_00471a08
- FUN_004e0928

---

## Material Database (46 items, from DFM ComboBox Items.Strings)

Complete ordered dropdown list extracted from binary at offset 0x7b4684:

| # | Material | Er | Tg (°C) |
|---|----------|-----|---------|
| 1 | FR-4 STD | 4.6 | 130 |
| 2 | FR-5 | 4.3 | ? |
| 3 | FR406 | 3.8 | ? |
| 4 | FR408 | 3.9 | ? |
| 5 | Getek ML200C | 4.2 | ? |
| 6 | Getek ML200D | 3.78 | ? |
| 7 | Getek ML200M | 2.94 | ? |
| 8 | Getek RG200D | 3.0 | ? |
| 9 | Isola P95 | 6.15 | ? |
| 10 | Isola P96 | 10.2 | ? |
| 11 | Isola P26N | 3.38 | ? |
| 12 | RO2800 | 3.66 | ? |
| 13 | RO3003 | 2.5 | ? |
| 14 | RO3006 | 2.35 | ? |
| 15 | RO3010 | 2.2 | ? |
| 16 | RO4003 | 2.1 | ? |
| 17 | RO4350 | 4.25 | ? |
| 18 | RT5500 | 4.5 | ? |
| 19 | RT5870 | 4.1 | ? |
| 20 | RT5880 | 3.7 | ? |
| 21 | RT6002 | 3.4 | ? |
| 22 | RT6006 | 4.15 | ? |
| 23 | RT6010 | 4.38 | ? |
| 24 | Teflon PTFE | ? | ? |
| 25 | Arlon 25N | ? | ? |
| 26 | Arlon 33N | ? | ? |
| 27 | Arlon 85N | ? | ? |
| 28 | PCL-FR-226 | ? | ? |
| 29 | PCL-FR-240 | ? | ? |
| 30 | PCL-FR-370 | ? | ? |
| 31 | PCL-FR-370HR | ? | ? |
| 32 | N4000-7 EF | ? | ? |
| 33 | N4000-13 | ? | ? |
| 34 | N4000-13SI | ? | ? |
| 35 | N4000-13 EP | ? | ? |
| 36 | N4000-13 EPSI | ? | ? |
| 37 | N4000-29 | ? | ? |
| 38 | N7000-1 | ? | ? |
| 39 | Ventec VT-47 | ? | ? |
| 40 | Ventec VT-901 | ? | ? |
| 41 | Ventec VT-90H | ? | ? |
| 42 | Megtron6 | ? | ? |
| 43 | Kappa 438 | ? | ? |
| 44 | Kapton | ? | ? |
| 45 | Air | 1.0 | N/A |
| 46 | Custom | user | user |

### Er Values Extracted (23 values at binary offset 0x4b9ae8)
Raw string sequence (dot-decimal, then comma-decimal duplicate for European locale):
```
4.6, 4.3, 3.8, 3.9, 4.2, 3.78, 2.94, 3.0, 6.15, 10.2, 3.38, 3.66,
2.5, 2.35, 2.2, 2.1, 4.25, 4.5, 4.1, 3.7, 3.4, 4.15, 4.38
```

**NOTE**: 23 Er values for 44 materials (46 minus Air and Custom). The mapping is done via switch/case in `ComboBox1Change` at `0x00494dd4` - some materials share Er values, or the mapping isn't 1:1. The table above shows a TENTATIVE direct 1-to-1 mapping for the first 23 materials; the remaining 21 materials' Er values are set by later case branches that haven't been decompiled.

### Material Roughness Correction (from ComboBox1Change decompilation)
- **0.98** for standard FR-4 variants (surface roughness penalty)
- **1.0** for ideal/smooth materials (Rogers, Teflon, etc.)
Stored as double at `DAT_008d6478/008d647c`, applied as multiplier to impedance.

---

## Verified Physical Constants (from binary)

| Constant | Value | Address(es) | Usage |
|----------|-------|-------------|-------|
| Speed of light | 299,792,458 m/s | 0x004435AC, 0x004BFF64 | Wavelength, propagation delay |
| 4/pi | 1.2732 | 0x004BA940 | Via capacitance geometry |
| In-to-cm | 2.54 | 0x004435BC, 0x004BFF74 | Unit conversion |
| Mil-to-m | 2.54e-5 | 0x004435DC | Unit conversion |
| H-J constant a | 0.457 | 0x004435C4 | Kirschning-Jansen dispersion |
| H-J constant b | 0.67 | 0x004435CC | Kirschning-Jansen dispersion |
| Copper roughness | 0.98 | DAT_008d6478 | FR-4 surface roughness factor |
| Min dimension | 0.0078125 (1/128) | 0x004BA928 | Threshold |

---

## Copper Weight to Thickness Tables (from binary)

### Mils mode (9 entries, from FUN_004b8104)
| Index | Thickness (mils) | Approx. Weight |
|-------|-----------------|----------------|
| 0 | 0.35 | 0.25 oz |
| 1 | 0.70 | 0.5 oz |
| 2 | 1.40 | 1 oz |
| 3 | 2.10 | 1.5 oz |
| 4 | 2.80 | 2 oz |
| 5 | 3.50 | 2.5 oz |
| 6 | 4.20 | 3 oz |
| 7 | 5.60 | 4 oz |
| 8 | 7.00 | 5 oz |

### mm mode (9 entries)
| Index | Thickness (mm) | Approx. Weight |
|-------|---------------|----------------|
| 0 | 0.009 | 0.25 oz |
| 1 | 0.018 | 0.5 oz |
| 2 | 0.035 | 1 oz |
| 3 | 0.053 | 1.5 oz |
| 4 | 0.070 | 2 oz |
| 5 | 0.088 | 2.5 oz |
| 6 | 0.106 | 3 oz |
| 7 | 0.142 | 4 oz |
| 8 | 0.178 | 5 oz |

---

## IPC-2221C Conductor Spacing Categories (from DFM)

### Device Types (8 items at binary offset 0x7b300c)
1. B1 - Bare PCB (Internal Conductors)
2. B2 - Bare PCB (External, Uncoated, Sea Level to 3050m)
3. B3 - Bare PCB (External, Uncoated, Over 3050m or Vacuum)
4. B4 - Bare PCB (External, Solder Mask, Any Elevation)
5. B5 - Bare PCB (External, Coated, Any Elevation or Vacuum)
6. A6 - Assembly (Component Lead, Coated, Any Elevation or Vacuum)
7. A7 - Assembly (Component Lead, Uncoated, Sea Level to 3050m)
8. A8 - Assembly (Component Lead, Uncoated, Over 3050m or Vacuum)

### Voltage Ranges (10 items at binary offset 0x7b317b)
0-15V, 16-30V, 31-50V, 51-100V, 101-150V, 151-170V, 171-250V, 251-300V, 301-500V, >500V

---

## Differential Pair Protocol Presets (18 items at binary offset 0x783121)

| # | Protocol | Target Zdiff (Ω) |
|---|----------|------------------|
| 1 | DDR2 CLK/DQS | 100 |
| 2 | DDR3 CLK/DQS | 100 |
| 3 | DDR4 CLK/DQS | 80 |
| 4 | USB 2.X | 90 |
| 5 | USB 3.X | 90 |
| 6 | LVDS | 100 |
| 7 | HDMI | 100 |
| 8 | SATA | 100 |
| 9 | Ethernet | 100 |
| 10 | DisplayPort | 100 |
| 11 | DisplayPort Eaglelake | 85 |
| 12 | DisplayPort Calpella | 85 |
| 13 | PCIe Gen1 | 100 |
| 14 | PCIe Gen2 | 85 |
| 15 | PCIe Gen3 | 85 |
| 16 | PCIe Gen4 | 85 |
| 17 | SSRX/SSTX | 85 |
| 18 | Custom | user |

---

## Helper Function Address Map

| Address | Purpose |
|---------|---------|
| FUN_0086b424 | ResourceString loader |
| FUN_0086ecf0 | Stack string builder |
| FUN_0086ee90 | String cleanup / refcount decrement |
| FUN_0086f0f4 | StrToFloat conversion |
| FUN_005396c8 | TEdit.SetText |
| FUN_00539678 | TEdit.GetText |
| FUN_0053957c | TControl.SetVisible |
| FUN_00538980 | TControl.SetTop |
| FUN_00538914 | TControl.SetWidth |
| FUN_005dff68 | ShowMessage / error dialog |
| FUN_008673c0 | sqrt() |
| FUN_008675ac | ln() / log() |
| FUN_00861e48 | FloatToStr |
| FUN_0071d964 | Format / FormatFloat |
| FUN_00805c4c | TRegistry.Create |
| FUN_00805f00 | TRegistry.OpenKey |
| FUN_00806860 | TRegistry.WriteString |

---

## Key Formulas Summary

### Microstrip Impedance (Hammerstad-Jensen 1980)
```
Er_eff = (Er+1)/2 + (Er-1)/2 * F(W/H)
  F(u) = (1 + 12/u)^(-0.5) + 0.04*(1-u)^2   for u=W/H <= 1
  F(u) = (1 + 12/u)^(-0.5)                     for u=W/H > 1

Zo (W/H<=1) = (60/sqrt(Er_eff)) * ln(8H/We + We/4H)
Zo (W/H>1)  = (120π/sqrt(Er_eff)) / (We/H + 1.393 + 0.667*ln(We/H + 1.444))

Thickness correction:
  dW = (T/π)(1 + ln(2H/T))     for W/H >= π/2
  dW = (T/π)(1 + ln(4πW/T))    for W/H < π/2
  We = W + dW

Dispersion (Kirschning-Jansen): constants 0.457 and 0.67
```

### Derived Quantities
```
Tpd = sqrt(Er_eff) / c          (propagation delay)
Lo = Zo * Tpd                    (inductance/length)
Co = Tpd / Zo                    (capacitance/length)
c = 299,792,458 m/s = 11.803 in/ns
```

### Wavelength
```
λ = c / (f * sqrt(Er_eff))
```

### Planar Inductor (Mohan et al. Modified Wheeler)
```
Lmw = K1 * μ₀ * n² * d_avg / (1 + K2 * ρ)
ρ = (d_out - d_in) / (d_out + d_in)    (fill factor)
d_avg = (d_out + d_in) / 2

K1/K2: Square=2.34/2.75, Hex=2.33/3.82, Oct=2.25/3.55, Circle=2.23/3.45
```

### Fusing Current (Onderdonk)
```
I = A * sqrt(log10(1 + (Tm-Ta)/(234+Ta)) / (33*t))
Tm = 1064.62°C (copper melting point)
A = cross-section in circular mils
```

### Conductor Current (IPC-2221A Legacy)
```
External: I = 0.048 * dT^0.44 * A^0.725
Internal: I = 0.024 * dT^0.44 * A^0.725
A = cross-section in sq.mils
dT = temperature rise in °C
```

### Reactance
```
Xc = 1 / (2πfC)
Xl = 2πfL
f_res = 1 / (2π*sqrt(LC))
```

### PPM / Hz Conversion
```
PPM = (Variation / Center_freq) × 1,000,000
Variation = Center_freq × PPM / 1,000,000
```

### XTAL Load Capacitor
```
C_load = (C1*C2)/(C1+C2) + C_stray
Rule of thumb: C1 = C2 = 2*(C_load - C_stray)
```

### Via Capacitance (from decompiled FUN_004b8104)
```
C_via uses 4/π = 1.2732 geometric correction for cylindrical geometry
Based on parallel-plate approximation: C = ε₀ * εr * π*D*L / (4*d)
```

---

## Padstack Sub-Calculators (7 types, from DFM at offset 0x606367)
1. Thru-Hole Pad
2. BGA Land Size
3. Conductor / Pad TH
4. Conductor / Pad BGA
5. 2 Conductors / Pad TH
6. 2 Conductor / Pad BGA
7. Corner to Corner

---

## Plating Thickness Options (from DFM at offset 0x7b3bfa)
Bare PCB, 0.5oz, 1oz, 1.5oz, 2oz, 2.5oz, 3oz

## Copper Weight Options (from DFM at offset 0x7b3d85)
0.25oz, 0.5oz, 1oz, 1.5oz, 2oz, 2.5oz, 3oz, 4oz, 5oz

---

## AWG Wire Gauge Table (44 entries at offset 0x793f33)
4/0, 3/0, 2/0, 1/0, 1 through 40

---

## References Found in Binary Resources
1. E. Hammerstad & O. Jensen, "Accurate Models for Microstrip Computer-Aided Design", IEEE MTT-S 1980
2. Douglas Brooks, "Embedded Microstrip, Impedance Formula"
3. Brian C. Wadell, "Transmission Line Design Handbook"
4. Eric Bogatin, "Bogatin's Practical Guide to Prototype Breadboard and PCB Design"
5. Howard Johnson & Martin Graham, "High-Speed Digital Design Handbook"
6. Mohan et al., "Simple Accurate Expressions for Planar Spiral Inductances", Stanford JSSC 1999
7. Bert Simonovich, Differential Via Modeling
8. IPC-2152, IPC-2221/2221A/2221C, IPC-7351A, IPC-2316, IPC-2141A
