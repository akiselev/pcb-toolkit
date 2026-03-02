use serde::{Deserialize, Serialize};

/// Result of an impedance calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpedanceResult {
    /// Characteristic impedance (Ohms).
    pub zo: f64,
    /// Effective dielectric constant (dimensionless).
    pub er_eff: f64,
    /// Propagation delay (ps/in).
    pub tpd_ps_per_in: f64,
    /// Inductance per unit length (nH/in).
    pub lo_nh_per_in: f64,
    /// Capacitance per unit length (pF/in).
    pub co_pf_per_in: f64,
}
