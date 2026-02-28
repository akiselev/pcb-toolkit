# Saturn PCB Toolkit - Reverse Engineering Overview

## Binary Info
- **File**: `toolkit/toolkit.exe`
- **Type**: PE32, Windows GUI, Intel i386
- **Size**: ~10.4 MB
- **Compiler**: Embarcadero C++ Builder / RAD Studio (Delphi VCL)
- **Version**: 8.44 (Jan 5, 2026)
- **NOT .NET** - native x86 binary with Delphi VCL framework

## Architecture
- Single main form (`TForm1`) with tabbed interface (each calculator is a tab)
- Additional dialog forms: TForm2, TForm4, TForm5, TForm6, TForm8, TForm9, TForm10, TForm11, TForm12, TForm100
- Event-driven: `ButtonNClick` and `NamedItem1Click` handlers trigger calculations
- Shared "Options" panel across all tabs: units, copper weight, substrate, temp rise, ambient temp

## Menu/Tab Click Handlers (from strings)
| Handler | Calculator |
|---------|-----------|
| `ConductorProperties1Click` | PCB Conductor Current |
| `RFImpedances1Click` | Conductor Impedance (Microstrip/Stripline) |
| `DifferentialPairs1Click` | Differential Pair + Crosstalk (NEXT) |
| `ViaProperties1Click` | Via Current / Via Properties |
| `OhmsLaw1Click` | Ohm's Law |
| `XlXCReactance1Click` | XC/XL Reactance |
| `Padstacks1Click` | Padstack Calculator |
| `PlanarInductors1Click` | Planar Inductor |
| `WavelengthCalculator1Click` | Wavelength |
| `PPMCalculator1Click` | PPM / XTAL Calculator |
| `PDNImpedance1Click` | PDN Calculator |
| `ConductorSpacing1Click` | Min Conductor Spacing |
| `Crosstalk1Click` / `CrosstalkCalculator1Click` | Crosstalk (standalone) |
| `EmbeddedRs1Click` | Embedded Resistors |

## Common Options (shared across all calculators)
- **Units**: Imperial (mils) / Metric (mm or microns)
- **Base Copper Weight**: 0.25oz, 0.5oz, 1oz, 1.5oz, 2oz, 2.5oz, 3oz, 4oz, 5oz
- **Plating Thickness**: Bare PCB, 0.5oz, 1oz, 1.5oz, 2oz, 2.5oz, 3oz
- **Substrate/Material Selection**: FR-4 STD, Isola P26N, Isola P95, Isola P96, + custom Er/Tg
- **Er (Dielectric Constant)**: e.g., 4.6 for FR-4 STD
- **Tg (Glass Transition)**: e.g., 130°C for FR-4 STD
- **Temp Rise (°C)**: default 20°C
- **Ambient Temp (°C)**: default 22°C
- **Conductor Layer**: Internal / External
- **Plane Thickness**: 0.5oz/1oz or 2oz

## IPC Standards Referenced
- **IPC-2152** (with/without modifiers) - conductor current capacity
- **IPC-2221** / **IPC-2221A** (obsolete for amperage) - legacy conductor current
- **IPC-2221C** - minimum conductor spacing
- **IPC-7351A** - BGA land sizes
- **IPC-2316** - embedded resistors
- **IPC-2141A** - simplified impedance formulas

## Key Academic/Industry References
- **Hammerstad & Jensen** - "Accurate Models for Microstrip Computer-Aided Design" (microstrip impedance)
- **Mohan et al.** - "Simple Accurate Expressions for Planar Spiral Inductances" (Stanford, JSSC 1999)
- **Onderdonk** - fusing current equation
- **Bert Simonovich** - differential via modeling methodology
- **Dankov, Levcheva, Hadjistamov** - "Two-Resonator Method for Characterization of Dielectric Substrate Anisotropy"

## Conductor Etch Factor
- **1:1**: Standard etch factor. Top width H2 = H1 - 2T (T = conductor thickness)
- **2:1**: Half the etch of 1:1
- **None**: Rectangular cross-section (ideal)
- Affects cross-sectional area calculations for current capacity

## Features We Want to Implement
1. Microstrip calculator → see `01-impedance.md`
2. Stripline calculator → see `01-impedance.md`
3. Differential pair calculator → see `02-differential-pairs.md`
4. Via current calculator → see `03-via-properties.md`
5. PCB conductor current calculator → see `04-conductor-current.md`
6. Planar inductor calculator → see `05-planar-inductor.md`
7. Padstack calculator → see `06-padstack.md`
8. Crosstalk calculator → see `07-crosstalk.md`
9. Ohm's Law calculator → see `08-ohms-law.md`
10. XC XL Reactance calculator → see `09-reactance.md`
11. BGA Land calculator → see `06-padstack.md` (part of padstack)
12. Er Effective calculator → see `10-er-effective.md`
13. Wavelength calculator → see `11-wavelength.md`
14. PPM Calculator → see `12-ppm-calculator.md`
