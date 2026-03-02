//! Differential pair impedance calculators.
//!
//! Supports edge-coupled (external, internal symmetric/asymmetric, embedded)
//! and broadside-coupled (shielded, non-shielded) configurations.
//!
//! Outputs: Zdiff, Zodd, Zeven, coupling coefficient Kb, NEXT voltage.

pub mod types;
pub mod edge_coupled_external;
pub mod edge_coupled_internal_sym;
pub mod edge_coupled_internal_asym;
pub mod edge_coupled_embedded;
pub mod broadside_coupled;
