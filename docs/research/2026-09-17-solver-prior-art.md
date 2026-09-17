# Prior art: 2-D quasi-static field solvers for transmission lines (2026-09-17)

Web-research report, unedited. Companion to `2026-09-17-roadmap-research.md` §4.

## 1. Comparison table

| Tool / method | Discretization | Open boundary | C extraction | Thin / trapezoid conductors | Accuracy evidence | Speed | Licence, language |
|---|---|---|---|---|---|---|---|
| atlc [1][2][3][4] | 5-pt FD on bitmap pixels, SOR r=1.95 (only in the single-dielectric pass), neighbour-εr weighted stencil `V=ΣV_n·εr_n/Σεr_n` | Closed box only; FAQ: "not very accurate" for open structures | Energy per pixel `0.5·ε0·εr_pixel·(Ex²+Ey²)`; L from vacuum solve; Zo=√(L/C); 4 solves for couplers | Staircase; rule "≥25 pixels per critical dimension"; 5-px conductor gave 3.5% error | 48 tests: typical 0.1–0.3%, max 3.05%; coax mean 0.017%; dual-dielectric coax 0.66–1.68%; coupled lines biased low (−0.02 to −1.2%) | Seconds–minutes; stops when relative change in C < 1e-4 | GPL, C |
| atlc2 [5] | Same FD plus "charge-shift" E-field prediction (~20 iterations) | Grid up to 3200², conductors kept in central 1600²; capacitance "missed" outside grid | Energy sum | Corner pixels ≥16 px apart for 5% Rs | 1000–3000 conductor pixels → ~1% | Rs/L cubic in conductor pixels (100 min → 1.2 min on 12 threads) | Freeware, Windows |
| lineforge [6][7][8][9] | 5-pt FD, arithmetic-mean εr at cell edges, red–black SOR ω=1.9 (max ΔV<1e-7 every 50 sweeps) or PyAMG above 250k free pixels | Uniform padding to 16× size or 3200 px, Dirichlet V=0, justified by 1/r² decay; no grading | Energy `C=ε0/V²∫εr|E|²dA` | Bitmap staircase; floating conductors by averaging | ±2% claim is for its closed forms vs openEMS (microstrip ±1.6% with MSLPort; stripline 57.75 vs 56.55 Ω, −2.1%); FD-vs-atlc validation is open issue #10 | Python/NumPy; Rust kernel only scaffolded | GitHub, Python |
| RF2DFieldSolver [10] | FD Laplace (gpollo/laplace lib) | Not documented | Gauss-integral charge, two solves (εr and air) | Not addressed | ~2–4% vs formulas (55.5 vs 54.5 Ω) | Not stated | GPL-3, C++/Qt |
| MMTL / TNT nmmtl [11][12][13] | BEM/MoM, quasi-TEM, segments on conductors and dielectric interfaces, dense LINPACK solve | Bottom ground plane mandatory (perfect, images); top plane optional; interfaces truncated laterally | C and L matrices from charge; asymmetry ratio of matrices used as convergence proxy | Rectangles, polygons (trapezoids), circles natively; "Cseg/Dseg ~100 not recommended" | "Accurate to about 5 GHz" for PCB; no numbers | Sub-second (dense N≈10²–10³) | GPL-2, C++/Fortran/Tcl |
| Edge-enforced BEM (Pan, Wang, Gilbert 1992) [14] | BEM with singular edge elements, Taylor-regularized Green's kernels | Layered Green's function | Charge | Edge singularity built into basis | Converged with 160 conductor + 190 interface subsections | Fast | Paper |
| FasterCap [15][16] | 2-D and 3-D BEM with automatic adaptive refinement to a preset error | Free space or dielectric interfaces meshed | Charge | Panels; auto-refined | Error-controlled convergence; >1M panels | Fast | LGPL-2.1, C++ |
| Bryant–Weiss 1968 / MSTRIP2 [17][18] | MoM with "bound charge" Green's function on substrate surface; even/odd for coupled pairs | Exact (open) | Charge | Zero thickness | The historical reference for coupled microstrip | Fast | Paper; DTIC report |
| Yamashita–Mittra 1968 [19][20] | Variational in Fourier domain with trial charge | Exact | 1/C stationary; lower bound on C | Zero thickness | Bounds: 2W/Φ² ≥ C ≥ Q²/(2W) | Analytic | Paper |
| FEMM [21][22][23] | 2-D FEM, Lua | Kelvin transformation (exterior disk + periodic BC, exact) or IABC shells with Dirichlet/Neumann outer edge; "accuracy rivals Kelvin" | Energy | Mesh follows geometry; corners need local refinement | Qualitative only | Seconds | Free (Aladdin), Windows |
| Elmer StatElecSolve [24][25] | FEM, "Calculate Capacitance Matrix" via body indices | Box | Charge/energy | Mesh follows geometry | Two-ball C12 1.6933 vs 1.691 tutorial | Seconds | GPL |
| FEATool / QuickField shielded microstrip [26][27] | FEM (P2 in FEATool) | Grounded box | Both energy `C=2W/U²` and charge | Interior boundary strip | FEATool both within 0.1% of 178.1 pF; QuickField charge 178.36, energy 177.19 vs 178.074 (+0.16% / −0.5%) | Seconds | Commercial |
| Polar Si8000m/Si9000e [28][29] | BEM (Si6000: MoM + Green's function) | Layered | RLGC | Trapezoid, resin-rich zones | No published error; "consistent, comparable" between its own methods | Fast | Commercial |
| Ansys 2D Extractor [30][31] | FEM, adaptive passes on "delta energy" (0.001% used in studies) | Box | Energy | Adaptive | SI Journal: delta-energy does not map directly to Zo error | Seconds | Commercial |
| Sonnet / Simbeor / HyperLynx [32][33][34] | Sonnet: FFT MoM on uniform grid (3-D planar, not a line solver); Simbeor: quasi-static SFS + method of lines + Trefftz FE; HyperLynx full-wave: accelerated BEM | Sonnet box: sidewalls ≥3–5 substrate thicknesses | — | — | PCD&F 2015: ADS, HyperLynx, Si9000e, Sonnet, Simbeor agree within ±0.6% (microstrip) and ±0.8% (stripline) on 5/5 mil pairs | — | Commercial |
| Rust crates [35]–[41] | No 2-D electrostatic/line solver crate exists (only pcb-toolkit and lineforge's empty kernel). Sparse LA: faer 0.24.4 (MIT, pure-Rust sparse LLT/LDLT/LBLT, LU, QR, AMD/COLAMD, MSRV 1.84, ~13 direct deps, 460k downloads/month); nalgebra-sparse 0.12 (Apache-2.0, "early but usable", no solvers); sprs (Apache/MIT, Cholesky needs LGPL opt-in, API WIP); rsparse 1.2.1 (MIT, zero deps, CSparse port, ~760 downloads/month); russell_sparse (MIT wrapper over UMFPACK/MUMPS/OpenBLAS system libs); scirs2-sparse 0.6.5 (Apache-2.0, CG/GMRES + IC/ILU + smoothed-aggregation AMG, no direct factorization) | | | | | | |

## 2. Lessons a new solver must apply

1. Snap mesh lines to every conductor face and dielectric interface; atlc's staircase gives 3.5% at 5 px and needs ≥25 px per feature, and its coupled-line bias grows as gaps shrink [2].
2. Use face (harmonic-mean) permittivity in a conservative finite-volume stencil, not atlc's neighbour-pixel weighting or lineforge's arithmetic mean; atlc's multi-dielectric error is 3–8× its single-dielectric error [2][3][7].
3. Compute C twice, from energy and from a Gauss-law flux contour, and treat the spread as the discretization error; QuickField's two estimators straddle the exact value by +0.16% and −0.5% [27]; Sadiku's FD chapter uses the flux contour [42].
4. Converge on the linear residual, not on the change in C between sweeps; atlc's stop rule (ΔC/C < 1e-4) can stall SOR before the field is converged [1][4].
5. Solve εr and air on the identical mesh so geometric error partially cancels in Zo=1/(c√(C·C_air)); every FD tool (atlc, RF2DFieldSolver, lineforge) does this [4][10].
6. Grade the mesh toward conductor edges: the rectangular-corner potential behaves as r^(2/3) and a zero-thickness edge as r^(1/2), so uniform grids converge at reduced order while graded meshes restore it [43][44]; Pan et al. needed singular edge elements for BEM [14].
7. Richardson-extrapolate with the measured order, not assumed h²; Duncan 1967 quantified FD field errors at right-angle corners and plate edges [45].
8. For the open top, either stretch coordinates (arctangent map of the unbounded domain, FD + SOR, matched Wheeler) [46], mimic FEMM's Kelvin/asymptotic shells [21][22], or run Dirichlet and Neumann outer boundaries and bracket; the FEMM builder exposes exactly that choice [21].
9. Do not trust a fixed padding rule: FEM shows εeff shifts 10.5% (w/h=1) as side walls approach and up to 36% for a low cover; the RF "4h" cover rule and Sonnet's "3–5 h" sidewall rule are 1%-class, not 0.1% [47][48][49].
10. Report an asymmetry check on the capacitance matrix (C12 vs C21) for coupled lines as MMTL does; it is a free consistency indicator [11].
11. Adaptive energy-change criteria (Ansys 0.001% ΔW) do not bound Zo error; publish Zo convergence directly [31].
12. Keep a BEM oracle (FasterCap 2-D, error-controlled, LGPL) alongside atlc; commercial solvers agree with each other to ±0.8%, so that is the bar for "independent agreement" [15][34].

## 3. Pitfalls

- Corner singularities: conforming FEM/FD on uniform grids converge at about order 1/2 in the energy norm at edge singularities; the capacitance error is roughly h^(4/3) at 90° corners and h^1 at knife edges, so a plain h² Richardson step over-corrects [43][44][45].
- Staircase on trapezoids: bitmap methods shift the effective edge by ±h/2, which is why atlc under-reads Zo for tight couplers [2]; use area-fraction εr or sub-cell grading, never pixels.
- Box-size sensitivity: fields of a strip over a plane decay as a dipole (1/r²); a Dirichlet wall adds C, a Neumann wall removes it; lineforge's 16× uniform padding costs 3200² cells for a few percent [8][47].
- Energy vs flux disagreement is expected and useful; if both agree to 0.1% but disagree with the exact coax or Cohn stripline, the geometry mapping is wrong, not the solver.
- False convergence: atlc's observable-based stop and SOR's slow low-frequency modes; lineforge checks ΔV only every 50 sweeps [4][7].
- Interface-pixel energy: atlc multiplies pixel energy by the pixel's own εr, so interface cells are misweighted [3].
- Open-structure claims: atlc's own FAQ disclaims open lines; lineforge's ±2% is a closed-form-vs-openEMS number with a 7 Ω FDTD z-mesh offset, not an FD-solver validation [1][9].
- Dense BEM: MMTL's LINPACK solve is O(N³) and its top boundary is a mandatory ground plane at infinity or a real plane; dielectric interfaces must be truncated laterally [11][13].

## 4. Recommendation

Pick the conservative finite-volume Laplace solver on a graded tensor-product mesh (matches roadmap §4 option b): mesh lines on all faces and interfaces, geometric grading toward edges (ratio ~1.15) and outward to ≥50 H with coordinate stretching or Dirichlet/Neumann bracketing, harmonic-mean face εr, trapezoids via area-weighted εr in the cut cells plus local grading, Dirichlet by elimination, PCG with SSOR or IC(0) (a 200×400 mesh solves in tens of ms; geometric multigrid later if sweeps need it), energy and flux estimators, three-level Richardson using the measured order. This meets pure-f64/no-deps, handles layered dielectrics and finite thickness natively, and is what every 0.1–0.5% quasi-static reference (FEATool, QuickField, Ansys) actually does with FEM.

Fallback: a Bryant–Weiss/MMTL-style 2-D BEM with pulse basis on conductor and interface segments, ground-plane images, edge-graded segments, dense LU (N≈300–800, milliseconds). It gives the open boundary exactly and thin conductors natively, and is worth writing anyway as the in-repo independent cross-check, with FasterCap 2-D and atlc as external oracles.

## Sources

[1] https://atlc.sourceforge.net/FAQ.html
[2] https://atlc.sourceforge.net/accuracy.html
[3] https://sources.debian.org/data/main/a/atlc/4.6.1-5/src/find_energy_per_metre.c and https://sources.debian.org/data/main/a/atlc/4.6.1-5/src/update_voltage_array.c
[4] https://sources.debian.org/data/main/a/atlc/4.6.1-5/src/do_fd_calculation.c and https://manpages.ubuntu.com/manpages/resolute/man1/atlc.1.html
[5] http://www.hdtvprimer.com/kq6qv/atlc2.html
[6] https://github.com/RFingAdam/lineforge
[7] https://raw.githubusercontent.com/RFingAdam/lineforge/main/src/lineforge/solvers/laplace.py
[8] https://raw.githubusercontent.com/RFingAdam/lineforge/main/src/lineforge/solvers/extension.py
[9] https://github.com/RFingAdam/lineforge/tree/main/examples/09_l3_sig1_em_validation
[10] https://github.com/jankae/RF2DFieldSolver
[11] https://mmtl.sourceforge.net/user-guide/node8.html
[12] https://github.com/corecode/mmtl-tnt
[13] https://github.com/corecode/mmtl-tnt/tree/master/bem/src
[14] https://www.researchgate.net/publication/3322230_Edge_Effect_Enforced_Boundary_Element_Analysis_of_Multilayered_Transmission_Lines
[15] https://github.com/ediloren/FasterCap
[16] https://www.fastfieldsolvers.com/fastercap.htm
[17] https://www.semanticscholar.org/paper/d5a0ff2cacd2e148eedd284b9582f460acab3859
[18] https://apps.dtic.mil/sti/tr/pdf/ADA117364.pdf
[19] https://ieeexplore.ieee.org/document/1126658/
[20] https://www.jpier.org/ac_api/download.php?id=0502042
[21] http://www.femm.info/wiki/openboundaryexample
[22] https://www.femm.info/Archives/doc/manual42.pdf
[23] https://www.researchgate.net/publication/260330258_Improvised_Open_Boundary_Conditions_for_Magnetic_Finite_Elements
[24] https://github.com/tehnick/elmerfem/blob/master/fem/src/modules/StatElecSolve.src
[25] https://wiki.freecad.org/FEM_Example_Capacitance_Two_Balls
[26] https://www.featool.com/doc/Electromagnetics_06_microstrip_capacitance1
[27] https://quickfield.com/advanced/elec1.htm
[28] https://www.polarinstruments.com/help/si9000/si9000transmissionlinefieldsolver.htm
[29] https://www.polarinstruments.com/support/si/AP8161.html
[30] https://www.ansys.com/Products/Electronics/Option-2D-Extractor-Solver
[31] https://www.signalintegrityjournal.com/articles/3438-assessing-the-accuracy-of-em-simulation-tools
[32] https://www.sonnetsoftware.com/products/sonnet-suites/how-em-works.html and https://www.sonnetsoftware.com/support/help-18/getting_started/SonnetTutorial.html
[33] https://www.simberian.com/Presentations/SimbeorOverview_Dec2019.pdf
[34] https://www.pcdandf.com/pcdesign/index.php/magazine/10472-simulation-1512
[35] https://docs.rs/faer/latest/faer/sparse/linalg/index.html and https://lib.rs/crates/faer
[36] https://lib.rs/crates/nalgebra-sparse
[37] https://github.com/sparsemat/sprs
[38] https://lib.rs/crates/rsparse
[39] https://github.com/cpmech/russell
[40] https://lib.rs/crates/scirs2-sparse
[41] https://crates.io/crates/kryst
[42] M. N. O. Sadiku, Numerical Techniques in Electromagnetics, FD chapter (shielded microstrip): https://archive.org/details/numericaltechniq0000sadi ; FD line course: https://empossible.net/wp-content/uploads/2018/03/Lecture-Transmission-Line-Analysis.pdf
[43] https://www10.cs.fau.de/publications/dissertations/Diss_2018-Behairy.pdf
[44] https://www.sciencedirect.com/science/article/abs/pii/0020768373900826
[45] J. W. Duncan, "The Accuracy of Finite-Difference Solutions of Laplace's Equation," IEEE MTT-15, 575–582, 1967: https://ieeexplore.ieee.org/document/1126537/
[46] https://ieeexplore.ieee.org/document/681131
[47] https://www.academia.edu/105823527/Analysis_of_shielding_effects_on_the_microstrip_effective_dielectric_constant_by_the_finite_element_method
[48] https://www.edn.com/shields-are-your-friend-except-when-part-3/
[49] https://www.sonnetsoftware.com/support/help-18/cadence/SonnetBox_cadence.html
Also: H. E. Green 1965 FD origin https://ieeexplore.ieee.org/abstract/document/1126063/ ; Itoh–Mittra SDA 1973 and Kobayashi–Ando 1987 https://ieeexplore.ieee.org/document/1133610 ; Farrar–Adams MoM https://ieeexplore.ieee.org/document/1127556/ ; Kowalski–Pregla AEÜ 25 (1971) 193–196 via https://link.springer.com/article/10.1007/BF02998749
