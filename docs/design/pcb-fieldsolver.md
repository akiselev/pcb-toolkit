# pcb-fieldsolver: 2-D quasi-static field solver — consolidated design

Status: proposal, 2026-09-17. Not yet implemented.
Inputs: `docs/research/2026-09-17-solver-prior-art.md`, `docs/research/2026-09-17-solver-numerics.md`
(with the experiment `docs/research/scripts/fv_check.py`), `docs/research/2026-09-17-solver-architecture.md`,
and `docs/research/2026-09-17-roadmap-research.md` §2 and §4 (why not ML, why not the sinbad ecosystem).

## 1. Purpose and requirements

Produce benchmark per-unit-length capacitances for PCB transmission-line cross-sections so that
pcb-toolkit's closed-form calculators can be validated, bounded and (where warranted) corrected:
layered piecewise-constant permittivity, one or two thin rectangular or trapezoidal conductors,
ground planes, closed box (stripline) or open top (microstrip), optional planar or conformal solder
mask. Outputs per case: C and C_air matrices, Zo = 1/(c√(C·C_air)), εeff = C/C_air, Zeven/Zodd/Zdiff,
and a reported uncertainty. Targets: 0.1–0.5% demonstrated by refinement, ≤100 ms per 400×200 solve
in release builds, deterministic results, no dependencies beyond `thiserror` and `serde`.

## 2. Decisions

| Topic | Decision | Rejected alternatives and why |
|---|---|---|
| Where it lives | New publishable workspace crate `crates/pcb-fieldsolver`; never depends on pcb-toolkit. Non-published `crates/pcb-bench` depends on both and writes golden CSVs; pcb-toolkit tests read the CSVs with a std-only parser. | Feature inside pcb-toolkit (dilutes the closed-form crate, cannot self-test); separate repo (goldens, solver version and toolkit tests must change atomically); sinbad ecosystem (interpreted kernels: 1924 s release-mode test at ~1,100 DOFs, unpublished path deps, MSRV 1.93). |
| Discretization | Vertex-centred five-point finite volume on a graded tensor grid; permittivity constant per cell; every interface, wall, conductor face and edge on a grid line. Identical to P1 FEM on right triangles, so the discrete energy is the exact Dirichlet integral and C_h ≥ C_box is a rigorous upper bound. | Cell-centred harmonic-mean scheme (equally exact for aligned interfaces but zero-thickness strips and multigrid coarsening are awkward; keep as an optional lower-bound-tendency cross-check); P1 FEM on unstructured triangles (needs a mesher, gains nothing); BEM/MoM (dense, singular kernels, multilayer Green's functions; keep MMTL/FasterCap as external oracles). |
| Conductors | Dirichlet by row elimination. Zero thickness = one node row. Thick conductors ≥ 4 cells thick with faces on grid lines. Trapezoid side walls via Shortley–Weller cut links (symmetric matrix, O(h²) solution error). | Bitmap/staircase (atlc: 3.5% at 5 px, needs ≥25 px per feature, oscillates with grid shift); area-fraction εr in cut cells (a conductor is a Dirichlet region, not a permittivity). |
| Corner singularities | Geometric grading toward every conductor edge (ratio ≤ 1.25); Richardson extrapolation with the *observed* order over three nested grids (knife edge converges at order 1.0, 90° corner at 4/3, graded grids approach 2). | Assumed h² extrapolation (over-corrects); singular enrichment (breaks SPD/variational structure). |
| Open boundary (microstrip) | Stretch cells geometrically beyond 2H (ratio ≤ 1.25); walls at ≥ 32 × max(H, conductor span); solve with Dirichlet and Neumann outer walls; report the mean and half the gap as a rigorous truncation bound. Measured: each wall type errs as 1/L², the mean as ~1/L⁴ (0.02% at 8H, 0.0001% at 32H). Verify per benchmark family by doubling L (gap must fall 3.5–4.5×). | Fixed padding rules (lineforge 16× uniform: 3200² cells for ~1%); Kelvin inversion/ballooning (exact but a second mesh and coupling code for no gain over the D/N mean). |
| Capacitance extraction | Q from the residual of the Dirichlet rows (= Gauss flux on the dual box = 2W/V², identical to rounding). Independence comes from other checks, not from energy-vs-flux. | Treating energy vs flux disagreement as an error estimate (they are one number computed three ways in this scheme). |
| Independent error checks | Richardson (discretization); D/N bracket (truncation); C12 = C21 from two solves (operator symmetry and solver convergence); one-cell conductor shift (cut-cell bias); optional cell-centred re-solve (opposite bound tendency); analytic cases and external solvers. | — |
| Coupled lines | Two solves (1,0),(0,1) → full Maxwell C matrix; check reciprocity to 1e-8; C_even = C11 + C12, C_odd = C11 − C12; C_air on the identical grid so correlated errors cancel in C/C_air and √(C·C_air). Asymmetric pairs: return C, C_air and leave the generalized eigenproblem to the caller. | Half-domain symmetry solves as default (cheaper but loses the reciprocity check). |
| Linear solver | Geometric multigrid V(2,1) with red-black Gauss-Seidel as a CG preconditioner; feature lines at node indices divisible by 2^L so coarse grids contain every feature and Dirichlet set; coarse operators re-discretized from area-averaged coarse cells; Richardson grids are the multigrid levels. Jacobi-PCG as fallback and mandatory cross-check (agree to 1e-10). Expected 8–12 cycles, 20–50 ms at 80k unknowns. | faer sparse Cholesky (+44 crates, +80 s cold build); banded Cholesky (1–2 s, 128 MB); IC(0)-PCG (fine but sequential and no faster). |
| Determinism | `Vec`/`BTreeMap` only, fixed reduction order, no threads inside a solve; `rayon` only over independent cases in `pcb-bench`, results sorted by case id; goldens never built with `target-cpu=native`. | — |
| Reported uncertainty | 1.25·|C∞ − C_h|/|C∞| (Roache GCI) + D/N half-width + 10 × solver tolerance, summed; accept when total ≤ target and 0.8 ≤ p ≤ 2.2, else refine one more level. | Ansys-style "delta energy" criteria (do not bound Zo error). |

## 3. Workspace layout and dependency direction

```
pcb-toolkit/
├── crates/pcb-toolkit/                 # unchanged deps; phase 4 adds optional feature "fieldsolver"
│   └── tests/fieldsolver_regressions.rs + tests/data/fieldsolver/*.csv   # std-only CSV reader
├── crates/pcb-toolkit-cli/             # phase 4: `fieldsolve` subcommand
├── crates/pcb-fieldsolver/             # publishable; deps: thiserror, serde
│   ├── src/{lib,geometry,presets,grid,raster,assembly,field,postprocess,refine,error,provenance}.rs
│   ├── src/linalg/{pcg,multigrid,dense}.rs
│   └── tests/{exact_cases,convergence,properties,slow}.rs
├── crates/pcb-bench/                   # publish = false; deps: pcb-toolkit, pcb-fieldsolver, clap, rayon, serde_json
└── docs/accuracy/                      # generated per-model residual tables feeding STATUS.md

pcb-fieldsolver  <──(optional feature, phase 4)── pcb-toolkit ── pcb-toolkit-cli
       ^
       └────(normal)── pcb-bench ──writes──> crates/pcb-toolkit/tests/data/fieldsolver/*.csv ──read as data──> pcb-toolkit tests
```

Crate name `pcb-fieldsolver` was free on crates.io on 2026-09-17 (also free: `tem-solver`,
`quasistatic`, `xsection`, `laplace2d`). Licence MIT OR Apache-2.0 like the workspace. MSRV 1.85.

## 4. Public API (sketch)

Plain structs with `Default`, mirroring the toolkit's `*Input` convention. Lengths in any consistent
unit (Laplace is scale-free), so mils pass through.

```rust
pub struct DielectricLayer { pub thickness: f64, pub er: f64 }             // bottom → top
pub enum Shape { Rect { x0, x1, y0, y1 }, Trapezoid { y0, y1, bottom: (f64, f64), top: (f64, f64) }, Sheet { x0, x1, y } }
pub enum Role { Signal, Ground }
pub struct Conductor { pub shape: Shape, pub role: Role, pub name: Option<String> }
pub enum Wall { Ground, Open }            // Open = Dirichlet/Neumann bracket with stretched far field
pub struct Domain { pub width: f64, pub left: Wall, pub right: Wall, pub top: Wall, pub bottom: Wall }
pub struct Mask { pub er: f64, pub thickness: f64, pub conformal: bool }
pub struct Geometry { pub layers: Vec<DielectricLayer>, pub conductors: Vec<Conductor>, pub domain: Domain, pub mask: Option<Mask> }

pub enum Resolution { CellsAcrossMinFeature(u32), MinCell(f64) }
pub enum LinearSolver { MgPcg, JacobiPcg }
pub struct SolverOptions { pub resolution: Resolution, pub richardson_levels: u8, pub grading: f64,
                           pub far_field_span_multiple: f64 /* default 32 */, pub linear: LinearSolver, pub rel_tol: f64, pub max_iter: u32 }

pub struct CMatrix { pub n: usize, pub f_per_m: Vec<f64> }                  // Maxwell convention, row-major
pub struct Uncertainty { pub total: f64, pub discretization: f64, pub truncation: f64, pub observed_order: f64,
                         pub reciprocity: f64, pub solver_residual: f64, pub in_asymptotic_range: bool }
pub struct GridReport { pub nx: u32, pub ny: u32, pub min_cell: f64, pub levels: u8, pub iterations: Vec<u32> }
pub struct Provenance { pub solver: &'static str, pub version: &'static str, pub method: &'static str,
                        pub options: SolverOptions, pub grid: GridReport, pub wall_ms: u64 }
pub struct LineParams { pub zo: f64, pub er_eff: f64, pub c_pf_per_m: f64, pub l_nh_per_m: f64 }
pub struct CoupledParams { pub zeven: f64, pub zodd: f64, pub zdiff: f64, pub zcomm: f64, pub er_eff_even: f64, pub er_eff_odd: f64 }
pub struct FieldResult { pub c: CMatrix, pub c_air: CMatrix, pub single: Option<LineParams>,
                         pub coupled: Option<CoupledParams>, pub uncertainty: Uncertainty, pub provenance: Provenance }

pub fn solve(geometry: &Geometry, options: &SolverOptions) -> Result<FieldResult, FieldError>;
pub fn solve_one_level(geometry: &Geometry, resolution: &Resolution, walls: WallKind) -> Result<Solution, FieldError>; // raw φ, for plots

pub enum FieldError { InvalidGeometry(String), Overlap { a: usize, b: usize }, OutsideDomain(usize),
                      GridTooCoarse { feature: f64, cell: f64 }, NotConverged { iterations: u32, residual: f64 },
                      NotAsymptotic { observed_order: f64 }, NonFinite(&'static str) }

pub static METHOD: MethodInfo;   // name, version, reference, limits ("quasi-TEM, lossless, isotropic; T ≥ 4 cells or Sheet")
// presets.rs: Stripline / Microstrip / CoupledMicrostrip / CoupledStripline / Embedded with the same field
// names as the pcb-toolkit *Input structs, so adapters are trivial `From` impls living in pcb-bench (phase 3)
// and behind pcb-toolkit's optional feature (phase 4).
```

## 5. Algorithm

```
solve(geom, opts):
  feature lines x/y = walls, interfaces, conductor faces/edges, mask edges, trapezoid corner abscissae
  per interval: n = k·2^L cells; geometric grading toward conductor edges (ratio ≤ opts.grading);
                geometric stretching from 2·span out to far walls (Open only)
  for level in L, L−1, L−2:                                 # Richardson grids = multigrid levels
    for walls in {Dirichlet, Neumann} (Open) or {closed}:
      eps[cell]; fixed[node]; Shortley–Weller cut links on trapezoid rows
      A = stencil a_E, a_W, a_N, a_S per node
      for excitation in unit vectors over signal conductors:
        phi = MG-PCG(A, rhs, tol); Q_i = Σ_{nodes of conductor i} (A·phi)_i      → C
      check |C12 − C21| ≤ 1e-8·C11
      re-solve with eps = 1 on the same grid                                     → C_air
      Zeven, Zodd, eps_eff from C, C_air
  for q in {Zeven, Zodd, eps_eff, C, C_air}:
    p = log2((q_{L−2} − q_{L−1})/(q_{L−1} − q_L)); q∞ = q_L + (q_L − q_{L−1})/(2^p − 1)
    disc = 1.25·|q∞ − q_L|/|q∞|; trunc = |q_D − q_N|/(q_D + q_N)
    report mean(q_D, q_N) or q∞ with total = disc + trunc + 10·tol; flag if p ∉ [0.8, 2.2]
```

## 6. Verification protocol

Exact-to-rounding: two-layer parallel plate with the interface on a node row; uniform-field Neumann box;
energy identity 2W/V² = Q (1e-12) on every solve. Operator: manufactured solution ∇·(ε∇φ) = f with a
piecewise-smooth φ continuous in φ and ε∂φ/∂y at the interface, observed order ≥ 1.9 on uniform and
graded grids. Analytic capacitances: coax annulus in a Cartesian grid (exercises the cut links, 0.05%);
wire over ground (open boundary, 0.1%); zero-thickness Cohn stripline (uniform grid order 1.0 ± 0.1,
Richardson ≤ 0.1%; graded order ≥ 1.8, ≤ 0.05%); Cohn 1955 coupled even/odd (0.1%, reciprocity 1e-8);
Hammerstad-Jensen homogeneous microstrip Z01(u) at u ∈ {0.5, 1, 2, 5} (0.2%). Box doubling 32H → 64H:
D/N half-width falls 3.5–4.5×. Properties (`proptest`): C symmetric positive definite; diag C ≥ diag
C_air; 1 ≤ εeff ≤ max εr; Zo(W) strictly decreasing; coupling decreasing in spacing; C(εr) = εr·C_air in a
homogeneous box; one-cell conductor shift changes C by less than the reported uncertainty; Zo continuous
as thickness passes from one cell to a Sheet. Solvers: MG-PCG vs Jacobi-PCG agree to 1e-10. Wall time
≤ 100 ms for 400×200 in release.

External oracles (slow tier, scheduled CI): atlc on ~20 bitmaps (grid method, does not test the
trapezoid path), FasterCap 2-D or MMTL BEM on ~10 cases (different method), and FEniCSx via
`sinbad-oracle-fenicsx` once an electrostatic-capacitance capability is added there (~10 cases,
unstructured mesh, independent corner handling). Commercial solvers agree with each other to ±0.8%,
so that is the realistic bar for "independent agreement".

Golden data: `crates/pcb-toolkit/tests/data/fieldsolver/*.csv` with a header carrying solver and bench
versions, git hash, options, and per-row uncertainty. pcb-toolkit's regression test asserts every
closed form within its stated bound, with the solver absent from its build graph. STATUS.md rule: a
model is "validated ±x% over region R" when max|closed form − solver| + max solver uncertainty ≤ x and
the solver uncertainty ≤ x/3 across the sampled region; `pcb-bench matrix` writes `docs/accuracy/<model>.md`.
Any change to discretization, extraction or extrapolation bumps the solver minor version and regenerates
goldens; regeneration deltas beyond the old uncertainty are findings, not re-baselines.

## 7. Phased plan

| Phase | Scope | Est. |
|---|---|---|
| 0 | Crate skeleton, workspace wiring, crates.io name reservation, `METHOD` metadata | 0.5 d |
| 1 | Geometry validation, graded/stretched tensor grid, vertex-centred FV assembly, Dirichlet elimination, Jacobi-PCG then MG-PCG, Richardson driver with observed order, D/N open-wall bracket, stripline and microstrip presets, exact and manufactured-solution tests, Cohn and H-J qualification | 4–6 d |
| 2 | N conductors and C matrix, reciprocity check, even/odd, Shortley–Weller trapezoids, planar and conformal mask, coax cut-cell test, Cohn 1955 qualification, one-cell-shift test | 3–4 d |
| 3 | `pcb-bench`: Latin-hypercube sweep (~500 coupled microstrip, 300 embedded, 300 coupled stripline with finite T), rayon over cases, CSV goldens with provenance, accuracy matrix into STATUS.md, pcb-toolkit CSV regression tests, scheduled slow-tier CI with atlc/BEM cross-checks | 3–4 d |
| 4 | Optional `fieldsolver` feature in pcb-toolkit with `From` adapters and one `CalcError::FieldSolver` variant; `pcb-toolkit fieldsolve <preset>` / `--case file.toml` / `--json` | 2–3 d |

Roughly 2.8k lines over 13–18 working days. Phases 1–3 deliver the audit's residual item
(independent benchmarks for the differential and finite-thickness models); phase 4 is a user feature.

## 8. Deferred and out of scope

In-repo 2-D BEM (Bryant–Weiss/MMTL style) as a second independent method: worth doing later, not
needed to ship. Cell-centred harmonic-mean re-solve as a lower-bound cross-check: optional after phase 2.
Losses, frequency dependence, anisotropy, 3-D vias: out of scope for this crate.
