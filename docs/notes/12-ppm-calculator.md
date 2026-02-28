# PPM / XTAL Calculator

## Overview
Two sub-calculators:
1. **XTAL Capacitor Value** - calculate load capacitors for crystal oscillators
2. **PPM ↔ Hertz conversion** - convert between PPM and frequency variation

## XTAL Capacitor Value

### Inputs
- **Load Capacitance** (pF) - from crystal datasheet
- **Stray Capacitance** (pF) - estimated parasitic capacitance from PCB
- **C1 Value** (pF) - capacitor value (if already selected)
- **C2 Value** (pF) - capacitor value (if already selected)

### Outputs
- **Based on C1-C2** (pF) - calculated load capacitance from C1, C2, and stray:
  ```
  C_load_calc = (C1 * C2) / (C1 + C2) + C_stray
  ```
  This should match the crystal's specified load capacitance.

- **Rule of Thumb** (pF) - starting value for C1/C2 when not yet selected:
  ```
  C_rule = 2 * (C_load - C_stray)
  ```
  (assumes C1 = C2, so series combination = C1/2, plus stray = C_load)

## Hertz to PPM

### Inputs
- **Center Frequency** (Hz) - nominal oscillator frequency
- **Maximum Frequency** (Hz) - highest observed frequency

### Outputs
- **Variation of Frequency** (Hz) = Max_freq - Center_freq
- **PPM Value** = (Variation / Center_freq) × 1,000,000

### Example from PDF
- Center: 32000 Hz, Max: 32001 Hz
- Variation: 1 Hz
- PPM: 31.25 PPM

Verification: (1 / 32000) × 1e6 = 31.25 ✓

## PPM to Hertz

### Inputs
- **Center Frequency** (Hz) - nominal frequency
- **PPM Value** - parts per million

### Outputs
- **Variation of Frequency** (Hz) = Center_freq × PPM / 1,000,000
- **Maximum Frequency** (Hz) = Center_freq + Variation
- **Minimum Frequency** (Hz) = Center_freq - Variation

### Example from PDF
- Center: 50000000 Hz (50 MHz), PPM: 25
- Variation: 1250.00 Hz
- Max: 50001250.00 Hz
- Min: 49998750.00 Hz

## Implementation Notes
- XTAL section has a "Help" button with detailed explanation
- The Rule of Thumb assumes symmetric capacitors (C1 = C2)
- PPM calculations are straightforward arithmetic
