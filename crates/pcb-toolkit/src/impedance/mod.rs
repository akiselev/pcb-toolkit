//! Transmission line impedance calculators.
//!
//! Supports multiple topologies: microstrip, stripline, embedded microstrip,
//! coplanar waveguide. Each topology module computes Zo, Er_eff, Tpd, Lo, Co.
//!
//! Primary reference: Hammerstad & Jensen, "Accurate Models for Microstrip
//! Computer-Aided Design", IEEE MTT-S 1980.

pub mod common;
pub mod embedded;
pub mod microstrip;
pub mod stripline;
pub mod types;
