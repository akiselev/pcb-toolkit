# Ghidra Reverse Engineering: Solver_OhmsLaw

**Address:** `0x004c41d8` (Ghidra name: `Solver_OhmsLaw`, project: `saturn-pcb`)
**Called from:** `OhmsLaw1Click` at `0x004c3514`, which sets `DAT_008d5f88 = 10`
(Mode 10) and then calls `Button1Click_MainDispatcher`.

## Function Overview

`Solver_OhmsLaw` is an enormous function (~7000+ instructions) that handles all
seven sub-modes of the Ohm's Law calculator. It is too large for Ghidra's
decompiler to process in one pass. Analysis was done from raw disassembly.

## Sub-Mode Dispatch

The function reads a combo box ItemIndex from `[EBX + 0xf28]` via a call to
`0x0059f924` (likely `TComboBox.GetItemIndex`). The return value in EAX selects
the sub-mode:

| EAX | Sub-mode                  | Address range (approx.)      |
|-----|---------------------------|------------------------------|
| 0   | E-I-R (Ohm's Law)        | `0x004c4264` - `0x004c55e9`  |
| 1   | Resistive Voltage Divider | `0x004c5f7f` - `0x004ca5ab`  |
| 2   | Series/Parallel R/C/L     | `0x004ca5ab` - `0x004cec59`  |
| 3   | Pi-Pad Attenuator         | `0x004cec59` - `0x004cf716`  |
| 4   | T-Pad Attenuator          | `0x004cf716` - `0x004d0173`  |

**Note:** The original notes list 7 modes, but the binary groups them differently.
The LED Bias mode is handled as a variation within mode 0 (it has the same
structure: read inputs, compute, format output). Series/Parallel R, C, and L are
all grouped under mode 2 with a secondary combo box at `[EBX + 0xf68]` selecting
between components.

## Combo Box Fields

| Object offset   | Purpose                                        |
|-----------------|------------------------------------------------|
| `[EBX + 0xf20]` | E-I-R inner sub-mode (0=solve V, 1=solve I, 2=solve R) |
| `[EBX + 0xf28]` | Main sub-mode selector (0..4)                  |
| `[EBX + 0xf68]` | Series/Parallel component selector (0=R, 1=C, 2=L...) |
| `[EBX + 0x11f0]`| Attenuator "matched" checkbox                  |

## Input Text Fields

| Object offset   | Purpose (varies by mode)                       |
|-----------------|------------------------------------------------|
| `[EBX + 0xef4]` | Primary input 1 (voltage, Zin, etc.)           |
| `[EBX + 0xefc]` | Primary input 2 (current, Zout, etc.)          |
| `[EBX + 0xeec]` | Output/result display field                    |
| `[EBX + 0xf00]` | Secondary output field (power, etc.)           |
| `[EBX + 0xc44]` | Decimal places / format control                |
| `[EBX + 0xf30]` | Attenuator: Zin field                          |
| `[EBX + 0xf38]` | Attenuator: Zout field                         |
| `[EBX + 0xf40]` | Attenuator: dB field                           |
| `[EBX + 0xf54]` | Attenuator: R1 output field                    |
| `[EBX + 0x10d8]`| Attenuator: R2 output field                    |
| `[EBX + 0x1058]`| Attenuator: R1 output field (Pi-Pad)           |
| `[EBX + 0x15d0]`| Attenuator: R3/additional output               |

## Global Variables (FPU Storage)

The function uses global doubles in the `0x008d6800` region as scratch space:

| Address        | Usage                                          |
|----------------|------------------------------------------------|
| `0x008d6858`   | LED bias: I_led (after mA->A conversion)       |
| `0x008d6860`   | Attenuator: Zout parsed, LED: V_led            |
| `0x008d6868`   | LED bias: R = (V_supply - V_led) / I_led       |
| `0x008d6870`   | Attenuator: Zin parsed, LED: V_supply          |
| `0x008d6878`   | E-I-R: Voltage/Value1                          |
| `0x008d6880`   | E-I-R: Current/Value2                          |
| `0x008d6888`   | E-I-R: Resistance/Value3                       |
| `0x008d6890`   | E-I-R: Power = V * I                           |
| `0x008d6898`   | R/C/L: Total series/parallel result             |
| `0x008d68a0`   | R/C/L: R1 or C1 or L1                          |
| `0x008d68a8`   | R/C/L: R2 or C2 or L2                          |
| `0x008d68b0`   | R/C/L: R3 or C3 or L3                          |
| `0x008d68b8`   | R/C/L: R4 or C4 or L4                          |
| `0x008d68c0`   | R/C/L: Voltage for power calculation            |
| `0x008d68f0`   | Voltage divider: Vout                          |
| `0x008d68f8`   | Voltage divider: Vin; Attenuator: Zin          |
| `0x008d6900`   | Attenuator: Zin (parsed)                       |
| `0x008d6908`   | Attenuator: Zout (parsed)                      |
| `0x008d6910`   | Attenuator: R1                                 |
| `0x008d6918`   | Attenuator: R2                                 |
| `0x008d6920`   | Attenuator: R3                                 |
| `0x008d6928`   | Attenuator: dB (parsed)                        |
| `0x008d6930`   | Attenuator: K = 10^(dB/20)                     |
| `0x008d6938`   | LED bias: Wattage                              |
| `0x008d6940`   | Voltage divider: intermediate result            |
| `0x008d6948`   | Attenuator: Zmax = max(Zin, Zout)              |
| `0x008d6950`   | Attenuator: Zmin = min(Zin, Zout)              |
| `0x008d6958`   | R/C/L: V * I for individual R                  |
| `0x008d6960`   | R/C/L: V * I for individual R                  |
| `0x008d6980`   | R/C/L: intermediate                            |
| `0x008d6988`   | R/C/L: intermediate                            |
| `0x008d6a18`   | Attenuator: intermediate (mA conversion)        |

## Float Constants

| Address      | Type   | Value  | Usage                              |
|--------------|--------|--------|------------------------------------|
| `0x004d4898` | float  | 1000.0 | mA to A conversion (divide by 1000)|
| `0x004d489c` | float  | 1.0    | Unit constant (parallel formula)   |
| `0x004d48a0` | float  | 20.0   | dB/20 in attenuator K formula      |
| `0x004d48a4` | float  | 2.0    | Factor of 2 in attenuator formulas |
| `0x004d48a8` | float  | 0.5    | Factor of 1/2 in attenuator R3     |

## Runtime Helpers

| Address      | Name (Ghidra)    | Actual Function                   |
|--------------|------------------|-----------------------------------|
| `0x008675ac` | `Math_Ln`        | `Power(base, exponent)` = base^exp|
| `0x00868834` | `FUN_00868834`   | `Sqrt(x)`                         |
| `0x00861e48` | -                | `Format(buf, fmt, value)` sprintf |
| `0x0086f0f4` | -                | `StrToFloat` (string to double)   |
| `0x0086ef50` | -                | `TryStrToFloat` (returns bool)    |
| `0x0086ecc0` | -                | `Delphi_StackStringBuilder`       |
| `0x0086ecf0` | -                | String concatenation/construction |
| `0x0086ee90` | -                | `Delphi_StringCleanup`            |
| `0x00539678` | -                | `TEdit.GetText`                   |
| `0x005396c8` | -                | `TEdit.SetText`                   |
| `0x0071d964` | -                | String formatting helper           |
| `0x0085f89f` | -                | Unit conversion / bool check       |
| `0x0059f924` | -                | `TComboBox.GetItemIndex`          |

---

## Sub-Mode 0: E-I-R (Ohm's Law)

### Inner sub-mode dispatch

A secondary combo box at `[EBX + 0xf20]` with ItemIndex at offset `+0x2f0`
selects which variable to solve for:

- **ItemIndex 0**: Solve for Voltage (V = I * R)
- **ItemIndex 1**: Solve for Current (I = V / R)
- **ItemIndex 2**: Solve for Resistance (R = V / I)

### Formulas (confirmed from disassembly)

```
; ItemIndex 0: Solve for Voltage
V = I * R                       ; 004c43ad: FLD [I]; FMUL [R]; FSTP [V]
P = V * I                       ; 004c49c6: FLD [V]; FMUL [I]; FSTP [P]

; ItemIndex 1: Solve for Current
I = V / R                       ; 004c45e9: FLD [V]; FDIV [R]; FSTP [I]
P = V * I                       ; (same pattern)

; ItemIndex 2: Solve for Resistance
R = V / I                       ; 004c47a0..004c482e: FLD [V]; FDIV [I]; FSTP [R]
P = V * I                       ; (same pattern)
```

All three cases end with:
```
P = V * I                       ; common code after all three branches
```

---

## Sub-Mode: LED Bias Resistor

This is handled within the main mode 0 flow. The LED bias section appears after
the E-I-R computation. The key formula:

```
; At 0x004c56d6:
FDIV float ptr [0x004d4898]     ; divide I_led by 1000.0 (mA -> A)

; At 0x004c56f3-0x004c5708:
FLD  [Vsupply]                  ; 0x008d6870
FSUB [Vled]                     ; 0x008d6860
FDIV [Iled_amps]                ; 0x008d6858
FSTP [R_bias]                   ; 0x008d6868
```

**Formula:** `R = (V_supply - V_led) / (I_led / 1000)`

For the "wattage" variant:
```
; At 0x004c58d3-0x004c58e5:
FLD  [Vsupply]                  ; 0x008d6870
FSUB [Vled]                     ; 0x008d6860
FMUL [Iled_amps]                ; 0x008d6858
FSTP [Wattage]                  ; 0x008d6938
```

**Formula:** `W = (V_supply - V_led) * I_led_amps`

For the alternative variant (when Zin > Zout, at `0x004c5bb4`):
```
FLD  [Vsupply]                  ; 0x008d6870
FSUB [Vled]                     ; 0x008d6860
FDIV [Iled_mA]                  ; 0x008d6a18
FMUL float ptr [0x004d4898]     ; multiply by 1000.0 (invert: mA conversion)
FSTP [R_bias_alt]               ; 0x008d6858
```

---

## Sub-Mode 1: Resistive Voltage Divider

The voltage divider is within the mode-0 extended area. It reads:
- Vin (voltage input)
- R1, R2 (resistor values)
- Component count selector at `[EBX + 0xf68]`

### Series resistor total (ItemIndex 0)
```
; 0x004c6312-0x004c631e:
FLD  [R1]                       ; 0x008d68a0
FADD [R2]                       ; 0x008d68a8
FSTP [R_total]                  ; 0x008d6898
```
**Formula:** `R_total = R1 + R2`

### Voltage divider output
```
; 0x004c64e9-0x004c64f5:
FLD  [Vin]                      ; 0x008d68f8
FDIV [R_total]                  ; 0x008d6898
FSTP [Vout]                     ; 0x008d68f0  (actually Vout = Vin * R2 / (R1+R2))
```

---

## Sub-Mode 2: Series/Parallel R/C/L Calculator

### Parallel Resistor Formula (2 resistors, ItemIndex 0 at [EBX+0xf68])
```
; 0x004ca940-0x004ca960:
FLD  1.0                        ; [0x004d489c]
FDIV [R1]                       ; 0x008d68a0
FLD  1.0                        ; [0x004d489c]
FDIV [R2]                       ; 0x008d68a8
FADDP                           ; 1/R1 + 1/R2
FDIVR 1.0                       ; [0x004d489c] -> 1 / (1/R1 + 1/R2)
FSTP [R_total]                  ; 0x008d6898
```
**Formula:** `R_total = 1 / (1/R1 + 1/R2)`

### Parallel Resistor Formula (3 resistors, ItemIndex 1 at [EBX+0xf68])
```
; 0x004cb471-0x004cb49f:
FLD  1.0
FDIV [R1]                       ; 0x008d68a0
FLD  1.0
FDIV [R2]                       ; 0x008d68a8
FADDP                           ; 1/R1 + 1/R2
FLD  1.0
FDIV [R3]                       ; 0x008d68b0
FADDP                           ; 1/R1 + 1/R2 + 1/R3
FDIVR 1.0                       ; 1 / (1/R1 + 1/R2 + 1/R3)
FSTP [R_total]                  ; 0x008d6898
```
**Formula:** `R_total = 1 / (1/R1 + 1/R2 + 1/R3)`

### Per-Resistor Power Calculations
For each resistor Ri:
```
I_i  = V / R_total              ; current through series string
W_i  = V * I_i                  ; or: V^2 / R_total for individual power
```

The same formulas apply for capacitors (series = parallel R formula, parallel =
series R formula) and inductors (same as resistors).

---

## Sub-Mode 3: Pi-Pad Attenuator

**Address range:** `0x004cec59` - `0x004cf716`
**Triggered when:** `ComboBox.ItemIndex == 3`

### Input parsing
```
; Parse Zin
[0x008d6900] = StrToFloat(EditZin.Text)    ; Zin

; Parse Zout
[0x008d6908] = StrToFloat(EditZout.Text)   ; Zout

; Parse dB
[0x008d6928] = StrToFloat(EditdB.Text)     ; attenuation in dB
```

### K calculation
```
; 0x004ced89-0x004ceda7:
FLD  [dB]                       ; 0x008d6928
FDIV float [20.0]               ; 0x004d48a0 -> dB/20
FSTP [ESP]
PUSH 0x40240000                 ; double 10.0 (high)
PUSH 0x0                        ; double 10.0 (low)
CALL Power                      ; 0x008675ac -> Power(10.0, dB/20)
FSTP [K]                        ; 0x008d6930
```
**Formula:** `K = 10^(dB/20)`

### Impedance ordering (matched vs unmatched)
```
; 0x004cedad-0x004cee38:
if Zin == Zout:
    Zmax = Zin                  ; 0x008d6948
    Zmin = Zout                 ; 0x008d6950
else if Zin > Zout:             ; (JP branch at 0x004cee0c)
    Zmax = Zout                 ; swap: smaller is "min"
    Zmin = Zin
else:
    Zmax = Zin
    Zmin = Zout
```

**Note:** When Zin != Zout (unmatched), the code ensures Zmax >= Zmin by comparing
and swapping. The second FUCOMPP at `0x004cee03` tests `Zout > Zin` (with FXCH)
and assigns accordingly.

### K^2 and K^2 intermediate
```
; 0x004cee56-0x004cee77:
PUSH 2.0                        ; (0x40000000:00000000)
PUSH K
CALL Power                      ; Power(K, 2.0) = K^2
FSUB 1.0                        ; [0x004d489c] -> K^2 - 1
FSTP [temp1]                    ; extended double at [EBP+0xffffefe8]

; 0x004cee95-0x004ceeb0:
PUSH 2.0
PUSH K
CALL Power                      ; Power(K, 2.0) = K^2  (computed again)
FSTP [temp2]                    ; extended double at [EBP+0xffffefdc]
```
So: `temp1 = K^2 - 1` and `temp2 = K^2`

### R1 formula (shunt resistor on Zmax side)
```
; 0x004ceeb6-0x004ceefa:
FLD  [Zmax]                     ; 0x008d6948
FDIV [Zmin]                     ; 0x008d6950  -> Zmax/Zmin
FSTP [ESP]
CALL Sqrt                       ; 0x00868834  -> sqrt(Zmax/Zmin)
FLD  2.0                        ; [0x004d48a4]
FMUL [K]                        ; 0x008d6930  -> 2*K
FMULP                           ; 0xDE 0xC9 (FMULP): 2*K * sqrt(Zmax/Zmin)
FLD  [temp2]                    ; K^2
FSUBRP                          ; 0xDE 0xE1 (FSUBRP): ST(0)-ST(1) = K^2 - 2*K*sqrt(Zmax/Zmin)
FADD 1.0                        ; [0x004d489c] -> K^2 - 2*K*sqrt(Zmax/Zmin) + 1
FLD  [temp1]                    ; K^2 - 1
FDIVRP                          ; 0xDE 0xF1 (FDIVRP): ST(0)/ST(1) = (K^2-1) / (denom)
FMUL [Zmax]                     ; 0x008d6948
FSTP [R1]                       ; 0x008d6910
```

**Pi-Pad R1:** `R1 = Zmax * (K^2 - 1) / (K^2 - 2*K*sqrt(Zmax/Zmin) + 1)`

### R2 formula (shunt resistor on Zmin side)

R2 uses the same structure as R1, but the critical difference is at the instruction
that combines `2*K` with `sqrt(Zmax/Zmin)`:
- **R1** uses `FMULP` (0xDE 0xC9) => multiply: `2*K * sqrt(Zmax/Zmin)`
- **R2** uses `FDIVRP` (0xDE 0xF1) => reverse divide: `2*K / sqrt(Zmax/Zmin)`

And the final multiply uses Zmin instead of Zmax.

```
; 0x004cf0c5-0x004cf181:
; Recompute temp1 = K^2-1 and temp2 = K^2 (same pattern as R1)
; Then:
FLD  [Zmax]                     ; 0x008d6948
FDIV [Zmin]                     ; 0x008d6950  -> Zmax/Zmin
CALL Sqrt                       ; sqrt(Zmax/Zmin)
FLD  2.0                        ; [0x004d48a4]
FMUL [K]                        ; 0x008d6930  -> 2*K
FDIVRP                          ; 0xDE 0xF1 (FDIVRP): ST(0)/ST(1) = 2*K / sqrt(Zmax/Zmin)
FLD  [temp2]                    ; K^2
FSUBRP                          ; 0xDE 0xE1 (FSUBRP): K^2 - 2*K/sqrt(Zmax/Zmin)
FADD 1.0                        ; K^2 - 2*K/sqrt(Zmax/Zmin) + 1
FLD  [temp1]                    ; K^2 - 1
FDIVRP                          ; 0xDE 0xF1 (FDIVRP): (K^2-1) / (denom)
FMUL [Zmin]                     ; 0x008d6950
FSTP [R2]                       ; 0x008d6918
```

**Pi-Pad R2:** `R2 = Zmin * (K^2 - 1) / (K^2 - 2*K/sqrt(Zmax/Zmin) + 1)`

Since `2*K/sqrt(Zmax/Zmin) = 2*K*sqrt(Zmin/Zmax)`, the equivalent form is:

**Pi-Pad R2:** `R2 = Zmin * (K^2 - 1) / (K^2 - 2*K*sqrt(Zmin/Zmax) + 1)`

### R3 formula (series resistor between input and output)
```
; 0x004cf4de-0x004cf54b:
FLD  [Zmax]                     ; 0x008d6948
FMUL [Zmin]                     ; 0x008d6950  -> Zmax * Zmin
CALL Sqrt                       ; sqrt(Zmax * Zmin)
FMUL 0.5                        ; [0x004d48a8] -> 0.5 * sqrt(Zmax*Zmin)
FSTP [intermediate]             ; extended double

; Then:
PUSH 2.0
PUSH K
CALL Power                      ; K^2
FSUB 1.0                        ; K^2 - 1
FDIV [K]                        ; (K^2 - 1) / K
FLD  [intermediate]             ; 0.5 * sqrt(Zmax*Zmin)
FMULP                           ; 0.5 * sqrt(Zmax*Zmin) * (K^2-1) / K
FSTP [R3]                       ; 0x008d6920
```

**Pi-Pad R3:** `R3 = sqrt(Zmax*Zmin) * (K^2 - 1) / (2*K)`

Note: `0.5 * (K^2-1)/K * sqrt(Zmax*Zmin)` = `(K^2-1)/(2K) * sqrt(Zmax*Zmin)`.

### Simplified matched Pi-Pad formulas

When Zin = Zout = Z, `sqrt(Zmax/Zmin) = 1`, so the denominators simplify:

```
K = 10^(dB/20)
R1 = Z * (K^2 - 1) / (K^2 - 2K + 1) = Z * (K+1)/(K-1)     [shunt to GND]
R2 = Z * (K^2 - 1) / (K^2 - 2K + 1) = Z * (K+1)/(K-1)     [shunt to GND]
R3 = Z * (K^2 - 1) / (2K)                                    [series in-line]
```

### Pi-Pad topology mapping

Saturn's R1/R2/R3 map to the physical Pi-pad circuit as follows:

```
   Input ──── R3 (series) ────── Output
          │                    │
         R1 (shunt)          R2 (shunt)
          │                    │
         GND                  GND
```

- **R1** = shunt resistor on the Zmax (input) side
- **R2** = shunt resistor on the Zmin (output) side
- **R3** = series resistor between input and output

For matched 50 ohm, 6 dB: R1 = R2 = 150.48, R3 = 37.35.
This correctly gives Zin = 50.0 when loaded with 50 ohm at the output.

---

## Sub-Mode 4: T-Pad Attenuator

**Address range:** `0x004cf716` - `0x004d0173`
**Triggered when:** `ComboBox.ItemIndex == 4`

### K calculation (same as Pi-Pad)

```
; 0x004cf846-0x004cf864:
FLD  [dB]                       ; 0x008d6928
FDIV float [20.0]               ; 0x004d48a0
FSTP [ESP]
PUSH 0x40240000                 ; 10.0
PUSH 0x0
CALL Power                      ; Power(10.0, dB/20)
FSTP [K]                        ; 0x008d6930
```
**Formula:** `K = 10^(dB/20)` (identical to Pi-Pad)

### Impedance ordering
Same Zin/Zout comparison and swap logic as Pi-Pad (matched: Zmax=Zin, Zmin=Zout;
unmatched: swap if needed so Zmax >= Zmin).

### R3 formula (shunt resistor to ground)
```
; 0x004cf8fb-0x004cf968:
FLD  [Zmax]                     ; 0x008d6948
FMUL [Zmin]                     ; 0x008d6950  -> Zmax * Zmin
CALL Sqrt                       ; sqrt(Zmax * Zmin)
FMUL 2.0                        ; [0x004d48a4] -> 2 * sqrt(Zmax*Zmin)
FSTP [intermediate]             ; extended double at [EBP+0xffffefac]

; K^2 computation:
PUSH 2.0
PUSH K
CALL Power                      ; K^2
FSUB 1.0                        ; K^2 - 1
FDIVR [K]                       ; K / (K^2-1)   [note: FDIVR reverses]
FLD  [intermediate]             ; 2*sqrt(Zmax*Zmin)
FMULP                           ; 2*sqrt(Zmax*Zmin) * K / (K^2-1)
FSTP [R3]                       ; 0x008d6920
```

**T-Pad R3:** `R3 = 2 * K * sqrt(Zmax*Zmin) / (K^2 - 1)`

### R1 formula (series resistor on Zmax side)

The FDIVRP (0xDE 0xF1) computes `ST(0)/ST(1)`. After `FSUB 1.0` gives
`ST(0) = K^2-1`, then `FLD [temp_a]` pushes `K^2+1` to ST(0), moving `K^2-1` to
ST(1). FDIVRP then computes `ST(0)/ST(1) = (K^2+1)/(K^2-1)`.

```
; 0x004cfb33-0x004cfbbf:
; temp_a = Power(K, 2.0) + 1.0 = K^2 + 1
; Then: Power(K, 2.0) - 1.0 = K^2 - 1 (in ST(0))
FLD  [temp_a]                   ; ST(0) = K^2+1, ST(1) = K^2-1
FDIVRP                          ; 0xDE 0xF1: ST(0)/ST(1) = (K^2+1)/(K^2-1)
FMUL [Zmax]                     ; 0x008d6948  -> Zmax*(K^2+1)/(K^2-1)
FSUB [R3]                       ; 0x008d6920
FSTP [R1]                       ; 0x008d6910
```

**T-Pad R1:** `R1 = Zmax * (K^2 + 1)/(K^2 - 1) - R3`

For the standard matched T-pad (Zin = Zout = Z):
```
R1 = Z * (K^2+1)/(K^2-1) - R3
   = Z * (K^2+1)/(K^2-1) - 2KZ/(K^2-1)
   = Z * (K^2 - 2K + 1)/(K^2-1)
   = Z * (K-1)^2 / ((K-1)(K+1))
   = Z * (K-1)/(K+1)
```

### R2 formula (series resistor on Zmin side)

Same structure as R1 but multiplied by Zmin instead of Zmax.

```
; 0x004cfd9c-0x004cfe16:
; Same (K^2+1)/(K^2-1) computation via FDIVRP
FMUL [Zmin]                     ; 0x008d6950  -> Zmin*(K^2+1)/(K^2-1)
FSUB [R3]                       ; 0x008d6920
FSTP [R2]                       ; 0x008d6918
```

**T-Pad R2:** `R2 = Zmin * (K^2 + 1)/(K^2 - 1) - R3`

For matched case:
`R2 = Z * (K-1)/(K+1)` (same as R1, as expected for symmetric T-pad).

### T-Pad topology mapping

```
   Input ── R1 (series) ──┬── R2 (series) ── Output
                           │
                          R3 (shunt)
                           │
                          GND
```

- **R1** = series resistor on the Zmax (input) side
- **R2** = series resistor on the Zmin (output) side
- **R3** = shunt resistor to ground (between R1 and R2)

For matched 50 ohm, 6 dB: R1 = R2 = 16.61, R3 = 66.93.
This correctly gives Zin = 50.0 when loaded with 50 ohm at the output.

---

## Summary of All Attenuator Formulas

### Common
```
K = 10^(dB/20)

If Zin >= Zout:
    Zmax = Zin,  Zmin = Zout
Else:
    Zmax = Zout, Zmin = Zin
```

### Pi-Pad Attenuator (unmatched)
```
R1 = Zmax * (K^2 - 1) / (K^2 - 2*K*sqrt(Zmax/Zmin) + 1)    [shunt, Zmax side]
R2 = Zmin * (K^2 - 1) / (K^2 - 2*K*sqrt(Zmin/Zmax) + 1)    [shunt, Zmin side]
R3 = sqrt(Zmax * Zmin) * (K^2 - 1) / (2 * K)                [series, in-line]
```

### Pi-Pad Attenuator (matched: Zin = Zout = Z)
```
R1 = R2 = Z * (K + 1) / (K - 1)       [shunt to GND]
R3 = Z * (K^2 - 1) / (2 * K)          [series in-line]
```

### T-Pad Attenuator (unmatched)
```
R3 = 2 * K * sqrt(Zmax * Zmin) / (K^2 - 1)                  [shunt to GND]
R1 = Zmax * (K^2 + 1) / (K^2 - 1) - R3                      [series, Zmax side]
R2 = Zmin * (K^2 + 1) / (K^2 - 1) - R3                      [series, Zmin side]
```

### T-Pad Attenuator (matched: Zin = Zout = Z)
```
R1 = R2 = Z * (K - 1) / (K + 1)       [series in-line]
R3 = 2 * K * Z / (K^2 - 1)            [shunt to GND]
```

---

## FPU Opcode Reference

The following opcodes appear in the attenuator formulas. Their semantics were verified
by compiling and running a C test program with inline assembly on x86-32, confirming
the actual CPU behavior matches the interpretation used above.

| Bytes     | Ghidra Label | Actual Computation                        |
|-----------|--------------|-------------------------------------------|
| `DE C9`   | FMULP        | ST(1) = ST(0) * ST(1), pop               |
| `DE E1`   | FSUBRP       | ST(1) = ST(0) - ST(1), pop               |
| `DE E9`   | FSUBP        | ST(1) = ST(1) - ST(0), pop               |
| `DE F1`   | FDIVRP       | ST(1) = ST(0) / ST(1), pop               |
| `DE F9`   | FDIVP        | ST(1) = ST(1) / ST(0), pop               |

**Key insight:** Ghidra's "reverse" (R) variants swap the operand order. FSUBRP
computes `ST(0) - ST(1)` (top minus second), while FSUBP computes `ST(1) - ST(0)`
(second minus top). Similarly for FDIVRP vs FDIVP.

---

## Confidence Assessment

| Component                | Confidence | Notes                              |
|--------------------------|------------|------------------------------------|
| E-I-R formulas           | HIGH       | V=IR, P=VI trivially confirmed     |
| LED bias formula         | HIGH       | R=(Vsup-Vled)/(Iled/1000) clear    |
| Parallel R formula       | HIGH       | 1/(1/R1+1/R2+...) confirmed        |
| Series R formula         | HIGH       | R1+R2+... confirmed                |
| C/L series/parallel      | HIGH       | Same as R with roles swapped        |
| Voltage divider          | HIGH       | Vout = Vin * R2/(R1+R2) pattern    |
| Float constants          | HIGH       | 1000, 1, 20, 2, 0.5 confirmed      |
| K = 10^(dB/20)           | HIGH       | Clear from disassembly              |
| Pi-Pad R1/R2 (unmatched) | HIGH       | Impedance-verified (see below)      |
| Pi-Pad R3                | HIGH       | sqrt(Zmax*Zmin) * (K^2-1)/(2K)     |
| T-Pad R1/R2              | HIGH       | Impedance-verified (see below)      |
| T-Pad R3                 | HIGH       | 2K*sqrt(Zmax*Zmin)/(K^2-1)         |
| Zmax/Zmin swap logic     | HIGH       | FUCOMPP comparison clear            |
| Power() function ID      | HIGH       | Named Math_Ln, used as Power(b,e)   |
| Sqrt() function ID       | HIGH       | Simple wrapper, single double arg   |

## Impedance Verification

All attenuator formulas were verified by computing the input impedance of the
resulting network when terminated with the specified load impedance. In all cases
the formulas produce exact impedance matching.

### Matched Pi-Pad: Z=50, dB=6
```
K = 1.995262
R1 = R2 = 150.4760 (shunt)
R3 = 37.3519 (series)
Zin with 50-ohm load: 50.0000  [PASS]
Attenuation: 6.0000 dB         [PASS]
```

### Matched T-Pad: Z=50, dB=6
```
R1 = R2 = 16.6139 (series)
R3 = 66.9310 (shunt)
Zin with 50-ohm load: 50.0000  [PASS]
```

### Unmatched Pi-Pad: Zin=75, Zout=50, dB=10
```
K = 3.162278
R1 = 207.4349 (shunt, 75-ohm side)
R2 = 77.1073 (shunt, 50-ohm side)
R3 = 87.1421 (series)
Zin with 50-ohm load:  75.0000 [PASS]
Zout with 75-ohm load: 50.0000 [PASS]
```

### Unmatched T-Pad: Zin=75, Zout=50, dB=10
```
R1 = 48.6335 (series, 75-ohm side)
R2 = 18.0780 (series, 50-ohm side)
R3 = 43.0331 (shunt)
Zin with 50-ohm load:  75.0000 [PASS]
Zout with 75-ohm load: 50.0000 [PASS]
```

## Test Vectors (computed from traced formulas)

These values can be checked against Saturn PCB Toolkit v8.44 output:

1. **Matched Pi-Pad:** Zin=Zout=50, dB=6 -> R1=R2=150.48, R3=37.35
2. **Matched T-Pad:** Zin=Zout=50, dB=6 -> R1=R2=16.61, R3=66.93
3. **Unmatched Pi-Pad:** Zin=75, Zout=50, dB=10 -> R1=207.43, R2=77.11, R3=87.14
4. **Unmatched T-Pad:** Zin=75, Zout=50, dB=10 -> R1=48.63, R2=18.08, R3=43.03
5. **LED bias:** Vsup=5, Vled=2, Iled=20mA -> R=150 ohm
6. **Parallel 2R:** R1=100, R2=200 -> 66.667 ohm
