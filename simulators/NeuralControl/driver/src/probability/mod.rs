mod gaussian_curve;
mod probability_distribution_function;
pub use gaussian_curve::GaussianCurve;
pub use probability_distribution_function::ProbabilityDistributionFunction;

// Creates an array of probability distribution functions from a plain array of
// numbers. Assumes the numbers range between 0 and 1
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
                // tan(pi * (x*2.0 - 1.0) / 2)
                mean: (std::f32::consts::PI * (arr[offset] * 2.0 - 1.0) / 2.0).tan(),
                deviation: (arr[offset + 1]).abs(),
                max: (arr[offset + 2]).abs(),
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
    fn test_from_array_mean() {
        assert_eq!(
            gaussian_approximations_from_array(vec![0.0, 2.0, 3.0], 1),
            vec![ProbabilityDistributionFunction {
                curves: vec![GaussianCurve {
                    mean: 22877334.0,
                    deviation: 2.0,
                    max: 3.0
                }]
            }]
        );
        assert_eq!(
            gaussian_approximations_from_array(vec![1.0, 2.0, 3.0], 1),
            vec![ProbabilityDistributionFunction {
                curves: vec![GaussianCurve {
                    mean: -22877334.0,
                    deviation: 2.0,
                    max: 3.0
                }]
            }]
        );
        assert_eq!(
            gaussian_approximations_from_array(vec![0.6, 2.0, 3.0], 1),
            vec![ProbabilityDistributionFunction {
                curves: vec![GaussianCurve {
                    mean: 0.32491982,
                    deviation: 2.0,
                    max: 3.0
                }]
            }]
        );
        assert_eq!(
            gaussian_approximations_from_array(vec![0.4, 2.0, 3.0], 1),
            vec![ProbabilityDistributionFunction {
                curves: vec![GaussianCurve {
                    mean: -0.32491967,
                    deviation: 2.0,
                    max: 3.0
                }]
            }]
        );
        assert_eq!(
            gaussian_approximations_from_array(vec![0.8, 2.0, 3.0], 1),
            vec![ProbabilityDistributionFunction {
                curves: vec![GaussianCurve {
                    mean: 1.3763821,
                    deviation: 2.0,
                    max: 3.0
                }]
            }]
        );
    }

    #[test]
    fn test_from_array() {
        assert_eq!(
            gaussian_approximations_from_array(vec![0.5, 2.0, 3.0], 1),
            vec![ProbabilityDistributionFunction {
                curves: vec![GaussianCurve {
                    mean: 0.0,
                    deviation: 2.0,
                    max: 3.0
                }]
            }]
        );

        assert_eq!(
            gaussian_approximations_from_array(vec![0.5, 2.0, 3.0, 0.5, 5.0, 6.0], 2),
            vec![ProbabilityDistributionFunction {
                curves: vec![
                    GaussianCurve {
                        mean: 0.0,
                        deviation: 2.0,
                        max: 3.0
                    },
                    GaussianCurve {
                        mean: 0.0,
                        deviation: 5.0,
                        max: 6.0
                    }
                ]
            }]
        );
        assert_eq!(
            gaussian_approximations_from_array(vec![0.5, 2.0, 3.0, 0.5, 5.0, 6.0], 1),
            vec![
                ProbabilityDistributionFunction {
                    curves: vec![GaussianCurve {
                        mean: 0.0,
                        deviation: 2.0,
                        max: 3.0
                    }]
                },
                ProbabilityDistributionFunction {
                    curves: vec![GaussianCurve {
                        mean: 0.0,
                        deviation: 5.0,
                        max: 6.0
                    }]
                }
            ]
        );
    }
}
