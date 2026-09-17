//! Transmission line impedance calculators.
//!
//! Supports microstrip, embedded microstrip, stripline and conductor-backed
//! coplanar waveguide. Each topology module computes Zo, Er_eff, Tpd, Lo, Co
//! and exposes a `MODEL` constant describing its provenance and range.

pub mod common;
pub mod coplanar;
pub mod embedded;
pub mod microstrip;
pub mod stripline;
pub mod types;
