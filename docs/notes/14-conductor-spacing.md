# Minimum Conductor Spacing (IPC-2221C)

## Overview
Displays minimum conductor spacing based on voltage and device type,
per the IPC-2221C specification. Handler at `ConductorSpacing1Click` = `0x004940CC`.

## Inputs
- **Voltage Between Conductors** - predefined ranges:
  - 0-15V, 16-30V, 31-50V, 51-100V, 101-150V, 151-170V,
  - 171-250V, 251-300V, 301-500V, >500V (custom entry)
- **Device Type Selection** (IPC-2221C categories):
  - B1 = Internal Conductors
  - B2 = External Conductors, Uncoated, Sea Level to 3050m
  - B3 = External Conductors, Uncoated, Over 3050m or in a Vacuum
  - B4 = External Conductors Covered With Solder Mask (Any Elevation)
  - B5 = External Conductors, Coated (Any Elevation or in a Vacuum)
  - A6 = External Component Lead, Coated (Any Elevation or in a Vacuum)
  - A7 = External Component Lead, Uncoated, Sea Level to 3050m
  - A8 = External Component Lead, Uncoated, Over 3050m or in a Vacuum

## Output
- **Minimum Conductor Spacing** (mils/mm/um) - per IPC-2221C Table 6-1

## Spacing Values Extracted from Binary (offset 0x47321c)

### 20 Unique Spacing Values (as stored in binary, both mil and mm format):

| Mils | mm | um |
|------|------|------|
| 1.97 | 0.050 | 50 |
| 2.95 | 0.075 | 75 |
| 3.94 | 0.100 | 100 |
| 5.12 | 0.130 | 130 |
| 7.87 | 0.200 | 200 |
| 9.84 | 0.250 | 250 |
| 11.81 | 0.300 | 300 |
| 15.75 | 0.400 | 400 |
| 19.69 | 0.500 | 500 |
| 25.17 | 0.640 | 640 |
| 31.50 | 0.800 | 800 |
| 39.37 | 1.000 | 1000 |
| 49.21 | 1.250 | 1250 |
| 59.06 | 1.500 | 1500 |
| 62.99 | 1.600 | 1600 |
| 98.43 | 2.500 | 2500 |
| 118.11 | 3.000 | 3000 |
| 125.98 | 3.200 | 3200 |
| 251.97 | 6.400 | 6400 |
| 492.13 | 12.500 | 12500 |

### String Storage Format
Values stored as three parallel string representations:
- Dot-decimal: "1.97 mils", "0.05 mm", "50 um"
- Comma-decimal: "1,97 mils", "0,05 mm" (European locale)
- All null-terminated at binary offset 0x47321c-0x473598

### IPC-2221C Table 6-1 (CONFIRMED via Ghidra decompilation)

Full details in `ghidra-conductor-spacing.md`. Table confirmed via disassembly of `FUN_00498e84`.

**All values in mm:**

| Type | 0-15V | 16-30V | 31-50V | 51-100V | 101-150V | 151-170V | 171-250V | 251-300V | 301-500V | >500V |
|------|-------|--------|--------|---------|----------|----------|----------|----------|----------|-------|
| B1   | 0.050 | 0.050  | 0.100  | 0.100   | 0.200    | 0.200    | 0.200    | 0.200    | 0.250    | formula |
| B2   | 0.100 | 0.100  | 0.640  | 0.640   | 0.640    | 1.250    | 1.250    | 1.250    | 2.500    | formula |
| B3   | 0.100 | 0.100  | 0.640  | 1.500   | 3.200    | 3.200    | 6.400    | 12.500   | 12.500   | formula |
| B4   | 0.075 | 0.075  | 0.300  | 0.300   | 0.800    | 0.800    | 0.800    | 0.800    | 1.600    | formula |
| B5   | 0.075 | 0.075  | 0.130  | 0.130   | 0.400    | 0.400    | 0.400    | 0.400    | 0.800    | formula |
| A6   | 0.130 | 0.130  | 0.130  | 0.130   | 0.400    | 0.400    | 0.400    | 0.400    | 0.800    | formula |
| A7   | 0.130 | 0.250  | 0.400  | 0.500   | 0.800    | 0.800    | 0.800    | 0.800    | 1.500    | formula |
| A8   | 0.130 | 0.250  | 0.800  | 1.000   | 1.600    | 1.600    | 1.600    | 1.600    | 3.000    | formula |

**>500V Formula**: `spacing_mm = (V - 500) * M + C` where V is user-entered voltage (must be >= 501V).
Per-device slopes (M) and intercepts (C):

| Device | M (mm/V) | C (mm) |
|--------|----------|--------|
| B1 | 0.0025 | 0.25 |
| B2 | 0.005 | 2.50 |
| B3 | 0.025 | 12.50 |
| B4 | 0.00305 | 1.60 |
| B5 | 0.00305 | 0.80 |
| A6 | 0.00305 | 0.80 |
| A7 | 0.00305 | 1.50 |
| A8 | 0.0061 | 3.00 |

## Binary Details
- DFM device type strings at offset: `0xBE6390` (RadioGroup10)
- DFM voltage range strings at offset: `0xBE64E2` (RadioGroup9)
- Spacing value strings (mils) at offset: `0x0087461C`
- Spacing value strings (mm) at offset: `0x008747D0`
- Handler function: `ConductorSpacing1Click` at `0x004940CC`
- Lookup function: `FUN_00498e84` (~60KB, dispatched via `DAT_008d5f88 == 9`)
- FPU constants for >500V formulas at: `0x004A7B34` - `0x004A7BEC`

## Implementation Notes
- Pure lookup table for voltages 0-500V (80 fixed string values for 8 devices x 10 ranges)
- For >500V, user enters custom voltage; linear formula computes spacing
- B5 and A6 share identical >500V formulas (same slope and intercept)
- B4, B5, A6, A7 share the same >500V slope (0.00305 mm/V) but different intercepts
- All values stored as pre-formatted strings with locale variants (dot and comma decimal)
- The function is too large for Ghidra's decompiler; analyzed via disassembly
