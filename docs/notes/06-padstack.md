# Padstack Calculator

## Overview
Calculates pad sizes for various padstack configurations including thru-hole,
BGA land sizes, and conductor routing between pads.

## Pad Calculator Types
1. **Thru-Hole Pad**
2. **BGA Land Size**
3. **Conductor / Pad TH** - conductor routing between thru-hole pads
4. **Conductor / Pad BGA** - conductor routing between BGA lands
5. **2 Conductors / Pad TH** - two conductors between thru-hole pads
6. **2 Conductors / Pad BGA** - two conductors between BGA lands
7. **Corner to Corner** - diagonal distance for square/rectangular pins

## Thru-Hole Pad

### Inputs
- **Hole Diameter** - finished hole diameter (mils/mm)
- **Hole Type** - preset list or "Custom"
- **Annular Ring** - minimum annular ring requirement (mils/mm)
- **Isolation Width** - thermal relief air gap (mils/mm)
- **Pad Style** - Plated / Non-Plated

### Outputs
- **External Layers** pad diameter = Hole Diameter + 2 × Annular Ring
- **Internal Signal Layers** pad diameter (may be larger)
- **Internal Plane Layers** pad diameter (may be larger for anti-pad)
- **Outer Diameter** - includes isolation width for thermal relief
- **Inner Diameter** - thermal relief inner diameter
- **Spoke Width** - suggested thermal spoke width

### Formulas
```
Pad_external = Hole_diameter + 2 * Annular_ring
Pad_internal_signal = Pad_external  (or larger per design rules)
Pad_plane = Pad_external + 2 * Isolation_width  (anti-pad)
Inner_diameter = Pad_plane  (inner of thermal relief)
Outer_diameter = Inner_diameter + 2 * Spoke_width (or similar)
```

## BGA Land Size (IPC-7351A)

### Inputs
- **Nominal Ball Diameter** - from BGA datasheet (preset dropdown)

### Outputs
- **Nominal Land Diameter** - average recommended PCB land size
- **Land Variation** - upper and lower limits
  - IPC-7351A uses the larger land variation for the proper BGA land size

### Common Ball Diameters (from datasheet dropdown)
Typical values: 0.2mm, 0.25mm, 0.3mm, 0.35mm, 0.4mm, 0.45mm, 0.5mm,
0.6mm, 0.75mm, 0.8mm, 1.0mm

## Conductor / Pad TH

### Inputs
- **Distance Between Pads** - center-to-center
- **Conductor Width** - trace width to route
- **Spacing Constraint** - pad-to-conductor spacing

### Outputs
- **Hole Diameter** - finished hole
- **Minimum Annular Ring**
- **Maximum Pad Diameter** - max pad that still fits the conductor
- **Calculated Annular Ring** - actual annular ring achieved

### Formula
```
Max_Pad_Diameter = (Distance_Between_Pads - 2*Spacing_Constraint - Conductor_Width) / 2
Calculated_Annular_Ring = (Max_Pad_Diameter - Hole_Diameter) / 2
```

## Conductor / Pad BGA

### Inputs
- **Distance Between Pads** - BGA pitch (center-to-center)
- **Land Diameter** - BGA land diameter
- **Spacing Constraint** - SMD pad to conductor spacing

### Output
- **Maximum Conductor Width** = Distance - Land_Diameter - 2 × Spacing

## 2 Conductors / Pad TH & BGA
Same concept but fitting TWO conductors (differential pair) between pads.
```
Max_Conductor_Width = (Distance - Pad_Diameter - 2*Spacing - Gap) / 2
```

## Corner to Corner

### Inputs
- **Length of side a** - one side of square/rectangular pin
- **Length of side b** - other side

### Outputs
- **Distance Between Corners** = `sqrt(a² + b²)`
  (string found: `c = sqrt(a^2+b^2)`)
- **Suggested Min Drill** - based on pin diagonal
- **Suggested Max Drill** - based on pin diagonal
