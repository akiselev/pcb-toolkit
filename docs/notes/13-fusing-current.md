# Fusing Current Calculator

## Overview
Calculates the current that will cause a copper conductor to fuse (melt)
based on Onderdonk's equation. Primarily for short fuse times (< 5 seconds).

## Reference
**Onderdonk's Equation** - originally developed for bare copper wire in air,
adapted for PCB traces. This is an estimate only.

## Inputs
- **Conductor Width** (mils/mm)
- **Time** (seconds) - how long the current flows
- **Etch Factor** - 1:1, 2:1, or None
- **Copper Weight** - determines conductor thickness
- **Onderdonk Multiplier** - correction factor (default 1)

## Outputs
- **Melting Temperature of Copper**: 1064.62°C
- **Cross Section in circular mils**
- **Conductor Cross Section** (sq.mils) - affected by etch factor
- **Conductor Current** (Amps) - the fusing current

## Onderdonk's Equation
Shown in PDF: `I = A*(log(1 + (Tm-Ta)/(234+Ta))/33*s)^0.5`

Formal version:
```
I = A * sqrt( log10(1 + (Tm - Ta) / (234 + Ta)) / (33 * t) )
```
Where:
- I = fusing current (Amps)
- A = cross-sectional area (circular mils)
- Tm = melting temperature of copper = 1064.62°C
- Ta = ambient temperature (°C)
- t = time in seconds
- 234 = inverse of copper temperature coefficient (1/0.00427 ≈ 234)
- 33 = constant for copper

### With Multiplier:
```
I = Multiplier * A * sqrt( log10(1 + (Tm - Ta) / (234 + Ta)) / (33 * t) )
```

## Cross-Section Conversion
```
Area_circular_mils = (4 / π) * Area_square_mils
Area_square_mils = Width * Thickness * EtchFactor_correction
```

## Implementation Notes
- Not intended for long periods (> 5 seconds)
- The formula was originally for bare copper in air - PCB adaptation is approximate
- Onderdonk multiplier allows tuning for PCB-specific conditions
- Reference: https://www.pcdandf.com/pcdesign/index.php/magazine/10293-pcb-design-1509
