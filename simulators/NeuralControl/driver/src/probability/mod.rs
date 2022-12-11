use serde::{Deserialize, Serialize};

mod gaussian_curve;
mod probability_distribution_function;
pub use gaussian_curve::GaussianCurve;
pub use probability_distribution_function::ProbabilityDistributionFunction;

pub fn gaussian_approximations_from_array(
    arr: Vec<f32>,
    approximations: usize,
) -> Vec<ProbabilityDistributionFunction> {
    let num_distributions = arr.len() / 3 / approximations;
    let mut probabilities = Vec::with_capacity(num_distributions);

    for i in 0..num_distributions {
        let mut pdf = ProbabilityDistributionFunction {
            curves: Vec::with_capacity(approximations),
        };

        for j in 0..approximations {
            let offset = i * approximations * 3 + j * 3;
            let curve = GaussianCurve {
                mean: arr[offset],
                deviation: arr[offset + 1],
                max: arr[offset + 2],
            };
            pdf.curves.push(curve);
        }

        probabilities.push(pdf);
    }

    probabilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_array() {
        assert_eq!(
            gaussian_approximations_from_array(vec![1.0, 2.0, 3.0], 1),
            vec![ProbabilityDistributionFunction {
                curves: vec![GaussianCurve {
                    mean: 1.0,
                    deviation: 2.0,
                    max: 3.0
                }]
            }]
        );

        assert_eq!(
            gaussian_approximations_from_array(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2),
            vec![ProbabilityDistributionFunction {
                curves: vec![
                    GaussianCurve {
                        mean: 1.0,
                        deviation: 2.0,
                        max: 3.0
                    },
                    GaussianCurve {
                        mean: 4.0,
                        deviation: 5.0,
                        max: 6.0
                    }
                ]
            }]
        );
        assert_eq!(
            gaussian_approximations_from_array(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 1),
            vec![
                ProbabilityDistributionFunction {
                    curves: vec![GaussianCurve {
                        mean: 1.0,
                        deviation: 2.0,
                        max: 3.0
                    }]
                },
                ProbabilityDistributionFunction {
                    curves: vec![GaussianCurve {
                        mean: 4.0,
                        deviation: 5.0,
                        max: 6.0
                    }]
                }
            ]
        );
    }
}
