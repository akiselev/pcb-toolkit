pub mod constants;
pub mod copper;
pub mod error;
pub mod materials;
pub mod units;

// Calculators
pub mod crosstalk;
pub mod current;
pub mod differential;
pub mod fusing;
pub mod impedance;
pub mod inductor;
pub mod ohms_law;
pub mod padstack;
pub mod ppm;
pub mod reactance;
pub mod spacing;
pub mod via;
pub mod wavelength;

// Lookup tables
pub mod tables;

pub use error::CalcError;
pub use units::{Capacitance, Freq, Inductance, Length, Temperature, UnitParseError};
