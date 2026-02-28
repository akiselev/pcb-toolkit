# Via Properties / Via Current Calculator

## Overview
Calculates via electrical properties: capacitance, inductance, impedance,
DC resistance, current capacity, thermal resistance, and more.

## Via Types (Layer Set)
- **2 Layer** - simple through-hole via
- **Multi Layer** - via through multiple layers (enables capacitance/impedance calcs)
- **Microvia** - small laser-drilled via (typically 1-2 layers)

## Inputs
- **Via Hole Diameter** - finished hole diameter (mils/mm)
- **Internal Pad Diameter** - inner layer pad diameter
- **Ref Plane Opening Diameter** - anti-pad opening in reference planes
- **Via Height** - length from start to end layer (mils/mm)
- **Via Plating Thickness** - copper plating thickness (mils/mm)
- **Er** - dielectric constant
- **Temp Rise** - maximum allowed temperature rise (°C)
- **Ambient Temp** - ambient temperature (°C)

## Outputs

### Electrical Properties
- **Via Capacitance** (pF) - only for multi-layer vias
  ```
  C_via = 1.41 * Er * T_plate * D_pad / (D_antipad - D_pad)
  ```
  (or similar coaxial capacitance formula)

- **Via Inductance** (nH)
  ```
  L_via = 5.08 * h * (ln(4*h/d) + 1)    [nH, mils]
  ```
  where h = via height, d = via diameter

- **Via Impedance** (Ohms) - only for multi-layer vias
  ```
  Z_via = sqrt(L_via / (C_via * 0.001))
  ```

- **Via DC Resistance** (Ohms)
  ```
  R_via = rho * h / A
  ```
  where A = cross-sectional area of the copper barrel (annular ring of plating)

- **Resonant Frequency** (MHz) - only for multi-layer
  ```
  f_res = 1 / (2 * pi * sqrt(L * C))
  ```

- **Step Response** (ps) - T10-90% rise time impact on 50Ω line

### Thermal Properties
- **Power Dissipation** (Watts) = I² × R_via
- **Power Dissipation (dBm)** = 10 × log10(P/0.001)
- **Via Voltage Drop** = I × R_via
- **Via Current** (Amps) - current for specified temp rise (IPC-2152 based)
- **Via Temperature** - ambient + calculated rise
- **Conductor Cross Section** (sq.mils) - equivalent flat conductor area
- **Current Density J** (A/m²)

### Via Thermal Resistance
- **Via Thermal Resistance** (°C/W) - per single via
- **Via Count** - number of thermal vias
- **°C/W per via** - thermal resistance considering count
- **Aspect Ratio** = via_height / via_hole_diameter (warning if > limit)

## Differential Vias / Via Stub Length (separate sub-calculator)
### Inputs
- Drill Hole Diameter (drill, not finished hole)
- Ref Plane Opening H / W
- Via Spacing (center to center)
- Anisotropy (default ~1.18 for most materials)
- Baud Rate

### Outputs
- Differential impedance based on via structure
- Effective dielectric constant
- Odd mode impedance
- Insertion loss estimate
- Maximum Stub Length

### References
- Bert Simonovich - "Method of Modeling Differential Vias"
- "Dispelling Via Stub Anxieties"
- Dankov et al. - "Two-Resonator Method for Characterization of Dielectric Substrate Anisotropy"

## Key Physical Constants
- Copper resistivity: ρ = 1.724 × 10⁻⁶ Ω·cm at 20°C
- Temperature coefficient of copper: α ≈ 0.00393 /°C

## IPC-2152 Notes
- Per IPC-2152, via height influences heat dissipation
- Microvias follow different rules per IPC-2152 standard
- Aspect ratio limit configurable (default 10:1 for TH, 1:1 for microvia)
