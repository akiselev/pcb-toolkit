# Differential Pairs with Crosstalk (NEXT)

## Overview
Calculates differential impedance (Zdiff), odd/even mode impedances, and
near-end crosstalk (NEXT) for coupled transmission lines.

## Differential Layer Types (from strings)
1. **Edge Cpld Ext** - Edge-coupled external (microstrip)
2. **Edge Cpld Int Sym** - Edge-coupled internal symmetric (stripline)
3. **Edge Cpld Int Asym** - Edge-coupled internal asymmetric
4. **Edge Cpld Embed** - Edge-coupled embedded microstrip
5. **Broad Cpld Shld** - Broadside-coupled shielded (stripline)
6. **Broad Cpld NShld** - Broadside-coupled non-shielded

## Differential Protocol Presets (target Zdiff auto-fill)
| Protocol | Typical Zdiff |
|----------|--------------|
| DDR2 CLK/DQS | 100Ω |
| DDR3 CLK/DQS | 100Ω |
| DDR4 CLK/DQS | 100Ω |
| LVDS | 100Ω |
| USB 2.X | 90Ω |
| USB 3.X | 90Ω |
| HDMI | 100Ω |
| SATA | 100Ω |
| PCIe Gen1 | 100Ω |
| PCIe Gen2 | 100Ω |
| PCIe Gen3 | 85Ω |
| PCIe Gen4 | 85Ω |
| Ethernet | 100Ω |

## Inputs
- **Conductor Width (W)** - trace width (mils/mm)
- **Conductor Spacing (S)** - edge-to-edge gap between pair
- **Conductor Height (H)** - dielectric thickness to reference plane
- **Target Zdiff** - target differential impedance (Ohms)
- **+/- Tolerance** - slider, default 10%
- **Applied Voltage** - signal voltage for NEXT calculation
- **Coupled Length** - length of parallel routing
- **Signal Risetime** - in nanoseconds

## Outputs
- **Zdifferential** - differential impedance of the pair
- **Zo** - single-conductor impedance
- **Zodd** - odd-mode impedance
- **Zeven** - even-mode impedance
- **Target Zdiff Plus/Minus** - tolerance range
- **Kb (Term)** - coupling coefficient (terminated)
- **Kb' (Unterm)** - coupling coefficient (unterminated)
- **dB** - coupling in decibels
- **NEXT Voltage** - near-end crosstalk voltage (terminated and unterminated)
- **Lsat** - saturated length (coupled length beyond which NEXT stops increasing)

## Key Relationships
```
Zdiff = 2 * Zodd
Zcommon = Zeven / 2

Zodd = Zo * (1 - Kb)     (approximately)
Zeven = Zo * (1 + Kb)    (approximately)

Kb = (Zeven - Zodd) / (Zeven + Zodd)    (coupling coefficient)
```

## Formula Restrictions (from binary strings)
```
0.1 < W/H < 3.0
0.1 < S/H < 3.0
```
W/H = ratio of conductor width to height
S/H = ratio of conductor spacing to height

## NEXT (Near-End Crosstalk)
```
Kb_term = (Zeven - Zodd) / (Zeven + Zodd)     for terminated line
Kb_unterm = 2 * Kb_term / (1 + Kb_term)        approximately

NEXT_voltage_term = Kb_term * V_applied * min(coupled_length / Lsat, 1)
NEXT_voltage_unterm = Kb_unterm * V_applied * min(coupled_length / Lsat, 1)

Lsat = risetime * v_prop / 2    (saturated length)
v_prop = c / sqrt(Er_eff)       (propagation velocity)
```

## Crosstalk dB
```
dB = 20 * log10(Kb)
```

## Implementation Notes
- Green/red indicator shows if calculated Zdiff is within tolerance of target
- "Send to Via Calculator" button transfers Zdiff to via properties tab
- Information area shows W/H, S/H ratios and Lsat
- Need different formulas for each layer configuration (edge-coupled vs broadside)
