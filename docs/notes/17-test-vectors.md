# Test Vectors

All test vectors with source attribution. Saturn PDF values are our primary
validation target — they represent the exact output we want to match.

## Sources

- **[SAT]** Saturn PCB Toolkit v8.44 Help PDF screenshots (authoritative)
- **[WEB]** Online calculators and application notes (secondary validation)
- **[CALC]** Computed from known formulas (tertiary)

---

## 1. Microstrip Impedance [SAT, PDF page 4]

```
Input:
  W = 17 mils, H = 10 mils, f = 500 MHz
  Er = 4.6, Cu = 0.5oz base + 0.5oz plating
  Total copper thickness = 2.10 mils (0.70 + 1.40)

Output:
  Zo      = 50.7426 Ω
  Lo      = 7.7883 nH/in
  Co      = 3.0248 pF/in
  Tpd     = 153.4869 ps/in
  Er_eff  = 3.2802
```

## 2. Er Effective [SAT, PDF page 15]

```
Input:
  W = 17 mils, H = 10 mils, f = 500 MHz
  Er = 4.6, Cu = 1oz + 1oz plating
  Total copper thickness = 2.10 mils (different composition, same total)

Output:
  Er_eff = 3.2802
```

Note: Same Er_eff as impedance test vector — same W/H/T/Er.

## 3. Differential Pairs [SAT, PDF page 11]

```
Input:
  W = 10 mils, S = 5 mils, H = 15 mils
  Layer = Edge Coupled External
  Er = 4.6, Cu = 1oz + 1oz plating (T = 2.10 mils)
  Protocol = DDR2 CLK/DQS (target 100Ω ±10%)

Output:
  Zdiff   = 100.979 Ω
  Zo      = 77.504 Ω
  Zodd    = 50.490 Ω
  Zeven   = 118.971 Ω
  Kb(term)    = 0.2111
  Kb(unterm)  = 0.4041
  dB(term)    = -13.512
  dB(unterm)  = -7.870
  NEXT V(term)   = 0.0064 V
  NEXT V(unterm) = 0.0123 V
  W/H = 0.667, S/H = 0.333
  Lsat = 3289.27 mils
```

## 4. Via Properties [SAT, PDF page 36]

```
Input:
  Via hole diameter = 10 mils
  Internal pad diameter = 20 mils
  Ref plane opening diameter = 40 mils
  Via height = 62 mils
  Via plating thickness = 1 mil
  Layer set = Multi Layer
  Er = 4.6, Temp rise = 20°C, Ambient = 22°C

Output:
  C_via   = 0.4021 pF
  L_via   = 1.3262 nH
  Z_via   = 57.429 Ω
  R_dc    = 0.00153 Ω
  f_res   = 6891.661 MHz
  Step response = 25.4032 ps
  Power dissipation = 0.00599 W (7.7739 dBm)
  Cross section = 34.56 sq.mils
  Current = 1.9785 A
  Thermal R = 179.3 °C/W
  Voltage drop = 3.0273 mV
  Aspect ratio = 6.20:1
```

## 5. Conductor Current [SAT, PDF page 6]

```
Input:
  Solve for = Amperage
  W = 50 mils, L = 1000 mils, PCB = 62 mils
  f = 1 MHz, Load current = 5 A
  External layer, Etch factor = 1:1
  Cu = 0.5oz base + 1oz plating (T = 2.10 mils)
  Temp rise = 20°C, Ambient = 22°C

Output:
  Skin depth = 2.59867 mils (100%)
  Power = 0.21849 W (23.3942 dBm)
  R_dc = 0.00848 Ω
  Cross section = 100.59 sq.mils
  Loaded V_drop = 0.0424 V
  V_drop = 0.0430 V
  Conductor current = 5.0763 A
  J = 7.8221 A/m²
```

## 6. Conductor Current (2:1 etch) [SAT, PDF page 46]

```
Input:
  Same as above but Etch factor = 2:1

Output:
  Skin depth = 2.59867 mils (100%)
  Power = 0.11501 W (20.6075 dBm)
  R_dc = 0.00830 Ω
  Cross section = 102.79 sq.mils
  Loaded V_drop = 0.0415 V
  V_drop = 0.0309 V
  Conductor current = 3.7232 A
```

## 7. Fusing Current [SAT, PDF page 16]

```
Input:
  W = 10 mils, t = 1 second
  Etch factor = 2:1, Onderdonk multiplier = 1
  Cu = 0.5oz base, Bare PCB (no plating)
  Ambient = 22°C

Output:
  Melting temp = 1064.62°C
  Cross section (circular mils) = 23.93
  Cross section (sq.mils) = 18.79
  Fusing current = 3.5147 A
  Total copper thickness = 0.70 mils
```

## 8. Planar Inductor [SAT, PDF page 30]

```
Input:
  Turns (n) = 5
  Conductor width (w) = 10 mils
  Conductor spacing (s) = 10 mils
  Outer diameter (dout) = 350 mils
  Geometry = Square (K1=2.34, K2=2.75)

Output:
  Inner diameter = 170.000 mils
  Fill factor (ρ) = 0.3462
  Inductance = 248.5936 nH
```

Verification:
```
din = 350 - 2*5*(10+10) + 2*10 = 350 - 200 + 20 = 170
ρ = (350 - 170) / (350 + 170) = 180/520 = 0.34615...
d_avg = (350 + 170) / 2 = 260
L = 2.34 * 4π×10⁻⁷ * 25 * 260 * 25.4e-6 / (1 + 2.75 * 0.34615)
  = 2.34 * 1.2566e-6 * 25 * 6.604e-3 / (1 + 0.9519)
  = 2.34 * 1.2566e-6 * 0.1651 / 1.9519
  (verify against Saturn's 248.5936 nH)
```

## 9. Reactance [SAT, PDF page 41]

```
Input:
  f = 1 MHz, C = 1 µF, L = 1 mH

Output:
  Xc = 0.1592 Ω
  Xl = 6283.1800 Ω
  f_res = 5032.9255 Hz
```

Verification:
```
Xc = 1 / (2π × 1e6 × 1e-6) = 1 / 6.28318... = 0.15915...
Xl = 2π × 1e6 × 1e-3 = 6283.185...
f_res = 1 / (2π × √(1e-3 × 1e-6)) = 1 / (2π × √1e-9)
      = 1 / (2π × 3.16228e-5) = 1 / 1.98692e-4 = 5032.92...
```

## 10. PPM — Hz to PPM [SAT, PDF page 32]

```
Input:
  Center frequency = 32000 Hz
  Maximum frequency = 32001 Hz

Output:
  Variation = 1.00 Hz
  PPM = 31.25
```

## 11. PPM — PPM to Hz [SAT, PDF page 32]

```
Input:
  Center frequency = 50000000 Hz (50 MHz)
  PPM = 25

Output:
  Variation = 1250.00 Hz
  Maximum = 50001250.00 Hz
  Minimum = 49998750.00 Hz
```

## 12. PPM — XTAL Load Capacitor [SAT, PDF page 32]

```
Input:
  C_load = 10 pF, C_stray = 3 pF
  C1 = 14 pF, C2 = 14 pF

Output:
  Based on C1/C2: (14×14)/(14+14) + 3 = 7 + 3 = 10.00 pF
  Rule of thumb: 2 × (10 - 3) = 14.00 pF
```

## 13. Wavelength [SAT, from notes, PDF page ~40]

```
Input:
  Period = 10 ns (f = 100 MHz)
  Er_eff = 4
  Conductor type = Microstrip

Output:
  Full wavelength = 59.01426 inches
```

Verification:
```
λ = c / (f × √Er_eff) = 11.803 in/ns × 10 ns / √4 = 118.03 / 2 = 59.015
(Saturn shows 59.01426 — slight precision difference)
```

## 14. Ohm's Law [SAT, PDF page 21]

```
E-I-R:
  V = 12 V, I = 1 A, R = 12 Ω → P = 12.0000 W

LED Bias:
  Vs = 12 V, Vled = 2 V, Iled = 10 mA
  → R = (12-2)/0.01 = 1000.0 Ω
  → P = 0.01 × (12-2) = 0.1000 W
```

## 15. Padstack — Thru-Hole [SAT, PDF page 23]

```
Input:
  Hole diameter = 32 mils
  Annular ring = 12 mils
  Isolation width = 12 mils

Output:
  External layers = 56.00 mils (32 + 2×12)
  Internal signal layers = 56.00 mils
  Internal plane layers = 80.00 mils (56 + 2×12)
```

## 16. Conductor Spacing [SAT, PDF page 20]

```
B1 (Internal Conductors):
  0-15V   → 1.97 mils
  16-30V  → 2.0 mils (from notes)
  31-50V  → 4.0 mils (from notes)
```

## 17. Crosstalk [SAT, PDF page 47]

```
Input:
  Signal rise time = 1 ns
  Signal voltage = 5 V
  Coupled length = 6350 µm
  Spacing (S) = 254 µm
  Height (H) = 762 µm
  Er = 4.6, Conductor type = Microstrip

Output:
  Crosstalk coefficient = -2.23327 dB
  Coupled voltage = 3.86640 V
```

## 18. Bandwidth [SAT, PDF page 2]

```
Input:
  Signal risetime = 1 ns
  Er = 4.6
  Passive circuits = Microstrip
  Sr divide by factor = 0.25Sr
  Lambda divide by factor = 1/7

Output:
  Bandwidth = 350.00000 MHz (= 0.35 / 1e-9)
  Propagation speed = 139778954.280 m/s (= c/√Er = 299792458/√4.6)
  Max conductor length (IPC-2251) = 1.77221 inches
  Full wavelength (in air) = 33.72244 inches
```

---

## Secondary Validation Vectors [WEB]

### Microstrip — 50Ω target designs [disk91.com]

```
Er=4.3, H=0.8mm, T=0.035mm → W≈1.52mm for 50Ω
Er=4.3, H=1.0mm, T=0.035mm → W≈1.90mm for 50Ω
Er=4.3, H=1.6mm, T=0.035mm → W≈3.06mm for 50Ω
```

### Via Properties — Hand Calculation [Sierra Circuits]

```
L = 5.08 × 0.050 × [ln(4×0.050/0.010) + 1] = 1.015 nH
C = 1.41 × 4.4 × 0.050 × 0.020 / (0.032-0.020) = 0.517 pF
Z = √(1.015e-9/0.517e-12) ≈ 44.3 Ω
```

### IPC-2221A Current [formula verification]

```
External, dT=10°C, A=100 sq.mils:
  I = 0.048 × 10^0.44 × 100^0.725 = 0.048 × 2.754 × 28.18 = 3.73 A

Internal, same:
  I = 0.024 × 10^0.44 × 100^0.725 = 1.86 A
```

### Reactance — Additional vectors [formula verification]

```
C=10nF, f=1kHz → Xc = 15915 Ω
L=100µH, f=1kHz → Xl = 0.6283 Ω
L=1mH, C=220pF → f_res = 339.32 kHz
L=50µH, C=100pF → f_res = 2.251 MHz
```

### Attenuators — Pi-pad at 50Ω [Electronics Notes]

```
 dB | R1(series) | R2(shunt)
  1 |    2.9 Ω   |  870 Ω
  3 |    8.5 Ω   |  292 Ω
  6 |   16.6 Ω   |  151 Ω
 10 |   26.0 Ω   |   96.2 Ω
 20 |   40.9 Ω   |   61.1 Ω
```

### Attenuators — T-pad at 50Ω [Electronics Notes]

```
 dB | R1(series) | R2(shunt)
  1 |    2.9 Ω   |  433 Ω
  3 |    8.5 Ω   |  142 Ω
  6 |   16.6 Ω   |   66.9 Ω
 10 |   26.0 Ω   |   35.1 Ω
 20 |   40.9 Ω   |   10.1 Ω
```

### IPC-2221C Spacing Table [smpspowersupply.com]

```
Voltage | B1 Internal | B2 Ext Uncoated | B4 Solder Mask
  0-15V |   0.05 mm   |    0.10 mm      |   0.05 mm
 16-30V |   0.05 mm   |    0.10 mm      |   0.05 mm
 31-50V |   0.10 mm   |    0.60 mm      |   0.13 mm
51-100V |   0.10 mm   |    0.60 mm      |   0.13 mm
101-150 |   0.20 mm   |    0.60 mm      |   0.40 mm
151-170 |   0.20 mm   |    1.25 mm      |   0.40 mm
171-250 |   0.20 mm   |    1.25 mm      |   0.40 mm
251-300 |   0.20 mm   |    1.25 mm      |   0.40 mm
301-500 |   0.25 mm   |    2.50 mm      |   0.80 mm
```

### Differential — Edge Coupled Stripline [elektroda.com]

```
Input: T=1.2mil, H=63mil, W=10mil, S=63mil, Er=4
Output: Zodd=78.4, Zeven=78.3, Zdiff=157, Zcommon=39.2
```

### Skin Depth [formula verification]

```
δ = √(ρ/(π×f×µ₀))
At 1 MHz: δ = √(1.724e-8 / (π × 1e6 × 4π×10⁻⁷))
        = √(1.724e-8 / 3.9478e-6)
        = √(4.367e-3) = 0.06608 mm = 2.602 mils
(matches Saturn's 2.59867 mils closely)
```
