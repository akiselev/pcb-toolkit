# 2-D quasi-static capacitance solver: numerical method and verification protocol

Date: 2026-09-17. Scope: design of the in-house 2-D electrostatic solver that will
produce benchmark capacitances for PCB transmission lines (layered piecewise-constant
permittivity, thin rectangular or trapezoidal conductors, ground planes, closed box or
open top). Outputs per solve: C and C_air per unit length, Zo = 1/(c·sqrt(C·C_air)),
eps_eff = C/C_air, even/odd modes for two conductors, and a rigorous error estimate.

The numbers quoted below were produced by the scratch experiment
`docs/research/scripts/fv_check.py` (numpy only, uniform grid, Jacobi-preconditioned
conjugate gradient). Run it with `python3 docs/research/scripts/fv_check.py`; the last
case (box half-width 64H) takes about two minutes.

---

## 1. Discretization: vertex-centred finite volume on a graded tensor grid

**Recommendation.** Vertex-centred ("box integration") five-point finite volume on a
non-uniform tensor grid, permittivity constant per cell, unknowns at grid nodes. Every
dielectric interface, ground plane, box wall, conductor face and conductor edge is placed
on a grid line; only trapezoid side walls cut through cells (Section 2).

**Why not the alternatives.**

- P1 FEM on triangles brings a mesher, unstructured connectivity and quadrature code for
  a geometry that is axis-aligned except for the etch slope, and gains nothing, because the
  tensor scheme below *is* a P1 FEM (see "Identity with P1 FEM").
- BEM/MoM handles open boundaries and zero-thickness strips naturally, but layered
  dielectrics require multilayer Green's functions or extra unknowns on every dielectric
  interface, the systems are dense (O(N^2) memory, O(N^3) direct solve), the kernels are
  singular, and there is no cheap conservation or variational-bound structure to test
  against. Keep BEM as the external oracle (MMTL/TNT), not as the in-house method.

**Stencil.** For node P with neighbours E, W, N, S, cell widths dx_E, dx_W, cell heights
dy_N, dy_S, and the permittivities of the four cells touching P (eps_NE, eps_NW, eps_SE,
eps_SW):

```
a_E = (eps_NE*dy_N + eps_SE*dy_S) / (2*dx_E)      a_W = (eps_NW*dy_N + eps_SW*dy_S) / (2*dx_W)
a_N = (eps_NE*dx_E + eps_NW*dx_W) / (2*dy_N)      a_S = (eps_SE*dx_E + eps_SW*dx_W) / (2*dy_S)

(a_E + a_W + a_N + a_S)*phi_P - a_E*phi_E - a_W*phi_W - a_N*phi_N - a_S*phi_S = 0
```

This is Gauss's law on the dual box around P (faces at the half-way points to the
neighbours). The matrix is symmetric positive definite, the scheme is conservative (face
fluxes telescope exactly), and Neumann walls are natural: a wall node simply has no face
on the wall side and a half-size control volume.

**Interface on a grid line, without error.** A horizontal dielectric interface lies on a
node row. The N and S dual faces of a node on that row lie entirely inside one material
and use that material's eps. The E and W dual faces straddle the interface and carry the
area-weighted (arithmetic) average of the two permittivities, which is exact because the
tangential field is continuous across the interface and the two half-faces act as
parallel capacitors. For the two-layer parallel plate the scheme reproduces the
series-capacitor result to rounding (Test 1 in Section 7).

The harmonic mean that the roadmap mentions belongs to the *cell-centred* variant
(unknowns at cell centres, interfaces on cell faces), where the flux across a face
between cells of different eps is exact for one-dimensional fields with the
distance-weighted harmonic mean 1/eps_f = (d_i/eps_i + d_j/eps_j)/(d_i + d_j) (Patankar,
*Numerical Heat Transfer and Fluid Flow*, 1980, section 4.2-3; convergence analysis in
arXiv 2502.09413). Both variants are exact for interfaces on their respective grid
entities. The vertex-centred variant is preferred here because (i) a zero-thickness
conductor is a single node row, (ii) coarse multigrid grids are node subsets of fine
grids, so features survive coarsening, and (iii) it has an exact FEM identity that gives
rigorous bounds.

**Identity with P1 FEM.** Split every rectangular cell along a diagonal into two right
triangles. For P1 elements the stiffness coupling between the two end nodes of an edge is
-(eps/2)*cot(theta) summed over the triangles sharing the edge, where theta is the angle
opposite the edge. Across the hypotenuse the opposite angle is 90 degrees, cot = 0, so
there is no diagonal coupling. Along a horizontal leg of length dx in a cell of height dy
the opposite angle has cot(theta) = dy/dx, giving -(eps/2)*dy/dx; summing the triangle
above and the triangle below the edge gives exactly -a_E above, for any dx/dy and either
choice of diagonal. Therefore:

- the scheme is P1 FEM on right triangles with cellwise-constant eps, exactly;
- the discrete energy (1/2)*phi^T*A*phi equals the exact Dirichlet integral
  (1/2)*Int eps*|grad phi_h|^2 of the piecewise-linear interpolant;
- the variational bounds of Section 4 hold rigorously;
- on smoothly graded grids the solution error stays O(h^2) even though the local
  truncation error on a non-uniform grid is only O(h) (supra-convergence; Manteuffel and
  White, "The numerical solution of second-order boundary value problems on nonuniform
  meshes", Math. Comp. 47, 1986).

---

## 2. Conductors, thin strips, trapezoids, corner singularities

**Dirichlet by elimination.** Conductor nodes are removed from the unknown set; their
couplings a_f*V move to the right-hand side of the neighbouring rows. A zero-thickness
strip is one node row. A thick rectangle is a block of nodes whose faces are grid lines;
interior nodes are inert and can be skipped.

**Trapezoid side walls.** The bottom and top corners are placed on grid lines. For each
node row through the conductor thickness the (straight) side wall crosses a horizontal
link at a distance d < dx from the nearest free node. Use the Shortley-Weller cut link
(Shortley and Weller, J. Appl. Phys. 9, 1938): replace that link's coefficient by
eps*(dy_N + dy_S)/(2d) and move the neighbour value V to the right-hand side. The matrix
stays symmetric (the modified coefficient couples an unknown to a known value only), the
local truncation error at the cut node is O(1), yet the solution error remains O(h^2)
(supra-convergence proven for Dirichlet problems: Matsunaga and Yamamoto, J. Comput.
Appl. Math. 116, 2000; Yoon and Min, J. Sci. Comput. 2016). A staircase approximation is
only O(h), oscillates when the grid is shifted, and is not recommended. The coax annulus in
a Cartesian grid (Section 7) exercises exactly this code path with a known answer.

**Corner singularity.** Near a conductor edge with dielectric opening angle beta the
potential behaves as r^(pi/beta) (Van Bladel, *Electromagnetic Fields*, ch. on singular
fields). The exponent lambda = pi/beta is 1/2 for a knife edge (beta = 2*pi), 2/3 for a
90-degree corner (beta = 3*pi/2), and between those values for trapezoid corners. The
energy-norm error of P1 FEM on quasi-uniform grids is O(h^lambda), so the error of the
capacitance, a quadratic functional, is O(h^(2*lambda)): first order for zero thickness,
h^(4/3) for rectangular conductors.

Measured on a uniform grid, zero-thickness centred stripline, W/B = 1, eps_r = 1, side
walls at 4B (fields decay as exp(-pi*x/B), so their effect is below 1e-9). Cohn's exact
value: C/eps0 = 4*K(k')/K(k) with k = sech(pi*W/(2B)) = 5.76448992.

| h      | nodes   | CG iterations | C/eps0     | error    | 2W/V^2 - Q | observed p |
|--------|---------|---------------|------------|----------|------------|------------|
| B/8    | 585     | 56            | 6.13468726 | +6.4220% | -8.9e-16   |            |
| B/16   | 2193    | 113           | 5.94548514 | +3.1398% | 0.0        |            |
| B/32   | 8481    | 226           | 5.85397211 | +1.5523% | -1.8e-15   | 1.048      |
| B/64   | 33345   | 446           | 5.80897801 | +0.7718% | -1.3e-14   | 1.024      |
| B/128  | 132225  | 878           | 5.78667080 | +0.3848% | +4.3e-14   | 1.012      |

Richardson extrapolation with the observed order over each triple of grids:

| grids used         | p     | extrapolated C/eps0 | error    |
|--------------------|-------|---------------------|----------|
| B/8, B/16, B/32    | 1.048 | 5.76824471          | +0.0651% |
| B/16, B/32, B/64   | 1.024 | 5.76545875          | +0.0168% |
| B/32, B/64, B/128  | 1.012 | 5.76473691          | +0.0043% |

Three observations: the order is 1.0 as predicted for the knife edge; every value lies
above the exact one, as the upper-bound property of Section 4 requires; extrapolation with
the observed order recovers 0.004% from grids whose raw error is 0.4 to 1.5%.

**Remedies, in order of preference.**

1. Geometric grading toward every conductor edge in both directions (ratio 1.15 to 1.3,
   smallest cell h_min at the edge). Babuska, Kellogg and Pitkaranta (Numer. Math. 33,
   1979) show that refinement graded toward the corner restores the optimal order; on a
   tensor grid geometric grading in each coordinate is the practical form.
2. Richardson extrapolation using the *observed* order from three grids, never an assumed
   2. On a graded grid the observed order should approach 2; on a uniform grid it is
   2*lambda.
3. Singular-function enrichment or corner-corrected stencils (Motz problem; Fix, Gulati
   and Wakoff 1973): not recommended. They break the SPD and variational structure and
   remedies 1 and 2 already reach the target.

Also require a thick conductor to span at least four cells in thickness (with grid lines
on both faces), otherwise its two corners interact on the coarser grids and the observed
order becomes erratic.

---

## 3. Open boundary for microstrip

**Truncation error scaling.** The strip charge +Q at height H together with its image -Q
in the ground plane forms a vertical line dipole. A grounded (Dirichlet) side wall at
distance L induces an image dipole at distance 2L whose potential at the strip is
(Q/(4*pi*eps0))*ln(L^2/(L^2 + H^2)) = -(Q/(4*pi*eps0))*(H/L)^2 to leading order: the
strip lies in the equatorial plane of the image dipole, so the 1/r dipole term vanishes
and the first surviving term is O((H/L)^2). An insulating (Neumann) wall produces an
image of the opposite sign. Hence

    dC/C = -/+ k*(H/L)^2,   k = C/(4*pi*eps0) (of order 1),

positive for Dirichlet walls and negative for Neumann walls, and the leading terms cancel
in the mean of the two. The same argument applies to the top wall.

Measured, air microstrip W/H = 1, zero thickness, fixed h = H/8 (so the discretization
error is identical for every L and the differences isolate truncation), box half-width and
box height both equal to L:

| L    | C_D/eps0  | C_N/eps0  | (D - N)/mean | mean      |
|------|-----------|-----------|--------------|-----------|
| 4H   | 3.2735112 | 2.9438493 | +10.605%     | 3.1086803 |
| 8H   | 3.1406088 | 3.0584883 | +2.649%      | 3.0995486 |
| 16H  | 3.1092425 | 3.0887178 | +0.662%      | 3.0989801 |
| 32H  | 3.1015101 | 3.0963791 | +0.166%      | 3.0989446 |
| 64H  | 3.0995838 | 3.0983010 | +0.041%      | 3.0989424 |

Relative to the D/N mean at L = 64H:

| L    | Dirichlet | Neumann  | mean of D and N |
|------|-----------|----------|-----------------|
| 4H   | +5.633%   | -5.005%  | +0.314%         |
| 8H   | +1.345%   | -1.305%  | +0.020%         |
| 16H  | +0.332%   | -0.330%  | +0.0012%        |
| 32H  | +0.083%   | -0.083%  | +0.0001%        |

Both single-wall errors fall by a factor of 4 per doubling of L (1/L^2), and the mean falls
by a factor of about 16 per doubling (1/L^4). A single wall type needs L of about 30H for
0.1%; the mean is within 0.02% already at 8H.

**Bracketing.** Grounding the outer wall adds a conductor at zero potential and can only
increase the capacitance; making it Neumann removes flux paths and can only decrease it.
Both follow from the monotonicity of the Dirichlet energy with respect to the admissible
set (Polya and Szego, *Isoperimetric Inequalities in Mathematical Physics*, 1951; Collin,
*Field Theory of Guided Waves*, variational-methods chapter). Hence, for the exact box
problems,

    C_N(L) <= C_open <= C_D(L).

**Practical rule.**

1. Beyond 2H from the nearest conductor, stretch cells geometrically (ratio 1.25 or
   less). The far field is a smooth dipole field, so coarse cells there cost essentially
   nothing. Reaching 32H from 2H takes about 13 stretched cells per side.
2. Put the side walls and the top wall at L >= 32 * max(H, total conductor span). For
   wide or coupled structures the span (W, or 2W + S) replaces H because the near field
   extends that far.
3. Solve with Dirichlet walls and with Neumann walls. Report the mean as the value and
   half the difference (below 0.1% at 32H) as a *rigorous* truncation bound.
4. Verify once per benchmark family by doubling L: the half-difference must fall by 3.5x
   to 4.5x. If it does not, the near zone is under-resolved or the walls are too close to
   the stretching start.

**Alternatives not adopted.** Kelvin inversion / ballooning maps the exterior region onto
a finite annulus, which removes the truncation error entirely (Silvester, Lowther,
Carpenter and Wyatt, "Exterior finite elements for 2-dimensional field problems with open
boundaries", Proc. IEE 124, 1977; Imhoff, Meunier, Brunotte and Sabonnadiere, IEEE Trans.
Magn. 26, 1990). It costs a second mesh, a coordinate transformation of the coefficients
and a coupling interface; the D/N mean reaches the same accuracy with about 20 extra cells
per side and no new code.

---

## 4. Capacitance extraction and what is actually independent

Let A be the full SPD matrix (before elimination), D the set of conductor nodes held at
V, and suppose the free rows are solved to zero residual. Then

    Q = sum_{i in D} (A*phi)_i                    (Gauss flux out of the conductor's dual box)
    W = (1/2)*phi^T*A*phi
      = (1/2)*sum_{i in D} phi_i*(A*phi)_i + (1/2)*sum_{i free} phi_i*0
      = (1/2)*V*Q.

So C = Q/V = 2W/V^2 is an algebraic identity of the discrete system, not a check
(measured |2W - Q| <= 4e-14 on every grid in the table of Section 2; the residual of the
free rows is the only source of discrepancy). Likewise, because face fluxes telescope,
*any* closed contour built from dual faces that encloses the conductor gives the same Q
exactly. Energy, residual charge and face-contour flux are one number computed three ways.
A contour that does not follow dual faces (for example a circle sampled with interpolated
gradients) differs by O(h^2), but it is still the same phi, so it checks only the
smoothness of the interpolation.

**Independent checks (each tests a different error source).**

| Check | Error source tested |
|-------|---------------------|
| Richardson over three grids, observed order | discretization |
| Dirichlet vs Neumann outer walls | box truncation |
| C12 = C21 from two separate solves | solver convergence, symmetry of the assembled operator |
| Cell-centred harmonic-mean re-solve | different scheme with the opposite bound tendency (below) |
| Grid shift of the conductor by one cell | staircase/cut-cell bias |
| Analytic cases and external solvers | everything, including the geometry code |

**Variational bounds.** Because the scheme is a conforming P1 FEM whose trial functions
satisfy the Dirichlet data exactly, the Dirichlet principle (the true potential minimizes
the energy over all admissible potentials) gives

    C_h >= C_box        (rigorous upper bound on the boxed problem, any grid),

and with Dirichlet outer walls C_h >= C_box,D >= C_open. Every stripline value in the
Section 2 table lies above Cohn's exact value, as it must. Thomson's principle (minimize
the complementary energy over divergence-free D fields) gives lower bounds; the
cell-centred harmonic-mean scheme is the lowest-order mixed Raviart-Thomas method up to a
quadrature rule (Baranger, Maitre and Oudin, M2AN 30, 1996) and therefore tends toward the
lower bound, although the quadrature makes that bound non-rigorous. Solving a case both
ways yields a sandwich that a single scheme cannot produce.

---

## 5. Coupled lines

Solve (V1, V2) = (1, 0) and (0, 1). From the first solve C11 = Q1 and C21 = Q2; from the
second C12 = Q1 and C22 = Q2 (Maxwell capacitance convention, C12 <= 0). Check
|C12 - C21| <= 1e-8 * C11. Then

    C_even = C11 + C12       C_odd = C11 - C12

equivalently the Q1 of the (1, 1) and (1, -1) excitations. A symmetric pair can use those
two excitations directly on a half domain with a Neumann (even) or Dirichlet (odd)
symmetry plane, halving the cost; the full-domain two-solve form is preferred by default
because it produces the complete matrix and the reciprocity check. Then

    Z_even = 1/(c*sqrt(C_even*C_even,air))      Z_odd = 1/(c*sqrt(C_odd*C_odd,air))
    Z_diff = 2*Z_odd                            Z_common = Z_even/2

Asymmetric pairs additionally need the generalized eigenproblem of C_air^-1 * C for the
modal decomposition; report the C and C_air matrices and leave that to the caller.

C_air is a re-solve on the *identical* grid with every eps set to 1 (same nodes, same
conductor mask, same cut links). The corner-singularity and truncation errors of C and
C_air are then strongly correlated and largely cancel in the ratio C/C_air and in the
product sqrt(C*C_air); a different grid would inject uncorrelated errors of the same
magnitude. The Richardson extrapolation and the error estimate are applied to Zo and
eps_eff themselves, not only to C.

---

## 6. Linear solver

Typical benchmark grid 400 x 200 = 80k unknowns; sweep grids up to about 800 x 400.
Measured Jacobi-PCG iteration counts (tolerance 1e-11 on the residual): 878 for
1024 x 129 (stripline), 2354 and 2911 for 1024 x 512 (open box, Dirichlet and Neumann
walls); iteration count scales with the longer grid dimension.

| Method | Iterations (80k) | Rust wall time (release, one core) | Notes |
|--------|------------------|------------------------------------|-------|
| Jacobi-PCG | 400 to 1200 | 0.2 to 0.6 s | 30 lines, always converges |
| IC(0)-PCG | 150 to 400 | 0.1 to 0.3 s | fill-free, sequential triangular solves |
| Banded Cholesky (bandwidth 200) | direct | 1 to 2 s, 128 MB | O(n * bw^2) flops, O(n * bw) memory |
| Sparse Cholesky (faer, nested dissection) | direct | 0.1 to 0.3 s | O(n^1.5) flops, adds about 30 crates |
| Geometric multigrid V(2,1), red-black Gauss-Seidel | 8 to 12 cycles for 1e-10 | 20 to 50 ms | about 200 lines |

**Recommendation: geometric multigrid as a preconditioner for CG.**

- Grid hierarchy. Feature lines (interfaces, conductor faces and edges, mask edges, walls)
  are placed at node indices divisible by 2^L, so every coarse grid contains every feature
  and coarse Dirichlet sets are exactly the fine Dirichlet sets restricted to coarse nodes.
  Graded and stretched grids coarsen by taking every other node; the grading survives.
- Coarse operators are re-discretized from the coarse cells with the Section 1 stencil
  (coarse-cell eps is the area average of its fine children, which is exact here because
  interfaces are on coarse lines). Operator-dependent interpolation (Alcouffe, Brandt,
  Dendy and Painter, SIAM J. Sci. Stat. Comput. 2, 1981) is only needed when interfaces
  cut coarse cells, which the construction prevents.
- Dirichlet nodes get zero correction on every level; bilinear prolongation is zeroed at
  them; restriction is the transpose (full weighting).
- Smoother: red-black Gauss-Seidel, two pre-sweeps and one post-sweep with the colour order
  reversed in the post-sweep so the V-cycle is a symmetric operator and CG remains valid.
- Stretched far-field cells weaken point smoothing once the cell aspect ratio exceeds
  about 3. Cap the stretch ratio at 1.25 and let CG absorb the degradation (a few extra
  iterations instead of divergence).
- Coarsest level (at least three cells between features): a few hundred Gauss-Seidel
  sweeps, or a small dense solve.
- The multigrid levels are the Richardson grids: solving levels L, L-1 and L-2 to
  tolerance as independent problems costs a few percent extra and provides the
  three-grid convergence study for free.

**Fallback and cross-check.** Jacobi-PCG, required in the test suite to agree with MG-PCG
to 1e-10 relative on every case. If MG-PCG ever exceeds 40 iterations the driver falls
back to Jacobi-PCG and flags the case.

References: Briggs, Henson and McCormick, *A Multigrid Tutorial*, 2nd ed., SIAM 2000;
Trottenberg, Oosterlee and Schuller, *Multigrid*, Academic Press 2001; Wesseling, *An
Introduction to Multigrid Methods*, 1992; Saad, *Iterative Methods for Sparse Linear
Systems*, 2nd ed., SIAM 2003, ch. 9 and 10 for PCG and IC(0).

---

## 7. Verification protocol

1. **Exact discrete cases** (agreement to 1e-10 relative): two-layer parallel plate with
   the interface on a node row; uniform-field box with Neumann side walls. These test the
   interface representation, the Dirichlet elimination and the charge extraction.
2. **Manufactured solution.** Solve div(eps*grad phi) = f with phi = sin(pi*x)*g(y), g
   piecewise smooth and chosen so that phi and eps*dphi/dy are continuous at the
   interface (for example g = sinh(pi*y) below and a*sinh(pi*y) + b*cosh(pi*y) above, with
   a and b fixed by the two continuity conditions), f computed analytically. Require the
   max-norm error to converge with observed order >= 1.9 over three grids on both uniform
   and graded grids. This tests the operator and boundary handling independently of any
   capacitance code.
3. **Analytic capacitances.**
   - Coax annulus in a Cartesian grid (Shortley-Weller path): C = 2*pi*eps/ln(b/a).
   - Wire over ground (open boundary, D/N bracket): C = 2*pi*eps/acosh(h/a).
   - Zero-thickness centred stripline: C_air = 4*eps0*K(k')/K(k), k = sech(pi*W/(2B)),
     as implemented in `crates/pcb-toolkit/src/impedance/stripline.rs`.
   - Cohn 1955 coupled stripline, zero thickness, even and odd modes, as implemented in
     `crates/pcb-toolkit/src/differential/edge_coupled_internal_sym.rs`.
   - Homogeneous-medium microstrip against the Hammerstad-Jensen Z01(u) formula, whose
     stated accuracy is 0.01% (Hammerstad and Jensen, IEEE MTT-S Digest 1980). This is
     the open-boundary qualification case.
4. **Observed order of convergence.** p = log2[(C_h - C_h/2)/(C_h/2 - C_h/4)]. Expect
   1.0 +/- 0.1 for a knife edge on a uniform grid, >= 1.8 on graded grids, >= 1.9 for
   smooth cases (parallel plate with source term, coax).
5. **Error estimate reported with every result.** Richardson value
   C_inf = C_fine + (C_fine - C_medium)/(2^p - 1). Reported discretization uncertainty is
   1.25*|C_inf - C_fine| (Roache's Grid Convergence Index with the three-grid safety
   factor, J. Fluids Eng. 116, 1994). Total reported uncertainty = discretization
   uncertainty + D/N truncation half-width (microstrip) + 10 times the solver tolerance,
   summed rather than combined in quadrature. Grid-independence rule: accept when the
   total is below the target (0.1 to 0.5%) and 0.8 <= p <= 2.2 (asymptotic range);
   otherwise refine by one more level and repeat.
6. **External oracles.** atlc on about 20 bitmaps (atlc is also a grid method with
   staircased geometry, so it does not independently validate the trapezoid treatment);
   FEniCSx electrostatics via the sinbad oracle on about 10 cases (unstructured mesh,
   different corner handling: an independent discretization).
7. **Properties.** C matrix symmetric and positive definite; diagonal of C >= diagonal of
   C_air; Zo(W) strictly decreasing; C(eps_r) = eps_r*C_air exactly in a homogeneous box;
   shifting the conductor by one cell changes C by less than the reported uncertainty;
   Zo continuous when the thickness passes from one cell to zero.

---

## 8. Solder mask, cover layers, conductor thickness in C_air

A planar mask is one more layer (eps_m, thickness t_m) with both faces on grid lines. A
conformal mask is the set of cells within t_m of any conductor cell; add grid lines at
x = +/-(W/2 + t_m) (and the corresponding trapezoid offsets) and at y = H + T + t_m so the
mask boundary is on grid lines. Embedded (cover) layers are ordinary layers.

C_air sets every eps to 1 but keeps the identical conductor mask and cut links, so finite
thickness and etch slope enter C_air too. Finite thickness raises C_air through side-wall
fringing; using a zero-thickness C_air together with a thick C mis-states Zo by about 1
to 2% at T/H = 0.1, which is precisely the thickness effect that the Hammerstad-Jensen
thickness correction accounts for.

---

## Algorithm outline

```
solve_line(geom, target_tol):
  lines_x, lines_y = feature lines: walls, layer interfaces, conductor faces and edges,
                     mask edges, trapezoid corner abscissae
  for each interval between feature lines:
      choose n = k * 2^L cells; geometric grading toward conductor edges
      (ratio <= 1.25, h_min at the edge); geometric stretching toward outer walls
  for level in L, L-1, L-2:                        # Richardson grids = multigrid levels
    for walls in {Dirichlet, Neumann}:             # microstrip; closed box: one pass
      eps[cell]   = layer permittivity (or mask permittivity)
      fixed[node] = conductor | ground | (walls == Dirichlet and outer wall)
      cut links   = Shortley-Weller coefficients on trapezoid rows
      A_level     = stencil coefficients a_E, a_N per node (re-discretized on coarse levels)
      for excitation in [(1,0), (0,1)]:            # or (1,1), (1,-1) on a half domain
          phi = MG-PCG(A_level, rhs(excitation), tol = 1e-10)
          Q_i = sum over conductor-i nodes of (A*phi)_i   -> C matrix
      assert |C12 - C21| <= 1e-8 * C11
      repeat with eps = 1 on the same grid           -> C_air matrix
      Z_even, Z_odd, eps_eff from C and C_air
  for each output quantity q in {Z_even, Z_odd, eps_eff, C, C_air}:
      p        = log2((q_L-2 - q_L-1)/(q_L-1 - q_L))
      q_inf    = q_L + (q_L - q_L-1)/(2^p - 1)
      disc_err = 1.25 * |q_inf - q_L| / |q_inf|
      trunc_err = |q_D - q_N| / (q_D + q_N)         # microstrip only
      report mean(q_D, q_N) or q_inf, uncertainty = disc_err + trunc_err + 10*tol
      flag if p outside [0.8, 2.2] or uncertainty above target
```

---

## Unit tests and expected tolerances

| Test | Expected result / tolerance |
|------|-----------------------------|
| Two-layer parallel plate, interface on node row | 1e-10 relative |
| Energy identity 2W/V^2 = Q on every solve | 1e-12 relative |
| Manufactured solution, three grids, uniform and graded | observed order >= 1.9 |
| Coax annulus with cut cells, Richardson | 0.05%; per-grid order >= 1.8 |
| Wire over ground, D/N mean, L = 32h | 0.1%; D/N half-width <= 0.1% |
| Cohn stripline T = 0, uniform grid | order 1.0 +/- 0.1; Richardson <= 0.1% |
| Cohn stripline T = 0, graded grid | order >= 1.8; Richardson <= 0.05% |
| Cohn 1955 coupled even/odd, T = 0 | 0.1% each; C12 = C21 to 1e-8 |
| Hammerstad-Jensen homogeneous microstrip, u in {0.5, 1, 2, 5} | 0.2% |
| Box doubling 32H to 64H | D/N half-width falls by 3.5x to 4.5x |
| MG-PCG vs Jacobi-PCG | 1e-10 relative |
| C SPD; diag C >= diag C_air; Zo(W) decreasing; C(eps_r) proportional to eps_r in a homogeneous box | exact / strict |
| Conductor shifted by one cell | change below reported uncertainty |
| atlc on 20 bitmaps | 0.5% |
| Wall time, 400 x 200, MG-PCG, release build | <= 100 ms |

---

## Sources

- Babuska, Kellogg, Pitkaranta, "Direct and inverse error estimates for finite elements
  with mesh refinements", Numer. Math. 33 (1979):
  https://link.springer.com/article/10.1007/BF01399326
- Shortley-Weller supra-convergence: Matsunaga and Yamamoto, J. Comput. Appl. Math. 116
  (2000), https://dl.acm.org/doi/10.1016/S0377-0427%2899%2900321-0 ; Jomaa and Macaskill,
  J. Comput. Phys. 202 (2005),
  https://www.sciencedirect.com/science/article/abs/pii/S0021999104002955
- Harmonic-mean interface coefficient analysis: https://arxiv.org/abs/2502.09413
- Patankar, *Numerical Heat Transfer and Fluid Flow*, Hemisphere 1980, section 4.2-3.
- Manteuffel and White, "The numerical solution of second-order boundary value problems
  on nonuniform meshes", Math. Comp. 47 (1986).
- Van Bladel, *Electromagnetic Fields*, 2nd ed., IEEE Press/Wiley 2007 (edge
  singularities): https://download.e-bookshelf.de/download/0000/5693/03/L-G-0000569303-0015227376.pdf
- Imhoff, Meunier, Brunotte, Sabonnadiere, "An original solution for unbounded
  electromagnetic 2D- and 3D-problems throughout the finite element method", IEEE Trans.
  Magn. 26 (1990):
  https://www.semanticscholar.org/paper/e9f0992cb137550a05c93055a8de3e10af5dc974
- Silvester, Lowther, Carpenter, Wyatt, "Exterior finite elements for 2-dimensional field
  problems with open boundaries", Proc. IEE 124 (1977).
- Polya and Szego, *Isoperimetric Inequalities in Mathematical Physics*, Princeton 1951.
- Collin, *Field Theory of Guided Waves*, 2nd ed., IEEE Press 1991 (variational bounds):
  https://archive.org/details/fieldtheoryofgui00coll
- Baranger, Maitre, Oudin, "Connection between finite volume and mixed finite element
  methods", M2AN 30 (1996): http://www.numdam.org/item/M2AN_1996__30_4_445_0/
- Alcouffe, Brandt, Dendy, Painter, "The multi-grid method for the diffusion equation with
  strongly discontinuous coefficients", SIAM J. Sci. Stat. Comput. 2 (1981):
  https://www.osti.gov/biblio/6425073
- Briggs, Henson, McCormick, *A Multigrid Tutorial*, 2nd ed., SIAM 2000:
  https://books.google.com/books/about/A_Multigrid_Tutorial.html?id=oSTGBm64o1AC
- Wesseling, *An Introduction to Multigrid Methods* (author-permitted scan):
  http://ftp.demec.ufpr.br/multigrid/Bibliografias/Wesseling_An%20Introduction%20to%20MultiGrid%20Methods.pdf
- Trottenberg, Oosterlee, Schuller, *Multigrid*, Academic Press 2001.
- Saad, *Iterative Methods for Sparse Linear Systems*, 2nd ed., SIAM 2003:
  https://www.stat.uchicago.edu/~lekheng/courses/324/saad.pdf
- Roache, "Perspective: A method for uniform reporting of grid refinement studies",
  J. Fluids Eng. 116 (1994):
  https://asmedigitalcollection.asme.org/fluidsengineering/article/116/3/405/411554
- Hammerstad and Jensen, "Accurate models for microstrip computer-aided design", IEEE
  MTT-S Digest 1980:
  https://www.semanticscholar.org/paper/2d551661d4d5207d0db3cf57d462e9421e9dccf4
- Cohn, "Characteristic impedance of the shielded-strip transmission line", IRE Trans.
  MTT-2 (1954); Cohn, "Shielded coupled-strip transmission line", IRE Trans. MTT-3 (1955).
- atlc FAQ (accuracy "errors of less than 0.3% are typical"; open structures "not
  properly evaluated"): https://atlc.sourceforge.net/FAQ.html
- Altair Flux documentation, energy-method capacitance C_ii = 2W/V_i^2:
  https://2023.help.altair.com/2023/flux/Flux/Help/english/UserGuide/English/topics/CalculDeMatricesDeCapacites.htm
