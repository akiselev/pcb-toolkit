# Wavelength Calculator

## Overview
Calculates the wavelength of a signal at a given frequency in a PCB substrate,
using Er Effective to account for propagation speed in the medium.

## Formula (shown in PDF screenshot)
```
λ = c / (f * √(Er_eff))
```
Where:
- c = speed of light (≈ 11.803 in/ns = 299,792,458 m/s)
- f = signal frequency
- Er_eff = effective dielectric constant

## Input Methods
1. **Period** - enter signal period (ns or ps)
2. **Frequency** - enter signal frequency directly

## Inputs
- **Period** (ns or ps) OR **Frequency** (calculated from period or entered directly)
- **Er Eff** - effective dielectric constant (can be sent from Er Eff calculator)
- **Wavelength Divide** - slider from Full (1) to 1/20th
- **Conductor Type** - Microstrip / Stripline (affects Er Eff if calculated)

## Outputs
- **Full Wave Length** (inches or mm) = c / (f × √(Er_eff))
- Fractional wavelengths via slider (λ/2, λ/4, λ/7, λ/10, λ/20, etc.)

## Links/Buttons
- **Wavelength Information** - opens external reference
- **Er Effective Information** - opens reference
- **Speed of Light** - opens Wikipedia speed of light page
- **Er Eff Calculator** - opens the Er Effective calculator

## Example from PDF
- Period: 10 ns, Er Eff: 4, Microstrip
- Full Wave Length: **59.01426 Inches**

Verification: f = 1/10ns = 100MHz = 1e8 Hz
λ = (11.803 in/ns × 1e9 ns/s) / (1e8 Hz × √4) = 11.803e9 / 2e8 = 59.015 inches ✓

## Notes
- For wavelength in air: set Er Eff = 1
- Er Eff for stripline ≈ Er (dielectric fills entire space)
- Er Eff for microstrip < Er (part of field in air)
- Important for determining when a trace becomes a transmission line
  (generally when length > λ/10 to λ/7)
