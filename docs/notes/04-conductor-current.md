# PCB Conductor Current (Conductor Properties)

## Overview
Calculates current vs temperature rise for PCB conductors, plus related
electrical properties like resistance, voltage drop, skin depth, and power dissipation.

## IPC Standard Options
1. **IPC-2152 with modifiers** - latest standard with correction factors for PCB thickness, planes, etc.
2. **IPC-2152 without modifiers** - base IPC-2152 charts without correction factors
3. **IPC-2221A** - legacy standard (marked "Obsolete for amperage")

## Solve Modes
- **Solve for Amperage** - given conductor width, find max current for temp rise
- **Solve for Conductor Width** - given current, find required width (IPC-2152 without modifiers and IPC-2221 only)

## Inputs
- **Conductor Width** (mils/mm)
- **Conductor Length** (mils/mm)
- **PCB Thickness** (mils/mm) - acts as heat sink
- **Frequency** (MHz) - for skin depth only, no effect on temp rise
- **DC checkbox** - when checked, frequency effects ignored
- **Load Current** (Amps) - separate from calculated conductor current
- **Plane Present** - Yes/No (copper plane helps dissipate heat)
- **Distance to Plane** - if plane present
- **Plane Thickness** - 0.5oz/1oz or 2oz
- **Parallel Conductors** - Yes/No
- **Parallel Conductor Count** - number of parallel traces
- **Conductor Layer** - Internal / External (different IPC charts)
- **Etch Factor** - 1:1, 2:1, or None

## Outputs
- **Conductor Current** (Amps) - calculated current for given temp rise
  - NOT the max current before failure; it's the current causing the specified temp rise
- **Conductor DC Resistance** (Ohms)
- **Conductor Cross Section** (sq.mils)
- **Loaded Voltage Drop** (V) - based on entered load current
- **Voltage Drop** (V) - based on calculated conductor current
- **Power Dissipation** (Watts)
- **Power Dissipation** (dBm)
- **Skin Depth** (mils/mm) - frequency-dependent
- **Skin Depth Percentage** (%)
- **Conductor Temperature** - ambient + temp rise from calculated current
- **Loaded Conductor Temperature** - ambient + temp rise from load current
- **Current Density J** (A/m²)
- **Material Tg** - glass transition temperature of selected material

## IPC-2152 Formula (simplified)
The IPC-2152 uses empirical charts. The basic relationship is:
```
I = k * dT^b * A^c
```
Where:
- I = current (Amps)
- dT = temperature rise (°C)
- A = cross-sectional area (sq.mils)
- k, b, c = empirical constants that differ for internal/external

### IPC-2221 (legacy):
External conductors:
```
I = 0.048 * dT^0.44 * A^0.725
```
Internal conductors:
```
I = 0.024 * dT^0.44 * A^0.725
```

### IPC-2152 Modifiers:
- Board thickness correction factor
- Plane proximity correction factor
- Parallel conductor factor

## Skin Depth
```
δ = sqrt(ρ / (π * f * μ))
```
Where:
- ρ = resistivity of copper ≈ 1.724 × 10⁻⁶ Ω·cm
- f = frequency in Hz
- μ = permeability = μ₀ = 4π × 10⁻⁷ H/m

## DC Resistance
```
R_dc = ρ_copper * L / A
```
Where:
- ρ_copper adjusted for temperature: ρ(T) = ρ₂₀ * (1 + α*(T - 20))
- α = 0.00393 /°C for copper
- L = conductor length
- A = cross-sectional area (accounting for etch factor)

## Cross-Sectional Area with Etch Factor
```
None:    A = W * T                    (rectangular)
1:1:     A = (W + (W - 2*T)) * T / 2 (trapezoidal, top = W-2T)
2:1:     A = (W + (W - T)) * T / 2   (trapezoidal, top = W-T)
```

## Copper Weight to Thickness
| Weight | Thickness (mils) | Thickness (μm) |
|--------|-----------------|-----------------|
| 0.25oz | 0.35 | 8.75 |
| 0.5oz  | 0.70 | 17.5 |
| 1oz    | 1.37 | 34.8 |
| 1.5oz  | 2.05 | 52.1 |
| 2oz    | 2.80 | 70.0 |
| 2.5oz  | 3.50 | 87.5 |
| 3oz    | 4.20 | 105.0 |
| 4oz    | 5.60 | 140.0 |
| 5oz    | 7.00 | 175.0 |

Total copper thickness = base weight thickness + plating thickness.
