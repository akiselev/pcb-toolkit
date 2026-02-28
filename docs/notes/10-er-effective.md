# Er Effective Calculator

## Overview
Calculates the effective dielectric constant (Er Effective) for microstrip geometry,
which accounts for the fact that the electric field exists partly in the substrate
and partly in air.

## Reference
Uses the complex formula by **E. Hammerstad and O. Jensen**, not the simplified
IPC-2141A formula.

Source: https://www.microwaves101.com/encyclopedias/keffective

## Inputs
- **Conductor Width (W)** - trace width (mils/mm)
- **Conductor Height (H)** - dielectric thickness to reference plane (mils/mm)
- **Frequency (MHz)** - signal frequency (causes slight change in Er effective)
- **Er** - bulk dielectric constant of substrate

## Output
- **Er Effective** - effective dielectric constant accounting for geometry

## Formulas

### Static Er Effective (Hammerstad-Jensen):
```
u = W / H

Er_eff_0 = (Er + 1)/2 + ((Er - 1)/2) * F(u)

F(u) = (1 + 12/u)^(-0.5) + 0.04*(1 - u)^2    for u ≤ 1
F(u) = (1 + 12/u)^(-0.5)                        for u > 1
```

### Frequency-Dependent Er Effective (dispersion model):
The frequency dependence uses the Kirschning-Jansen or similar model:
```
Er_eff(f) = Er - (Er - Er_eff_0) / (1 + G*(f/f_p)^2)
```
Where:
- `f_p` is a characteristic frequency
- `G` is a geometry-dependent factor

### Conductor Thickness Correction:
When conductor thickness T is significant:
```
Er_eff_t = Er_eff_0 - (Er - 1) * T / (4.6 * H * sqrt(u))
```

## Example from PDF
- W = 17 mils, H = 10 mils, f = 500 MHz, Er = 4.6
- **Er Effective = 3.2802**

## Implementation Notes
- "Send to Wavelength Calculator" button transfers Er Effective value
- Er Effective is also calculated internally by the Conductor Impedance tab
- This is the same formula used in the impedance calculator, exposed standalone
- W/H ratio drives the calculation significantly
