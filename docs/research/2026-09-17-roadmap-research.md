# Roadmap research, 2026-09-17

Three web-research reports produced after the 0.2.x audit response, unedited.
They inform the roadmap; nothing here is implemented yet.

1. Feature gaps versus comparable tools
2. Machine-learning and data-driven extensions
3. Integrations and ecosystem directions

---

## 1. Feature gaps versus comparable tools

**Scope check against the current tree.** Propagation delay and L/C per unit length are already fields of the impedance result type, and `current::skin_depth_mils` exists, so those are not listed as new features. Every comparable tool surveyed (Saturn v8.45, KiCad 9 TransLine, wcalc, Qucs/transcalc) offers synthesis (solve for width), and none of pcb-toolkit's calculators does. Saturn's "Bandwidth & Max Conductor Length" tab and its via resonance/thermal outputs are the largest parity gaps.

### Ranked candidates (value ÷ effort)

| # | Feature | Formula source | Validation route | Effort | Value |
|---|---------|----------------|------------------|--------|-------|
| 1 | Inverse mode: width (or gap) for target Z0, all single-ended and differential topologies | Brent 1973 root finding over existing Z0(W); H-J/Cohn monotone in W | Round trip Z(W*) = Zt to 1e-9 Ω; bracket failure returns error; parity with KiCad "Synthesize" | M | H |
| 2 | Offset (asymmetric) single-ended stripline / dual stripline | Wadell 1991 §4.5 (Cohn 1955 half-space combination); IPC-2141A asymmetric eq. | Reduces to Cohn symmetric at H1 = H2; plane-swap symmetry; IPC-2141A example | S | H |
| 3 | Rise time ↔ bandwidth, knee frequency, critical length, max unterminated length | Johnson & Graham 1993 §1.1 (Fknee = 0.5/Tr, BW = 0.35/Tr); IPC-2251 (2003) | Exact algebra; Saturn help vector; Sierra calculator values | S | H |
| 4 | Conductor + dielectric loss (dB/in) with Hammerstad roughness, insertion-loss budget over length at Nyquist | Pucel-Massé-Hartwig 1968; Hammerstad-Jensen 1980 αc; Pozar 2012 eq. 3.199 (stripline αc, αd); Wheeler 1942 incremental-inductance rule; Hammerstad-Bekkadal 1975 Kr = 1 + (2/π)atan(1.4(Δ/δ)²) | KiCad/transcalc numeric values; Pozar Ex. 3.7; αd ∝ f; Kr → 1 (smooth), → 2 (saturation); Wheeler rule applied numerically to existing air-line Z0 as cross-check | M–L | H |
| 5 | Djordjevic-Sarkar wideband Debye εr(f), Df(f) from one Dk/Df point | Djordjevic et al., IEEE Trans. EMC 43(4), 2001; m1 = 4, m2 = 12 | Reproduces Dk/Df at f0; Dk monotone decreasing; ANSYS reference formula; feeds #4 and K-J dispersion | S | H |
| 6 | Physics-based NEXT/FEXT from Zeven/Zodd (replaces experimental crosstalk) | Hall-Hall-McCall 2000 §4.2; Bogatin 2010 ch. 10; Kb = (Ze−Zo)/(2(Ze+Zo)), Lsat = Tr/(2 tpd) | FEXT = 0 in homogeneous stripline (invariant); Kb → 0 for wide spacing; weak-coupling limit equals L/C-matrix definition | S–M | H |
| 7 | Via stub resonance and back-drill depth | f = c/(4 L √εr); Bogatin EDN Rule #17 (1.5 GHz·in in FR4); "first null ≥ 7× Nyquist" | Exact; Sierra max-stub-length tool; Saturn via resonant frequency vector | S | H |
| 8 | Via current capacity, via count for a current, via thermal resistance | Barrel area π(d·t − t²) as an internal conductor in IPC-2221A; IPC-2152 (2009) barrel guidance; R = L/(k·A) | Area identity: via with same area as an internal trace returns the same current; Saturn help p.36 thermal vector; KiCad Via Size | S | M–H |
| 9 | Plane-pair PDN: capacitance, sheet inductance (32 pH/mil), spreading inductance, cavity resonances f_mn | Bogatin & Smith 2017 ch. 7–8; Novak 2007; EDN Rule #16; Pozar cavity modes | 32 pH/mil check; C = ε0εrA/h exact; f_mn = c/(2√εr)·√((m/a)²+(n/b)²) | S | M–H |
| 10 | Decoupling capacitor SRF, inter-cap anti-resonance, |Z(f)| of N parallel RLC | Bogatin & Smith 2017 ch. 5; f_AR² = (C1+C2)/(4π²C1C2(L1+L2)) | Complex-arithmetic oracle; peak location vs. closed form; single-cap limit | S | M–H |
| 11 | Huray/Cannonball roughness from Rz (extends #4 past 5 GHz) | Huray et al. DesignCon 2010; Simonovich 2015/2016 (a = 0.5 µm, N = 45–85 per Polar AP8155) | K → 1 at low f; high-f limit 1 + (3/2)N·4πa²/A_tile; Polar/Simonovich tabulated points | S | M–H |
| 12 | Coax (with TE11 cutoff), twisted pair, wire over ground | Pozar 2012 §3.5 (Z = 60/√εr·ln(D/d)); Lefferson 1971, IEEE Trans. PHP-7 (εeff = 1 + q(εr−1), q = 0.25 + 0.0004θ², θ = atan(Tπd)); Ramo-Whinnery-Van Duzer (Z = 60/√εr·acosh(2h/d)) | KiCad TransLine values; air coax D/d = 2.30 gives 50 Ω; acosh large-h limit 60·ln(4h/d); twist rate 0 gives q = 0.25 | S | M |
| 13 | Power-trace voltage drop and power loss vs. width and temperature; voltage divider (listed as missing in STATUS.md) | Exact; IPC-2221A copper resistivity, α = 0.00393/K | Exact algebra; equals DC limit of the loss package (#4) | S | M |
| 14 | Reflection and termination: Γ, VSWR, return loss, series / parallel / Thevenin termination values | Pozar §2.3; Johnson & Graham 1993 ch. 6 | Exact; Sierra Circuits reflection calculator values; Γ = 0 at ZL = Z0 | S | M |
| 15 | Coplanar strips (CPS) and ungrounded CPW | Gupta-Garg-Bahl 1996 "Microstrip Lines and Slotlines"; Z_CPS = 120π·K(k)/K(k')/√εeff | Duality Z_CPS·Z_CPW = (60π)²/εeff on an infinite substrate; reuses existing AGM elliptic integrals | S | M |
| 16 | RF set: quarter-wave transformer (+ bandwidth for a given Γm), Wilkinson divider (equal/unequal), L-network, single-stub match | Pozar 2012 §5.1 (eq. 5.3-5.6), §5.2 (eq. 5.9-5.12), §5.5 (eq. 5.36), §7.3; Wilkinson 1960 IRE Trans. MTT | Complex-arithmetic oracle: Γ = 0 at f0; Wilkinson S-matrix (S21 = S31 = −j/√2, S23 = 0); Pozar Ex. 5.1-5.3 | S each | M |
| 17 | IPC-7351B land-pattern RSS equations with density-level (A/B/C) toe/heel/side goals | IPC-7351B (2010): Zmax = Lmin + 2JT + √(CL²+F²+P²), Gmin = Smax − 2JH − √(CS²+F²+P²), Xmax = Wmin + 2JS + √(CW²+F²+P²). Superseded by IPC-7352 (2023) | Worked examples in the standard; KiCad / PCB Libraries calculators. Caveat: the per-family J tables are copyrighted | M-L | M |
| 18 | Z-axis CTE expansion through Tg; annular-ring and aspect-ratio checks | Piecewise ΔL = L·(α1(Tg−T0) + α2(T−Tg)); IPC-2221A / IPC-6012 annular-ring minimums; aspect ratio = thickness / drill | Exact algebra; table cells | S | L-M |

Notes
- Items 1-3 are the clear first wave. They need no new physics, reuse the existing exact Cohn/AGM machinery, and close the most visible Saturn and KiCad parity gaps (every surveyed tool has synthesis; Saturn has a Bandwidth & Max Conductor Length tab).
- Items 4, 5 and 11 form one loss package. Do 5 first (small, exact algebra), then 4, then 11. The Wheeler incremental-inductance rule gives a topology-independent numerical cross-check of any closed-form conductor loss, which fits the validate-everything rule.
- Item 6 is the cheapest way to retire the only module that reproduces nothing. The stripline family already has exact even/odd modes.
- Already present, no action: propagation delay and L/C per unit length are fields of the impedance result type; skin depth exists in the current module.
- Deprioritised: real IPC-2152 charts (data copyrighted, published fits vendor-proprietary per Jouppi's IPC paper), IEC 60664-1 creepage tables (same licensing problem), differential GCPW (no citable closed form), rectangular waveguide, embedded resistors (trivial, low demand). Sierra's via and impedance tools are 2D field solvers and out of scope for closed forms.
- CLI-only win: a convert command (oz to µm, mil to mm, dB to V) mirrors Saturn's Conversion tab at near-zero cost.

Key sources
- Saturn help: https://saturnpcb.com/saturn-pcb-toolkit-help-htm/
- KiCad calculator: https://docs.kicad.org/9.0/en/pcb_calculator/pcb_calculator.html
- wcalc: https://wcalc.sourceforge.net/about.html
- Qucs transcalc: https://qucs.sourceforge.net/tech/technical.html
- Sierra tools: https://www.protoexpress.com/tools/ and https://www.protoexpress.com/tools/bandwidth-rise-time-and-critical-length-calculator/
- Polar AP8155 roughness: https://www.polarinstruments.com/support/si/AP8155.html
- Huray DesignCon 2010: https://www.oldfriend.url.tw/article/IEEE_paper/roughness/5_TA2_Paul_Huray.pdf
- Simonovich Cannonball-Huray: https://www.signalintegrityjournal.com/articles/15-practical-method-for-modeling-conductor-surface-roughness-using-close-packing-of-equal-spheres
- Djordjevic-Sarkar formula: https://ansyshelp.ansys.com/public/Views/Secured/Electronics/v251/en/Subsystems/Q3DExtractor/Content/Q3D/TheDjordjevicSarkarModel.htm
- EDN Rule #17 via stub: https://www.edn.com/the-quarter-wave-stub-frequency-rule-of-thumb-17/
- EDN Rule #16 sheet inductance: https://www.edn.com/sheet-inductance-of-a-cavity-rule-of-thumb-16/
- Lefferson 1971: https://ieeexplore.ieee.org/abstract/document/1136426
- Wheeler incremental inductance rule: https://en.wikipedia.org/wiki/Wheeler_incremental_inductance_rule
- IPC-7351B: https://webstore.ansi.org/standards/ipc/ipc7351b2010
- Jouppi on IPC-2152: https://www.electronics.org/system/files/technical_resource/E7&S22_03.pdf
- Altium spreading inductance: https://resources.altium.com/p/what-spreading-inductance

---

## 2. Machine-learning and data-driven extensions

### Bottom line

ML adds nothing for the single-ended lines. Hammerstad-Jensen is within 0.06% of the AWR TXLine field solver across 0.001<w/h<1000 (f4inx comparison), and Cohn stripline and zero-thickness CPW are exact conformal maps. The one published "neural net for microstrip Zo" paper (arXiv 2406.04357) trains on the Gupta-Garg closed forms and reaches 1.7% max error, i.e. strictly worse than calling the formula. The real gaps in STATUS.md are elsewhere: IPC-2141A coupled microstrip (closed-form differential models disagree by 10 to 25%, and IPC-2141 single-ended hits 44% error outside 0.1<w/h<2), finite-thickness and trapezoid corrections, asymmetric/embedded coupled lines, and IPC-2152, which has no data at all. The highest-value project is therefore a solver-generated benchmark set, not a model. A surrogate only earns its place afterwards, where residuals exceed the solver's own error bar.

### Ranked table

| # | Idea | Data source | Method | Expected gain | Effort | Risks |
|---|------|-------------|--------|---------------|--------|-------|
| 1 | 2-D quasi-static benchmark set for all coupled modes, finite T, embedded, asym | In-house Rust FD Laplace solver; atlc (FD, <0.3% typical) and MMTL (BEM, GPL) as independent oracles | Grid-converged C and C_air per geometry, Zeven/Zodd from (+,+)/(+,-) excitations | Turns 5 "compatibility" models into validated ones with quantified bounds | M | Solver must first be proven on exact cases (coax, Cohn stripline, Cohn 1955 coupled); open-boundary microstrip needs a box-size study |
| 2 | Replace IPC-2141A coupled microstrip with Hammerstad-Jensen 1980 coupled + Kirschning-Jansen 1984 dispersion (as Qucs does) | None (published closed forms) | Even/odd Zo <0.8% error for 0.1<=u<=10, g>=0.01; dispersion <=1.4% to fn=25 | 10-25% -> ~1-2% | S | Zero-thickness; use the Qucs thickness correction; K-J can give unphysical positive mutual C at extreme gaps |
| 3 | Residual-correction surrogate on top of #2 for thick copper, trapezoid etch, solder mask | Dataset from #1 | PySR expression first (shippable as f64 constants); fallback 2x16 tanh MLP, ~600 const weights, log-scaled inputs | Only where residual >2%; typically 1-3% | M | Extrapolation; reject outside the sampled hull with the existing validate helpers; SPI-2026 MLP paper only reached "a few %", worse than closed forms |
| 4 | IPC-2152 recalibration from published fits | Brooks/Adam "Trace Currents and Temperatures Revisited" (Eq. 3-2: dT = 215.3 I^2 W^-1.15 Th^-1.0 external; internal table per oz) + Olson HowTo2152 modifiers (copper weight, plane distance, board thickness, material) | Replace Saturn-reconstructed modifiers with published coefficient tables | Removes reverse-engineered guesswork; still "compatibility" | S-M | IPC-2152 raw charts are copyrighted, no open digitized dataset exists; Brooks concludes equations cannot capture plane/adjacent-trace effects |
| 5 | Thermal FEM dataset for current capacity | Elmer or FEniCS steady conduction + convective h | Fit plane-distance/board-thickness modifiers | Physics-based modifiers | L | Heat transfer coefficient calibration is the whole problem (Brooks tuned h to match IPC); no measurements to anchor to |
| 6 | Frequency-dependent Dk/Df (Djordjevic-Sarkar) from datasheets | Isola/Rogers datasheet Dk/Df tables | Two-parameter causal closed form (one Dk/Df point, m1=4, m2=12) or least squares over multi-frequency tables | Enables loss and dispersion per material | S | Datasheet test-method inconsistency (stripline vs SPP) |
| 7 | Fabricator-specific coupon corrections | None public; SPIL study used 2,635 measured samples (LightGBM, MAPE 2.5%, RMSE 2.8 ohm, R^2 0.79); 11,961-S-parameter SerDes study does not release data | Per-fab multiplicative correction table loaded from TOML, not a model | Capability only | S (blocked on data) | Would need a fab partner |
| 8 | Inverse design (solve W, S for Zo/Zdiff) | None | Bisection/Newton on monotone forward model; 2-D Newton for (W,S)->(Zdiff,Zcomm) | Complete feature in microseconds | S | None; ML never beats root finding when the forward model costs 1 us |
| 9 | MCP server over the CLI | None | stdio JSON-RPC wrapper over --json output (lineforge already does this in Python) | Agent access | S | Scope creep |
| 10 | Stackup anomaly detection | None | Rule-based checks (asymmetry, Dk vs prepreg table, tolerance) | Modest | S | ML has no training data here; skip ML |
| 11 | 3-D solvers (openEMS, MEEP, palace, Tidy3D) as training source | - | MSL port Z0 extraction | Cross-check only | - | Minutes per run, port-placement sensitivity; gprMax is irrelevant (ground radar) |

### Literature summary

- arXiv 2406.04357 (2024): ANN for microstrip Zo trained on Gupta-Garg formulas, w/h 1-9.5; 1.7% max error vs the formula. Cautionary example.
- IEEE SPI 2026, "ML-Based Surrogate Modeling of Capacitance Matrices in Multiconductor PCB Interconnects": MLP on quasi-static FDFD ground truth for coupled microstrip; few-% per-element error, 5.6% Frobenius. Closest analogue to idea #3, and evidence the gain is marginal.
- arXiv 2605.18170 (2026): buffer-parameterized SI surrogates, 44 inputs, eye height/width. Anisotropic GPR wins at low data, NNs at large data. Not applicable to Zo.
- arXiv 2106.10693: PDN impedance DNN trained on >1M BEM-generated boards, 0.1 s inference, 100x faster than BEM. Shows the data scale for layout-level problems, out of scope for a closed-form toolkit.
- Micromachines 14(2):265 (2023): tree models on 2,635 fabricator-measured single-ended and differential impedances (7 geometric inputs). Fab-specific correction is feasible only with fab data.
- arXiv 2301.10176: 11,961 measured 4-port S-parameters on manufacturing variability; dataset not released.
- Brooks and Adam, "Trace Currents and Temperatures Revisited": fits to IPC-2152 external/internal/vacuum curves, TRM simulation; IPC curves are worst case; internal traces run cooler than external.
- Kirschning and Jansen 1984 (IEEE MTT-32, 83-90) plus Qucs technical doc node77: accuracy and range statements for coupled microstrip.
- PySR (arXiv 2305.01582): symbolic regression producing compact expressions with fitted constants, ideal for a dependency-free Rust crate.

### Recommended first project: solver-backed coupled-line benchmark set

1. Write a 2-D finite-difference Laplace solver as a dev-only crate or tools/ binary (no deps): epsilon_r-weighted 5-point stencil, Dirichlet conductors, box boundary at least 10H laterally and above for microstrip, SOR then geometric multigrid. Compute C with dielectrics and C_air; Zo = 1/(c sqrt(C C_air)), eps_eff = C/C_air. Pairs: excite (+1,+1) and (+1,-1) for Zeven, Zodd; Zdiff = 2 Zodd. Trapezoid and solder mask are just pixel masks.
2. Qualify the solver on exact cases already in the toolkit: coax, parallel plate, zero-thickness Cohn stripline, Cohn 1955 coupled stripline. Richardson-extrapolate over h, 2h, 4h grids. Accept only when <0.5% on all of them. Run atlc on ~20 of the same bitmaps as an independent oracle.
3. Sweep: Latin hypercube over W/H 0.1-10, S/H 0.1-5, epsilon_r 2-12, T/H 0-0.3, optional etch angle and mask thickness. About 500 coupled microstrip, 300 embedded, 300 coupled stripline with finite T. A 400x200 multigrid solve in Rust should take well under a second, so the full set is minutes to an hour. Commit CSVs to tests/data with the generator, grid parameters, and git hash, and add regression tests asserting each closed form within its stated bound.
4. Publish error tables per model and region in STATUS.md; tighten ranges and statuses from the data.
5. Implement #2 (H-J coupled + K-J dispersion) and re-evaluate. Expect residuals of 1-2%.
6. Only where residuals stay above 2% in regions users hit (2 oz copper, tight gaps), fit a correction: PySR first, then a 2x16 tanh MLP with const f64 weight arrays. Hold out 20%, report max absolute error, state the validity range as the sampled box and reject outside it. Do not ship any correction whose claimed gain is inside the solver's own error bar.
7. Cross-check ~10 points with openEMS MSL-port extraction for independence of method.

### Key URLs

- lineforge (MCP, FD solver, validated +-2% vs openEMS): https://github.com/RFingAdam/lineforge
- atlc: https://atlc.sourceforge.net/ and FAQ https://atlc.sourceforge.net/FAQ.html
- MMTL/TNT BEM solver: https://github.com/corecode/mmtl-tnt
- Qucs coupled microstrip formulas and accuracy: https://qucs.github.io/tech/node77.html
- Microstrip formula comparison vs TXLine: https://f4inx.github.io/posts/microstrip-formulas-comparison.html
- Brooks/Adam IPC-2152 paper: https://www.mathscinotes.com/wp-content/uploads/2016/06/pcbtempr.pdf
- Olson HowTo2152: https://www.frontdoor.biz/PCBportal/HowTo2152.pdf
- Capacitance-matrix MLP surrogate (SPI 2026): https://eurekamag.com/research/108/966/108966470.php
- Microstrip ANN paper: https://arxiv.org/abs/2406.04357
- SI surrogate benchmark (GPR vs NN): https://arxiv.org/abs/2605.18170
- PDN DNN: https://arxiv.org/abs/2106.10693
- Fab-measured impedance tree models: https://pmc.ncbi.nlm.nih.gov/articles/PMC9960110/
- Manufacturing variability S-parameters: https://arxiv.org/abs/2301.10176
- PySR: https://arxiv.org/abs/2305.01582
- openEMS MSL discussion: https://github.com/thliebig/openEMS-Project/discussions/426
- Djordjevic-Sarkar identification (Shlepnev): https://www.simberian.com/AppNotes/Shlepnev_PCB_DesignMagazine_Feb2014.pdf

---

## 3. Integrations and ecosystem directions

### Findings that changed the ranking

- KiCad 10 shipped 2026-03-20 with Tuning Profiles. Width/gap are auto-calculated from the board stackup and a target impedance, profiles attach to netclasses, and DRC can flag tracks that do not match the profile. "Replace KiCad's calculator" and "DRC impedance check" are therefore mostly built in now. The remaining niche is an independent second opinion with model-status metadata and fabricator presets. Sources: https://techexplorations.com/kicad/kicad-10-review-new-features-high-speed-tuning-variants-and-more/ , https://docs.kicad.org/10.0/en/pcbnew/pcbnew.html
- The KiCad IPC API exposes the full stackup. `BoardStackupLayer` carries thickness, material name, type, and for dielectrics a list of sublayers with `epsilon_r`, `loss_tangent`, `spec_frequency`, `dielectric_model`. Compiled (non-Python) plugins are supported via the "executable" runtime, but talking to KiCad needs protobuf over nng. Sources: https://gitlab.com/kicad/code/kicad/-/raw/master/api/proto/board/board.proto , https://dev-docs.kicad.org/en/apis-and-binding/ipc-api/for-addon-developers/index.html , https://pypi.org/project/kicad-python/
- Prior art for MCP + Python exists: lineforge (Python, FastMCP, 14 tools, AGPLv3, 1 star). It exposes no validity/status metadata to agents. Differentiator for us: validated `ModelStatus` per result and a single Rust binary. https://github.com/RFingAdam/lineforge
- Demand signal: a KiCad forum thread shows KiCad vs JLCPCB widths differing 19% on an outer layer and 6% on an inner layer, with no one able to say which is right. https://forum.kicad.info/t/jlcpcb-impedance-calculator-vs-kicad-calculation/69673
- Library gap: no solve-for-width exists anywhere in `crates/pcb-toolkit/src` (no bisection/root finder). It is a prerequisite for stackup solving and any KiCad "50 Ω width" feature. Zo(W) is monotonic for every model, so a bracketed bisection is a small addition.
- PyPI name `pcb-toolkit` is free (404). rmcp 3.4.0 was released 2026-09-15.

### Ranked table

| # | Direction | Effort | Value | Key dependency / tooling | Main risk |
|---|-----------|--------|-------|--------------------------|-----------|
| 1 | MCP server (`pcb-toolkit-mcp`) | S | H | rmcp 3.4.0: `#[tool_router]`/`#[tool_handler]`, schemars, `Json<T>` structured content, ToolAnnotations. https://docs.rs/rmcp | rmcp API churn; lineforge overlap |
| 2 | Stackup TOML + fabricator presets + solve-for-width + stackup report | M | H | JLCPCB impedance-template web API (562 stackups scraped by https://github.com/gsuberland/jlcpcb_autogenerated_stackups ); OSH Park FR408 docs; PCBWay stackup page; kiutils_rs / kiparse for `.kicad_pcb` import | Provenance and staleness of fabricator numbers; JLCPCB terms on raw data |
| 3 | Validation harness + accuracy matrix | M | H | scikit-rf `MLine` (H-J + K-J defaults, Djordjevic-Svensson loss), KiCad `pcb_calculator/transline/*.cpp` (GPL, compare via subprocess, do not link), atlc FD solver (CLI, bitmap input), Sierra Circuits free 2D solver (manual sampling) | No free scriptable 2D solver beyond atlc; atlc grid error 1-2% |
| 4 | Python bindings (PyO3 + maturin, abi3 wheels) | S-M | M/H | maturin, pyo3 abi3; serde structs to dicts. Interop: feed Zo/εeff into `skrf.media.DefinedGammaZ0`. https://www.maturin.rs/tutorial.html | Wheel matrix maintenance |
| 5a | KiCad plugin: pull stackup via IPC API, compute all layers' widths, compare against KiCad's tuning profile and fabricator preset | M | M | kicad-python (needs #4) or Rust with prost + nng | KiCad 10 natively covers the basic case; kicad-python is 0.x |
| 5b | Standalone DRC-like check over a `.kicad_pcb` | L | M | kiutils_rs / kiparse | KiCad 10 DRC already flags profile mismatches |
| 6 | WASM browser calculator | S (bindings) / M (UI) | M | wasm-bindgen + serde-wasm-bindgen; opt-level z + lto + wasm-opt, roughly 100-200 KB. Competitors: pcbviewer.app, miniwebtool, JLCPCB, Sierra | Crowded; differentiation only via presets and status display |
| 7 | Batch/sweep (CSV/JSON in, JSON out) | S | M | clap + serde_json; fold into #2 | Low standalone value |
| 8 | Distribution: cargo-dist (Homebrew tap, shell installers, binstall metadata) | S | M | https://crates.io/crates/cargo-dist | None significant |
| 9 | mdBook site from `docs/notes` + docs.rs | S | M-L | mdBook, GitHub Pages | Maintenance |
| 10 | TUI (ratatui) | M | L | ratatui | Little over CLI `--json` |
| 11 | VS Code extension | L | L | wasm or CLI shell-out | Tiny audience |
| 12 | Altium / Cadence scripting | L | L | Altium ships the Simbeor field solver in Layer Stack Manager; only `.stackupx` import is useful | No user need |

Stackup formats surveyed: KiCad `(stackup ...)` in the board file (per-dielectric sublayers with epsilon_r, loss_tangent); Altium `.stackupx` XML; IPC-2581 `<Stackup>` section (KiCad's exporter omits it, gitlab issue 16665); ODB++ `matrix/stackup.xml` (spec 8.1, https://odbplusplus.com/wp-content/uploads/sites/2/2024/08/odb_spec_user.pdf ). Recommendation: own TOML schema plus KiCad and Altium importers.

### Recommended next projects

**A. `pcb-toolkit-mcp` crate (1-2 days).**
1. New workspace crate depending on rmcp 3.4 with `server`, `transport-io`, `schemars` features; tokio rt + io-std.
2. One `#[tool]` per calculator, inputs as schemars-derived structs mirroring the library `*Input` types.
3. Return `Json<T>` structured content where `T` embeds a `model: {name, status, reference, validity}` object taken from each `MODEL` constant, plus a text block ending with `ModelInfo::caveat()`. Put the status word in the tool description ("experimental: estimate only") and set `readOnlyHint = true`, `idempotentHint = true`. Add a `list_models` tool and an MCP resource `models://status` so agents can query validity ranges up front.
4. Reject out-of-range inputs with the existing `CalcError` text so the agent sees the domain, never a clamped number.

**B. Stackup subsystem in the core crate (1-2 weeks).**
1. Add `solve::width_for_impedance(f, target, lo, hi)` bracketed bisection with domain checks, then wrap for microstrip, stripline, embedded, coplanar and the differential family.
2. Define a `Stackup` type and TOML schema (layers, copper weights, dielectric sublayers with Dk/Df/frequency, provenance string). Add `pcb-toolkit stackup report board.toml --z0 50 --zdiff 100` producing widths/gaps per signal layer with model status per row.
3. Presets: JLCPCB JLC04161H-7628/3313/1080 and 6-layer set from https://jlcpcb.com/impedance , OSH Park 4-layer FR408 (7.87 mil prepreg) from https://docs.oshpark.com/services/four-layer/ , PCBWay 4-layer from https://www.pcbway.com/multi-layer-laminated-structure.html , with source URL and fetch date in each entry. Eurocircuits (975+ buildups) only via their Buildup Editor, so document rather than embed.
4. Importers: KiCad stackup via kiutils_rs ( https://docs.rs/kiutils_rs ) and Altium `.stackupx`. This also gives the CSV/JSON sweep for free.

**C. Validation harness repo `pcb-toolkit-bench` (1 week, Python + Rust).**
1. Generate a geometry grid per calculator from each `MODEL` validity range.
2. Oracles: scikit-rf `skrf.media.MLine` and `CPW`, KiCad transline compiled from https://github.com/KiCad/kicad-source-mirror/tree/master/pcb_calculator/transline as a tiny CLI, atlc ( https://atlc.sourceforge.net/ ) as the field-solver reference for stripline and microstrip, Saturn vectors already in repo.
3. Emit a Markdown accuracy matrix (max/mean error per model per oracle) and paste into STATUS.md; upgrade "compatibility" entries to "validated" only where atlc agrees within the stated tolerance.

Do A first (smallest, most visible, and it makes the status metadata a product feature), B second (enables the KiCad plugin later and answers the forum-thread question), C alongside B since B's presets need C's numbers to be trusted. Defer 5a until B exists and kicad-python reaches 1.0.

---

## 4. Can the sinbad ecosystem be the 2-D field solver? (survey, 2026-09-17)

Verdict: no, not as a dependency. Write the standalone finite-difference/finite-volume Laplace solver inside pcb-toolkit (option b). Keep the ecosystem as a possible future independent cross-check via the FEniCSx oracle.

### Decisive evidence (verified in the gate logs)

- `gates/2026-09-08-w8-final/elasticity-release.log:117`: the 3-D elasticity reference test, built `--release --locked`, on 2³ then 4³ Kuhn bricks (~1,100 DOFs), finished in **1924.49 s**; cold release build 6 m 53 s. `elasticity-perf-summary.json` attributes the time to `malleus::interpreter::eval` (14.2% self) and `::offset` (8.5%): element kernels run through a tree-walking IR interpreter with no codegen backend.
- `gates/2026-09-08-w8-final/sinbad-first-full.log:75,77`: the 2-D `electrostatics_runs_end_to_end_with_expected_order` and `poisson_runs_end_to_end_with_expected_order` tests each exceed 60 s on a 64×64 unit square (~4,200 vertices). A benchmark sweep needs hundreds to thousands of solves at seconds each.

### What exists

1. Roles: Quantitas = units; Resolvent = exact-algebra CAS; Scientia = `.res` language and semantic compiler; Malleus = local kernel compiler (interpreter only, no meshes); Finitum = meshes, elements, DOFs, constraints, operators; Methodus = solvers; Krasis = coupled stateful runtime; Sinbad = product/case layer; Artifactum = evidence store; Solverang = geometric constraint engine (not applicable).
2. Electrostatics already exists: `sinbad/physics/corpus/06-electrostatics.res` solves `-div(ε∇φ) = ρ` with Dirichlet conductors and declares the energy observable `integrate(0.5·dot(E, D))`; case `sinbad/cases/06-electrostatics.toml`. Piecewise-constant coefficients via `BindingModel::Piecewise` (`sinbad/src/case.rs:186`) and `CoefficientLayout::Cell` (`finitum/src/realization.rs:50`).
3. Meshing is the API gap: Finitum offers only `MeshProfile::SimplexBox`, `CadRectangle`, `CadFamily` (`finitum/src/profile.rs:92`) and global `refine_uniform`; no layered/embedded-conductor mesher, no trapezoids, no local refinement algorithm (only a `HangingNodeConstraint` primitive). Dirichlet is gathered from exterior `facet_regions` only (`profile.rs:861`), so interior conductors need hand-built `ConstraintSet::new` rows and a hand-built `Mesh::new`, bypassing Sinbad's case/receipt layer (the part worth reusing).
4. Finitum elements/functionals: P1/P2 Lagrange, P0, RT0; `CellFunctionalPlan` energy integration (`finitum/src/functional.rs:914`); `FieldSampler` for point values/gradients; boundary flux via exterior-facet centroid rule only.
5. Methodus: CG, MINRES, GMRES, BiCGSTAB, Newton-Krylov, BDF, adjoints; only block-structure preconditioners, no Jacobi/IC/multigrid; no size or timing claims in tests.
6. Case execution: TOML `sinbad-case/2`, `compile_case`/`run_plan`; CLI at `sinbad/src/main.rs`. Dependency closure: Finitum 51 crates (blake3 with `cc`, dashu, sha2, cadabra geometry); full Sinbad 141 crates incl. bundled SQLite. All inter-repo deps are path deps on unpublished siblings.
7. `sinbad-oracle-fenicsx`: independent dolfinx recomputation under Docker; capabilities poisson, nonlinear_heat, linear_elasticity, stokes, mixed_darcy; no electrostatics yet, additive to add (`registry.py:23,54,183`).
8. Blockers: every consuming repo is ahead of origin (sinbad +4, finitum +3, malleus +2, scientia +1, krasis +1); W8 gates G2/G3 not closed; first full run was 166 passed / 25 failed. Licences MIT OR Apache-2.0 (methodus, solverang Apache-2.0 only). MSRV 1.93 for scientia/malleus/resolvent/sinbad vs 1.85 for pcb-toolkit.

### Effort

- (a) Reuse Finitum directly (bypassing Sinbad): layered graded mesher with thin trapezoids, hanging-node grading, hand-built interior Dirichlet, Neumann box, scalar preconditioner; 2–3 weeks; still 4–6 orders of magnitude too slow; inherits unpublished-heads risk.
- (b) Standalone solver in pcb-toolkit: five-point finite-volume Laplace on a graded tensor mesh, harmonic-mean face permittivity at interfaces, Dirichlet by row elimination on conductor cells, Neumann ghost faces, capacitance from energy with flux-contour cross-check, Richardson extrapolation over 2–3 levels; 300–500 lines, no deps, milliseconds per solve; 2–4 days including test vectors. **Recommended.**
- Later: add an electrostatic-capacitance capability to `sinbad-oracle-fenicsx` as an independent oracle (reaches FEniCSx directly, skipping the interpreter).
