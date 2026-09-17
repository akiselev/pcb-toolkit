# Field-solver crate architecture design (2026-09-17)

Research report, unedited. Companion to `2026-09-17-solver-prior-art.md` and `2026-09-17-roadmap-research.md` §4.

Recommendation: a separate publishable workspace crate `pcb-fieldsolver` with zero deps beyond thiserror+serde and a hand-written multigrid-preconditioned CG; a non-published `pcb-bench` crate that depends on both it and pcb-toolkit and writes golden CSVs; pcb-toolkit consumes the CSVs in tests with no build-time dependency on the solver.

## 1. Crate boundary

**Recommendation: `crates/pcb-fieldsolver` in this workspace, published to crates.io, never depending on `pcb-toolkit`.**

- A cargo feature inside `pcb-toolkit` was rejected: features don't remove code from the published tarball, 1–3k lines of solver would dilute a crate whose contract is "closed forms with a `MODEL` constant", and the crate could not test itself against the solver as a dev-dependency.
- A separate repository was rejected: golden data, solver version and toolkit regression tests must change in one atomic commit.
- Direction: `pcb-toolkit` → `pcb-fieldsolver` only (optional feature, phase 4). Cargo does permit dev-dependency cycles, but they compile pcb-toolkit twice with two distinct type identities in tests, so avoid. No `pcb-toolkit-reference` crate is needed: the only exact oracles the solver needs are K(k)/K(k′) via AGM (~25 lines, duplicated in the solver's tests) and the parallel-plate/two-dielectric formulas. The bench crate cross-checks the duplicate against `pcb_toolkit::math::elliptic_ratio`.
- Names checked on the crates.io API (all 404 = free): `pcb-fieldsolver`, `tem-solver`, `quasistatic`, `xsection`, `fieldsolver`, `laplace2d`, `quasi-tem`. Pick `pcb-fieldsolver`: discoverable next to `pcb-toolkit`, honest about scope (2-D quasi-TEM cross sections); `xsection`/`quasistatic` are too generic to own.

```
pcb-toolkit/
├── Cargo.toml                       # add pcb-fieldsolver + rayon to [workspace.dependencies]
├── crates/
│   ├── pcb-toolkit/                 # deps unchanged; phase 4 adds optional feature "fieldsolver"
│   │   └── tests/
│   │       ├── fieldsolver_regressions.rs     # std-only CSV parser (~20 lines), no solver dep
│   │       └── data/fieldsolver/*.csv         # golden data, provenance header
│   ├── pcb-toolkit-cli/             # phase 4: `fieldsolve` subcommand
│   ├── pcb-fieldsolver/             # publishable; deps: thiserror, serde
│   │   ├── src/{lib,geometry,presets,grid,raster,assembly,field,postprocess,refine,error,provenance}.rs
│   │   ├── src/linalg/{mod,pcg,multigrid,dense}.rs
│   │   └── tests/{exact_cases,convergence,properties,slow}.rs
│   └── pcb-bench/                   # publish = false; deps: pcb-toolkit, pcb-fieldsolver, clap, rayon, serde_json
│       └── src/main.rs              # sweep | qualify | matrix | check
└── docs/accuracy/                   # generated per-model residual tables feeding STATUS.md
```

```
 pcb-fieldsolver (thiserror, serde)        <-- no dependency on pcb-toolkit, ever
      ^ optional feature          ^ normal
      |                           |
 pcb-toolkit                 pcb-bench (publish=false)
      ^                           |  writes
 pcb-toolkit-cli                  v
                     crates/pcb-toolkit/tests/data/fieldsolver/*.csv
                                  |  read at test time (data, not code)
                                  v
                     pcb-toolkit tests/fieldsolver_regressions.rs
```

## 2. Public API

Plain structs with `Default`, matching the toolkit's `*Input` convention; a few constructor presets stand in for builders. Lengths are in any consistent unit (Laplace is scale-free; C′ = ε₀·shape factor), so the toolkit's mils pass through unchanged.

```rust
// geometry.rs
pub struct DielectricLayer { pub thickness: f64, pub er: f64 }          // stacked bottom→top
pub enum Shape {
    Rect { x0: f64, x1: f64, y0: f64, y1: f64 },
    Trapezoid { y0: f64, y1: f64, bottom: (f64, f64), top: (f64, f64) },
    Sheet { x0: f64, x1: f64, y: f64 },                                 // zero-thickness strip
}
pub enum Role { Signal, Ground }
pub struct Conductor { pub shape: Shape, pub role: Role, pub name: Option<String> }
pub enum Wall { Ground, Open /* Neumann */ }
pub struct Domain { pub width: f64, pub left: Wall, pub right: Wall, pub top: Wall, pub bottom: Wall }
pub struct Geometry { pub layers: Vec<DielectricLayer>, pub conductors: Vec<Conductor>, pub domain: Domain }
impl Geometry { pub fn validate(&self) -> Result<(), FieldError> }      // overlap, inside domain, er ≥ 1, finite

// presets.rs  (field names mirror pcb-toolkit inputs so From impls are trivial)
pub struct Stripline { pub width: f64, pub height: f64, pub thickness: f64, pub er: f64 }
pub struct Microstrip { pub width: f64, pub height: f64, pub thickness: f64, pub er: f64, pub box_margin: f64 }
pub struct CoupledMicrostrip { /* + spacing */ }
impl From<Stripline> for Geometry { /* two Ground walls, one Signal Rect or Sheet */ }

// options
pub enum Resolution { CellsAcrossMinFeature(u32), MinCell(f64) }
pub enum LinearSolver { MgPcg, JacobiPcg }                              // like russell's `Genie`
pub struct Convergency { pub rel_tol: f64, pub max_iter: u32 }          // like the `roots` crate
pub struct SolverOptions {
    pub resolution: Resolution, pub richardson_levels: u8 /* 2 or 3 */,
    pub bracket_open_walls: bool /* solve Neumann and Dirichlet far box, report half-gap */,
    pub linear: LinearSolver, pub convergency: Convergency, pub grading: f64 /* max cell ratio */,
}

// results (all Serialize + Deserialize)
pub struct CMatrix { pub n: usize, pub f_per_m: Vec<f64> }              // Maxwell form, row-major
pub struct ErrorEstimate { pub relative: f64, pub observed_order: f64, pub discretization: f64, pub boundary: f64, pub energy_flux_mismatch: f64 }
pub struct GridReport { pub nx: u32, pub ny: u32, pub min_cell: f64, pub levels: u8, pub iterations: Vec<u32>, pub final_residual: f64 }
pub struct Provenance { pub solver: &'static str, pub version: &'static str, pub method: &'static str, pub options: SolverOptions, pub grid: GridReport, pub wall_ms: u64 }
pub struct LineParams { pub zo: f64, pub er_eff: f64, pub c_pf_per_m: f64, pub l_nh_per_m: f64 }
pub struct CoupledParams { pub zeven: f64, pub zodd: f64, pub zdiff: f64, pub zcomm: f64, pub er_eff_even: f64, pub er_eff_odd: f64 }
pub struct FieldResult {
    pub c: CMatrix, pub c_air: CMatrix,
    pub single: Option<LineParams>,      // n = 1
    pub coupled: Option<CoupledParams>,  // n = 2 (eigen-decomposition of C_air⁻¹C, exact for asymmetric pairs too)
    pub error: ErrorEstimate, pub provenance: Provenance,
}
pub fn solve(geometry: &Geometry, options: &SolverOptions) -> Result<FieldResult, FieldError>;
pub fn solve_one_level(geometry: &Geometry, resolution: &Resolution, ...) -> Result<Solution, FieldError>; // raw φ field, for plots

// error.rs — solver-specific, not CalcError (no dependency)
pub enum FieldError { InvalidGeometry(String), Overlap { a: usize, b: usize }, OutsideDomain(usize),
    GridTooCoarse { feature: f64, cell: f64 }, NotConverged { iterations: u32, residual: f64 }, NonFinite(&'static str) }
```

Adapter without a cycle: `pcb-fieldsolver::presets::Stripline` has the same fields as `pcb_toolkit::impedance::stripline::StriplineInput`. Phase 3 puts the conversion functions in `pcb-bench`; phase 4 moves them into `pcb_toolkit::fieldsolver` behind an off-by-default feature, adding one `CalcError::FieldSolver(String)` variant for the mapping. Default builds of pcb-toolkit gain no dependency.

## 3. Internal layout and determinism

Pipeline: `geometry` → `grid` (1-D graded coordinate lines snapped to every material and conductor edge; refinement halves every interval, so Richardson levels and the multigrid hierarchy are the same nested `GridLevels`) → `raster` (εr per cell by area fraction for trapezoids; Dirichlet node mask per conductor; `Sheet` is a Dirichlet node line, which makes the zero-thickness Cohn cases exact) → `assembly` (finite-volume 5-point stencil, harmonic-mean face permittivity, Dirichlet by row elimination, Neumann ghost faces; stored as five coefficient arrays, no CSR needed) → `linalg` (MG-PCG: V-cycle, red-black Gauss-Seidel, coefficient-averaged coarse operators) → `field` (one solve per signal conductor at 1 V; C from energy, cross-checked against Gauss flux, mismatch reported) → `postprocess` (Zo = 1/(c√(C·C_air)), εeff = C/C_air, even/odd) → `refine` (h, 2h, 4h; observed order p = log₂ of successive differences; extrapolate; error = |C* − C_h|; never assume p = 2, edge singularities give 1 < p < 2).

Determinism: `Vec` and `BTreeMap` only; all reductions in fixed index order; no `rayon` inside a solve. Parallelism lives only in `pcb-bench` (`par_iter` over independent cases, then sort by case id), so results are bit-identical across thread counts. Do not build goldens with `target-cpu=native` (FMA contraction differences); comparisons use `err_est` tolerances anyway.

## 4. Dependencies (measured, rustc 1.97, cold `cargo build --release`)

| Scratch crate | Crates in graph | Build |
|---|---|---|
| thiserror + serde baseline | 13 | 9.3 s |
| + faer 0.24.4 (`sparse` feature) | 57 | 91.3 s |
| + sprs 0.11.5 + sprs-ldl | 37 | 16.2 s |
| + nalgebra-sparse 0.12.0 | 32 | 29.5 s |

faer is the only pure-Rust option with a real sparse direct solver (simplicial/supernodal LLᵀ/LDLᵀ, AMD/COLAMD orderings; docs.rs/faer `sparse::linalg`), actively maintained (1.45M recent downloads), but costs ~80 s and 44 crates. nalgebra-sparse offers only `CscCholesky` with no fill-reducing ordering, which fills badly on 2-D grids. sprs-ldl is pure Rust by default with RCM only; CAMD needs SuiteSparse C. `faer-sparse` 0.17.1 (Feb 2024) is superseded by faer's feature. `russell_sparse` wraps UMFPACK/MUMPS (needs C/Fortran). `fenris` 0.0.33 (2023) states "no API stability" and ships no solvers.

**Recommendation: zero dependencies.** An 80k-unknown 5-point Laplacian converges in 10–20 MG-PCG iterations (milliseconds); even Jacobi-PCG is ~0.3 s. The MG is ~300 lines and reuses the refinement hierarchy. Keep faer as a documented fallback if εr contrast ever stalls MG (PCG still guarantees convergence). `serde` for case files; `serde_json`, `clap`, `rayon` only in `pcb-bench` and the CLI.

## 5. Testing and CI

- Exact unit cases: parallel plate (exact on grid), two-dielectric parallel plate (tests the harmonic-mean interface), zero-thickness Cohn stripline, Cohn 1955 coupled stripline, scaling invariance (×2 geometry → identical C), mirror symmetry. Acceptance: < 0.5 % after Richardson, per the roadmap.
- Convergence tests: observed order in [0.9, 2.1] on smooth cases, and the error estimate must bracket the true error on every Cohn case.
- `proptest`: C symmetric, C_ii > 0, C_ij ≤ 0, C_air ≤ C on the diagonal, 1 ≤ εeff ≤ max εr, Zo decreasing in width, coupling decreasing in spacing.
- Golden CSVs in `crates/pcb-toolkit/tests/data/fieldsolver/`, header comments carrying `pcb-bench` and `pcb-fieldsolver` versions, git hash, options and per-row `err_est`. pcb-toolkit's regression test parses them with std only and asserts each closed form within its stated bound; the solver is not in its build graph.
- Tiers: fast tests run on every PR (< 1 min); `#[ignore]` slow tier (fine grids, box-size study, atlc cross-check bitmaps) plus `pcb-bench check` (regenerates a 30-case subset and diffs against goldens within `err_est`) on a weekly scheduled CI job.

## 6. Precedent to copy

- scikit-rf `Media` (MLine, CPW, DefinedGammaZ0): geometry object exposes `Z0`, `gamma`, `ep_reff` as plain data. Copy for `LineParams`. https://scikit-rf.readthedocs.io/en/latest/api/media/index.html
- FEMM: strict preprocessor/postprocessor split (`ei_*` problem/geometry/analyze vs `eo_*` block integrals). Copy as `Geometry` → `solve` → `Solution` → `postprocess`. Manual: https://www.femm.info/doku/lib/exe/fetch.php?media=upload:documentation:manual.pdf
- openEMS: geometry (CSXCAD `ContinuousStructure`) separate from the engine, ports as post-processing objects; MEEP: `Simulation(cell_size, geometry, resolution, boundary_layers)` with a single `resolution` scalar. https://meep.readthedocs.io/en/latest/Python_User_Interface/
- atlc: Zo from C with and without dielectric, even/odd via excitations, bitmap input; states < 0.3 % typical but poor on open structures, which is why the open-wall bracket exists. https://atlc.sourceforge.net/FAQ.html
- lineforge: same FD pipeline in Python, ±2 % claimed, layered kernel/orchestration/interfaces; confirms the parametric-plus-bitmap input split. https://github.com/RFingAdam/lineforge
- Rust: faer's symbolic/numeric split and `Params` + `Default`; russell's `Genie` solver-choice enum; `roots`' `Convergency` struct; argmin's observer callback for sweep progress; fenris's assembler/solver separation. https://docs.rs/faer https://docs.rs/russell_sparse https://docs.rs/nalgebra-sparse/latest/nalgebra_sparse/factorization/ https://docs.rs/sprs-ldl

## 7. Documentation and provenance

The solver exposes a static `METHOD: MethodInfo { name, version: env!("CARGO_PKG_VERSION"), reference, limits: "quasi-TEM, lossless, isotropic; T ≥ 1 cell or Sheet" }` mirroring `ModelInfo`, plus the dynamic `Provenance` in every result. STATUS.md rule: a model is "validated ±x % over region R" when max |closed form − solver| + max solver `err_est` ≤ x over the sampled region, and `err_est` ≤ x/3; `pcb-bench matrix` generates `docs/accuracy/<model>.md` with these numbers per region. Versioning: any change to discretization, extraction or extrapolation bumps the solver minor version and regenerates goldens; a regeneration whose deltas exceed the old `err_est` is a finding to investigate before re-baselining. Golden CSV schema version in the header.

## 8. Phased plan

| Phase | Scope | Lines | Effort |
|---|---|---|---|
| 0 | Crate skeleton, workspace wiring, name reservation | 100 | 0.5 d |
| 1 | Geometry, graded grid, FV assembly, Jacobi-PCG then MG, Richardson driver, single stripline and microstrip presets, exact-case qualification, ~10 atlc cross-checks | 1200 | 4–6 d |
| 2 | N conductors, C matrix, even/odd and 2×2 modal, trapezoid and solder-mask rasterization, Cohn 1955 qualification | 500 | 3–4 d |
| 3 | `pcb-bench`: LHS sweep (~1100 cases), rayon, CSV with provenance, accuracy matrix, pcb-toolkit CSV regression tests, STATUS.md update, scheduled CI | 600 | 3–4 d |
| 4 | `fieldsolver` feature in pcb-toolkit with adapters, `pcb-toolkit fieldsolve <preset>` and `--case file.toml`, `--json` | 400 | 2–3 d |

Total roughly 2.8k lines over 13–18 working days.
