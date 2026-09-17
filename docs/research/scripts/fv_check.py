# fv_check.py -- numerical evidence for docs/research/2026-09-17-solver-numerics.md
#
# Demonstrates, with a vertex-centred five-point finite-volume scheme (identical to P1 FEM
# on right triangles) on a uniform grid and a Jacobi-preconditioned conjugate gradient
# solver (numpy only, no scipy):
#   1. the discrete energy identity 2W/V^2 == Q (charge from the Dirichlet-row residual)
#      holds to rounding on every grid, so energy and charge are not independent checks;
#   2. a zero-thickness centred stripline (W/B = 1, eps_r = 1) converges to Cohn's exact
#      C/eps0 = 4 K(k')/K(k) at first order (knife-edge singularity), from above (upper
#      bound), and Richardson extrapolation with the observed order reaches 0.004%;
#   3. for an air microstrip (W/H = 1) grounded (Dirichlet) and insulating (Neumann) outer
#      walls bracket the open-boundary capacitance with errors falling as 1/L^2, while the
#      mean of the two falls as about 1/L^4 (0.02% at L = 8H).
#
# Run:  python3 docs/research/scripts/fv_check.py
# Runtime about 2.5 minutes; the L = 64H case dominates. Requires numpy >= 1.20.
import numpy as np, math, time, sys

def agm(a, b):
    for _ in range(64):
        a, b = (a + b) / 2, math.sqrt(a * b)
        if abs(a - b) <= 1e-16 * a:
            break
    return a

def K(k):
    return math.pi / (2 * agm(1.0, math.sqrt(1 - k * k)))

class Grid:
    def __init__(self, nx, ny, hx, hy, eps):  # eps: (nx, ny) cells; nodes (nx+1, ny+1)
        self.nx, self.ny, self.hx, self.hy = nx, ny, hx, hy
        # node-to-node coupling coefficients
        # east faces between node (i,j) and (i+1,j), i in 0..nx-1, j in 0..ny
        e_up = np.zeros((nx, ny + 1)); e_dn = np.zeros((nx, ny + 1))
        e_up[:, :-1] = eps * hy / 2          # cell (i, j) lies above node row j
        e_dn[:, 1:] = eps * hy / 2           # cell (i, j-1) lies below node row j
        self.aE = (e_up + e_dn) / hx
        n_l = np.zeros((nx + 1, ny)); n_r = np.zeros((nx + 1, ny))
        n_r[:-1, :] = eps * hx / 2           # cell (i, j) right of node column i
        n_l[1:, :] = eps * hx / 2            # cell (i-1, j) left of column i
        self.aN = (n_l + n_r) / hy

    def apply(self, phi):
        """(A phi)_P = sum_faces a_f (phi_P - phi_nb)  -- SPD operator, Neumann natural."""
        out = np.zeros_like(phi)
        dE = self.aE * (phi[:-1, :] - phi[1:, :])
        out[:-1, :] += dE; out[1:, :] -= dE
        dN = self.aN * (phi[:, :-1] - phi[:, 1:])
        out[:, :-1] += dN; out[:, 1:] -= dN
        return out

    def diag(self):
        d = np.zeros((self.nx + 1, self.ny + 1))
        d[:-1, :] += self.aE; d[1:, :] += self.aE
        d[:, :-1] += self.aN; d[:, 1:] += self.aN
        return d

def solve(g, fixed, values, tol=1e-11, maxit=200000):
    """CG on free nodes.  fixed: bool mask; values: potentials at fixed nodes."""
    phi = np.where(fixed, values, 0.0)
    free = ~fixed
    b = -g.apply(phi) * free          # move Dirichlet couplings to rhs
    x = np.zeros_like(phi)
    r = b.copy()
    Minv = 1.0 / np.where(g.diag() > 0, g.diag(), 1.0)
    z = Minv * r * free
    p = z.copy()
    rz = np.sum(r * z)
    bnorm = math.sqrt(np.sum(b * b))
    for it in range(maxit):
        Ap = g.apply(p) * free
        alpha = rz / np.sum(p * Ap)
        x += alpha * p
        r -= alpha * Ap
        rn = math.sqrt(np.sum(r * r))
        if rn <= tol * bnorm:
            break
        z = Minv * r * free
        rz_new = np.sum(r * z)
        p = z + (rz_new / rz) * p
        rz = rz_new
    phi = phi + x * free
    return phi, it + 1

def charge_and_energy(g, phi, cond):
    Aphi = g.apply(phi)
    Q = np.sum(Aphi[cond])              # residual of Dirichlet rows = Gauss flux
    W = 0.5 * np.sum(phi * Aphi)        # discrete energy
    return Q, W

# ---------------------------------------------------------------- stripline
def stripline(B, W, L, h, er=1.0):
    nx = int(round(2 * L / h)); ny = int(round(B / h))
    eps = np.full((nx, ny), er)
    g = Grid(nx, ny, h, h, eps)
    fixed = np.zeros((nx + 1, ny + 1), bool); vals = np.zeros_like(fixed, float)
    fixed[:, 0] = fixed[:, -1] = True                   # ground planes
    fixed[0, :] = fixed[-1, :] = True                   # side walls (fields decay ~e^{-pi x/B})
    j = ny // 2; i0 = int(round((L - W / 2) / h)); i1 = int(round((L + W / 2) / h))
    cond = np.zeros_like(fixed); cond[i0:i1 + 1, j] = True
    fixed |= cond; vals[cond] = 1.0
    phi, it = solve(g, fixed, vals)
    Q, Wn = charge_and_energy(g, phi, cond)
    return Q, Wn, it, (nx + 1) * (ny + 1)

print("=== zero-thickness centred stripline, W/B = 1, er = 1 (Cohn exact) ===")
B, W, L = 1.0, 1.0, 4.0
k = 1 / math.cosh(math.pi * W / (2 * B))
C_exact = 4 * K(math.sqrt(1 - k * k)) / K(k)   # C/eps0 = 4 K'/K
print(f"Cohn C/eps0 = {C_exact:.8f}")
res = []
for n in (8, 16, 32, 64, 128):
    h = B / n
    t = time.time(); Q, Wn, it, N = stripline(B, W, L, h); dt = time.time() - t
    res.append(Q)
    print(f"h=B/{n:3d}  N={N:7d}  CG it={it:5d}  {dt:5.1f}s  C={Q:.8f}  err={(Q/C_exact-1)*100:+.4f}%  2W/V^2-Q={2*Wn-Q:+.2e}")
for a, b, c in zip(res, res[1:], res[2:]):
    p = math.log2((a - b) / (b - c))
    Cinf = c + (c - b) / (2 ** p - 1)
    print(f"   observed order p={p:.3f}  Richardson C={Cinf:.8f}  err={(Cinf/C_exact-1)*100:+.4f}%")

# ---------------------------------------------------------------- microstrip box study
def microstrip(H, W, Lx, Ly, h, wall):
    nx = int(round(2 * Lx / h)); ny = int(round(Ly / h))
    eps = np.ones((nx, ny))
    g = Grid(nx, ny, h, h, eps)
    fixed = np.zeros((nx + 1, ny + 1), bool); vals = np.zeros_like(fixed, float)
    fixed[:, 0] = True                                   # ground plane
    if wall == "D":
        fixed[0, :] = fixed[-1, :] = fixed[:, -1] = True # grounded outer walls
    j = int(round(H / h)); i0 = int(round((Lx - W / 2) / h)); i1 = int(round((Lx + W / 2) / h))
    cond = np.zeros_like(fixed); cond[i0:i1 + 1, j] = True
    fixed |= cond; vals[cond] = 1.0
    phi, it = solve(g, fixed, vals)
    Q, Wn = charge_and_energy(g, phi, cond)
    return Q, it

print("\n=== air microstrip W/H = 1, zero thickness, fixed h = H/8; box half-width and height L ===")
H, W, h = 1.0, 1.0, 1.0 / 8
out = {}
for L in (4, 8, 16, 32, 64):
    t = time.time()
    cd, itd = microstrip(H, W, L, L, h, "D")
    cn, itn = microstrip(H, W, L, L, h, "N")
    out[L] = (cd, cn)
    print(f"L={L:3d}H  C_D={cd:.7f}  C_N={cn:.7f}  (D-N)/mean={(cd-cn)/(0.5*(cd+cn))*100:+.4f}%  mean={(cd+cn)/2:.7f}  it={itd},{itn}  {time.time()-t:5.1f}s")
ref = 0.5 * sum(out[64])
print("relative to mean at L=64H:")
for L in (4, 8, 16, 32):
    cd, cn = out[L]
    print(f"L={L:3d}H  D: {(cd/ref-1)*100:+.4f}%   N: {(cn/ref-1)*100:+.4f}%   mean: {((cd+cn)/2/ref-1)*100:+.4f}%")
