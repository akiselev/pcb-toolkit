# Stripline Impedance: Clean Implementable Formulas

Cross-reference of Saturn PCB Toolkit v8.44 decompilation (`ghidra-stripline.md`)
against published Cohn/Wadell formulas. All formulas are directly translatable to
Rust `f64` arithmetic.

---

## References

- **[Cohn 1954]** S.B. Cohn, "Characteristic Impedance of the Shielded-Strip
  Transmission Line," IRE Trans. MTT, Vol. 2, No. 2, July 1954, pp. 52-57.
- **[Cohn 1955]** S.B. Cohn, "Shielded Coupled-Strip Transmission Line," IRE Trans.
  MTT, Vol. MTT-3, Oct. 1955, pp. 29-38.
- **[Wadell 1991]** B.C. Wadell, *Transmission Line Design Handbook*, Artech House,
  1991, Chapter 3.
- **[Steer]** M. Steer, *Microwave and RF Design II: Transmission Lines*, LibreTexts,
  Section 3.7: Stripline.
- **[wcalc]** D. McMahill, wcalc transmission line calculator, `stripline.c` and
  `coupled_stripline.c` (open source, implements Cohn/Wadell).

---

## 1. Geometry and Input Variables

```
Symmetric stripline cross-section:

    ─────────────────────  Ground plane (top)
    |         H1         |
    |    ┌──────────┐    |
    |    │  W x T   │    |  Strip conductor
    |    └──────────┘    |
    |         H2         |
    ─────────────────────  Ground plane (bottom)

B = H1 + H2 + T   (total ground-to-ground spacing, but see Note below)
```

**Saturn convention (from decompilation):**
Saturn reads H1 and H2 as the dielectric heights (distance from strip edge to
ground plane), and computes `B = H1 + H2` directly. The strip thickness T sits
inside B. This matches the standard Wadell convention where B is the full
ground-plane-to-ground-plane distance.

| Symbol | Description                       | Saturn Global     | Canonical Unit |
|--------|-----------------------------------|-------------------|----------------|
| W      | Strip/trace width                 | `[008d6034]`      | mils           |
| T      | Strip/trace thickness             | `[008d6064]`      | mils           |
| H1     | Dielectric height above strip     | `[008d61b8]`      | mils           |
| H2     | Dielectric height below strip     | `[008d6668]`      | mils           |
| B      | Ground plane spacing (H1+H2)     | `[008d603c]`      | mils           |
| Er     | Relative permittivity             | `[008d6054]`      | dimensionless  |
| Rho    | Conductor resistivity (uOhm-cm)  | `[008d67c8]`      | (divided by 1000 internally) |

---

## 2. Published Formulas: Two Approaches

Saturn implements a **hybrid** approach. For the core Z0, it appears to use the
**Wadell finite-thickness formula** (Section 2B). For coupled lines, it uses the
**Cohn conformal-mapping formula** (Section 2A). Both are documented below.

### 2A. Cohn Exact Formula (Zero Thickness)

For a zero-thickness strip centered between ground planes (from [Cohn 1954]):

```
Z0 = (eta_0 / (4 * sqrt(Er))) * K(k') / K(k)
```

Where:
- `eta_0 = 120 * pi = 376.991 ohms` (free-space impedance)
- `k = sech(pi * W / (2 * B)) = 1 / cosh(pi * W / (2 * B))`
- `k' = sqrt(1 - k^2) = tanh(pi * W / (2 * B))`
- `K(k)` = complete elliptic integral of the first kind with modulus k

**Equivalent forms:**

```
Z0 = (30 * pi / sqrt(Er)) * K(k') / K(k)        [since eta_0/4 = 30*pi]
   = (94.25 / sqrt(Er)) * K(k') / K(k)           [numerical]
```

**Binary constant mapping:**
| Published Value  | Binary Constant   | Address      |
|------------------|-------------------|--------------|
| `120 * pi`       | `376.9908`        | `0x00422c5c` |
| `120 * pi`       | `376.991`         | `0x00422d84` |
| `60 * pi`        | `188.4954`        | `0x00422dbc` |
| `2 * pi`         | `6.28318`         | `0x00422c64` |
| `pi`             | `3.14159`         | `0x00422bfc` |

### 2B. Wadell Finite-Thickness Formula

For a strip with finite thickness T (from [Wadell 1991], as implemented in wcalc):

#### Step 1: Thickness correction (effective width increase)

```
m = 6 * (B - T) / (3 * B - T)

delta_W = (T / pi) * (1 - 0.5 * ln((T / (2*B - T))^2
          + (0.0796 * T / (W + 1.1*T))^m))

W_eff = W + delta_W
```

Where all dimensions are in consistent units (mils or mm).

#### Step 2: Impedance from effective width

```
A = 4 * (B - T) / (pi * W_eff)

Z0 = (30 / sqrt(Er)) * ln(1 + A * (2*A + sqrt(4*A^2 + 6.27)))
```

**Accuracy:** Better than 0.5% for `A > 0.25` (i.e., for `(B-T)/(pi*W_eff) > 0.0625`).

**Binary constant mapping for Wadell formula:**
| Published Value | Binary Constant  | Address      | Notes                          |
|-----------------|------------------|--------------|--------------------------------|
| `6.27`          | `6.28318`        | `0x00422c64` | Saturn uses `2*pi` (see note)  |
| `30` (= eta_0 / (4*pi))| `30.666`| `0x00422c44` | See discrepancy analysis below |
| `0.0796`        | *(see Step 5)*   | *(in delta_W)* | Wadell thickness correction  |

> **DISCREPANCY:** The constant `30.666` at `0x00422c44` does not match a clean
> `30/sqrt(Er)` for any standard Er value. It could be a precomputed value for a
> specific material, or a slightly different formula variant. The value `30.666`
> is close to `60/sqrt(3.83)` = `30.66`, suggesting Saturn may precompute
> `60/sqrt(Er)` or use a formula variant with `60` rather than `30`, and include
> the `ln(...)` denominator differently.
>
> **Most likely interpretation:** Saturn uses the **IPC-2141A formula** (see
> Section 2C) where the leading constant is `60/sqrt(Er)` and the argument to
> `ln()` contains `4*B / (0.67 * pi * (0.8*W + T))`. The `30.666` is then
> a **runtime intermediate**, not a standalone constant. See Section 3 for the
> reconstructed Saturn formula.

### 2C. IPC-2141A Simplified Formula

The IPC-2141A standard provides a simplified closed-form approximation:

```
Z0 = (60 / sqrt(Er)) * ln(4 * B / (0.67 * pi * (0.8 * W + T)))
```

Where:
- B = total ground-to-ground spacing
- W = trace width
- T = trace thickness
- Er = relative permittivity

This is accurate to about 1-2% for typical PCB geometries. It is a simplification
of the Wadell formula where the `0.67` factor accounts for fringing and the
`(0.8*W + T)` provides an effective width estimate.

**Binary constant candidates:**
| Published | Binary Value | Address      |
|-----------|-------------|--------------|
| 60.0      | 60.0        | `0x00422d10` |
| 0.8       | 0.8         | `0x00422d14` |

### 2D. Steer/Cohn Formula with Fringing Capacitance

The form presented in [Steer], attributed to Cohn:

**Finite thickness:**
```
Z0 = (30 * pi / sqrt(Er)) * (1 - t/b) / (w_eff/b + Cf)
```

Where:
```
w_eff/b = w/b - (0.35 - w/b)^2 / (1 + 12*t/b)   for w/b < 0.35
w_eff/b = w/b                                      for w/b >= 0.35

Cf = (2/pi) * ln(1/(1 - t/b) + 1) - (t/(pi*b)) * ln(1/(1 - t/b)^2 - 1)
```

**Zero thickness:**
```
Z0 = (30 * pi / sqrt(Er)) * 1 / (w_eff/b + 2*ln(2)/pi)
   = 94.25 / sqrt(Er) * 1 / (w_eff/b + 0.441)
```

**Binary constant mapping:**
| Published          | Value    | Binary       | Address      |
|--------------------|----------|--------------|--------------|
| `2*ln(2)/pi`       | `0.4413` | `0.432`      | `0x00422c0c` |
| `30*pi/sqrt(Er)`   | `94.25`  | *(computed)* |              |
| `0.35` (threshold) | `0.35`   | `0.35`       | `0x00422d4c` |

> **DISCREPANCY:** Saturn uses `0.432` where published Cohn/Steer gives
> `2*ln(2)/pi = 0.4413`. The difference is ~2%. This could be a deliberate
> empirical adjustment by Saturn's author to match measured data.

---

## 3. Saturn's Reconstructed Core Formula (Mode 0: Solve for Z0)

Based on the FPU instruction trace at `0x0040c647 - 0x0040ce1c`, Saturn's actual
computation appears to be a multi-stage process that combines elements from both
the Cohn fringing capacitance model and the IPC/Wadell formulas.

### Step 1: Unit Conversion

```rust
// Convert mils to mm (all internal computation in mm)
let w_mm = W_mils / 1000.0 * 25.4;    // [0x00422be0, 0x00422be4]
let t_mm = T_mils / 1000.0 * 25.4;
let b_mm = B_mils / 1000.0 * 25.4;    // B = H1 + H2
```

Binary constants:
- `1000.0` at `0x00422be0`
- `25.4` at `0x00422be4`

### Step 2: Normalize to Thickness

```rust
let b = b_mm / t_mm;   // B/T ratio, stored at [008d6720]
let w = w_mm / t_mm;   // W/T ratio, stored at [008d6730]
```

### Step 3: Effective Width (First Correction)

```rust
// Cohn first effective width correction
// Uses constants: 6.517, 10.8731272 = 4*e, pi
let w_eff_1 = w + ln(4.0 * E / b) * (6.517 * w) / PI;
// Stored at [008d6728]
```

Binary constants:
- `6.517` at `0x00422bec` -- possibly a Cohn geometry coefficient
- `10.8731272` at `0x00422bf4` -- this is `4 * e` where e = Euler's number = 2.71828
  (`4 * 2.71828 = 10.87313`)
- `pi` at `0x00422bfc`
- `e` at `0x00422d7c` (value 2.71828)

> **Identification:** `10.8731272 = 4*e`. In the Wadell formula, the argument to
> the `ln()` function involves `4*(B-T)/(pi*W_eff)`. The factor `4*e` arises in
> the Cohn conformal mapping as `4*e*B/(pi*W)` which appears in some formulations
> of the zero-thickness strip impedance as an intermediate value. In the
> decompiled code, `ln(4*e / b)` computes `ln(4*e / (B/T))` = `ln(4*e*T/B)`.

### Step 4: Er-Dependent Width Correction

```rust
// Adjusts W_eff for dielectric constant
// Uses cosh(ln(x)) identity: cosh(ln(x)) = (x + 1/x) / 2
let correction = 1.0 / cosh(ln(er - 1.0)) + 1.0;
let w_eff_2 = (w_eff_1 - w) * correction / 2.0 + w;
// Stored at [008d6738]
```

> **Note:** `cosh(ln(x)) = (x + 1/x) / 2`. So `1/cosh(ln(Er-1))` =
> `2 / ((Er-1) + 1/(Er-1))` = `2*(Er-1) / ((Er-1)^2 + 1)`. This is an
> interpolation factor that approaches 1 for Er near 2 and decreases for
> higher Er.

### Step 5: Fringe Capacitance Factor (Cf)

```rust
// Complex multi-term fringe correction
// Uses constants: 0.432, 2704, 49, 18.1, 18.7, 0.9
let term_a = ln(w_eff_2) + ln(w_eff_2) / 2704.0;
let term_b = term_a + ln(w_eff_2) + 0.432;     // [0x00422c0c]
let term_c = term_b.powf(w_eff_2);
let result = term_c / 49.0 + 1.0;              // [0x00422c14]

let sub_term = w_eff_2 / 18.1;                 // [0x00422c18]
let ln_sub = ln(sub_term) + 1.0;
let pow_sub = ln_sub.powf(/* ... */);
let cf = pow_sub / 18.7 + result;              // [0x00422c20]
// Stored at [008d6740]
```

Binary constants:
- `0.432` at `0x00422c0c` -- fringe correction (cf. published `0.441 = 2*ln(2)/pi`)
- `2704.0` at `0x00422c08` -- large divisor, likely empirical smoothing
- `49.0` at `0x00422c14` -- divisor
- `18.1` at `0x00422c18` -- Cohn model constant
- `18.7` at `0x00422c20` -- Cohn model constant
- `0.9` at `0x00422c28` -- thickness correction

> **Note:** The fringe capacitance computation in Saturn is significantly more
> elaborate than the simple Cf formula in published references. The constants
> `2704`, `49`, `18.1`, `18.7` do not appear in any standard published formula.
> These are likely **empirical curve-fit coefficients** proprietary to Saturn,
> or from a lesser-known reference.

### Step 6: Er Correction Factor

```rust
// Matches published: involves ln((Er-0.9)/(Er+3)) * 0.564
let er_corr = ln((er - 0.9) / (er + 3.0)) * 0.564;
// Stored at [008d6748]
```

Binary constants:
- `0.564` at `0x00422c34` -- this is a well-known microstrip Er_eff correction
  coefficient from Hammerstad-Jensen. It appears in the formula:
  `Er_eff = (Er+1)/2 + (Er-1)/2 * F(W/H) - 0.564 * ((Er-0.9)/(Er+3))^0.053`
  (though here Saturn uses it linearly, not as an exponent).
- `0.9` at `0x00422c28`
- `3.0` at `0x00422c30`

> **DISCREPANCY:** The Hammerstad-Jensen formula uses `0.564 * ((Er-0.9)/(Er+3))^0.053`,
> but Saturn applies `ln((Er-0.9)/(Er+3)) * 0.564` -- a different functional form.
> This might be Saturn's own adaptation.

### Step 7: Effective Dielectric Constant

```rust
// Modified effective Er for stripline
let er_eff = (er + 1.0) / 2.0 + (er - 1.0) / 2.0 * ln(10.0 / w_eff_2 + 1.0);
// Stored at [008d6760]
```

Binary constants:
- `10.0` at `0x00422c40`

> **Note:** For pure symmetric stripline, Er_eff = Er (the strip is fully
> surrounded by dielectric). This computation suggests Saturn applies a
> small correction factor, possibly for asymmetry or for the finite-thickness
> case. In a true symmetric stripline, this factor would equal Er.

### Step 8: Core Impedance Computation

```rust
// Two parallel impedance terms for W_eff_1 and W_eff_2
// Uses constants: 30.666, 376.9908, 6.28318, 0.28318

// For each effective width W_eff (both _1 and _2):
let z_arg = 30.666 / w_eff;                    // [0x00422c44]
let z = -ln(z_arg);

let corr_a = ln(w_eff) * 0.28318 + 6.0;       // [0x00422c4c, 0x00422c54]
let corr_b = 4.0 / w_eff;                      // [0x00422c58]
let corr_c = ln(corr_b + 1.0);
let z_full = (corr_a + corr_c).exp() * 376.9908 / 6.28318;
// [0x00422c5c, 0x00422c64]
```

Binary constants:
- `30.666` at `0x00422c44` -- see discrepancy analysis above
- `0.28318` at `0x00422c4c` -- close to `0.9/pi = 0.2865` or `1/pi - 1/(2*pi^2)`
- `376.9908` at `0x00422c5c` -- `120 * pi` (free-space impedance)
- `6.28318` at `0x00422c64` -- `2 * pi`
- `376.9908 / 6.28318 = 60.0` -- this ratio gives exactly `60`, the classic
  stripline impedance coefficient

> **Key insight:** The expression `376.9908 / 6.28318` = `120*pi / (2*pi)` = `60`.
> This confirms that the core computation uses the `60/sqrt(Er)` form from the
> Cohn/IPC formula, computed symbolically rather than as a literal `60`.

### Step 9: Surface Roughness and Coupling Corrections

```rust
// Polynomial correction factors (empirical)
// Constants: 0.525, 0.6315, 0.27488, -8.7513, 0.065683
//            -0.03442, 0.33622, 38.7, -4.6, 0.0363
//            15.916, 2.751, 0.1844

let skin_factor = 0.0157 * rho * t_mm + 1.0;   // [0x00422c6c]

// Polynomial A: geometry-dependent
let a = (ln(w) * 0.525 + 0.6315) * w + 0.27488;          // [0x00422c74, 0x00422c7c, 0x00422c84]
let b_term = (-8.7513 * w).exp() * 0.065683;              // [0x00422c8c, 0x00422c94]
let a_coeff = a - b_term;                                  // [008d6780]

// Polynomial C: Er-dependent damping
let c_coeff = (1.0 - (-0.03442 * er).exp()) * 0.33622;   // [0x00422c9c, 0x00422ca4]
// [008d6788]

// D: resistivity/thickness
let d_coeff = -ln(rho * t_mm / 38.7);                     // [0x00422cac]
// [008d67b8]

// E: exponential decay
let e_coeff = (1.0 - (-4.6 * w).exp()) * 0.0363;         // [0x00422cb4, 0x00422cbc]
// [008d6790]

// Composite Er correction
let g = er / 15.916;                                       // [0x00422cc4]
let i_coeff = (1.0 - (-g).exp()) * 2.751 + 1.0;          // [0x00422ccc]
// [008d6798]

// Final compound correction
let correction_factor = /* compound of above terms */;     // [008d67a0]

// Apply correction to get final Z0
let z0_final = er - (er - er_eff) / (1.0 + correction_factor);
// [008d67c0]
```

Binary constants:
- `0.0157` at `0x00422c6c` -- skin effect related
- `0.525` at `0x00422c74` -- polynomial coefficient
- `0.6315` at `0x00422c7c` -- polynomial coefficient
- `0.27488` at `0x00422c84` -- polynomial coefficient
- `-8.7513` at `0x00422c8c` -- exponential decay rate
- `0.065683` at `0x00422c94` -- scaling factor
- `-0.03442` at `0x00422c9c` -- exponential decay rate
- `0.33622` at `0x00422ca4` -- scaling factor
- `38.7` at `0x00422cac` -- normalization (~`120*pi / pi = 120`)
- `-4.6` at `0x00422cb4` -- exponential decay rate
- `0.0363` at `0x00422cbc` -- scaling factor
- `15.916` at `0x00422cc4` -- normalization (~`50/pi = 15.915`)
- `2.751` at `0x00422ccc` -- coefficient
- `0.1844` at `0x00422cd4` -- coefficient

> **Note:** These polynomial correction coefficients are NOT from any standard
> published formula. They appear to be empirical curve-fit corrections
> proprietary to Saturn PCB Toolkit, possibly calibrated against measured data
> or field-solver results. The value `15.916 ~ 50/pi` and `38.7 ~ 120*pi/pi`
> suggest systematic derivation from electromagnetic constants.

---

## 4. Propagation Delay and Derived Quantities

### 4A. Propagation Delay

For symmetric stripline (strip fully immersed in dielectric):

```rust
// Stripline propagation delay: Er_eff = Er (ideally)
// Speed of light in vacuum: c = 11.803 inches/ns = 299,792,458 m/s
// Tpd = sqrt(Er_eff) / c_inches_per_ns
// Tpd = sqrt(Er) * 85.0 ps/inch   (approximately)
//      = sqrt(Er) * 84.72 ps/inch  (exact: 1e6 / 11.803 = 84.72)

let tpd_ps_per_inch = (er_eff).sqrt() * 1000.0 / 11.8;
// Binary constant: 11.8 at [0x00422cf4]
```

Binary constant: `11.8` at `0x00422cf4` -- speed of light in inches/ns (truncated
from 11.803).

The formula `1000.0 / 11.8 = 84.75 ps/inch` is the free-space delay; multiply
by `sqrt(Er_eff)` for the material.

### 4B. Capacitance Per Unit Length

```rust
// C = 1 / (Z0 * v_p) = Er_eff / (Z0 * c)
// Or equivalently: C = Tpd / Z0
let c_per_length = tpd / z0;   // pF/inch
```

### 4C. Inductance Per Unit Length

```rust
// L = Z0 * Tpd = Z0 / v_p
let l_per_length = z0 * tpd;   // nH/inch
```

### 4D. Skin Resistance

```rust
// R_skin = sqrt(pi * f * mu_0 * rho) / W_eff
// The constant 0.08457 ~ 1/(4*pi)^0.5 is used in the skin depth formula
// Binary: 0.08457 at [0x00422d24]
// Actual: 1/sqrt(4*pi) = 0.2821 -- so 0.08457 may be 1/(4*pi) = 0.07958
// (closer to 0x00422da0 which is 0.079577538)
```

Binary constants:
- `0.08457` at `0x00422d24` -- skin depth related factor
- `0.079577538` at `0x00422da0` -- `1/(4*pi)` (exact: 0.07957747...)
- `0.31831015` at `0x00422da8` -- `1/pi` (exact: 0.31830989...)
- `0.102101675` at `0x00422d98` -- possibly `1/(2*pi*sqrt(Er))` for some Er

---

## 5. Coupled/Differential Stripline (Cohn Exact Formula)

For edge-coupled striplines (two parallel strips centered between ground planes),
Saturn uses the **Cohn conformal-mapping formulas** from [Cohn 1955].

### 5A. Zero-Thickness Coupling Parameters

```rust
// Even mode modulus (Cohn eq. 3)
let ke = (PI * w / (2.0 * b)).tanh() * (PI * (w + s) / (2.0 * b)).tanh();

// Odd mode modulus (Cohn eq. 6)
let ko = (PI * w / (2.0 * b)).tanh() * (PI * (w + s) / (2.0 * b)).coth();
```

Where:
- `w` = trace width (one trace)
- `s` = edge-to-edge spacing between the two traces
- `b` = ground-to-ground spacing

### 5B. Even and Odd Mode Impedances (Zero Thickness)

```rust
// Even mode impedance (Cohn eq. 2)
let z0e = (ETA_0 / (4.0 * er.sqrt())) * k_ratio(ke);

// Odd mode impedance (Cohn eq. 5)
let z0o = (ETA_0 / (4.0 * er.sqrt())) * k_ratio(ko);
```

Where `k_ratio(k)` computes `K(k') / K(k)`:
- `K(k)` = complete elliptic integral of the first kind
- `k' = sqrt(1 - k^2)` = complementary modulus

Constants:
- `ETA_0 = 120 * pi = 376.991 ohms`
- `ETA_0 / 4 = 30 * pi = 94.248`

### 5C. Thickness-Corrected Coupled Impedances

From [Cohn 1955], the finite-thickness corrections use fringing capacitance:

```rust
// Fringing capacitance with thickness (Cohn eq. 13 / [Steer])
let cf_t = (EPSILON_0 * er / PI) * (
    (2.0 / (1.0 - t/b)) * ln(1.0 / (1.0 - t/b) + 1.0)
    - ((1.0 / (1.0 - t/b)) - 1.0) * ln(1.0 / (1.0 - t/b).powi(2) - 1.0)
);

// Zero-thickness fringing capacitance
let cf_0 = (EPSILON_0 * er / PI) * 2.0 * LN_2;
// cf_0 / (epsilon_0 * er) = 2*ln(2)/pi = 0.4413
```

**Even mode with thickness (Cohn eq. 18):**
```rust
let z0e = 1.0 / (1.0/z0s - (cf_t/cf_0) * (1.0/z0s_0t - 1.0/z0e_0t));
```

Where:
- `z0s` = single-strip impedance with finite thickness
- `z0s_0t` = single-strip impedance with zero thickness
- `z0e_0t` = even-mode impedance with zero thickness

**Odd mode with thickness, s >= 5*t (Cohn eq. 20):**
```rust
let z0o = 1.0 / (1.0/z0s + (cf_t/cf_0) * (1.0/z0o_0t - 1.0/z0s_0t));
```

**Odd mode with thickness, s < 5*t (Cohn eq. 22):**
```rust
let z0o = 1.0 / (1.0/z0o_0t + (1.0/z0s - 1.0/z0s_0t)
    - (2.0/ETA_0) * (cf_t/EPSILON_0 - cf_0/EPSILON_0)
    + 2.0*t / (ETA_0 * s));
```

### 5D. Differential and Common Mode Impedances

```rust
let z0     = (z0e * z0o).sqrt();     // characteristic impedance
let z_diff = 2.0 * z0o;              // differential impedance
let z_comm = z0e / 2.0;              // common-mode impedance
let k_b    = (z0e - z0o) / (z0e + z0o);  // coupling coefficient
```

### 5E. Saturn's Differential Implementation

From the decompilation at `0x004118bf`, Saturn's coupled computation uses `sin()`
and `cosh()` which map to the conformal mapping formulas:

```rust
// From decompilation (0x00411a33 - 0x00411c5e):
let sin_term = sin(PI * w_half / b_total);
let cosh_a = cosh(-PI * ratio / (2.0 * b_total));
let cosh_b = cosh(-PI * ratio / (2.0 * b_total));
let coupled = sin_term * (cosh_a + cosh_b) / (cosh_a - cosh_b);
```

These `sin/cosh` expressions are related to the Cohn conformal mapping via the
identities:
- `tanh(x) = (e^x - e^-x) / (e^x + e^-x)`
- The coupling parameter involves `tanh(pi*w/(2b))` which can be expressed
  using `sin()` and `cosh()` through Jacobi theta function identities.

**Polynomial correction in coupled mode:**
Uses constants: `1.023`, `1.0235`, `0.5008`, `1.1564`, `0.4749`

| Binary Value | Address      | Role                                      |
|--------------|--------------|-------------------------------------------|
| `1.023`      | `0x00422d54` | Coupling polynomial coefficient           |
| `1.0235`     | `0x00422d5c` | Coupling polynomial coefficient           |
| `0.5008`     | `0x00422d64` | Coupling polynomial offset                |
| `1.1564`     | `0x00422d6c` | Coupling polynomial coefficient           |
| `0.4749`     | `0x00422d74` | Coupling polynomial coefficient           |

> These do NOT appear in standard Cohn/Wadell formulas. They are likely
> Saturn-specific empirical corrections for finite-thickness coupled lines,
> possibly calibrated against field-solver data.

---

## 6. Elliptic Integral K(k) Implementation

The Cohn formulas require computing the ratio `K(k') / K(k)` where `K` is the
complete elliptic integral of the first kind. Two approaches:

### 6A. Iterative AGM Method (from wcalc)

The Arithmetic-Geometric Mean (AGM) iteration converges quadratically:

```rust
fn k_over_kp(k: f64) -> f64 {
    // Returns K(k) / K(k')
    let mut kp = (1.0 - k * k).sqrt();  // complementary modulus
    let mut k = k;
    let mut r = 1.0;
    loop {
        let k_new = 2.0 * k.sqrt() / (1.0 + k);
        let kp_new = 2.0 * kp.sqrt() / (1.0 + kp);
        r *= (1.0 + k) / (1.0 + kp);
        k = k_new;
        kp = kp_new;
        if (1.0 - k).abs() < 1e-15 {
            break;
        }
    }
    r
}
```

For our purposes we want `K(k') / K(k) = 1 / k_over_kp(k)`.

### 6B. Approximation (Hilberg, as used in many PCB tools)

For `0 <= k <= 1/sqrt(2)`:
```rust
K(k')/K(k) = pi / ln(2 * (1 + kp.sqrt()) / (1 - kp.sqrt()))
```

For `1/sqrt(2) <= k <= 1`:
```rust
K(k')/K(k) = ln(2 * (1 + k.sqrt()) / (1 - k.sqrt())) / pi
```

Where `kp = sqrt(1 - k^2)`.

> Saturn may bypass the elliptic integral entirely and use the polynomial
> correction terms (Section 5E) as a curve-fit approximation, which would
> explain the proprietary constants.

---

## 7. Iterative Solvers (Modes 1-3)

### 7A. Solve for Width (Mode 1, at 0x0040e395)

Given target Z0, Er, T, B, find W:

```rust
// Initial guess from IPC-2141A inverse:
// W_guess = B * (exp(Z0_target * sqrt(Er) / 60.0) * 0.67 * pi)^(-1) * 4.0 / 0.8 - T/0.8

// Newton-Raphson iteration:
loop {
    let z0_computed = compute_z0(w_guess, t, b, er);
    let error = z0_computed - z0_target;
    if error.abs() < 0.1 {  // convergence threshold [0x00422cfc]
        break;
    }
    // Numerical derivative by perturbation
    let dz0_dw = (compute_z0(w_guess + delta, t, b, er) - z0_computed) / delta;
    w_guess -= error / dz0_dw;
}
```

Binary constants:
- `0.1` at `0x00422cfc` -- convergence threshold
- Range checks: `5.0` at `0x00422d2c`, `15.0` at `0x00422d04`

### 7B. Solve for Height (Mode 2, at 0x0040f825)

Same Newton-Raphson approach, iterating on B (or H) instead of W.
Range checks: `40.0` at `0x00422d30`, `90.0` at `0x00422d34`.

### 7C. Solve for Spacing (Mode 3, at 0x00410d4e)

Iterates on ground-plane spacing B given target impedance and other parameters.

### 7D. Thickness Correction in Iterative Solve

From the decompilation at `0x0040e674`:
```rust
// Effective width in iterative solver
let exp_term = (-1.55 * w_guess / t).exp();     // [0x00422d08]
let delta_w = (1.0 - exp_term) * er;            // effective width correction

// Thickness-corrected width
let w_corr = ((0.8 * w + b / 1000.0) / (5.98 * t)).powf(t_eff);
// Constants: 0.8 [0x00422d14], 5.98 [0x00422d1c], -1.55 [0x00422d08]
```

Binary constants:
- `-1.55` at `0x00422d08`
- `0.8` at `0x00422d14`
- `5.98` at `0x00422d1c`

---

## 8. Full Constant Table with Identifications

| Address      | Value          | Published Formula Term                     | Confidence |
|--------------|----------------|--------------------------------------------|------------|
| `0x00422bd8` | `1.0`          | Unity                                      | Certain    |
| `0x00422bdc` | `0.0`          | Zero                                       | Certain    |
| `0x00422be0` | `1000.0`       | mil-to-inch factor                         | Certain    |
| `0x00422be4` | `25.4`         | mm per inch                                | Certain    |
| `0x00422bec` | `6.517`        | Cohn geometry coefficient (proprietary?)   | Uncertain  |
| `0x00422bf4` | `10.8731272`   | `4 * e` (4 * 2.71828)                     | Certain    |
| `0x00422bfc` | `3.14159`      | `pi`                                       | Certain    |
| `0x00422c04` | `2.0`          | Constant 2                                 | Certain    |
| `0x00422c08` | `2704.0`       | Empirical smoothing divisor (proprietary)  | Uncertain  |
| `0x00422c0c` | `0.432`        | Fringe factor (cf. `2*ln(2)/pi = 0.4413`) | High       |
| `0x00422c14` | `49.0`         | Empirical divisor (proprietary)            | Uncertain  |
| `0x00422c18` | `18.1`         | Cohn model constant (proprietary)          | Uncertain  |
| `0x00422c20` | `18.7`         | Cohn model constant (proprietary)          | Uncertain  |
| `0x00422c28` | `0.9`          | Er correction offset (from H-J: Er - 0.9) | High       |
| `0x00422c30` | `3.0`          | Er correction divisor (from H-J: Er + 3)  | High       |
| `0x00422c34` | `0.564`        | Hammerstad-Jensen Er coefficient           | Certain    |
| `0x00422c3c` | `-1.0`         | Negation                                   | Certain    |
| `0x00422c40` | `10.0`         | Constant 10                                | Certain    |
| `0x00422c44` | `30.666`       | ~`60 / sqrt(Er)` for some Er, or runtime intermediate | Medium |
| `0x00422c4c` | `0.28318`      | ~`0.9/pi` or `(pi-e)/pi` (uncertain)      | Low        |
| `0x00422c54` | `6.0`          | Constant 6                                 | Certain    |
| `0x00422c58` | `4.0`          | Constant 4                                 | Certain    |
| `0x00422c5c` | `376.9908`     | `120 * pi` (eta_0, free-space impedance)   | Certain    |
| `0x00422c64` | `6.28318`      | `2 * pi`                                   | Certain    |
| `0x00422c6c` | `0.0157`       | Skin effect scaling                        | Medium     |
| `0x00422c74` | `0.525`        | Polynomial coefficient (proprietary)       | Uncertain  |
| `0x00422c7c` | `0.6315`       | Polynomial coefficient (proprietary)       | Uncertain  |
| `0x00422c84` | `0.27488`      | Polynomial coefficient (proprietary)       | Uncertain  |
| `0x00422c8c` | `-8.7513`      | Exponential decay rate (proprietary)       | Uncertain  |
| `0x00422c94` | `0.065683`     | Scaling factor (proprietary)               | Uncertain  |
| `0x00422c9c` | `-0.03442`     | Exponential decay rate (proprietary)       | Uncertain  |
| `0x00422ca4` | `0.33622`      | Scaling factor (proprietary)               | Uncertain  |
| `0x00422cac` | `38.7`         | ~`120*pi / pi^2 = 12.16`? Or `120/pi = 38.20`? | Medium |
| `0x00422cb4` | `-4.6`         | Exponential decay rate (proprietary)       | Uncertain  |
| `0x00422cbc` | `0.0363`       | Scaling factor (proprietary)               | Uncertain  |
| `0x00422cc4` | `15.916`       | `50/pi = 15.9155` (very close match)       | High       |
| `0x00422ccc` | `2.751`        | Polynomial coefficient (proprietary)       | Uncertain  |
| `0x00422cd4` | `0.1844`       | Polynomial coefficient (proprietary)       | Uncertain  |
| `0x00422cdc` | `1.1`          | Thickness ratio correction                 | Medium     |
| `0x00422ce4` | `8.0`          | Constant 8                                 | Certain    |
| `0x00422ce8` | `14.0`         | Constant 14                                | Certain    |
| `0x00422cec` | `11.0`         | Constant 11                                | Certain    |
| `0x00422cf0` | `16.0`         | Constant 16                                | Certain    |
| `0x00422cf4` | `11.8`         | Speed of light (in/ns)                     | Certain    |
| `0x00422cfc` | `0.1`          | Convergence threshold                      | Certain    |
| `0x00422d04` | `15.0`         | Range check upper bound                    | Medium     |
| `0x00422d08` | `-1.55`        | Exp decay in thickness correction          | Medium     |
| `0x00422d10` | `60.0`         | `eta_0 / (2*pi)` = impedance factor       | Certain    |
| `0x00422d14` | `0.8`          | Effective width factor (IPC: 0.8*W)        | High       |
| `0x00422d1c` | `5.98`         | Thickness correction (~6)                  | Medium     |
| `0x00422d24` | `0.08457`      | Skin depth coefficient                     | Medium     |
| `0x00422d2c` | `5.0`          | Range check lower bound                    | Medium     |
| `0x00422d30` | `40.0`         | Range check for height                     | Medium     |
| `0x00422d34` | `90.0`         | Range check max for height                 | Medium     |
| `0x00422d38` | `1.9`          | IPC-2141A: `1.9*(2h+t)` in diff formula   | High       |
| `0x00422d40` | `1.017`        | Near-unity correction (proprietary)        | Uncertain  |
| `0x00422d48` | `12.0`         | Constant 12                                | Certain    |
| `0x00422d4c` | `0.35`         | w/b threshold (Cohn w_eff condition)       | Certain    |
| `0x00422d54` | `1.023`        | Coupled polynomial coefficient             | Uncertain  |
| `0x00422d5c` | `1.0235`       | Coupled polynomial coefficient             | Uncertain  |
| `0x00422d64` | `0.5008`       | Coupled polynomial offset                  | Uncertain  |
| `0x00422d6c` | `1.1564`       | Coupled polynomial coefficient             | Uncertain  |
| `0x00422d74` | `0.4749`       | Coupled polynomial coefficient             | Uncertain  |
| `0x00422d7c` | `2.71828`      | `e` (Euler's number)                       | Certain    |
| `0x00422d84` | `376.991`      | `120 * pi` (eta_0, duplicate)              | Certain    |
| `0x00422d8c` | `6.27`         | Wadell formula constant (cf. `2*pi`)       | High       |
| `0x00422d94` | `0.5`          | Constant 0.5                               | Certain    |
| `0x00422d98` | `0.102101675`  | Unknown, possibly `1/(2*pi*sqrt(Er))`      | Low        |
| `0x00422da0` | `0.079577538`  | `1/(4*pi)` (exact: 0.07957747...)          | Certain    |
| `0x00422da8` | `0.31831015`   | `1/pi` (exact: 0.31830989...)              | Certain    |
| `0x00422db0` | `80.0`         | Unknown (display/range related?)           | Low        |
| `0x00422dbc` | `188.4954`     | `60 * pi` (= eta_0/2)                     | Certain    |
| `0x00422dc4` | `2.54`         | mm per 0.1 inch (0.1 * 25.4)              | Certain    |
| `0x00422dcc` | `0.1269`       | Unknown correction                         | Low        |
| `0x00422dd4` | `0.3811`       | Unknown correction                         | Low        |

---

## 9. Recommended Rust Implementation Strategy

### 9A. For Single-Ended Z0 (Match Saturn)

Saturn's formula is a hybrid that does NOT cleanly match any single published
reference. To match Saturn's output exactly, you would need to reproduce all the
proprietary coefficients from Section 3. However, for initial implementation:

**Option A: Wadell formula (recommended for accuracy)**
```rust
pub fn stripline_z0_wadell(w: f64, t: f64, b: f64, er: f64) -> f64 {
    // Wadell finite-thickness formula
    let t_norm = t / b;
    let w_norm = w / b;
    let m = 6.0 * (1.0 - t_norm) / (3.0 - t_norm);
    let delta_w_norm = (t_norm / (PI * (1.0 - t_norm))) * (
        1.0 - 0.5 * ((t_norm / (2.0 - t_norm)).powi(2)
            + (0.0796 * t_norm / (w_norm + 1.1 * t_norm)).powf(m)).ln()
    );
    let w_eff = w_norm + delta_w_norm;
    let a = 4.0 * (1.0 - t_norm) / (PI * w_eff);
    (30.0 / er.sqrt()) * (1.0 + a * (2.0 * a + (4.0 * a * a + 6.27).sqrt())).ln()
}
```

**Option B: Cohn/Steer formula with fringing Cf**
```rust
pub fn stripline_z0_cohn(w: f64, t: f64, b: f64, er: f64) -> f64 {
    let t_b = t / b;
    let w_b = w / b;
    let w_eff_b = if w_b < 0.35 {
        w_b - (0.35 - w_b).powi(2) / (1.0 + 12.0 * t_b)
    } else {
        w_b
    };
    let cf = (2.0 / PI) * (1.0 / (1.0 - t_b) + 1.0).ln()
        - (t_b / PI) * (1.0 / (1.0 - t_b).powi(2) - 1.0).ln();
    (30.0 * PI / er.sqrt()) * (1.0 - t_b) / (w_eff_b + cf)
}
```

**Option C: IPC-2141A simplified (quick reference)**
```rust
pub fn stripline_z0_ipc2141a(w: f64, t: f64, b: f64, er: f64) -> f64 {
    (60.0 / er.sqrt()) * (4.0 * b / (0.67 * PI * (0.8 * w + t))).ln()
}
```

### 9B. For Coupled Stripline (Cohn Conformal Mapping)

```rust
pub fn coupled_stripline(w: f64, s: f64, t: f64, b: f64, er: f64)
    -> (f64, f64, f64, f64)  // (Z0e, Z0o, Zdiff, Zcomm)
{
    const ETA_0: f64 = 120.0 * PI;  // 376.991

    // Zero-thickness coupling parameters
    let ke = (PI * w / (2.0 * b)).tanh() * (PI * (w + s) / (2.0 * b)).tanh();
    let ko = (PI * w / (2.0 * b)).tanh() / (PI * (w + s) / (2.0 * b)).tanh();

    // Compute K(k')/K(k) via AGM
    let z0e_0t = (ETA_0 / (4.0 * er.sqrt())) * kp_over_k(ke);
    let z0o_0t = (ETA_0 / (4.0 * er.sqrt())) * kp_over_k(ko);

    // Apply thickness correction if t > 0
    let (z0e, z0o) = if t > 0.0 {
        let z0s = stripline_z0_cohn(w, t, b, er);      // single strip, finite t
        let z0s_0t = stripline_z0_cohn(w, 0.0, b, er);  // single strip, zero t
        let cf_ratio = cf_with_thickness(t, b, er) / cf_zero_thickness(er);

        let z0e = 1.0 / (1.0/z0s - cf_ratio * (1.0/z0s_0t - 1.0/z0e_0t));
        let z0o = if s >= 5.0 * t {
            // Cohn eq. 20
            1.0 / (1.0/z0s + cf_ratio * (1.0/z0o_0t - 1.0/z0s_0t))
        } else {
            // Cohn eq. 22 (close spacing)
            let delta_cf = cf_with_thickness(t, b, er) - cf_zero_thickness(er);
            1.0 / (1.0/z0o_0t + (1.0/z0s - 1.0/z0s_0t)
                - 2.0 * delta_cf / ETA_0 + 2.0*t / (ETA_0 * s))
        };
        (z0e, z0o)
    } else {
        (z0e_0t, z0o_0t)
    };

    let zdiff = 2.0 * z0o;
    let zcomm = z0e / 2.0;
    (z0e, z0o, zdiff, zcomm)
}
```

### 9C. Elliptic Integral Helper

```rust
/// Compute K(k') / K(k) using AGM iteration.
/// k is the elliptic modulus (0 < k < 1).
fn kp_over_k(k: f64) -> f64 {
    let mut k = k;
    let mut kp = (1.0 - k * k).sqrt();
    let mut r = 1.0;
    for _ in 0..50 {
        let k_next = 2.0 * k.sqrt() / (1.0 + k);
        let kp_next = 2.0 * kp.sqrt() / (1.0 + kp);
        r *= (1.0 + kp) / (1.0 + k);
        k = k_next;
        kp = kp_next;
        if (1.0 - k).abs() < 1e-15 && (1.0 - kp).abs() < 1e-15 {
            break;
        }
    }
    r
}
```

---

## 10. Discrepancy Summary

| Item                       | Published Value       | Saturn Value  | Delta  | Notes                        |
|----------------------------|-----------------------|---------------|--------|------------------------------|
| Fringe factor              | `2*ln(2)/pi = 0.4413`| `0.432`       | -2.1%  | Deliberate empirical adjust? |
| Wadell constant            | `6.27`                | `6.28318`     | +0.2%  | Saturn uses exact `2*pi`     |
| Er correction formula      | `0.564*((Er-0.9)/(Er+3))^0.053` | `ln((Er-0.9)/(Er+3))*0.564` | N/A | Different functional form |
| Empirical poly coefficients| Not published         | 0.525, 0.6315, etc. | N/A | Saturn-proprietary           |
| Coupled poly coefficients  | Not published         | 1.023, 1.0235, etc. | N/A | Saturn-proprietary           |
| `15.916`                   | `50/pi = 15.9155`     | `15.916`      | ~0%    | Match confirmed              |
| `38.7`                     | `120/pi = 38.197`     | `38.7`        | +1.3%  | Possible `120/pi + corr`     |
| `10.8731272`               | `4*e = 10.87313`      | `10.8731272`  | ~0%    | Match confirmed              |

---

## 11. Testing Strategy

To validate the Rust implementation:

1. **Use IPC-2141A as baseline** -- compute Z0 for standard geometries and compare
   against the simple formula. This gives ~1-2% accuracy.

2. **Use Wadell formula for primary implementation** -- more accurate than IPC-2141A,
   well-documented, used by wcalc and other open-source tools.

3. **Match Saturn output** -- once the Wadell formula is working, add Saturn's
   proprietary correction factors (Section 3, Steps 5 and 9) to match its output
   exactly. This may require further decompilation to pin down the exact
   FPU instruction sequence.

4. **Test vectors from Saturn help PDF** -- the help file has example calculations
   that can serve as test cases.

5. **Cross-validate with wcalc** -- the open-source wcalc tool implements the same
   Cohn/Wadell formulas and can provide independent reference values.
