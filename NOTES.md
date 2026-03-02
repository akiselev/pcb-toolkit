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
| Button1Click | 0x00403398 | **Main dispatcher** - reads DAT_008d5f88 and calls solver |
| ComboBox1Change | 0x00494dd4 | Material Er/Tg selector (sets roughness factor 0.98 or 1.0) |

### Menu/Tab Click Handlers (set DAT_008d5f88 mode and call dispatcher)
| Handler | Address | Sets Mode |
|---------|---------|-----------|
| SignalProperties1Click | 0x0049021c | 0 (Microstrip) |
| RFImpedances1Click | 0x004a8bb8 | 1 (Stripline/RF) |
| ConductorProperties1Click | 0x0048f1e4 | 2 (Conductor Current) |
| DifferentialPairs1Click | 0x00490f44 | 4 (Differential Pairs) |
| EmbeddedRs1Click | 0x004ba9d0 | 5 (Embedded Microstrip) |
| ErEffective1Click | 0x004c25fc | 6 (Er Effective) |
| FuseCurrent1Click | 0x004b7334 | 7 (Fusing Current) |
| ConductorSpacing1Click | 0x004940cc | 9 (Conductor Spacing) |
| OhmsLaw1Click | 0x004c3514 | 10 (Ohm's Law) |
| Padstacks1Click | 0x00491a2c | 11 (Padstack) |
| PlanarInductors1Click | 0x004b04d4 | 13 (Planar Inductor) |
| PPMCalculator1Click | 0x004bbf34 | 14 (PPM Calculator) |
| PDNImpedance1Click | 0x004b14d0 | 12 (PDN Impedance) |
| ThermalManagement1Click | 0x004b1fe4 | 15 (Thermal Management) |
| ViaProperties1Click | 0x0048e4c4 | 16 (Via Properties) |
| WavelengthCalculator1Click | 0x004be8fc | 17 (Wavelength) |
| XlXCReactance1Click | 0x004d59d0 | 18 (Reactance) |
| CrosstalkCalculator1Click | 0x004bde04 | N/A (separate form) |
| ConversionData1Click | 0x004afb40 | 3 (Edge Coupled External) |
| Mechanical1Click | 0x00492768 | 8 (Broadside Coupled) |
| CopperWeight1Click | 0x004b4e88 | N/A (separate form) |

### Solver Functions (Dispatched by Button1Click_MainDispatcher based on DAT_008d5f88)

**CORRECTED** - Mode mapping verified by tracing menu click handlers to DAT_008d5f88 assignments.

| Mode | Address | Ghidra Name | Calculator | Verified By |
|------|---------|-------------|-----------|-------------|
| 0 | 0x00440e34 | Solver_Microstrip | **Microstrip Impedance** | SignalProperties1Click |
| 1 | 0x0040bc00 | Solver_Stripline | **Stripline Impedance** | RFImpedances1Click |
| 2 | 0x004343e4 | Solver_ConductorCurrent | **Conductor Current** (IPC-2152/2221A) | ConductorProperties1Click |
| 3 | 0x00422ddc | Solver_EdgeCoupledExternal | **Edge Coupled External** | ConversionData1Click |
| 4 | 0x004435f8 | Solver_DifferentialPairs | **Differential Pairs** | DifferentialPairs1Click |
| 5 | 0x004066b0 | Solver_EmbeddedMicrostrip | **Embedded Microstrip** | EmbeddedRs1Click |
| 6 | 0x00403f40 | Solver_ErEffective_REAL | **Er Effective** | ErEffective1Click |
| 7 | 0x004b8104 | Solver_FusingCurrent | **Fusing Current** (Onderdonk) | FuseCurrent1Click |
| 8 | 0x00482648 | Solver_BroadsideCoupledNonShielded | **Wire Gauge Properties** (UI: "Broad Cpld NShld") | Mechanical1Click |
| 9 | 0x00498e84 | Solver_ConductorSpacing | **Conductor Spacing** (IPC-2221C) | ConductorSpacing1Click |
| 10 | 0x004c41d8 | Solver_OhmsLaw | **Ohm's Law / E-I-R** | OhmsLaw1Click |
| 11 | 0x0045fc54 | Solver_Padstack | **Padstack Calculator** | Padstacks1Click |
| 12 | 0x00408b68 | Solver_PDNImpedance_12 | **PDN Impedance** | PDNImpedance1Click |
| 13 | 0x0040a0f4 | Solver_PlanarInductor | **Planar Inductor** | PlanarInductors1Click |
| 14 | 0x004bca88 | Solver_PPMCalculator | **PPM / XTAL Calculator** | PPMCalculator1Click |
| 15 | 0x004081b0 | Solver_ThermalManagement | **Thermal Management** | ThermalManagement1Click |
| 16 | 0x00427090 | Solver_ViaProperties | **Via Properties** | ViaProperties1Click |
| 17 | 0x004bf410 | Solver_Wavelength | **Wavelength Calculator** | WavelengthCalculator1Click |
| 18 | 0x004d662c | Solver_Reactance | **Reactance (Xc/Xl)** | XlXCReactance1Click |

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

### Er Values — FULLY MAPPED from ComboBox1Change disassembly

23 unique Er values shared non-sequentially across 44 materials. Full mapping in
`docs/notes/materials-er-mapping.md`. Key corrections from earlier (sequential) assumption:
- FR406 = 4.6 (NOT 3.8), Isola P95/P96 = 3.78 (NOT 6.15/10.2)
- RO3006 = 6.15, RO3010 = 10.2, RO4003 = 3.38, RO4350 = 3.66
- Bug: RO4350 has Er=3.66 (impedance form) vs 3.48 (crosstalk form)

Sorted unique values:
```
2.1, 2.2, 2.35, 2.5, 2.94, 3.0, 3.38, 3.4, 3.66, 3.7, 3.78, 3.8,
3.9, 4.1, 4.15, 4.2, 4.25, 4.3, 4.38, 4.5, 4.6, 6.15, 10.2
```

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
| FUN_00867350 | ln(x) — natural log via FLDLN2 + FYL2X |
| FUN_008673c0 | log10(x) — common logarithm (NOT sqrt) |
| FUN_008675ac | pow(x, y) — x^y via 2^(y*log2(x)) (NOT ln) |
| FUN_00868834 | sqrt(x) — via FSQRT |
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

K1/K2: Square=2.34/2.75, Hex=2.33/3.82, Oct=2.25/3.55, Circle=2.275/3.575
Note: Saturn uses non-standard circular shape constants (published: 2.23/3.45)
```

### Fusing Current (Onderdonk)
```
I = A * sqrt(log10(1 + (Tm-Ta)/(234+Ta)) / (33*t))
Tm = 1084.62°C (copper melting point, NOT 1064.62 which is gold)
A = cross-section in circular mils
```

### Conductor Current (IPC-2221A Legacy / IPC-2152 Base)
```
External: I = 0.048 * dT^0.44 * A^0.725
Internal: I = 0.024 * dT^0.44 * A^0.725
A = cross-section in sq.mils
dT = temperature rise in °C
```

IPC-2152 with modifiers:
```
External: I = base * M_area * M_temp * M_board * M_material * M_user
Internal: I = base * M_area * M_temp / plane_dist * M_board * M_material * M_user
```
Where M_area is piecewise power law, M_temp/M_board are lookup tables.
Full details in `docs/notes/ghidra-conductor-current.md`.

### Ohm's Law (5 sub-modes)
```
E-I-R: V=IR, I=V/R, R=V/I, P=VI
LED Bias: R = (Vsup - Vled) / (Iled_mA / 1000), W = (Vsup - Vled) * Iled_A
Voltage Divider: Vout = Vin * R2 / (R1 + R2)
Parallel R: R_total = 1 / (1/R1 + 1/R2 + ...)
```

### Pi-Pad Attenuator (unmatched)
```
K = 10^(dB/20)
R1 = Zmax * (K²-1) / (K² - 2K*sqrt(Zmax/Zmin) + 1)
R2 = Zmin * (K²-1) / (K² - 2K*sqrt(Zmin/Zmax) + 1)
R3 = sqrt(Zmax*Zmin) * (K²-1) / (2K)
```

### T-Pad Attenuator (unmatched)
```
K = 10^(dB/20)
R3 = 2K * sqrt(Zmax*Zmin) / (K²-1)
R1 = Zmax * (K²+1)/(K²-1) - R3
R2 = Zmin * (K²+1)/(K²-1) - R3
```

### Planar Inductor (Mohan/Wheeler Modified — Saturn-specific)
```
Lmw = K1 * µ₀ * n² * d_avg / (1 + K2 * ρ)
Saturn K1/K2: Square=2.34/2.75, Hex=2.33/3.82, Oct=2.25/3.55, Circle=2.275/3.575
Published K1/K2 for Circle: 2.23/3.45 (Saturn differs!)
```

### Stripline Impedance (Mode 1)

Saturn uses a **hybrid** approach combining Cohn/Wadell/IPC-2141A with proprietary
empirical correction coefficients. Full analysis in `docs/notes/stripline-formulas-clean.md`.

**Core formula approaches identified:**
- **Wadell finite-thickness**: `Z0 = (30/√Er) * ln(1 + A*(2A + √(4A² + 6.27)))` where `A = 4(B-T)/(πW_eff)`
- **IPC-2141A simplified**: `Z0 = (60/√Er) * ln(4B / (0.67π(0.8W + T)))`
- **Cohn zero-thickness**: `Z0 = (η₀/(4√Er)) * K(k')/K(k)` where `k = sech(πW/(2B))`
- **Cohn/Steer with fringing Cf**: `Z0 = (30π/√Er) * (1-t/b)/(w_eff/b + Cf)`

**Key discrepancies from published formulas:**
- Fringe factor: Saturn uses 0.432, published is 2*ln(2)/π = 0.4413 (~2% diff)
- Er correction: Saturn uses `ln((Er-0.9)/(Er+3))*0.564`, H-J uses `0.564*((Er-0.9)/(Er+3))^0.053`
- Proprietary polynomial coefficients: 0.525, 0.6315, 0.27488, -8.7513, 0.065683, etc.
- Coupled stripline uses Cohn conformal mapping (tanh/coth) with corrections: 1.023, 1.0235, 0.5008, 1.1564, 0.4749

**Coupled/Differential Stripline** (Cohn 1955):
```
ke = tanh(πW/(2B)) * tanh(π(W+S)/(2B))    # even mode modulus
ko = tanh(πW/(2B)) * coth(π(W+S)/(2B))    # odd mode modulus (note: /tanh → coth)
Z0e = (η₀/(4√Er)) * K(k'_e)/K(k_e)
Z0o = (η₀/(4√Er)) * K(k'_o)/K(k_o)
Zdiff = 2*Z0o, Zcomm = Z0e/2
```

**Propagation delay**: `Tpd = √Er_eff * 1000/11.8` ps/inch (11.8 = c in in/ns)

**60+ binary constants mapped** at 0x00422bd8–0x00422dd4 (see stripline-formulas-clean.md §8).

### Wire Gauge Properties (Mode 8, UI: "Broad Cpld NShld")
```
area_result = diameter_mils^2 / 700.0
result = (val_A * wire_resistance) / 1000.0 * val_B
```
44 AWG entries (4/0 through 40) with diameter and DC resistance lookup tables.
NOT an impedance formula despite the UI label.

### PDN Impedance (Mode 12)
```
Z_target = (V_supply * V_ripple_pct/100) / (I_max * I_step_pct/100)
C_plane  = 0.225 * Er * A_sqin / (d_mils / 1000)     [pF, imperial]
C_plane  = 0.008858 * Er * A_mm2 / d_mm               [pF, metric]
Xc       = 1 / (2*pi * f_Hz * C_F)                    [Ohms, skipped if DC]
```
Constant 0.225 at 0x0040a0c8 is epsilon_0 in pF for sq.in/inch units.

### Thermal Management (Mode 15)
```
T_junction = R_theta_ja * P_dissipated + T_ambient
```
Two independent channels (A and B), each computing junction temp from thermal resistance
(C/W) and power dissipation (W). Sub-mode controls formatting only.

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

### Via Properties (from decompiled Solver_ViaProperties, 0x00427090)
```
Cross Section: A = pi/4 * ((d+2t)^2 - d^2)
Capacitance:   C = 1.41 * Er * h * D_pad / (D_anti - D_pad)   [pF, Goldfarb constant]
Inductance:    L = 5.08 * h * (ln(4h/d) + 1)                  [nH, h/d in inches]
Impedance:     Z = sqrt(L_nH / (C_pF * 0.001))                [Ohms]
Resonant Freq: f = 1/(2pi*sqrt(L_H * C_F)) / 1e6              [MHz]
Step Response:  T = 2.2 * C_pF * Z / 2.0                      [ps]
DC Resistance:  R = rho(T) * h / A   (with temp-corrected resistivity)
Current:        I = 0.024 * dT^0.44 * A^0.725 * modifiers     [IPC-2221A internal]
```
Note: 4/π (1.2732 at 0x004BA940) is used in Solver_FusingCurrent, NOT the via calculator.
Via capacitance uses 1.41 (Goldfarb constant at 0x004342dc).

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
