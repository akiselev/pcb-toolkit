# Material/Substrate Data Tables

## Overview
Shared across all calculators via the "Substrate Options" / "Material Selection"
dropdown in the Options panel. The ComboBox1Change handler at `0x00494dd4`
processes 45 materials (indices 0-44) plus Air and Custom.

## Complete Material Database (46 items + Custom)

### Materials 1-23 (Er values extracted as strings at binary offsets 0x4730e7 and 0x4b9ae8)

| #  | Material         | Er    | Tg (°C) | Roughness | Notes |
|----|------------------|-------|---------|-----------|-------|
| 1  | FR-4 STD         | 4.6   | 130     | 0.98      | Er hardcoded, Tg from string |
| 2  | FR-5             | 4.3   | 170     | 0.98      | High-Tg FR-4 variant |
| 3  | FR406            | 3.8   | 180     | 0.98      | Isola FR406 |
| 4  | FR408            | 3.9   | -       | 0.98      | Isola FR408 |
| 5  | Getek ML200C     | 4.2   | -       | 0.98      | |
| 6  | Getek ML200D     | 3.78  | 260     | 0.98      | High-performance |
| 7  | Getek ML200M     | 2.94  | -       | 0.98      | |
| 8  | Getek RG200D     | 3.0   | -       | 0.98      | Er hardcoded as float |
| 9  | Isola P95        | 6.15  | -       | 1.0       | |
| 10 | Isola P96        | 10.2  | -       | 1.0       | |
| 11 | Isola P26N       | 3.38  | 280     | 1.0       | |
| 12 | RO2800           | 3.66  | -       | 1.0       | Rogers |
| 13 | RO3003           | 2.5   | -       | 1.0       | Er hardcoded as float |
| 14 | RO3006           | 2.35  | -       | 1.0       | Rogers |
| 15 | RO3010           | 2.2   | -       | 1.0       | Rogers |
| 16 | RO4003           | 2.1   | 240     | 1.0       | Rogers |
| 17 | RO4350           | 4.25  | -       | 1.0       | Rogers |
| 18 | RT5500           | 4.5   | 140     | 1.0       | Rogers |
| 19 | RT5870           | 4.1   | 165     | 1.0       | Rogers |
| 20 | RT5880           | 3.7   | 210     | 1.0       | Rogers |
| 21 | RT6002           | 3.4   | -       | 1.0       | Rogers |
| 22 | RT6006           | 4.15  | -       | 1.0       | Rogers |
| 23 | RT6010           | 4.38  | -       | 1.0       | Rogers |

### Materials 24-44 (Er values reuse same 23 string constants; exact mapping needs disassembly)

| #  | Material         | Er    | Tg (°C) | Roughness | Notes |
|----|------------------|-------|---------|-----------|-------|
| 24 | Teflon PTFE      | ?     | -       | 1.0       | Likely Er≈2.1 |
| 25 | Arlon 25N        | ?     | -       | 1.0       | |
| 26 | Arlon 33N        | ?     | -       | 1.0       | |
| 27 | Arlon 85N        | ?     | -       | 1.0       | |
| 28 | PCL-FR-226       | ?     | -       | 0.98      | FR-4 variant |
| 29 | PCL-FR-240       | ?     | -       | 0.98      | FR-4 variant |
| 30 | PCL-FR-370       | ?     | -       | 0.98      | FR-4 variant |
| 31 | PCL-FR-370HR     | ?     | -       | 0.98      | FR-4 variant |
| 32 | N4000-7 EF       | ?     | -       | 0.98      | Nelco |
| 33 | N4000-13         | ?     | -       | 0.98      | Nelco |
| 34 | N4000-13SI       | ?     | -       | 0.98      | Nelco |
| 35 | N4000-13 EP      | ?     | -       | 0.98      | Nelco |
| 36 | N4000-13 EPSI    | ?     | -       | 0.98      | Nelco |
| 37 | N4000-29         | ?     | -       | 0.98      | Nelco |
| 38 | N7000-1          | ?     | -       | 1.0       | Nelco (low-loss) |
| 39 | Ventec VT-47     | ?     | -       | 0.98      | |
| 40 | Ventec VT-901    | ?     | -       | 1.0       | |
| 41 | Ventec VT-90H    | ?     | -       | 1.0       | |
| 42 | Megtron6         | ?     | -       | 1.0       | Panasonic (low-loss) |
| 43 | Kappa 438        | ?     | -       | 1.0       | |
| 44 | Kapton           | ?     | -       | 1.0       | Polyimide film |
| 45 | Air              | 1.0   | N/A     | special   | Air=1.0, blocked for some calcs |
| 46 | Custom           | user  | user    | user      | User-editable |

### Roughness Factor (from decompiled ComboBox1Change at 0x00494dd4)

Stored as double at `DAT_008d6478/008d647c`, applied as multiplier to impedance.

- **0.98** = FR-4 type materials with copper surface roughness penalty
  - Materials 1-8 (FR-4 STD through Getek RG200D)
  - Materials 28-37 (PCL-FR series, N4000 series)
  - Material 39 (Ventec VT-47)
- **1.0** = Ideal/smooth materials (Rogers, Teflon, specialty)
  - Materials 9-27 (Isola P95 through Arlon 85N)
  - Material 38 (N7000-1)
  - Materials 40-44 (Ventec VT-901/90H, Megtron6, Kappa 438, Kapton)
  - Material 45 (Air)

### Tg Values (9 values extracted from binary string area 0x4730e7)

Values appear as null-terminated strings interleaved with Er values:
130, 170, 180, 260, 280, 240, 140, 165, 210

Mapping confirmed for materials 1-3 (FR-4: 130°C, FR-5: 170°C, FR406: 180°C).
Remaining Tg-to-material mapping tentative (needs disassembly verification).

### Er String Storage

- **23 unique Er string values** at two locations:
  - `0x4730e7` - used by impedance calculator form
  - `0x4b9ae8` - used by crosstalk form (identical values)
- **3 hardcoded Er values** (4.6, 3.0, 2.5) - as float constants in code
- Materials 24-44 reuse the same 23 Er string values (some materials share Er)
- European locale duplicates (comma-decimal) follow each set

### Er Values (dot-decimal string sequence at 0x4b9ae8)
```
4.6, 4.3, 3.8, 3.9, 4.2, 3.78, 2.94, 3.0, 6.15, 10.2, 3.38, 3.66,
2.5, 2.35, 2.2, 2.1, 4.25, 4.5, 4.1, 3.7, 3.4, 4.15, 4.38
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
| 4/π | 1.2732 | 0x004BA940 | Via capacitance geometry |
| In-to-cm | 2.54 | 0x004435BC, 0x004BFF74 | Unit conversion |
| Mil-to-m | 2.54e-5 | 0x004435DC | Unit conversion |
| H-J constant a | 0.457 | 0x004435C4 | Kirschning-Jansen dispersion |
| H-J constant b | 0.67 | 0x004435CC | Kirschning-Jansen dispersion |
| Copper roughness | 0.98 | DAT_008d6478 | FR-4 surface roughness factor |
| Copper melting point | 1064.62°C | | Onderdonk equation |
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
