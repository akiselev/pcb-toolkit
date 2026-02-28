# Conductor Impedance Calculator

## Overview
Calculates characteristic impedance (Zo), inductance (Lo), capacitance (Co), and
propagation delay (Tpd) for various transmission line geometries.

## Supported Circuit Types (Passive Circuits)
1. **Microstrip** - single conductor above a ground plane
2. **Microstrip Embed** (Embedded Microstrip) - conductor embedded in dielectric above ground plane
3. **Stripline** - conductor centered between two ground planes
4. **Stripline Asym** (Asymmetric Stripline) - conductor off-center between two ground planes
5. **Dual Stripline** - two conductors between ground planes
6. **Coplanar Wave** (Coplanar Waveguide) - conductor with ground on same plane

## Inputs
- **Conductor Width (W)** - width of the trace (mils or mm)
- **Conductor Height (H)** - distance from conductor to reference plane (dielectric thickness)
- **Frequency (MHz)** - for microstrip: affects Er effective, causing slight impedance change
- **Er** - dielectric constant of substrate material
- **Copper Weight** - determines conductor thickness (T)
- **Plating Thickness** - additional copper from plating process

## Outputs
- **Zo** (Ohms) - characteristic impedance
- **Lo** (nH/in) - inductance per unit length
- **Co** (pF/in) - capacitance per unit length
- **Tpd** (ps/in) - propagation delay per unit length
- **Er Effective** - effective dielectric constant (shown in info area)
- **Total Copper Thickness** - base copper + plating

## Formula Source
**Hammerstad & Jensen** (1980): "Accurate Models for Microstrip Computer-Aided Design"
- Referenced string: `GE. Hammerstad and O. Jensen, "Accurate Models for Microstrip Computer-`
- Also references: `"Width and Effective Dielectric Constant Equations for`
- And: `"With Finite Metal Thickness in the PEEC Formulation"`

The Er Effective note in the app says:
> "This calculator uses a complex formula presented by E. Hammerstad and O. Jensen,
> not the simplified formula presented by the IPC-2141A."

## Microstrip Impedance (Hammerstad-Jensen)

### Er Effective (frequency-independent approximation):
```
Er_eff = (Er + 1)/2 + (Er - 1)/2 * (1 + 12*H/W)^(-0.5)
```
With correction for W/H < 1:
```
Er_eff += (Er - 1)/2 * 0.04 * (1 - W/H)^2    (when W/H < 1)
```

### Characteristic Impedance:
For W/H <= 1:
```
Zo = (60 / sqrt(Er_eff)) * ln(8*H/W + W/(4*H))
```
For W/H > 1:
```
Zo = (120*pi / sqrt(Er_eff)) / (W/H + 1.393 + 0.667*ln(W/H + 1.444))
```

### With conductor thickness correction:
Effective width We accounts for finite conductor thickness T:
```
dW = (T/pi) * (1 + ln(4*pi*W/T))     for W/H >= 0.5*pi
dW = (T/pi) * (1 + ln(2*H/T))        for W/H < 0.5*pi
We = W + dW
```

### Frequency-dependent Er Effective (dispersion):
Uses Kirschning & Jansen model or similar for frequency-dependent correction.

## Stripline Impedance
For centered stripline (conductor centered between two ground planes):
```
Zo = (60 / sqrt(Er)) * ln(4*b / (pi*d*0.67*(0.8 + T/W)))
```
Where b = total distance between ground planes, d = effective width factor.

Simplified (zero-thickness):
```
Zo = (60 / sqrt(Er)) * ln(4*H / (0.67 * (0.8 + W/H) * pi * We))
```

## Derived Quantities
```
Tpd = sqrt(Er_eff) / c      (propagation delay, c = speed of light)
Lo = Zo * Tpd               (inductance per unit length)
Co = Tpd / Zo               (capacitance per unit length)
```

## Implementation Notes
- Need to handle both imperial (mils) and metric (mm) inputs
- Copper weight to thickness conversion: 1oz ≈ 1.37 mils (34.8 μm)
- Speed of light: c = 11.803 in/ns (299,792,458 m/s)
- The app auto-calculates when "Solve" is pressed or Enter is hit in any input field
- Formula restrictions noted in binary: `0.1 < W/H < 3.0` and `0.1 < S/H < 3.0`
