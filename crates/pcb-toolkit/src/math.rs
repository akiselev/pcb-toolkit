//! Numerical helpers: complete elliptic integrals via the arithmetic-geometric mean.

use crate::CalcError;

/// Arithmetic-geometric mean of two non-negative numbers.
pub fn agm(mut a: f64, mut b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        return 0.0;
    }
    for _ in 0..64 {
        let (an, bn) = ((a + b) / 2.0, (a * b).sqrt());
        if (an - bn).abs() <= 1e-16 * an {
            return an;
        }
        a = an;
        b = bn;
    }
    a
}

/// Complete elliptic integral of the first kind K(k) for modulus 0 ≤ k < 1.
pub fn elliptic_k(k: f64) -> f64 {
    let kp = (1.0 - k * k).sqrt();
    std::f64::consts::PI / (2.0 * agm(1.0, kp))
}

/// Ratio K(k)/K(k') with k' = √(1 − k²), for modulus 0 < k < 1.
///
/// Computed as agm(1, k)/agm(1, k'), which is exact (to rounding) and stays
/// well-conditioned for moduli very close to 0 or 1, unlike the Hilberg
/// logarithmic approximation.
pub fn elliptic_ratio(k: f64) -> Result<f64, CalcError> {
    if !k.is_finite() || k <= 0.0 || k >= 1.0 {
        return Err(CalcError::OutOfRange {
            name: "elliptic modulus",
            value: k,
            expected: "0 < k < 1 (geometry aspect ratio too extreme for this model)",
        });
    }
    let kp = (1.0 - k * k).sqrt();
    let r = agm(1.0, k) / agm(1.0, kp);
    if r.is_finite() && r > 0.0 {
        Ok(r)
    } else {
        Err(CalcError::NonFiniteResult {
            name: "elliptic ratio",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn k_at_zero_is_pi_over_two() {
        assert_relative_eq!(
            elliptic_k(0.0),
            std::f64::consts::FRAC_PI_2,
            max_relative = 1e-15
        );
    }

    #[test]
    fn k_known_value() {
        // K(1/√2) = Γ(1/4)² / (4√π) = 1.854074677301372
        assert_relative_eq!(
            elliptic_k(std::f64::consts::FRAC_1_SQRT_2),
            1.854074677301372,
            max_relative = 1e-14
        );
    }

    #[test]
    fn ratio_symmetry() {
        let k = 0.3_f64;
        let kp = (1.0 - k * k).sqrt();
        assert_relative_eq!(
            elliptic_ratio(k).unwrap() * elliptic_ratio(kp).unwrap(),
            1.0,
            max_relative = 1e-13
        );
    }

    #[test]
    fn ratio_matches_hilberg_in_its_accurate_zone() {
        // Hilberg: K/K' ≈ π / ln(2(1+√k')/(1−√k')) for k ≤ 1/√2, accurate to ~3 ppm.
        let k = 0.5_f64;
        let kp = (1.0 - k * k).sqrt();
        let hilberg = std::f64::consts::PI / (2.0 * (1.0 + kp.sqrt()) / (1.0 - kp.sqrt())).ln();
        assert_relative_eq!(elliptic_ratio(k).unwrap(), hilberg, max_relative = 1e-5);
    }

    #[test]
    fn ratio_rejects_domain_edges() {
        assert!(elliptic_ratio(0.0).is_err());
        assert!(elliptic_ratio(1.0).is_err());
        assert!(elliptic_ratio(f64::NAN).is_err());
    }
}
