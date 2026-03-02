# Material/Substrate Data Tables

## Overview
Shared across all calculators via the "Substrate Options" / "Material Selection"
dropdown in the Options panel. The ComboBox1Change handler at `0x00494dd4`
processes 45 materials (indices 0-44) plus Air and Custom.

## Complete Material Database (46 items + Custom)

**CORRECTED** from disassembly of ComboBox1Change at 0x00494dd4. Previous notes assumed
sequential Er string assignment, which was wrong for 21 of 23 materials.
Full disassembly details in `materials-er-mapping.md`.

| #  | Material         | Er    | Tg (°C) | Roughness | Notes |
|----|------------------|------:|--------:|----------:|-------|
| 1  | FR-4 STD         |  4.60 |     130 |      0.98 | |
| 2  | FR-5             |  4.30 |     170 |      0.98 | |
| 3  | FR406            |  4.60 |     170 |      0.98 | Same Er as FR-4 STD |
| 4  | FR408            |  3.80 |     180 |      0.98 | |
| 5  | Getek ML200C     |  3.80 |     175 |      0.98 | Same Er as FR408 |
| 6  | Getek ML200D     |  3.90 |     175 |      0.98 | |
| 7  | Getek ML200M     |  3.80 |     175 |      0.98 | Same Er as FR408 |
| 8  | Getek RG200D     |  4.20 |     175 |      0.98 | |
| 9  | Isola P95        |  3.78 |     260 |      1.00 | |
| 10 | Isola P96        |  3.78 |     260 |      1.00 | Same Er/Tg as P95 |
| 11 | Isola P26N       |  3.90 |     250 |      1.00 | Same Er as Getek ML200D |
| 12 | RO2800           |  2.94 |     N/A |      1.00 | Rogers |
| 13 | RO3003           |  3.00 |     N/A |      1.00 | Rogers |
| 14 | RO3006           |  6.15 |     N/A |      1.00 | Rogers |
| 15 | RO3010           | 10.20 |     N/A |      1.00 | Rogers |
| 16 | RO4003           |  3.38 |     280 |      1.00 | Rogers |
| 17 | RO4350           |  3.66 |     280 |      1.00 | Rogers; crosstalk form uses 3.48 (bug) |
| 18 | RT5500           |  2.50 |     260 |      1.00 | Rogers |
| 19 | RT5870           |  2.35 |     260 |      1.00 | Rogers |
| 20 | RT5880           |  2.20 |     260 |      1.00 | Rogers |
| 21 | RT6002           |  2.94 |     N/A |      1.00 | Rogers; same Er as RO2800 |
| 22 | RT6006           |  6.15 |     N/A |      1.00 | Rogers; same Er as RO3006 |
| 23 | RT6010           | 10.20 |     N/A |      1.00 | Rogers; same Er as RO3010 |
| 24 | Teflon PTFE      |  2.10 |     240 |      1.00 | |
| 25 | Arlon 25N        |  3.38 |     260 |      1.00 | Same Er as RO4003 |
| 26 | Arlon 33N        |  4.25 |     250 |      1.00 | |
| 27 | Arlon 85N        |  4.20 |     250 |      1.00 | Same Er as Getek RG200D |
| 28 | PCL-FR-226       |  4.50 |     140 |      0.98 | |
| 29 | PCL-FR-240       |  4.50 |     140 |      0.98 | Same Er/Tg as PCL-FR-226 |
| 30 | PCL-FR-370       |  4.50 |     175 |      0.98 | Same Er as PCL-FR-226 |
| 31 | PCL-FR-370HR     |  4.60 |     180 |      0.98 | Same Er as FR-4 STD |
| 32 | N4000-7 EF       |  4.10 |     165 |      0.98 | Nelco |
| 33 | N4000-13         |  3.70 |     210 |      0.98 | Nelco |
| 34 | N4000-13SI       |  3.40 |     210 |      0.98 | Nelco |
| 35 | N4000-13 EP      |  3.70 |     210 |      0.98 | Nelco; same Er as N4000-13 |
| 36 | N4000-13 EPSI    |  3.40 |     210 |      0.98 | Nelco; same Er as N4000-13SI |
| 37 | N4000-29         |  4.50 |     185 |      0.98 | Nelco; same Er as PCL-FR-226 |
| 38 | N7000-1          |  3.90 |     260 |      1.00 | Nelco; same Er as Getek ML200D |
| 39 | Ventec VT-47     |  4.60 |     180 |      0.98 | Same Er as FR-4 STD |
| 40 | Ventec VT-901    |  4.15 |     250 |      1.00 | |
| 41 | Ventec VT-90H    |  4.15 |     250 |      1.00 | Same Er/Tg as VT-901 |
| 42 | Megtron6         |  3.40 |     185 |      1.00 | Panasonic; same Er as N4000-13SI |
| 43 | Kappa 438        |  4.38 |     280 |      1.00 | |
| 44 | Kapton           |  3.40 |     400 |      1.00 | Polyimide; same Er as N4000-13SI |
| 45 | Air              |  1.00 |     N/A |      1.00 | Blocked for crosstalk (mode 6) |
| 46 | Custom           |  user |    user |      user | User-editable |

### Roughness Factor (from decompiled ComboBox1Change at 0x00494dd4)

Stored as double at `DAT_008d6478/008d647c`, applied as multiplier to impedance.

- **0.98** (`0x3fef5c28f5c28f5c`): FR-4 type materials with copper surface roughness penalty
  - Indices 0-7 (FR-4 STD through Getek RG200D)
  - Indices 27-36 (PCL-FR series, N4000 series, N4000-29)
  - Index 38 (Ventec VT-47)
- **1.0** (`0x3ff0000000000000`): Smooth/ideal materials (PTFE, Rogers, specialty)
  - Indices 8-26 (Isola, Rogers, Teflon PTFE, Arlon)
  - Index 37 (N7000-1)
  - Indices 39-44 (Ventec VT-901/90H, Megtron6, Kappa 438, Kapton, Air)

### Tg Values (13 unique values, FULLY MAPPED from disassembly)

| Tg (°C) | Materials |
|---------|-----------|
| 130 | FR-4 STD |
| 140 | PCL-FR-226, PCL-FR-240 |
| 165 | N4000-7 EF |
| 170 | FR-5, FR406 |
| 175 | Getek ML200C/D/M, Getek RG200D, PCL-FR-370 |
| 180 | FR408, PCL-FR-370HR, Ventec VT-47 |
| 185 | N4000-29, Megtron6 |
| 210 | N4000-13, N4000-13SI, N4000-13 EP, N4000-13 EPSI |
| 240 | Teflon PTFE |
| 250 | Isola P26N, Arlon 33N, Arlon 85N, Ventec VT-901, Ventec VT-90H |
| 260 | Isola P95, Isola P96, RT5500, RT5870, RT5880, Arlon 25N, N7000-1 |
| 280 | RO4003, RO4350, Kappa 438 |
| 400 | Kapton |
| N/A | RO2800, RO3003, RO3006, RO3010, RT6002, RT6006, RT6010, Air |

### Er String Storage

- **23 unique Er values** used across all 44 non-Air materials
- Two copies in binary: dot-decimal (impedance form) and comma-decimal (crosstalk form)
- Materials share Er strings non-sequentially (e.g., FR406 reuses FR-4 STD's string)
- **Bug**: RO4350 has Er=3.66 in impedance form but Er=3.48 in crosstalk form

### 23 Unique Er Values (sorted)
```
2.1, 2.2, 2.35, 2.5, 2.94, 3.0, 3.38, 3.4, 3.66, 3.7, 3.78, 3.8,
3.9, 4.1, 4.15, 4.2, 4.25, 4.3, 4.38, 4.5, 4.6, 6.15, 10.2
```

---

## Copper Weight to Thickness Conversion

### Mils mode (9 entries, from FUN_004b8104)
| Weight | Thickness (mils) | Thickness (mm) |
|--------|-----------------|----------------|
| 0.25oz | 0.35            | 0.00889        |
| 0.5oz  | 0.70            | 0.01778        |
| 1oz    | 1.40            | 0.03556        |
| 1.5oz  | 2.10            | 0.05334        |
| 2oz    | 2.80            | 0.07112        |
| 2.5oz  | 3.50            | 0.08890        |
| 3oz    | 4.20            | 0.10668        |
| 4oz    | 5.60            | 0.14224        |
| 5oz    | 7.00            | 0.17780        |

### mm mode (9 entries)
| Weight | Thickness (mm) |
|--------|---------------|
| 0.25oz | 0.009         |
| 0.5oz  | 0.018         |
| 1oz    | 0.035         |
| 1.5oz  | 0.053         |
| 2oz    | 0.070         |
| 2.5oz  | 0.088         |
| 3oz    | 0.106         |
| 4oz    | 0.142         |
| 5oz    | 0.178         |

## Plating Thickness Options
- Bare PCB (0)
- 0.5oz, 1oz, 1.5oz, 2oz, 2.5oz, 3oz
- Max plating: 3 mils
- Min plating: 0.5 mils

---

## Physical Constants (used across calculators, verified from binary)

| Constant | Value | Binary Address | Usage |
|----------|-------|---------------|-------|
| Speed of light | 299,792,458 m/s | 0x004435AC, 0x004BFF64 | Wavelength, propagation delay |
| Speed of light | 11.803 in/ns | derived | Alternative unit |
| 4/π | 1.2732 | 0x004BA940 | Fusing current (Solver_FusingCurrent only, NOT via) |
| In-to-cm | 2.54 | 0x004435BC, 0x004BFF74 | Unit conversion |
| Mil-to-m | 2.54e-5 | 0x004435DC | Unit conversion |
| H-J constant a | 0.457 | 0x004435C4 | Kirschning-Jansen dispersion |
| H-J constant b | 0.67 | 0x004435CC | Kirschning-Jansen dispersion |
| Copper roughness | 0.98 | DAT_008d6478 | FR-4 surface roughness factor |
| Copper melting point | 1084.62°C | 0x004ba930 | Onderdonk equation (NOT 1064.62 which is gold) |
| Copper resistivity | 1.724e-6 Ω·cm | | DC resistance |
| Copper temp coeff | 0.00393 /°C | | Temperature adjustment |
| µ₀ | 4π × 10⁻⁷ H/m | | Skin depth, inductance |
| ε₀ | 8.854 × 10⁻¹² F/m | | Capacitance |
| Min dimension | 0.0078125 (1/128) | 0x004BA928 | Threshold |

## Unit Conversions
- 1 mil = 0.001 inch = 0.0254 mm = 25.4 µm
- 1 oz copper ≈ 1.40 mils thickness (impedance calcs) / 1.37 mils (current calcs)
- 1 inch = 25.4 mm
- Temperature: °F = °C × 9/5 + 32
