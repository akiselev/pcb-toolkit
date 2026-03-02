# Ohm's Law Calculator

## Overview
Basic electrical calculator with multiple modes: E-I-R triangle,
LED bias, R/C/L series/parallel, and PI/T pad attenuators.

## Tabs/Modes

### 1. E-I-R (Ohm's Law)
**Solve for**: Volts, Amps, or Ohms

```
V = I * R        (Voltage)
I = V / R        (Current)
R = V / I        (Resistance)
P = V * I        (Power/Wattage)
P = I² * R
P = V² / R
```

### 2. LED Bias
#### Inputs
- **+V Value** - supply voltage (V)
- **LED Voltage Drop** - forward voltage of LED (V)
- **LED Current** - desired LED current (mA)

#### Output
- **Resistor Value** = (V_supply - V_led) / I_led

### 3. R Series / R Parallel
2-4 resistors in series or parallel
```
Series:   R_total = R1 + R2 + R3 + R4
Parallel: 1/R_total = 1/R1 + 1/R2 + 1/R3 + 1/R4
```
With voltage input: calculates individual amperage and wattage for each resistor.

### 4. PI Pad Attenuator (verified from decompilation)
RF attenuator design. R1/R2 are shunt resistors; R3 is series resistor.
#### Inputs
- **Attenuation** (dB)
- **Zin** - input impedance (Ω)
- **Zout** - output impedance (Ω)

#### Formulas (unmatched, from disassembly of mode 10)
```
K = 10^(dB/20)
Zmax = max(Zin, Zout), Zmin = min(Zin, Zout)

R1 = Zmax * (K²-1) / (K² - 2K*sqrt(Zmax/Zmin) + 1)
R2 = Zmin * (K²-1) / (K² - 2K*sqrt(Zmin/Zmax) + 1)
R3 = sqrt(Zmax*Zmin) * (K²-1) / (2K)
```

#### Matched (Zin = Zout = Z):
```
R1 = R2 = Z * (K+1)/(K-1)    [shunt]
R3 = Z * (K²-1)/(2K)          [series]
```

### 5. T Pad Attenuator (verified from decompilation)
R1/R2 are series resistors; R3 is shunt resistor.
#### Formulas (unmatched)
```
K = 10^(dB/20)
Zmax = max(Zin, Zout), Zmin = min(Zin, Zout)

R3 = 2K * sqrt(Zmax*Zmin) / (K²-1)
R1 = Zmax * (K²+1)/(K²-1) - R3
R2 = Zmin * (K²+1)/(K²-1) - R3
```

#### Matched (Zin = Zout = Z):
```
R1 = R2 = Z * (K-1)/(K+1)    [series]
R3 = 2KZ / (K²-1)             [shunt]
```

### 6. C Series / C Parallel
2-4 capacitors
```
Series:   1/C_total = 1/C1 + 1/C2 + 1/C3 + 1/C4
Parallel: C_total = C1 + C2 + C3 + C4
```

### 7. L Series / L Parallel
2-4 inductors
```
Series:   L_total = L1 + L2 + L3 + L4
Parallel: 1/L_total = 1/L1 + 1/L2 + 1/L3 + 1/L4
```

## Implementation Notes
- Very straightforward formulas
- Multiple sub-calculators in one tab
- Good candidate for simple CLI subcommands
- LED bias: I_led input is in mA (divided by 1000 internally)
- Wattage: W = (Vsupply - Vled) * I_led_amps
- Full decompilation details in `ghidra-ohmslaw.md`
- All attenuator formulas verified via FPU opcode testing and impedance matching
