# XC / XL Reactance Calculator

## Overview
Calculates capacitive reactance (Xc), inductive reactance (Xl),
and resonant frequency from input frequency, capacitance, and inductance.

## Inputs
- **Frequency** - with unit selector: GHz, MHz, kHz, Hz
- **Capacitance** - with unit selector: uF, nF, pF
- **Inductance** - with unit selector: mH, uH, nH

## Outputs
- **Xc** (Ohms) - capacitive reactance
- **Xl** (Ohms) - inductive reactance
- **Resonant Frequency** (Hz) - frequency where Xc = Xl

## Formulas (shown in PDF screenshot)
```
Xc = 1 / (2π * f * C)

Xl = 2π * f * L

f_resonant = 1 / (2π * √(L * C))
```

## Unit Conversions
```
GHz → Hz: × 1e9
MHz → Hz: × 1e6
kHz → Hz: × 1e3

uF → F: × 1e-6
nF → F: × 1e-9
pF → F: × 1e-12

mH → H: × 1e-3
uH → H: × 1e-6
nH → H: × 1e-9
```

## Implementation Notes
- Very simple calculator
- The formulas are displayed directly in the GUI (visible in PDF screenshot)
- Straightforward to implement
