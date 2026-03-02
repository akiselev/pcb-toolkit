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
- **Melting Temperature of Copper**: 1084.62°C
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
- Tm = melting temperature of copper = 1084.62°C (NOT 1064.62 which is gold)
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
Area_circular_mils = 1.2732 * Area_square_mils   (Saturn uses truncated 4/π)
```

### Etch Factor Modes (from decompilation)
```
1:1 etch:  area = (width - thickness) * thickness
2:1 etch:  area = (width - thickness/2) * thickness
None:      area = width * thickness
```

### mm mode conversion
```
area_sq_mils = area_sq_mm * 1550.0031   (= (1/0.0254)^2)
```

## Binary Constants (verified addresses)
| Address | Value | Purpose |
|---------|-------|---------|
| 0x004ba930 | 1084.62 (double) | Copper melting point Tm |
| 0x004ba938 | 234.0 (float) | Onderdonk 1/α constant |
| 0x004ba93c | 33.0 (float) | Onderdonk copper constant |
| 0x004ba940 | 1.2732 (double) | 4/π (truncated) |
| 0x004ba950 | 1550.0031 (double) | sq mm to sq mils |

## Implementation Notes
- Not intended for long periods (> 5 seconds)
- The formula was originally for bare copper in air - PCB adaptation is approximate
- Onderdonk multiplier allows tuning for PCB-specific conditions
- Use truncated constant 1.2732 (not exact 4/π) to match Saturn output
- Ambient temperature must be >= 0°C, multiplier must be >= 1
- Full decompilation details in `ghidra-fusing-inductor.md`
- Reference: https://www.pcdandf.com/pcdesign/index.php/magazine/10293-pcb-design-1509
