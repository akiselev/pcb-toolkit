# Reverse Engineering Progress

## Status: Complete (~99%)

### What's Done

1. **Binary identification**: toolkit/toolkit.exe is a PE32 Delphi/C++ Builder (RAD Studio) native x86 app, NOT .NET. ~10.4 MB, version 8.44.

2. **Help PDF fully read**: All 47 pages of `toolkit/Saturn PCB Toolkit Help.pdf` analyzed. Contains UI screenshots, input/output descriptions, formula references, and example values (usable as test vectors).

3. **String extraction complete**: All UI strings, material names, protocol presets, error messages, format strings, and formula references extracted from binary.

4. **20+ note files written** in `docs/notes/`:
   - `00-overview.md` through `15-materials-data.md` (16 calculator docs)
   - `16-rust-design-research.md` — Rust implementation design decisions
   - `ghidra-impedance.md` — decompiled microstrip impedance calculator (610 lines)
   - `ghidra-stripline.md` — stripline solver analysis (790 lines)
   - `ghidra-edge-coupled.md` — 4 edge-coupled solver modes (754 lines)
   - `ghidra-conductor-spacing.md` — IPC-2221C spacing lookup analysis
   - `ghidra-conductor-current.md` — IPC-2152/2221A conductor current analysis (complete)
   - `ghidra-fusing-inductor.md` — Fusing current (Onderdonk) + Planar inductor (Mohan/Wheeler)
   - `ghidra-ppm-reactance-mode15.md` — PPM, Reactance, and Thermal Management solvers
   - `ghidra-ohmslaw.md` — Ohm's Law with 5 sub-modes including attenuators

5. **Material database extracted** (46 materials + Air + Custom):
   - Complete ordered list from DFM ComboBox Items.Strings
   - 23 Er values extracted as strings from binary (see `15-materials-data.md`)
   - Complete roughness factor mapping for all 45 materials (0.98 vs 1.0)
   - 9 Tg values extracted: 130, 170, 180, 260, 280, 240, 140, 165, 210
   - Materials 24-44 reuse same 23 Er string constants (exact mapping still needs full disassembly)

6. **Mode-to-handler mapping CORRECTED and VERIFIED**:
   - Traced 15+ menu click handlers to their DAT_008d5f88 mode assignments
   - Previous mode names were incorrect for modes 2-18 (based on wrong assumptions)
   - All 19 solver modes now correctly identified (see NOTES.md)

7. **40 functions renamed in Ghidra** (project: `saturn-pcb`):
   - 19 solver functions with correct calculator names
   - 6 UI handler / pre-computation functions
   - 7 Delphi RTL helpers
   - 7 VCL control / math helpers
   - 5 menu click handlers

8. **IPC-2221C spacing table FULLY EXTRACTED**:
   - Complete 8×9 lookup table (8 device types × 9 voltage ranges)
   - All 20 unique spacing values confirmed via disassembly
   - >500V linear formula for all 8 device types with slope/intercept constants
   - Documented in `docs/notes/14-conductor-spacing.md`

9. **All 19 solver functions decompiled/analyzed**:
   - Mode 0: Microstrip (Hammerstad-Jensen) — full decompilation
   - Mode 1: Stripline (Cohn/Wadell) — disassembly analysis, constants confirmed
   - Mode 2: Conductor Current (IPC-2152/2221A) — full disassembly analysis, all formulas + modifier tables
   - Mode 3: Edge Coupled External — decompiled
   - Mode 4: Differential Pairs (Edge Coupled Int Sym) — disassembly analysis
   - Mode 5: Embedded Microstrip (Edge Coupled Int Asym) — decompiled
   - Mode 6: Er Effective (Edge Coupled Embedded) — decompiled
   - Mode 7: Fusing Current (Onderdonk) — decompiled, constants confirmed (Tm=1084.62°C)
   - Mode 8: Wire Gauge Properties — decompiled, AWG lookup table (NOT broadside coupled impedance)
   - Mode 9: Conductor Spacing — complete table extraction
   - Mode 10: Ohm's Law — full disassembly (E-I-R, LED bias, voltage divider, R/C/L, Pi/T-pad attenuators)
   - Mode 11: Padstack — decompiled, all 7 sub-types with geometry formulas
   - Mode 12: PDN Impedance — full decompilation, 3 formulas verified against Help PDF
   - Mode 13: Planar Inductor (Mohan/Wheeler) — decompiled, Saturn-specific K1/K2 for circular shape
   - Mode 14: PPM Calculator — decompiled, Hz↔PPM + XTAL load cap formulas
   - Mode 15: Thermal Management — identified, basic analysis done
   - Mode 16: Via Properties — full disassembly, 16 formulas, Goldfarb capacitance, Simonovich diff via
   - Mode 17: Wavelength — full decompilation
   - Mode 18: Reactance — decompiled, standard Xc/Xl/f_res formulas

10. **Physical constants verified** at specific binary addresses:
    - Speed of light, µ₀, ε₀, copper resistivity/temp coeff
    - All unit conversion factors (mil/mm/inch)
    - PCB thickness table (0.254mm to 2.286mm in 9 steps)

11. **PE section mapping resolved**:
    - .text: VA 0x00401000 - 0x00871000, file 0x600
    - .data: VA 0x00871000 - 0x00905000, file 0x46fc00
    - Correct VA-to-file-offset conversion documented

### What's NOT Done

1. **Crosstalk calculator**: Uses a separate form, doesn't go through the main dispatcher. Handler at 0x004bde04 is in an unanalyzed region of the binary. Low priority (marked "unsupported" in original).

2. ~~PDN Impedance (Mode 12)~~: DONE. Three formulas verified with test vectors from Help PDF.

3. ~~Stripline formula cleanup~~: DONE. 941-line cross-reference document with clean Rust pseudocode. See `stripline-formulas-clean.md`.

### Resolved Items

- **All 19 solver modes analyzed** — Mode 8 = wire gauge property lookup, Mode 16 = Via Properties (16 formulas).
- **Er-to-material mapping COMPLETE**: All 46 materials fully mapped from ComboBox1Change disassembly. Major correction: 21 of 23 materials had wrong Er in earlier notes due to sequential string assumption. See `materials-er-mapping.md`.
- **Thermal Management (Mode 15) COMPLETE**: Simple formula `T_j = R_theta * P + T_ambient`. The "opaque PreCompute_3" was just ambient temperature read. See `ghidra-thermal-management.md`.
- **Impedance sub-mode routing**: Each mode has its own menu click handler, not RadioGroup-switched.
- **Via capacitance**: Uses Goldfarb constant 1.41, NOT 4/π. The 4/π constant at 0x004BA940 is only used in Solver_FusingCurrent.
- **Math function corrections**: `FUN_008673c0` = log10 (not sqrt), `FUN_008675ac` = pow (not ln). Correct: `FUN_00867350` = ln, `FUN_00868834` = sqrt.
- **RO4350 Er bug found**: Impedance form uses Er=3.66 (correct), crosstalk form uses Er=3.48 (bug).
- **Stripline Cohn/Wadell formula cleanup COMPLETE**: Saturn uses hybrid approach with proprietary empirical corrections. 60+ constants mapped, Wadell/Cohn/IPC-2141A formulas cross-referenced, clean Rust pseudocode ready. See `stripline-formulas-clean.md`.

### Ghidra Project State

```
Project: saturn-pcb (at ~/.cache/ghidra-cli/projects/)
Program: toolkit.exe
Analysis: Complete (14,689 functions)
Named functions: 40+
```

### How to Resume

```bash
# 1. Kill any lingering ghidra processes
pkill -9 -f GhidraCliBridge
pkill -9 -f "ghidra.GhidraClassLoader"
sleep 3

# 2. Clean up lock and bridge files
rm -f ~/.cache/ghidra-cli/projects/saturn-pcb.lock~
rm -f ~/.local/share/ghidra-cli/bridge-*.port
rm -f ~/.local/share/ghidra-cli/bridge-*.pid

# 3. If project was deleted, reimport WITHOUT analysis first:
~/.local/share/ghidra-cli/ghidra/ghidra_12.0.1_PUBLIC/support/analyzeHeadless \
  ~/.cache/ghidra-cli/projects saturn-pcb \
  -import toolkit/toolkit.exe -overwrite -noanalysis

# 4. Then analyze separately with memory limits:
~/.local/share/ghidra-cli/ghidra/ghidra_12.0.1_PUBLIC/support/analyzeHeadless \
  ~/.cache/ghidra-cli/projects saturn-pcb \
  -process toolkit.exe -analysisTimeoutPerFile 900 -max-cpu 1

# 5. Set defaults:
ghidra set-default project saturn-pcb
ghidra set-default program toolkit.exe
```

### Priority for Next Session

1. **Begin Rust implementation** using extracted formulas and table data
2. All reverse engineering is complete — all 19 modes have clean, implementable formulas

### Files Summary
- `NOTES.md` — Master consolidated notes (all findings, handler mapping)
- `PROGRESS.md` — This file
- `docs/notes/00-overview.md` through `15-materials-data.md` — Per-calculator notes
- `docs/notes/16-rust-design-research.md` — Rust design decisions
- `docs/notes/ghidra-impedance.md` — Decompiled microstrip impedance (610 lines)
- `docs/notes/ghidra-stripline.md` — Stripline solver analysis (790 lines)
- `docs/notes/ghidra-edge-coupled.md` — Edge-coupled solver analysis (754 lines)
- `docs/notes/ghidra-conductor-spacing.md` — IPC-2221C spacing analysis
- `docs/notes/ghidra-conductor-current.md` — IPC-2152/2221A conductor current (complete)
- `docs/notes/ghidra-fusing-inductor.md` — Fusing current + Planar inductor decompilation
- `docs/notes/ghidra-ppm-reactance-mode15.md` — PPM, Reactance, Thermal Management
- `docs/notes/ghidra-ohmslaw.md` — Ohm's Law with 5 sub-modes + attenuators
- `docs/notes/ghidra-padstack.md` — Padstack calculator (7 sub-types)
- `docs/notes/ghidra-broadside-coupled.md` — Mode 8 wire gauge lookup (44 AWG entries)
- `docs/notes/ghidra-via-properties.md` — Via Properties (16 formulas, 53KB function)
- `docs/notes/ghidra-thermal-management.md` — Thermal Management (T_j = R*P + T_ambient)
- `docs/notes/materials-er-mapping.md` — Complete Er/Tg mapping for all 46 materials
- `docs/notes/ghidra-pdn-impedance.md` — PDN Impedance (3 formulas, test vectors)
- `docs/notes/stripline-formulas-clean.md` — Stripline cross-reference: Cohn/Wadell/IPC-2141A vs Saturn (941 lines)
