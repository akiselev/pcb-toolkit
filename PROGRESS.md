# Reverse Engineering Progress

## Status: Partially Complete (~75%)

### What's Done

1. **Binary identification**: toolkit/toolkit.exe is a PE32 Delphi/C++ Builder (RAD Studio) native x86 app, NOT .NET. ~10.4 MB, version 8.44.

2. **Help PDF fully read**: All 47 pages of `toolkit/Saturn PCB Toolkit Help.pdf` analyzed. Contains UI screenshots, input/output descriptions, formula references, and example values (usable as test vectors).

3. **String extraction complete**: All UI strings, material names, protocol presets, error messages, format strings, and formula references extracted from binary.

4. **17 note files written** in `docs/notes/`:
   - `00-overview.md` through `15-materials-data.md` (16 files)
   - `ghidra-impedance.md` - decompiled impedance calculator internals (610 lines)
   - Cover all 14 target calculators + materials database
   - Include formulas from the help PDF and known engineering references
   - Include test vectors from the PDF examples

5. **Material database extracted** (46 materials in dropdown):
   - Complete ordered list from DFM ComboBox Items.Strings
   - 23 Er values extracted as strings from binary (see NOTES.md)
   - Mapping of Er values to materials partially resolved (first 23)

6. **Handler function addresses identified** from Delphi published method table:
   - All key click/change handlers mapped to code addresses (see NOTES.md)

7. **Main dispatcher architecture discovered** (FUN_00403398):
   - Global mode selector at DAT_008d5f88 dispatches to 19 solver functions
   - 4 common pre-computation functions called before every solver
   - Mode-to-function mapping fully documented (see NOTES.md)

8. **Three solver functions decompiled** (via background agent):
   - FUN_00440e34 (Mode 0: Microstrip) - Hammerstad-Jensen formulas confirmed
   - FUN_004bf410 (Mode 17: Wavelength) - frequency/wavelength conversion
   - FUN_004b8104 (Mode 7: Via/Broadside Coupled) - via capacitance with 4/π correction
   - Physical constants verified at specific addresses (c, 4/π, 2.54, etc.)
   - Kirschning-Jansen dispersion constants (0.457, 0.67) confirmed

9. **Helper function map**: ~20 Delphi RTL/VCL functions identified and documented

10. **Global variables map**: ~30 data addresses mapped to their purposes

11. **Four additional calculator modules implemented**:
    - `impedance::stripline` — Cohn/Wadell centered stripline formula
    - `impedance::embedded` — Embedded microstrip with burial correction (Brooks)
    - `current` — IPC-2221A current capacity, DC resistance, skin depth
    - `crosstalk` — Backward crosstalk (NEXT) estimation (standard Kb formula)

### What's NOT Done

1. **Remaining solver decompilation**: 16 of 19 solver modes not yet decompiled:
   - FUN_0040bc00 (Stripline) - too large to decompile in Ghidra
   - FUN_004343e4 (Differential) - too large
   - FUN_00427090 (Er Effective) - too large
   - Modes 3-6, 8-16, 18 - not attempted

2. **Er-to-material mapping for materials 24-44**: We have 23 Er values but 44 materials. The remaining 21 materials' Er values need decompilation of `ComboBox1Change` (0x00494dd4).

3. **Tg values**: Not extracted from binary. Likely in `ComboBox1Change`.

4. **IPC-2221C spacing table**: Full lookup table data not extracted.

5. **IPC-2152 conductor current charts**: Coefficient data not extracted.

6. **Conductor current IPC-2152 mode**: IPC-2221A implemented but IPC-2152 table data still needed for full accuracy. Saturn PDF vectors (page 6/46) use IPC-2152 mode.

7. **Differential pair calculator**: Not decompiled (FUN_004343e4 too large).

8. **Crosstalk Saturn match**: Standard Kb formula implemented but doesn't match Saturn's test vector (-2.23 dB / 3.87 V). Saturn likely uses a different formula.

### Ghidra Issues Encountered

- **Project lock file**: Ghidra leaves `saturn-pcb.lock~` in `~/.cache/ghidra-cli/projects/`. Must delete it after killing processes.
- **OOM during auto-analysis**: Full auto-analysis of this 10.4MB PE32 gets OOM-killed (exit 137). Solution: import with `-noanalysis`, then run targeted analysis.
- **Bridge is single-threaded**: Cannot run multiple ghidra commands in parallel. Background agents fight over the project lock.
- **Bridge port files**: `~/.local/share/ghidra-cli/bridge-*.port` and `bridge-*.pid` must be cleaned up after killing processes.
- **Large Delphi functions**: Several key solver functions are too large for Ghidra's decompiler (FUN_0040bc00, FUN_004343e4, FUN_00427090).

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

# 4. Then analyze separately (if RAM allows):
ghidra analyze

# 5. Or just start and use the bridge (analysis may happen on demand):
ghidra set-default project saturn-pcb
ghidra set-default program toolkit.exe

# 6. Priority functions to decompile next:
#    ComboBox1Change (full Er/Tg mapping): 0x00494dd4
#    Conductor current solver:             find via FUN_00403398 dispatch
#    Differential pair solver:             FUN_004343e4 (mode 2) - may need -Xmx4G
#    Er Effective:                         FUN_00427090 (mode 16) - may need -Xmx4G

# 7. For large functions that fail decompilation, try:
#    - ghidra disasm 0xADDRESS -n 500  (get assembly instead)
#    - Look for sub-functions they call and decompile those individually
#    - ghidra function calls 0xADDRESS (find called subroutines)
```

### Priority for Next Session

1. **Import with -noanalysis** (avoids OOM) - MUST DO SEQUENTIALLY, NO PARALLEL AGENTS
2. Decompile `ComboBox1Change` (0x00494dd4) to get complete material Er/Tg table
3. Use `ghidra function calls FUN_0040bc00` to find sub-functions of the large solvers
4. Decompile the sub-functions individually (they should be smaller)
5. Extract IPC-2221C spacing lookup table data
6. Extract IPC-2152 current capacity coefficients

### Files Summary
- `NOTES.md` - Master consolidated notes (all findings)
- `PROGRESS.md` - This file
- `docs/notes/00-overview.md` - Architecture, handler mapping
- `docs/notes/01-impedance.md` through `15-materials-data.md` - Per-calculator notes
- `docs/notes/ghidra-impedance.md` - Decompiled impedance code (610 lines)
