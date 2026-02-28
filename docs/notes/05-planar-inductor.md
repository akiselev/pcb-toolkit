# Planar Inductor Calculator

## Overview
Calculates inductance of planar (spiral) PCB inductors using the
Mohan et al. modified Wheeler formula.

## Reference
**Sunderarajan S. Mohan, Maria del Mar Hershenson, Stephen P. Boyd, and Thomas H. Lee**
"Simple Accurate Expressions for Planar Spiral Inductances"
IEEE JSSC, October 1999
http://smirc.stanford.edu/papers/JSSC99OCT-mohan.pdf

## Inductor Geometry Options
1. **Square**
2. **Hexagonal**
3. **Octagonal**
4. **Circular**

## Inputs
- **Turns (n)** - number of turns
- **Conductor Width (w)** - trace width (mils/mm)
- **Conductor Spacing (s)** - gap between turns (mils/mm)
- **Outer Diameter (dout)** - outer radius of structure (mils/mm)

## Outputs
- **Inner Diameter (din)** - calculated: `din = dout - 2*n*(w + s) + 2*s`
- **Fill Factor (ρ)** - `ρ = (dout - din) / (dout + din)`
- **Inductance (Lmw)** - in nH

## Formula (Modified Wheeler)
Visible in the PDF screenshot:
```
Lmw = K1 * μ₁ * n² * d_avg / (1 + K2 * ρ)
```

Where:
- `n` = number of turns
- `d_avg = (dout + din) / 2` = average diameter
- `ρ = (dout - din) / (dout + din)` = fill factor
- `μ₁ = μ₀ = 4π × 10⁻⁷ H/m` (permeability of free space)
- `K1`, `K2` are geometry-dependent constants

### Geometry Constants (from Mohan et al.)
| Geometry | K1 | K2 |
|----------|-----|-----|
| Square | 2.34 | 2.75 |
| Hexagonal | 2.33 | 3.82 |
| Octagonal | 2.25 | 3.55 |
| Circular | 2.23 | 3.45 |

## Implementation Notes
- Inner diameter is derived: `din = dout - 2*n*(w+s) + 2*s`
  (outer diameter minus the space taken by n turns of width w with spacing s)
- If din becomes negative or zero, the geometry is invalid
- Units must be consistent - convert mils to meters for μ₀ calculation,
  or use normalized version with appropriate constant
- Output in nH
