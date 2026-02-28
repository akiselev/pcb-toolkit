# Material/Substrate Data Tables

## Overview
Shared across all calculators via the "Substrate Options" / "Material Selection"
dropdown in the Options panel.

## Known Materials (from binary strings)
| Material | Er | Tg (°C) |
|----------|-----|---------|
| FR-4 STD | 4.6 | 130 |
| Isola P26N | ? | ? |
| Isola P95 | ? | ? |
| Isola P96 | ? | ? |

(More materials likely exist in the dropdown - need to extract from binary)

## Copper Weight to Thickness Conversion
| Weight | Thickness (mils) | Thickness (mm) |
|--------|-----------------|----------------|
| 0.25oz | 0.35 | 0.00889 |
| 0.5oz | 0.70 | 0.01778 |
| 1oz | 1.37 | 0.03480 |
| 1.5oz | 2.05 | 0.05207 |
| 2oz | 2.80 | 0.07112 |
| 2.5oz | 3.50 | 0.08890 |
| 3oz | 4.20 | 0.10668 |
| 4oz | 5.60 | 0.14224 |
| 5oz | 7.00 | 0.17780 |

## Plating Thickness Options
- Bare PCB (0)
- 0.5oz, 1oz, 1.5oz, 2oz, 2.5oz, 3oz
- Max plating: 3 mils (from binary: "Maximum plating thickness can't be greater than 3mils")
- Min plating: 0.5 mils (from binary: "Minimum plating thickness can't be less than 0.5mils")

## Physical Constants (used across calculators)
- Speed of light: c = 299,792,458 m/s = 11.803 in/ns
- Permeability of free space: μ₀ = 4π × 10⁻⁷ H/m
- Permittivity of free space: ε₀ = 8.854 × 10⁻¹² F/m
- Copper resistivity (at 20°C): ρ = 1.724 × 10⁻⁶ Ω·cm
- Copper temperature coefficient: α = 0.00393 /°C (or 1/234 per Onderdonk)
- Copper melting point: 1064.62°C
- Copper density: 8.96 g/cm³

## Unit Conversions
- 1 mil = 0.001 inch = 0.0254 mm = 25.4 μm
- 1 oz copper = 1.37 mils thickness
- 1 inch = 25.4 mm
- Temperature: °F = °C × 9/5 + 32

## Generic Copper Weight vs Conductor Spacing Chart
(from binary - internal design rules)
| Copper Weight | Min Gerber Width |
|--------------|-----------------|
| 0.25oz | 3 mil |
| 0.5oz | 3 mil |
| 1oz | 4 mil |
| 2oz | 5 mil |
| 3oz | 6 mil |
| 4oz | 7 mil |
| 5oz | 8 mil |
| 6oz | 9 mil |
