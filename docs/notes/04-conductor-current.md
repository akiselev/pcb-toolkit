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

### IPC-2152 Modifiers (extracted from decompilation):
```
External: I = base * M_area * M_temp * M_board * M_material * M_user
Internal: I = base * M_area * M_temp / plane_dist * M_board * M_material * M_user
```

**Area Chart Correction (M_area)** — piecewise power law:
| Area Range (sq.mils) | Formula |
|---|---|
| area <= 20 | 3.0364 * area^(-0.145) |
| 20 < area <= 60 | 2.9143 * area^(-0.129) |
| 60 < area <= 100 | 2.7877 * area^(-0.114) |
| area > 100 | 2.801 * area^(-0.111) |

**Temperature Rise Correction (M_temp)** — 11-step lookup from 0.40 (dT≤10) to 1.30 (dT=100)

**Board Thickness Correction (M_board)** — 2 tables (with/without copper plane), 11 steps each

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

## Etch Factor Formulas (from decompilation)
```
Mode 0 (1:1):  area = (width - thickness) * thickness
Mode 1 (2:1):  area = (width - thickness/2) * thickness
Mode 2 (none): area = width * thickness
```

## Binary Constants
- k_ext = 0.048, k_int = 0.024, b = 0.44, c = 0.725
- Copper temp coefficient: 0.00393 /°C (at 0x00440d54)
- Copper resistivity: 1.72e-8 ohm·m (at 0x00440d68)
- µ factor: 1.256636e-5 (4π×10⁻⁶, at 0x008d6418)

Full decompilation details in `ghidra-conductor-current.md`
