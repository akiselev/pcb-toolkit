# Crosstalk Calculator

## Overview
Standalone crosstalk calculator (separate from the differential pairs NEXT calculator).
Marked as "Unsupported" in the help PDF - Saturn PCB states they have
"lack of faith in the formula" driving it.

## Inputs
- **Signal Rise Time** (ns)
- **Signal Voltage** (V)
- **Coupled Length** (um/mils)
- **Conductor Spacing (S)** (um/mils)
- **Conductor Height (H)** (um/mils)
- **Material Selection** - FR-4 STD or custom
- **Er** - dielectric constant
- **Conductor Type** - Microstrip / Stripline

## Outputs
- **Crosstalk Coefficient** (dB)
- **Coupled Voltage** (V)

## Formula (estimated from the PDF example values)
The crosstalk calculator likely uses a simplified backward crosstalk model.

### For Microstrip:
```
Kb = (1/4) * (S/H)^(-2) * [Er_eff / (Er_eff + some_correction)]
```

### Near-End Crosstalk Coefficient:
```
NEXT_coeff = Kb * (1 - e^(-2*coupled_length/Lsat))
```
or simplified when coupled_length >= Lsat:
```
NEXT_coeff ≈ Kb
```

### Lsat (saturation length):
```
Lsat = risetime * v_prop / 2
v_prop = c / sqrt(Er_eff)
```

### Coupled Voltage:
```
V_coupled = NEXT_coeff * V_signal
```

### Crosstalk in dB:
```
dB = 20 * log10(NEXT_coeff)
```

## Example from PDF (page 47)
- Rise time: 1 ns, Voltage: 5V, Coupled length: 6350 um
- Spacing: 254 um, Height: 762 um, Er: 4.6, Microstrip
- **Crosstalk Coefficient: -2.23327 dB**
- **Coupled Voltage: 3.86640 V**

## Implementation Notes
- Separate popup dialog (not a main tab)
- Uses metric units (um) in the dialog - different from main app
- "Unsupported" status suggests we should implement but note limitations
- Consider implementing both microstrip and stripline crosstalk models
