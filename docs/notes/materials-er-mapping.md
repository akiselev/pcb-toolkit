# Complete Material Er/Tg/Roughness Mapping

## Source

Extracted from `ComboBox1Change` handler at `0x00494dd4` via disassembly of
Saturn PCB Toolkit v8.44. Dropdown item names confirmed from DFM resource
(Delphi form) `Items.Strings` at file offsets `0x7b4694` and `0x8e7758`.

## Method

The function contains a chain of 45 `if (ItemIndex == N)` blocks (indices 0-44),
each setting:
- **Er** string to field at `[self + 0x67c]` via string constant PUSH
- **Tg** string to field at `[self + 0x680]` via string constant PUSH
- **Roughness** double at `DAT_008d6478/008d647c` (0.98 or 1.0)

Two identical copies of the if-chain exist in the function:
1. **Impedance form** (dot-decimal Er strings, e.g. "3.66")
2. **Crosstalk form** (comma-decimal Er strings, e.g. "3,48")

Index 45 (Custom) is not handled by the if-chain; it falls through, leaving
user-entered values in place.

## Important Corrections to Previous Notes

The old `15-materials-data.md` assumed Er strings were assigned sequentially in
memory order. **This was wrong for materials 3-23.** The actual mapping from
disassembly shows that materials reuse string constants in a non-sequential
pattern. Only materials 1-2 (FR-4 STD, FR-5) had correct Er values in the old
notes. All other materials 3-23 had incorrect Er assignments.

## Complete Material Table (46 entries + Custom)

| Index | Material        | Er    | Tg (C) | Roughness | Er String Addr | Notes |
|------:|-----------------|------:|-------:|----------:|:--------------:|-------|
|     0 | FR-4 STD        |  4.6  |    130 |      0.98 | 0x8738b6       | |
|     1 | FR-5            |  4.3  |    170 |      0.98 | 0x8744eb       | |
|     2 | FR406           |  4.6  |    170 |      0.98 | 0x8738b6       | Same Er addr as FR-4 STD |
|     3 | FR408           |  3.8  |    180 |      0.98 | 0x8744f3       | |
|     4 | Getek ML200C    |  3.8  |    175 |      0.98 | 0x8744f3       | Same Er addr as FR408 |
|     5 | Getek ML200D    |  3.9  |    175 |      0.98 | 0x8744fb       | |
|     6 | Getek ML200M    |  3.8  |    175 |      0.98 | 0x8744f3       | Same Er addr as FR408 |
|     7 | Getek RG200D    |  4.2  |    175 |      0.98 | 0x8744ff       | |
|     8 | Isola P95       |  3.78 |    260 |      1.00 | 0x874503       | |
|     9 | Isola P96       |  3.78 |    260 |      1.00 | 0x874503       | Same Er/Tg as Isola P95 |
|    10 | Isola P26N      |  3.9  |    250 |      1.00 | 0x8744fb       | Same Er addr as Getek ML200D |
|    11 | RO2800          |  2.94 |    N/A |      1.00 | 0x87450c       | Rogers |
|    12 | RO3003          |  3.0  |    N/A |      1.00 | 0x87155c       | Rogers |
|    13 | RO3006          |  6.15 |    N/A |      1.00 | 0x874511       | Rogers |
|    14 | RO3010          | 10.2  |    N/A |      1.00 | 0x874516       | Rogers |
|    15 | RO4003          |  3.38 |    280 |      1.00 | 0x87451b       | Rogers |
|    16 | RO4350          |  3.66 |    280 |      1.00 | 0x874524       | Rogers; crosstalk form uses 3.48 (bug?) |
|    17 | RT5500          |  2.5  |    260 |      1.00 | 0x8725fb       | Rogers |
|    18 | RT5870          |  2.35 |    260 |      1.00 | 0x874529       | Rogers |
|    19 | RT5880          |  2.2  |    260 |      1.00 | 0x87452e       | Rogers |
|    20 | RT6002          |  2.94 |    N/A |      1.00 | 0x87450c       | Rogers; same Er addr as RO2800 |
|    21 | RT6006          |  6.15 |    N/A |      1.00 | 0x874511       | Rogers; same Er addr as RO3006 |
|    22 | RT6010          | 10.2  |    N/A |      1.00 | 0x874516       | Rogers; same Er addr as RO3010 |
|    23 | Teflon PTFE     |  2.1  |    240 |      1.00 | 0x874532       | |
|    24 | Arlon 25N       |  3.38 |    260 |      1.00 | 0x87451b       | Same Er addr as RO4003 |
|    25 | Arlon 33N       |  4.25 |    250 |      1.00 | 0x87453a       | |
|    26 | Arlon 85N       |  4.2  |    250 |      1.00 | 0x8744ff       | Same Er addr as Getek RG200D |
|    27 | PCL-FR-226      |  4.5  |    140 |      0.98 | 0x87453f       | |
|    28 | PCL-FR-240      |  4.5  |    140 |      0.98 | 0x87453f       | Same Er/Tg as PCL-FR-226 |
|    29 | PCL-FR-370      |  4.5  |    175 |      0.98 | 0x87453f       | Same Er addr as PCL-FR-226 |
|    30 | PCL-FR-370HR    |  4.6  |    180 |      0.98 | 0x8738b6       | Same Er addr as FR-4 STD |
|    31 | N4000-7 EF      |  4.1  |    165 |      0.98 | 0x874547       | Nelco |
|    32 | N4000-13        |  3.7  |    210 |      0.98 | 0x87454f       | Nelco |
|    33 | N4000-13SI      |  3.4  |    210 |      0.98 | 0x874557       | Nelco |
|    34 | N4000-13 EP     |  3.7  |    210 |      0.98 | 0x87454f       | Nelco; same Er addr as N4000-13 |
|    35 | N4000-13 EPSI   |  3.4  |    210 |      0.98 | 0x874557       | Nelco; same Er addr as N4000-13SI |
|    36 | N4000-29        |  4.5  |    185 |      0.98 | 0x87453f       | Nelco; same Er addr as PCL-FR-226 |
|    37 | N7000-1         |  3.9  |    260 |      1.00 | 0x8744fb       | Nelco; same Er addr as Getek ML200D |
|    38 | Ventec VT-47    |  4.6  |    180 |      0.98 | 0x8738b6       | Same Er addr as FR-4 STD |
|    39 | Ventec VT-901   |  4.15 |    250 |      1.00 | 0x87455b       | |
|    40 | Ventec VT-90H   |  4.15 |    250 |      1.00 | 0x87455b       | Same Er/Tg as VT-901 |
|    41 | Megtron6        |  3.4  |    185 |      1.00 | 0x874557       | Panasonic; same Er addr as N4000-13SI |
|    42 | Kappa 438       |  4.38 |    280 |      1.00 | 0x874560       | |
|    43 | Kapton          |  3.4  |    400 |      1.00 | 0x874557       | Polyimide film; same Er addr as N4000-13SI |
|    44 | Air             |  1.0  |    N/A |      1.00 | 0x87137a       | Blocked for some calculators (mode 6 special case) |
|    45 | Custom          |  user |   user |      user | N/A            | Falls through if-chain; user-editable |

## Unique Er Values

23 unique dielectric constant values across all 44 non-Air materials:

```
2.1, 2.2, 2.35, 2.5, 2.94, 3.0, 3.38, 3.4, 3.66, 3.7, 3.78, 3.8,
3.9, 4.1, 4.15, 4.2, 4.25, 4.3, 4.38, 4.5, 4.6, 6.15, 10.2
```

## Unique Tg Values

12 unique glass transition temperature values (plus "N/A"):

```
130, 140, 165, 170, 175, 180, 185, 210, 240, 250, 260, 280, 400
```

## Roughness Factor

Stored as IEEE 754 double at `DAT_008d6478/008d647c`, applied as a multiplier
to impedance results.

- **0.98** (`0x3fef5c28f5c28f5c`): FR-4 type materials with copper surface roughness penalty
  - Indices 0-7 (FR-4 STD through Getek RG200D)
  - Indices 27-36 (PCL-FR series, N4000 series, N4000-29)
  - Index 38 (Ventec VT-47)

- **1.0** (`0x3ff0000000000000`): Smooth/ideal materials (PTFE, Rogers, specialty)
  - Indices 8-26 (Isola, Rogers, Teflon PTFE, Arlon)
  - Index 37 (N7000-1)
  - Indices 39-44 (Ventec VT-901/90H, Megtron6, Kappa 438, Kapton, Air)

## Tg Address Mapping

| Tg String Addr | Value | Used by indices |
|:---------------|------:|-----------------|
| 0x8744e7       |   130 | 0 |
| 0x874543       |   140 | 27, 28 |
| 0x87454b       |   165 | 31 |
| 0x8744ef       |   170 | 1, 2 |
| 0x873958       |   175 | 4, 5, 6, 7, 29 |
| 0x8744f7       |   180 | 3, 30, 38 |
| 0x873934       |   185 | 36, 41 |
| 0x874553       |   210 | 32, 33, 34, 35 |
| 0x874536       |   240 | 23 |
| 0x871912       |   250 | 10, 25, 26, 39, 40 |
| 0x874508       |   260 | 8, 9, 17, 18, 19, 24, 37 |
| 0x874520       |   280 | 15, 16, 42 |
| 0x8721d7       |   400 | 43 |
| 0x87130a       |   N/A | 11, 12, 13, 14, 20, 21, 22, 44 |

## Er String Duplication (Impedance vs Crosstalk Form)

The function contains two copies of the material if-chain:
1. **First half**: dot-decimal strings (e.g. "4.6") for the impedance calculator
2. **Second half**: comma-decimal strings (e.g. "4,6") for the crosstalk calculator

All Er values match between the two halves **except** index 16 (RO4350):
- Impedance form: Er = **3.66** (string at `0x874524`)
- Crosstalk form: Er = **3.48** (string at `0x8745b9` = "3,48")

This appears to be a data entry bug in the Saturn Toolkit. The impedance value
(3.66) matches Rogers' published RO4350 datasheet.

## Air Special Case

When index 44 (Air) is selected and `DAT_008d5f88 == 6` (crosstalk calculator
mode), the handler displays the error message "Air cannot be selected for this
calculator" (string at `0x874565`) and resets the ComboBox to index 0.

## Corrections to 15-materials-data.md

The following materials had **incorrect** Er values in the original notes, which
assumed sequential Er assignment from the string table. The disassembly reveals
the actual non-sequential mapping:

| Material     | Old (wrong) Er | Actual Er | Correction |
|:-------------|---------------:|----------:|:-----------|
| FR406        |           3.8  |      4.6  | Uses same Er string as FR-4 STD |
| FR408        |           3.9  |      3.8  | Shifted by one |
| Getek ML200C |           4.2  |      3.8  | Same Er as FR408 |
| Getek ML200D |           3.78 |      3.9  | |
| Getek ML200M |           2.94 |      3.8  | Same Er as FR408 |
| Getek RG200D |           3.0  |      4.2  | |
| Isola P95    |           6.15 |      3.78 | |
| Isola P96    |          10.2  |      3.78 | Same Er as P95 |
| Isola P26N   |           3.38 |      3.9  | |
| RO2800       |           3.66 |      2.94 | |
| RO3003       |           2.5  |      3.0  | |
| RO3006       |           2.35 |      6.15 | |
| RO3010       |           2.2  |     10.2  | |
| RO4003       |           2.1  |      3.38 | |
| RO4350       |           4.25 |      3.66 | |
| RT5500       |           4.5  |      2.5  | |
| RT5870       |           4.1  |      2.35 | |
| RT5880       |           3.7  |      2.2  | |
| RT6002       |           3.4  |      2.94 | |
| RT6006       |           4.15 |      6.15 | |
| RT6010       |           4.38 |     10.2  | |

The Tg and roughness values in the original notes were also partially incorrect
due to the same sequential assumption. The table above provides the definitive
mapping from disassembly.
