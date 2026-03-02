//! PPM / frequency conversion and crystal load capacitor calculator.
//!
//! Sub-calculators:
//! 1. XTAL load capacitor value: C_load = (C1×C2)/(C1+C2) + C_stray
//! 2. Hz to PPM: PPM = (variation / center_freq) × 1,000,000
//! 3. PPM to Hz: variation = center_freq × PPM / 1,000,000
