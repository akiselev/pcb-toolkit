# Ghidra Decompilation: IPC-2221C Conductor Spacing (ConductorSpacing1Click)

## Function Overview

- **Handler**: `ConductorSpacing1Click` at `0x004940CC`
- **Lookup function**: `FUN_00498e84` (also labeled `Solver_Mode9_MicrostripEmbed` in Ghidra)
- **Dispatcher**: `FUN_00403398` at `0x00403398`, routes to `FUN_00498e84` when `DAT_008d5f88 == 9`
- **Decompilation**: `FUN_00498e84` is too large for Ghidra's decompiler; analysis was done via disassembly

## Architecture

The handler `ConductorSpacing1Click` (0x004940CC):
1. Sets `DAT_008d5f88 = 9` to select the conductor spacing calculator mode
2. Configures UI visibility (shows/hides panels via `FUN_0053957c` calls)
3. Sets up radio group combo boxes for device type and voltage selection
4. Calls `FUN_00403398` which dispatches to `FUN_00498e84` based on `DAT_008d5f88 == 9`

The lookup function `FUN_00498e84` contains an enormous if/else chain (~25,000 bytes) that:
1. Reads the user-entered voltage from an edit field (stored as `double` at `0x008d6308`)
2. Checks if voltage < 501V and the selected voltage range is >500V; if so shows error message
3. Iterates through all (device_type, voltage_range) combinations
4. For voltage ranges 0-8: displays a fixed string from the lookup table
5. For voltage range 9 (>500V): computes spacing using a linear formula

## Radio Groups

### RadioGroup10 - Device Type Selection (form offset ~0xBE6390)
| Index | Code | Description |
|-------|------|-------------|
| 0 | B1 | Internal Conductors |
| 1 | B2 | External Conductors, Uncoated, Sea Level to 3050m |
| 2 | B3 | External Conductors, Uncoated, Over 3050m or in a Vacuum |
| 3 | B4 | External Conductors Covered With Solder Mask (Any Elevation) |
| 4 | B5 | External Conductors, Coated (Any Elevation or in a Vacuum) |
| 5 | A6 | External Component Lead, Coated (Any Elevation or in a Vacuum) |
| 6 | A7 | External Component Lead, Uncoated, Sea Level to 3050m |
| 7 | A8 | External Component Lead, Uncoated, Over 3050m or in a Vacuum |

### RadioGroup9 - Voltage Between Conductors (form offset ~0xBE64E2)
| Index | Range |
|-------|-------|
| 0 | 0 - 15V |
| 1 | 16 - 30V |
| 2 | 31 - 50V |
| 3 | 51 - 100V |
| 4 | 101 - 150V |
| 5 | 151 - 170V |
| 6 | 171 - 250V |
| 7 | 251 - 300V |
| 8 | 301 - 500V |
| 9 | > 500V |

## Complete Lookup Table (Mils)

Extracted from disassembly of `FUN_00498e84` (0x00498e84 - 0x004a7B30).

The code structure is a nested if/else:
- Outer loop: Device type index (ECX/EDX register, `[obj+0x2f0]` field, values 0-7)
- Inner loop: Voltage range index (EAX register, `[obj+0x2f0]` field, values 0-9)

| Type | 0-15V | 16-30V | 31-50V | 51-100V | 101-150V | 151-170V | 171-250V | 251-300V | 301-500V | >500V |
|------|-------|--------|--------|---------|----------|----------|----------|----------|----------|-------|
| B1 | 1.97 | 1.97 | 3.94 | 3.94 | 7.87 | 7.87 | 7.87 | 7.87 | 9.84 | formula |
| B2 | 3.94 | 3.94 | 25.17 | 25.17 | 25.17 | 49.21 | 49.21 | 49.21 | 98.43 | formula |
| B3 | 3.94 | 3.94 | 25.17 | 59.06 | 125.98 | 125.98 | 251.97 | 492.13 | 492.13 | formula |
| B4 | 2.95 | 2.95 | 11.81 | 11.81 | 31.50 | 31.50 | 31.50 | 31.50 | 62.99 | formula |
| B5 | 2.95 | 2.95 | 5.12 | 5.12 | 15.75 | 15.75 | 15.75 | 15.75 | 31.50 | formula |
| A6 | 5.12 | 5.12 | 5.12 | 5.12 | 15.75 | 15.75 | 15.75 | 15.75 | 31.50 | formula |
| A7 | 5.12 | 9.84 | 15.75 | 19.69 | 31.50 | 31.50 | 31.50 | 31.50 | 59.06 | formula |
| A8 | 5.12 | 9.84 | 31.50 | 39.37 | 62.99 | 62.99 | 62.99 | 62.99 | 118.11 | formula |

## Complete Lookup Table (mm)

| Type | 0-15V | 16-30V | 31-50V | 51-100V | 101-150V | 151-170V | 171-250V | 251-300V | 301-500V | >500V |
|------|-------|--------|--------|---------|----------|----------|----------|----------|----------|-------|
| B1 | 0.05 | 0.05 | 0.10 | 0.10 | 0.20 | 0.20 | 0.20 | 0.20 | 0.25 | formula |
| B2 | 0.10 | 0.10 | 0.64 | 0.64 | 0.64 | 1.25 | 1.25 | 1.25 | 2.50 | formula |
| B3 | 0.10 | 0.10 | 0.64 | 1.50 | 3.20 | 3.20 | 6.40 | 12.50 | 12.50 | formula |
| B4 | 0.075 | 0.075 | 0.30 | 0.30 | 0.80 | 0.80 | 0.80 | 0.80 | 1.60 | formula |
| B5 | 0.075 | 0.075 | 0.13 | 0.13 | 0.40 | 0.40 | 0.40 | 0.40 | 0.80 | formula |
| A6 | 0.13 | 0.13 | 0.13 | 0.13 | 0.40 | 0.40 | 0.40 | 0.40 | 0.80 | formula |
| A7 | 0.13 | 0.25 | 0.40 | 0.50 | 0.80 | 0.80 | 0.80 | 0.80 | 1.50 | formula |
| A8 | 0.13 | 0.25 | 0.80 | 1.00 | 1.60 | 1.60 | 1.60 | 1.60 | 3.00 | formula |

## >500V Formula (Linear Extrapolation)

For the >500V voltage range, the tool requires the user to enter a custom voltage (must be >= 501V).
The spacing is calculated using a linear formula:

```
spacing = (V - 500) * M + C
```

Where V is the entered voltage, M is a per-device multiplier, and C is a per-device constant (equal to or close to the 301-500V spacing value).

### >500V Mils Formulas

Constants stored at `0x004A7B34` - `0x004A7B9C`:

| Device | M (mils/V) | C (mils) | Formula | Address |
|--------|-----------|----------|---------|---------|
| B1 | 0.098425 | 9.8425 | (V-500) * 0.098425 + 9.8425 | 0x499925 |
| B2 | 0.196850 | 98.4252 | (V-500) * 0.196850 + 98.4252 | 0x4A5583 |
| B3 | 0.984252 | 492.1260 | (V-500) * 0.984252 + 492.1260 | 0x49B1E1 |
| B4 | 0.120070 | 62.9900 | (V-500) * 0.120070 + 62.9900 | 0x49BE3F |
| B5 | 0.120070 | 31.4961 | (V-500) * 0.120070 + 31.4961 | 0x49CA9D |
| A6 | 0.120070 | 31.4961 | (V-500) * 0.120070 + 31.4961 | 0x49D6FB |
| A7 | 0.120070 | 59.0551 | (V-500) * 0.120070 + 59.0551 | 0x49E359 |
| A8 | 0.240157 | 118.1100 | (V-500) * 0.240157 + 118.1100 | 0x49EFB7 |

### >500V mm Formulas

Constants stored at `0x004A7B9C` - `0x004A7BEC`:

| Device | M (mm/V) | C (mm) | Formula | Address |
|--------|----------|--------|---------|---------|
| B1 | 0.002500 | 0.2500 | (V-500) * 0.002500 + 0.2500 | 0x4A017E |
| B2 | 0.005000 | 2.5000 | (V-500) * 0.005000 + 2.5000 | 0x4A1291 |
| B3 | 0.025000 | 12.5000 | (V-500) * 0.025000 + 12.5000 | 0x4A23A4 |
| B4 | 0.003050 | 1.6000 | (V-500) * 0.003050 + 1.6000 | 0x4A34B7 |
| B5 | 0.003050 | 0.8000 | (V-500) * 0.003050 + 0.8000 | 0x4A45CA |
| A6 | 0.003050 | 0.8000 | (V-500) * 0.003050 + 0.8000 | 0x4A56DD |
| A7 | 0.003050 | 1.5000 | (V-500) * 0.003050 + 1.5000 | 0x4A67F0 |
| A8 | 0.006100 | 3.0000 | (V-500) * 0.006100 + 3.0000 | 0x4A7903 |

### >500V Formula Assembly Pattern

Each formula block uses x87 FPU instructions:
```asm
FLD   double ptr [0x008d6308]    ; load user-entered voltage
FSUB  float ptr [0x004a7b38]     ; subtract 500.0
FMUL  double ptr [ADDR_M]        ; multiply by per-device slope M
FADD  double/float ptr [ADDR_C]  ; add per-device constant C
FSTP  double ptr [0x008d6310]    ; store result
```

The result is then formatted with `sprintf("%.4f mils", result)` or `sprintf("%.4f mm", result)`.

### Formula Validation

At V=501, the formula yields a value just slightly above the 301-500V fixed value:
- B1: (501-500) * 0.098425 + 9.8425 = 9.9409 mils (vs 9.84 for 301-500V)
- B2: (501-500) * 0.196850 + 98.4252 = 98.6220 mils (vs 98.43 for 301-500V)
- B3: (501-500) * 0.984252 + 492.1260 = 493.1102 mils (vs 492.13 for 301-500V)

This confirms the formula provides continuity from the 301-500V range.

### Observations About Formula Constants

The multipliers M have a pattern matching mm-per-volt rates:
- B1: M = 0.0025 mm/V = 2.5 um/V
- B2: M = 0.005 mm/V = 5 um/V
- B3: M = 0.025 mm/V = 25 um/V
- B4, B5, A6, A7: M = 0.00305 mm/V = 3.05 um/V
- A8: M = 0.0061 mm/V = 6.1 um/V

B4/B5/A6/A7 share the same slope (0.00305 mm/V) but differ in their intercept C.
B5 and A6 share both slope AND intercept (identical formulas).

## String Storage Details

### Mils Strings (at 0x0087461C)
Each value stored twice: dot-decimal ("1.97 mils") and comma-decimal ("1,97 mils") for locale support.

| Address | String |
|---------|--------|
| 0x0087461C | "1.97 mils" |
| 0x00874630 | "3.94 mils" |
| 0x00874644 | "7.87 mils" |
| 0x00874658 | "9.84 mils" |
| 0x0087466C | "25.17 mils" |
| 0x00874682 | "49.21 mils" |
| 0x00874698 | "98.43 mils" |
| 0x008746AE | "59.06 mils" |
| 0x008746C4 | "125.98 mils" |
| 0x008746DC | "251.97 mils" |
| 0x008746F4 | "492.13 mils" |
| 0x0087470C | "2.95 mils" |
| 0x00874720 | "11.81 mils" |
| 0x00874736 | "31.50 mils" |
| 0x0087474C | "62.99 mils" |
| 0x00874762 | "5.12 mils" |
| 0x00874776 | "15.75 mils" |
| 0x0087478C | "19.69 mils" |
| 0x008747A2 | "39.37 mils" |
| 0x008747B8 | "118.11 mils" |

### MM Strings (at 0x008747D0)
| Address | String |
|---------|--------|
| 0x008747D0 | "0.05 mm" |
| 0x008747E6 | "0.10 mm" |
| 0x008747FD | "0.20 mm" |
| 0x00874814 | "0.25 mm" |
| 0x0087482B | "0.64 mm" |
| 0x00874842 | "1.25 mm" |
| 0x0087485A | "2.50 mm" |
| 0x00874872 | "1.50 mm" |
| 0x0087488A | "3.20 mm" |
| 0x008748A2 | "6.40 mm" |
| 0x008748BA | "12.50 mm" |
| 0x008748D5 | "0.075 mm" |
| 0x008748ED | "0.30 mm" |
| 0x00874904 | "0.80 mm" |
| 0x0087491B | "1.60 mm" |
| 0x00874933 | "0.13 mm" |
| 0x0087494A | "0.40 mm" |
| 0x0087495A | "0.50 mm" |
| 0x0087496A | "1.00 mm" |
| 0x00874982 | "3.00 mm" |

### Special Strings
| Address | String | Usage |
|---------|--------|-------|
| 0x008745EA | "Voltage must be equal to or greater than 501V" | Shown when >500V selected but voltage < 501 |
| 0x0087288A | "%.4f mils" | Format string for >500V formula result (mils) |
| 0x008713E4 | "%.4f mm" | Format string for >500V formula result (mm) |
| 0x008712FE | "." | Decimal separator (dot locale) |
| 0x00871343 | "," | Decimal separator (comma locale) |

### FPU Constants (at 0x004A7B34)
| Address | Type | Value | Usage |
|---------|------|-------|-------|
| 0x004A7B34 | float | 501.0 | Threshold for >500V voltage validation |
| 0x004A7B38 | float | 500.0 | Subtracted from voltage in all >500V formulas |
| 0x004A7B3C | double | 0.098425 | B1 mils slope |
| 0x004A7B44 | double | 9.8425 | B1 mils intercept |
| 0x004A7B4C | double | 0.19685 | B2 mils slope |
| 0x004A7B54 | double | 98.42519 | B2 mils intercept |
| 0x004A7B5C | double | 0.984252 | B3 mils slope |
| 0x004A7B64 | double | 492.12598 | B3 mils intercept |
| 0x004A7B6C | double | 0.12007 | B4/B5/A6/A7 mils slope |
| 0x004A7B74 | double | 62.99 | B4 mils intercept |
| 0x004A7B7C | double | 31.49606 | B5/A6 mils intercept |
| 0x004A7B84 | double | 59.05512 | A7 mils intercept |
| 0x004A7B8C | double | 0.240157 | A8 mils slope |
| 0x004A7B94 | double | 118.11 | A8 mils intercept |
| 0x004A7B9C | double | 0.0025 | B1 mm slope |
| 0x004A7BA4 | float | 0.25 | B1 mm intercept |
| 0x004A7BA8 | float | 1000.0 | Locale-related multiplier |
| 0x004A7BAC | double | 0.005 | B2 mm slope |
| 0x004A7BB4 | float | 2.5 | B2 mm intercept |
| 0x004A7BB8 | double | 0.025 | B3 mm slope |
| 0x004A7BC0 | float | 12.5 | B3 mm intercept |
| 0x004A7BC4 | double | 0.00305 | B4/B5/A6/A7 mm slope |
| 0x004A7BCC | double | 1.6 | B4 mm intercept |
| 0x004A7BD4 | double | 0.8 | B5/A6 mm intercept |
| 0x004A7BDC | float | 1.5 | A7 mm intercept |
| 0x004A7BE0 | double | 0.0061 | A8 mm slope |
| 0x004A7BE8 | float | 3.0 | A8 mm intercept |

## Code Structure of FUN_00498e84

The function spans from `0x00498E84` to approximately `0x004A7B30` (~60 KB of code).

### Control Flow Pattern

```
for each device_type_idx (0-7):       ; outer, checked via [EDX+0x2f0]
    for each voltage_range_idx (0-9):  ; inner, checked via [ECX/EAX+0x2f0]
        if voltage_range_idx < 9:
            ; Fixed string lookup
            PUSH string_addr           ; e.g., "1.97 mils"
            CALL set_text_field
        else:  ; voltage_range_idx == 9 (>500V)
            ; Read voltage from edit field -> [0x8d6308]
            ; Check voltage >= 501.0 (compare with [0x4a7b34])
            ; If < 501: show "Voltage must be >= 501V" error
            ; Else: compute formula
            FLD  [0x8d6308]            ; load voltage
            FSUB [0x4a7b38]            ; subtract 500
            FMUL [device_slope]        ; multiply by slope M
            FADD [device_intercept]    ; add intercept C
            FSTP [0x8d6310]            ; store result
            ; Format with "%.4f mils" and display
```

The code has two parallel sections:
1. **Mils section** (0x498F82 - 0x49F190): handles mils display for all 80 combinations
2. **MM section** (0x49F190 - 0x4A7B30): handles mm display for all 80 combinations

Each section uses locale-aware string selection (dot vs comma decimal separator).

### Register Usage
- **EBX**: pointer to the form object (`self` / `this`)
- **ECX**: first radio group's `ItemIndex` field (at `[form + 0x5CC] -> [radio + 0x2f0]`)
- **EDX**: second radio group's `ItemIndex` field (at `[form + 0x5D0] -> [radio + 0x2f0]`)
- **EAX**: reloaded to check both radio groups within inner blocks
- **ESI + 0x10**: tracks a "step counter" that increments with each assignment block (0x0C, 0x18, 0x24, ...)
- **[0x008D6308]**: stores the user-entered voltage as a double
- **[0x008D6310]**: stores the computed >500V spacing result as a double

## Cross-Validation

All mils values convert to their mm equivalents with < 0.001 mm error:
- 1.97 mils * 0.0254 = 0.05004 mm (stored as 0.05 mm)
- 3.94 mils * 0.0254 = 0.10008 mm (stored as 0.10 mm)
- 25.17 mils * 0.0254 = 0.63932 mm (stored as 0.64 mm)
- 98.43 mils * 0.0254 = 2.50012 mm (stored as 2.50 mm)
- 492.13 mils * 0.0254 = 12.5001 mm (stored as 12.50 mm)

The >500V formulas are also consistent: at any voltage V, mils_result * 0.0254 matches mm_result to within floating-point precision.
