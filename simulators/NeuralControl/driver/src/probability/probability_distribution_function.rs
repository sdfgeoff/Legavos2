use super::GaussianCurve;
use serde::{Deserialize, Serialize};

/// Represents a probability distribution function as a set
/// of gaussian curves
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProbabilityDistributionFunction {
    pub curves: Vec<GaussianCurve>,
}

impl ProbabilityDistributionFunction {
    pub fn how_likely(&self, data: f32) -> f32 {
        // Returns the likelihood of this value being the truth
        self.curves.iter().map(|curve| curve.how_likely(data)).sum()
    }

    pub fn how_normalized(&self) -> f32 {
        // Returns how normalized the current distribution function is, where
        // zero is prefectly normalized and any positive number is the deviation
        // from that.
        let sum_max: f32 = self.curves.iter().map(|x| x.max.abs()).sum();
        sum_max - 1.0
    }

    pub fn normalize(&mut self) {
        // Ensures the distribution function sums to 1.
        let sum_max: f32 = self.curves.iter().map(|x| x.max.abs()).sum();
        if sum_max != 0.0 {
            for curve in self.curves.iter_mut() {
                curve.max = curve.max.abs() / sum_max;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_how_normalized() {
        let mut under_test = ProbabilityDistributionFunction {
            curves: vec![
                GaussianCurve {
                    max: 3.0,
                    deviation: 10.0,
                    mean: 2.0,
                }
            ]
        };
        assert_eq!(under_test.how_normalized(), 2.0);
        let mut under_test = ProbabilityDistributionFunction {
            curves: vec![
                GaussianCurve {
                    max: -3.0,
                    deviation: 10.0,
                    mean: 2.0,
                }
            ]
        };
        assert_eq!(under_test.how_normalized(), 2.0);

        let mut under_test = ProbabilityDistributionFunction {
            curves: vec![
                GaussianCurve {
                    max: 1.0,
                    deviation: 10.0,
                    mean: 2.0,
                }
            ]
        };
        assert_eq!(under_test.how_normalized(), 0.0);
    }

    #[test]
    fn test_normalize() {
        let mut under_test = ProbabilityDistributionFunction {
            curves: vec![
                GaussianCurve {
                    max: 3.0,
                    deviation: 10.0,
                    mean: 2.0,
                }
            ]
        };
        under_test.normalize();
        assert_eq!(under_test.curves[0].max, 1.0);

        let mut under_test = ProbabilityDistributionFunction {
            curves: vec![
                GaussianCurve {
                    max: -3.0,
                    deviation: 10.0,
                    mean: 2.0,
                }
            ]
        };
        under_test.normalize();
        assert_eq!(under_test.curves[0].max, 1.0);

        let mut under_test = ProbabilityDistributionFunction {
            curves: vec![
                GaussianCurve {
                    max: 3.0,
                    deviation: 10.0,
                    mean: 2.0,
                },
                GaussianCurve {
                    max: 3.0,
                    deviation: 10.0,
                    mean: 2.0,
                }
            ]
        };
        under_test.normalize();
        assert_eq!(under_test.curves[0].max, 0.5);
        assert_eq!(under_test.curves[1].max, 0.5);

        let mut under_test = ProbabilityDistributionFunction {
            curves: vec![
                GaussianCurve {
                    max: 4.0,
                    deviation: 10.0,
                    mean: 5.0,
                },
                GaussianCurve {
                    max: 2.0,
                    deviation: 10.0,
                    mean: 2.0,
                }
            ]
        };
        under_test.normalize();
        assert_eq!(under_test.curves[0].max, 0.6666667);
        assert_eq!(under_test.curves[1].max, 0.33333334);
    }
}
