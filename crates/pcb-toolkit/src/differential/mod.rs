//! Differential pair impedance calculators.
//!
//! Supports edge-coupled (external, internal symmetric/asymmetric, embedded)
//! and shielded broadside-coupled configurations.
//!
//! Outputs: Zdiff, Zo, Zodd, Zeven and the backward coupling coefficient Kb
//! (unterminated and terminated, linear and dB). Each module exposes a
//! `MODEL` constant describing its provenance and range.

pub mod broadside_coupled;
pub mod edge_coupled_embedded;
pub mod edge_coupled_external;
pub mod edge_coupled_internal_asym;
pub mod edge_coupled_internal_sym;
pub mod types;
