# Saturn PCB Toolkit - Impedance Calculator Reverse Engineering Notes

## Overview

The impedance calculator in Saturn PCB Toolkit (`toolkit.exe`) is a Delphi/C++ Builder
application. The impedance module uses a global mode selector (`DAT_008d5f88`) to dispatch
to different solver functions based on the selected impedance topology. The three
originally-identified functions (FUN_004a8bb8, FUN_0049021c, FUN_004be8fc) are **UI
setup/layout functions** that configure form visibility; the actual math lives in
separate solver functions dispatched through FUN_00403398.

---

## Architecture

### Mode Selector: `DAT_008d5f88`

A global integer at address `0x008d5f88` selects which impedance topology is active.
It is written by UI click handlers and read by the main dispatcher `FUN_00403398`.

### UI Setup Functions (Originally Identified)

| Function       | Mode Value | Purpose                              |
|----------------|------------|--------------------------------------|
| FUN_0049021c   | 0          | UI setup for Microstrip mode         |
| FUN_004a8bb8   | 1          | UI setup for RF Impedances / Conductor Impedance mode |
| FUN_004be8fc   | 0x11 (17)  | UI setup for Wavelength Calculator mode |

These functions:
1. Call `FUN_0086b424()` with a string table address
2. Set `DAT_008d5f88` to the mode value
3. Configure visibility/state of dozens of form controls via `FUN_0053957c()` (SetVisible)
   and vtable calls at offset `+0xA0` (likely `SetEnabled`)
4. Populate column headers in list/grid controls
5. Read/write registry via `FUN_00805c4c` / `FUN_00805f00` / `FUN_00806860`
6. Call `FUN_00403398(param_1, param_2)` at the end to trigger computation

### Main Dispatcher: `FUN_00403398` (address `0x00403398`)

This function reads `DAT_008d5f88` and dispatches to the appropriate solver:

```
Mode  0 (0x00): FUN_00440e34 -> Microstrip
Mode  1 (0x01): FUN_0040bc00 -> Stripline (decompilation failed - function too large)
Mode  2 (0x02): FUN_004343e4 -> Differential Layer (decompilation failed)
Mode  3 (0x03): FUN_00422ddc -> Edge Coupled External
Mode  4 (0x04): FUN_004435f8 -> Edge Coupled Internal Symmetric
Mode  5 (0x05): FUN_004066b0 -> Edge Coupled Internal Asymmetric
Mode  6 (0x06): FUN_00403f40 -> Edge Coupled Embedded
Mode  7 (0x07): FUN_004b8104 -> Broadside Coupled (Shielded)
Mode  8 (0x08): FUN_00482648 -> Broadside Coupled (Non-Shielded)
Mode  9 (0x09): FUN_00498e84 -> (Unknown - possibly Microstrip Embed)
Mode 10 (0x0A): FUN_004c41d8 -> (Unknown - possibly Stripline Asymmetric)
Mode 11 (0x0B): FUN_0045fc54 -> (Unknown - possibly Dual Stripline)
Mode 12 (0x0C): FUN_00408b68 -> Coplanar Waveguide
Mode 13 (0x0D): FUN_0040a0f4 -> (Unknown)
Mode 14 (0x0E): FUN_004bca88 -> (Unknown)
Mode 15 (0x0F): FUN_004081b0 -> (Unknown)
Mode 16 (0x10): FUN_00427090 -> Er Effective display (decompilation failed)
Mode 17 (0x11): FUN_004bf410 -> Wavelength / Frequency Calculator
Mode 18 (0x12): FUN_004d662c -> (Unknown)
```

Additionally, `FUN_00403398` always calls these common functions before dispatch:
- `FUN_00471678` - common pre-computation
- `FUN_00471e7c` - common pre-computation
- `FUN_00471a08` - common pre-computation
- `FUN_004e0928` - common pre-computation

### Impedance Topology List (from binary strings)

Primary topologies (radio button group):
1. Microstrip
2. Stripline
3. Differential Layer
4. Edge Cpld Ext (Edge Coupled External)
5. Edge Cpld Int Sym (Edge Coupled Internal Symmetric)
6. Edge Cpld Int Asym (Edge Coupled Internal Asymmetric)
7. Edge Cpld Embed (Edge Coupled Embedded)
8. Broad Cpld Shld (Broadside Coupled Shielded)
9. Broad Cpld NShld (Broadside Coupled Non-Shielded)

Extended topologies:
- Microstrip Embed (Embedded Microstrip)
- Stripline Asym (Asymmetric Stripline)
- Dual Stripline
- Coplanar Wave (Coplanar Waveguide)

### Material Correction: `FUN_00494dd4` (address `0x00494dd4`)

This is a large lookup function that sets a material correction factor at
`DAT_008d6478/008d647c` (stored as a double). It assigns either:
- **0.98** for certain substrate materials (likely FR-4 variants with surface roughness)
- **1.0** for ideal/other materials

This factor is applied as a multiplier to the final impedance value and likely accounts
for copper surface roughness effects (Hammerstad correction).

---

## Decompiled Solver: FUN_00440e34 (Mode 0 -- Microstrip)

### Overview

This is the primary Microstrip impedance solver. It handles both "solve for impedance"
(mode 0 in an inner selector at `param_1 + 0xd20 + 0x2f0`) and "solve for width" (mode 1).

### Physical Constants (embedded in .text section)

| Address      | Value         | Type   | Meaning                                        |
|--------------|---------------|--------|------------------------------------------------|
| `0x0044359C` | 1.0           | float  | Minimum impedance threshold (ohms)             |
| `0x004435A0` | 0.35          | double | Copper thickness ratio constant (t/W default)  |
| `0x004435A8` | 1000.0        | float  | Unit conversion (mm to um or similar)          |
| `0x004435AC` | 299,792,458   | double | **Speed of light (m/s)**                       |
| `0x004435B4` | 0.0001        | double | Unit conversion (m to 0.1 mm)                  |
| `0x004435BC` | 2.54          | double | **Inches to centimeters**                      |
| `0x004435C4` | 0.457         | double | **Hammerstad-Jensen constant a**               |
| `0x004435CC` | 0.67          | double | **Hammerstad-Jensen constant b**               |
| `0x004435D4` | 1.0e-9        | double | GHz to seconds (nano)                          |
| `0x004435DC` | 2.54e-5       | double | **Mils to meters** (1 mil = 25.4 um)           |
| `0x004435E4` | 0.001         | double | mm to meters                                   |
| `0x004435EC` | 1.0e-6        | double | um to meters                                   |
| `0x004435F4` | 100.0         | float  | Unit conversion                                |

### Key Formulas Identified

From the decompiled code of FUN_00440e34:

#### Wavelength Calculation

```
w/h = (0.35 / H) * 1000          // trace width to height ratio (unit adjusted)
wavelength = (c * 0.0001) / (w/h * 2.54)
```

Where `c = 299,792,458 m/s` (speed of light).

#### Er Effective (Effective Dielectric Constant)

The code references the Hammerstad-Jensen constants 0.457 and 0.67. The standard
Hammerstad-Jensen formula for effective dielectric constant is:

```
Er_eff = (Er + 1)/2 + (Er - 1)/2 * F(W/H)
```

where:
```
F(W/H) = (1 + 12*H/W)^(-0.5) + 0.04*(1 - W/H)^2    for W/H <= 1
F(W/H) = (1 + 12*H/W)^(-0.5)                          for W/H > 1
```

The constants 0.457 and 0.67 appear in the thickness correction formulas in the
Hammerstad-Jensen model. Specifically, in the original paper by E. Hammerstad and
O. Jensen, "Accurate Models for Microstrip Computer-Aided Design" (IEEE MTT-S, 1980),
these constants appear in the effective width correction for finite conductor thickness:

```
dW = (t/pi) * ln(4*e / sqrt((t/W)^2 + (t/(W + 1.1*t))^2))
```

And the characteristic impedance formula:
```
Z0 = (eta_0 / (2*pi*sqrt(Er_eff))) * ln(F/u + sqrt(1 + (2/u)^2))
```

where `eta_0 = 120*pi` ohms (free-space impedance) and `u = W_eff / H`.

The 0.457 and 0.67 values specifically relate to the frequency-dependent corrections:
- 0.457 appears in Kirschning and Jansen's dispersion model extension
- 0.67 relates to the normalized frequency parameter

#### Impedance Computation Pattern

The computation follows this pattern (reconstructed from x87 FPU operations):

```
input_H = read_from_UI(param_1 + 0xC2C)     // dielectric height
input_Er = read_from_UI(param_1 + 0x67C)     // relative permittivity

// Validate Er
if (Er < 1.0) {
    show_error();
    set_default_Er();
    recompute();
}

// Convert units
H_converted = (0.35 / H) * 1000

// Compute wavelength
wavelength = (c / H_converted) * 0.0001 / 2.54

// Compute frequency-related parameter
freq_param = (c * 1e-9 * c_light) / sqrt_er_eff

// Apply copper thickness correction
impedance_corrected = impedance * copper_thickness_factor

// Apply frequency scaling
scaled = impedance * thickness_selector * (100 or other factor)

// Compute propagation delay
prop_delay = c_light / wavelength

// Apply surface roughness factor (from DAT_008d6478: 0.98 or 1.0)
result = prop_delay * roughness_factor
```

#### Copper Thickness Selector Values

The first inner dropdown (`param_1 + 0xd38 + 0x2f8`) selects these double values:

| Index | Value | Likely Meaning          |
|-------|-------|-------------------------|
| 0     | 0.20  | 0.20 oz copper          |
| 1     | 0.25  | 0.25 oz (1/4 oz)       |
| 2     | 0.30  | 0.30 oz copper          |
| 3     | 0.35  | 0.35 oz copper          |
| 4     | 0.40  | 0.40 oz copper          |
| 5     | 0.45  | 0.45 oz copper          |
| 6     | 0.50  | 0.50 oz (1/2 oz)       |

The second dropdown (`param_1 + 0xd58 + 0x2f8`) selects:

| Index | Value    | Likely Meaning                     |
|-------|----------|------------------------------------|
| 0     | 0.25     | Tolerance or precision factor      |
| 1     | 0.142857 | 1/7 (septimal rounding)           |
| 2     | 0.10     | 10% tolerance                      |
| 3     | 0.05     | 5% tolerance                       |

---

## Decompiled Solver: FUN_004bf410 (Mode 0x11 -- Wavelength Calculator)

### Overview

This function computes wavelength based on frequency input. It supports both
"solve for wavelength" (outer `param_1 + 0x684 + 0x2f0 == 0`) and
"solve for impedance" (`== 1`) modes.

### Constants

| Address      | Value         | Type   | Meaning                                |
|--------------|---------------|--------|----------------------------------------|
| `0x004BFF3C` | 1.0e9         | float  | **GHz to Hz conversion**               |
| `0x004BFF40` | 1.0           | float  | Identity                               |
| `0x004BFF44` | 1.0e6         | float  | **MHz to Hz conversion**               |
| `0x004BFF48` | 1.0e12        | double | **THz to Hz conversion**               |
| `0x004BFF50` | 1000.0        | float  | **kHz or mm conversion**               |
| `0x004BFF54` | 0.001         | double | mm to meters                           |
| `0x004BFF5C` | 1.0e-6        | double | um to meters                           |
| `0x004BFF64` | 299,792,458   | double | **Speed of light (m/s)**               |
| `0x004BFF6C` | 0.0001        | double | m to 0.1 mm conversion                |
| `0x004BFF74` | 2.54          | double | **Inches to cm**                       |

### Frequency Unit Selector

The dropdown at `param_1 + 0xC08 + 0x2f8` selects a frequency scaling factor
stored at `DAT_008d6398/008d639c`:

| Index | Value   | Meaning                     |
|-------|---------|-----------------------------|
| 0     | 1.0     | Base unit                   |
| 1     | 0.75    | 3/4 scaling                 |
| 2     | 0.625   | 5/8 scaling                 |
| 3     | 0.5     | Half                        |
| 4     | 0.25    | Quarter                     |
| 5     | 0.1     | Tenth                       |
| 6     | 0.0667  | ~1/15                       |
| 7     | 0.0625  | 1/16                        |
| 8     | 0.05    | 1/20                        |

### Key Formula

The central wavelength/impedance formula:

```c
// Unit conversion for frequency input
if (unit_selector == 0) {  // GHz input
    freq_hz = (1.0 / (input / 1e9)) / 1e6;
    // Simplifies to: freq_hz = 1e15 / input (in GHz)
}
if (unit_selector == 1) {  // THz input
    freq_hz = (1.0 / (input / 1e12)) / 1e6;
}

// If input is in mils/mm directly:
if (length_unit == 0) {
    dimension = input * 1000.0;    // mm to um
}
if (length_unit == 1) {
    dimension = input * 1.0;       // already in um
}
if (length_unit == 2) {
    dimension = input * 0.001;     // um to mm
}
if (length_unit == 3) {
    dimension = input * 1e-6;      // nm to mm
}

// Read Er and other inputs
Er = read_from_UI(param_1 + 0x67C);
Dk = read_from_UI(param_1 + 0xC00);

// Main impedance calculation
sqrt_term = FUN_00868834();  // likely sqrt(Er_eff)
impedance = (c / (freq_hz * dimension)) * 0.0001 / 2.54 * scale_factor;
// where c = 299,792,458 m/s
```

The formula `(299792458 / (sqrt_Er * frequency)) * 0.0001 / 2.54` computes wavelength
in mils from the speed of light, effective permittivity, and frequency.

---

## Decompiled Solver: FUN_004b8104 (Mode 7 -- Via Properties / Broadside Coupled)

### Constants

| Address      | Value    | Type   | Meaning                                   |
|--------------|----------|--------|-------------------------------------------|
| `0x004BA920` | 2.0      | float  | Divisor (half-distance or symmetric)      |
| `0x004BA928` | 0.0078125| double | 1/128, minimum dimension threshold        |
| `0x004BA92C` | 1.0      | float  | Minimum Er threshold                      |
| `0x004BA940` | 1.2732   | double | **4/pi** (geometric constant)             |
| `0x004BA948` | 1000.0   | float  | mm to um conversion                       |
| `0x004BA950` | 1550.003 | double | Conversion or material constant           |

### Copper Thickness Values (Mils Mode)

| Index | Value (mils) | Standard Weight |
|-------|-------------|-----------------|
| 0     | 0.35        | ~0.5 oz         |
| 1     | 0.70        | ~1.0 oz         |
| 2     | 1.40        | ~2.0 oz         |
| 3     | 2.10        | ~3.0 oz         |
| 4     | 2.80        | ~4.0 oz         |
| 5     | 3.50        | ~5.0 oz         |
| 6     | 4.20        | ~6.0 oz         |
| 7     | 5.60        | ~8.0 oz         |
| 8     | 7.00        | ~10.0 oz        |

### Copper Thickness Values (mm Mode)

| Index | Value (mm) | Standard Weight |
|-------|-----------|-----------------|
| 0     | 0.009     | ~0.25 oz        |
| 1     | 0.018     | ~0.5 oz         |
| 2     | 0.035     | ~1.0 oz         |
| 3     | 0.053     | ~1.5 oz         |
| 4     | 0.070     | ~2.0 oz         |
| 5     | 0.088     | ~2.5 oz         |
| 6     | 0.106     | ~3.0 oz         |
| 7     | 0.142     | ~4.0 oz         |
| 8     | 0.178     | ~5.0 oz         |

### Key Formulas

```c
// Total height = copper_plating + copper_thickness
total_H = plating_thickness + copper_thickness;

// H in mils (divided by 2 for half)
H_mils = total_H / 2.0;

// Cross-sectional area
area = (pad_diameter - total_H) * total_H;

// Capacitance uses 4/pi constant
cap_factor = area * 1.2732;  // area * (4/pi)

// Impedance result
impedance = cap_factor * (4/pi) * sqrt_Er * frequency;

// Inductance
inductance = area * 1.2732;
capacitance = inductance * (4/pi);
```

The `4/pi = 1.2732` factor appears in the via barrel capacitance formula, consistent
with the parallel-plate capacitor approximation for cylindrical vias:

```
C_via = epsilon_0 * epsilon_r * (pi * D * L) / (4 * d)
```

where D is pad diameter, L is via length, and d is clearance.

---

## Hammerstad-Jensen Model Identification

### Evidence

1. **String reference**: The binary contains the full citation:
   > "Hammerstad and O. Jensen, 'Accurate Models for Microstrip Computer-Aided Design'
   > IEEE MTT-S, International Symposium Digest, 1980"
   (found at addresses `0x00B29D3B` and `0x00E23856` in the resource section)

2. **Constants 0.457 and 0.67**: These are Hammerstad-Jensen specific constants
   used in the effective width and dispersion correction formulas.

3. **Additional reference**: "Embedded Microstrip, Impedance Formula" by Douglas Brooks
   is cited in the resource section.

4. **Wadell reference**: "Wadell, Brian C., Transmission Line Design Handbook"
   is also cited.

### Standard Hammerstad-Jensen Microstrip Formulas

The Hammerstad-Jensen model computes microstrip characteristic impedance as:

**For W/H <= 1:**
```
Z0 = (60 / sqrt(Er_eff)) * ln(8*H/W_eff + W_eff/(4*H))
```

**For W/H > 1:**
```
Z0 = (120*pi / sqrt(Er_eff)) / (W_eff/H + 1.393 + 0.667*ln(W_eff/H + 1.444))
```

**Effective dielectric constant:**
```
Er_eff = (Er+1)/2 + (Er-1)/2 * (1 + 12*H/W)^(-0.5)   [W/H > 1]
Er_eff = (Er+1)/2 + (Er-1)/2 * ((1 + 12*H/W)^(-0.5) + 0.04*(1-W/H)^2)   [W/H <= 1]
```

**Effective width (thickness correction):**
```
W_eff = W + dW
dW = (t/pi) * (1 + ln(2*H/t))                    [W/H >= 0.5*pi]
dW = (t/pi) * (1 + ln(4*pi*W/t))                  [W/H < 0.5*pi]
```

The constants 0.457 and 0.67 specifically appear in the Kirschning-Jansen dispersion
extension to the Hammerstad model for frequency-dependent Er_eff:

```
Er_eff(f) = Er - (Er - Er_eff(0)) / (1 + G*(f/fp)^2)
```

where G involves 0.457 and 0.67 as fitting parameters.

---

## Helper Functions

| Address      | Purpose                                                |
|--------------|--------------------------------------------------------|
| FUN_0086b424 | String resource loader (Delphi ResourceString)         |
| FUN_0086ecf0 | Stack string builder (returns pointer)                 |
| FUN_0086ee90 | String cleanup / reference count decrement             |
| FUN_0086ecc0 | String allocation                                      |
| FUN_0086eeb4 | String conversion                                      |
| FUN_0086ef50 | Boolean check (returns char, '\\0' = false)            |
| FUN_0086f0f4 | String-to-float conversion (StrToFloat)                |
| FUN_005396c8 | Set TEdit.Text (writes string to edit control)         |
| FUN_00539678 | Get TEdit.Text (reads string from edit control)        |
| FUN_0053957c | SetVisible on a control (0=hide, 1=show)               |
| FUN_00538980 | Set control Top position                               |
| FUN_00538914 | Set control Width                                      |
| FUN_005ec60c | Set panel/control enable state                         |
| FUN_005dff68 | Show error message dialog                              |
| FUN_0059f924 | Get TabControl index                                   |
| FUN_00868834 | FP validation (checks for NaN/Inf)                     |
| FUN_00861e48 | Float-to-string conversion (FloatToStr)                |
| FUN_0085f89f | Format string preparation                              |
| FUN_0071d964 | String formatting (Format / FormatFloat)               |
| FUN_008673c0 | Math function (likely sqrt)                            |
| FUN_008675ac | Math function (likely ln/log)                          |
| FUN_00805c4c | TRegistry.Create                                       |
| FUN_00805da4 | TRegistry.RootKey := HKEY_CURRENT_USER (0x80000001)   |
| FUN_00805f00 | TRegistry.OpenKey                                      |
| FUN_00806860 | TRegistry.WriteString                                  |
| FUN_00403638 | Common format/display helper                           |

---

## Global Variables (BSS/Data Section)

| Address        | Type    | Purpose                                    |
|----------------|---------|---------------------------------------------|
| `DAT_008d5f88` | int     | Mode selector (0..18)                       |
| `DAT_008d6054` | double  | Er (relative permittivity) input            |
| `DAT_008d6034` | double  | Trace width / pad diameter input            |
| `DAT_008d603c` | double  | Total height (plating + copper)             |
| `DAT_008d607c` | double  | Computed impedance result                   |
| `DAT_008d60f4` | double  | Cross-sectional area                        |
| `DAT_008d60fc` | double  | Capacitance/impedance result                |
| `DAT_008d609c` | uint32  | Copper thickness factor (low dword)         |
| `DAT_008d60a0` | uint32  | Copper thickness factor (high dword)        |
| `DAT_008d61b8` | double  | Copper thickness (mils or mm)               |
| `DAT_008d6240` | double  | Propagation speed result                    |
| `DAT_008d6248` | double  | Propagation delay result                    |
| `DAT_008d6258` | double  | Wavelength (converted)                      |
| `DAT_008d6280` | uint32  | Second selector factor (low dword)          |
| `DAT_008d6284` | uint32  | Second selector factor (high dword)         |
| `DAT_008d6298` | double  | Adjusted propagation delay                  |
| `DAT_008d62a0` | double  | Er input (second reading)                   |
| `DAT_008d6390` | double  | Impedance calculation output                |
| `DAT_008d6398` | uint32  | Frequency unit factor (low dword)           |
| `DAT_008d639c` | uint32  | Frequency unit factor (high dword)          |
| `DAT_008d6448` | double  | H in mils (half-height)                     |
| `DAT_008d6478` | uint32  | Material correction factor (low dword)      |
| `DAT_008d647c` | uint32  | Material correction factor (high dword)     |
| `DAT_008d64e8` | double  | Dk (dissipation factor) input               |
| `DAT_008d6668` | double  | Plating thickness                           |
| `DAT_008d6710` | double  | Converted dimension (for unit handling)     |
| `DAT_008d6a20` | double  | Er input (via calculator)                   |
| `DAT_008d6a28` | double  | W/H ratio (converted)                       |
| `DAT_008d6a30` | double  | Height input (converted)                    |
| `DAT_008d6a38` | double  | Frequency parameter                         |
| `DAT_008d6a48` | double  | Scaled impedance with copper correction     |
| `DAT_008d6c04` | double  | Inductance per length                       |

---

## Form Object References

The `param_1` pointer is the form (TForm) self reference. Key field offsets:

| Offset   | Control Purpose                              |
|----------|----------------------------------------------|
| +0x490   | TabControl (topology selector, index 0x10)   |
| +0x620   | Plating selector RadioGroup                  |
| +0x67C   | Er (dielectric constant) TEdit               |
| +0x684   | Units mode selector (0=mils, 1=mm)           |
| +0x800   | Frequency input TEdit                        |
| +0x7A8   | Secondary input TEdit                        |
| +0x990   | Copper weight selector                       |
| +0x998   | Label/panel for parameter display             |
| +0x99C   | Label/panel for parameter display             |
| +0x9A8   | Height display TEdit                         |
| +0x9AC   | Label/panel                                  |
| +0x9B0   | Grid/ListView for results                    |
| +0x9BC   | Grid/ListView for secondary results          |
| +0xA0C   | Custom copper thickness input TEdit          |
| +0xA14   | Custom plating thickness input TEdit         |
| +0xA6C   | Trace width TEdit                            |
| +0xA70   | Er input TEdit (secondary)                   |
| +0xA88   | Cross-section area result TEdit              |
| +0xA8C   | Capacitance result TEdit                     |
| +0xC00   | Dk (dissipation factor) TEdit                |
| +0xC08   | Frequency unit selector                      |
| +0xC10   | Impedance result display TEdit               |
| +0xC2C   | Dielectric height TEdit                      |
| +0xC44   | Format string TEdit (number formatting)      |
| +0xD20   | Solve-for selector (0=impedance, 1=width)    |
| +0xD2C   | W/H ratio display TEdit                      |
| +0xD30   | Propagation speed display TEdit              |
| +0xD38   | Copper thickness dropdown                    |
| +0xD44   | Impedance result TEdit (secondary display)   |
| +0xD54   | Propagation delay display TEdit              |
| +0xD58   | Tolerance/precision dropdown                 |
| +0xD64   | Adjusted delay display TEdit                 |
| +0x1040  | Input unit selector (mils/mm/um/nm)          |
| +0x1104  | Length unit mode (0=freq, 1=dimension)        |
| +0x1108  | Dimension input TEdit                        |
| +0x1114  | Dimension unit selector                      |
| +0x1200  | Er input TEdit (wavelength mode)             |
| +0x120C  | Status label                                 |

---

## References Found in Binary Resources

The binary resource section (`.rsrc`) contains these academic references:

1. **E. Hammerstad and O. Jensen**, "Accurate Models for Microstrip Computer-Aided Design",
   IEEE MTT-S, International Symposium Digest, 1980.

2. **Douglas Brooks**, "Embedded Microstrip, Impedance Formula"

3. **Brian C. Wadell**, *Transmission Line Design Handbook*

4. **Eric Bogatin**, *Bogatin's Practical Guide to Prototype Breadboard and PCB Design*
   (Amazon link found: https://www.amazon.com/Bogatins-Practical-Prototype-Breadboard-Design/dp/163081962X)

5. **Howard Johnson & Martin Graham**, *High-Speed Digital Design Handbook*
   (Amazon link found: https://www.amazon.com/High-Speed-Digital-Design-Handbook/dp/0133957241)

6. Web reference for Keffective: https://www.microwaves101.com/encyclopedias/keffective

7. Wikipedia wavelength reference: http://en.wikipedia.org/wiki/Wavelength

---

## Summary of Impedance Model

Saturn PCB Toolkit implements the **Hammerstad-Jensen microstrip impedance model** (1980)
with the following characteristics:

1. **Core model**: Hammerstad-Jensen closed-form equations for Z0 and Er_eff
2. **Thickness correction**: Conductor thickness is accounted for via effective width
   adjustment using the standard H-J thickness correction formulas
3. **Surface roughness**: A material-dependent correction factor (0.98 for standard FR-4,
   1.0 for ideal) is applied
4. **Dispersion**: The Kirschning-Jansen frequency-dependent extension is likely used
   (evidenced by constants 0.457 and 0.67)
5. **Unit system**: Supports both mils and mm, with internal computation in SI units
   using the speed of light c = 299,792,458 m/s
6. **Via model**: Uses parallel-plate capacitor approximation with 4/pi geometric
   correction for cylindrical geometry
7. **Multiple topologies**: 19 different impedance modes covering microstrip, stripline,
   differential, edge-coupled, broadside-coupled, embedded, and coplanar waveguide
   configurations
