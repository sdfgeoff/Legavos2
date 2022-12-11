use serde::{Deserialize, Serialize};

/// Represents a single gaussion distribution
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GaussianCurve {
    pub mean: f32,
    pub deviation: f32,
    pub max: f32,
}

impl GaussianCurve {
    pub fn standard_normal(data: f32) -> f32 {
        f32::powf(std::f32::consts::E, -(data * data) / 2.0) / f32::sqrt(2.0 * std::f32::consts::PI)
    }

    pub fn how_likely(&self, data: f32) -> f32 {
        if self.deviation <= 0.0 {
            return 0.0
        }
        self.max / self.deviation * Self::standard_normal((data - self.mean) / self.deviation) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PEAK: f32 = 0.3989423;
    const AT_1: f32 = 0.24197073;

    #[test]
    fn test_standard_normal() {
        assert_eq!(
            GaussianCurve::standard_normal(0.0),
            PEAK
        );
        assert_eq!(
            GaussianCurve::standard_normal(1.0),
            AT_1
        );
        assert_eq!(
            GaussianCurve::standard_normal(-1.0),
            AT_1
        );
    }

    #[test]
    fn test_offset() {
        assert_eq!(
            GaussianCurve {
                mean: 1.0,
                max: 1.0,
                deviation: 1.0,
            }.how_likely(0.0),
            AT_1
        );
        assert_eq!(
            GaussianCurve {
                mean: 1.0,
                max: 1.0,
                deviation: 1.0,
            }.how_likely(2.0),
            AT_1
        );
        assert_eq!(
            GaussianCurve {
                mean: -1.0,
                max: 1.0,
                deviation: 1.0,
            }.how_likely(0.0),
            AT_1
        );
        assert_eq!(
            GaussianCurve {
                mean: -1.0,
                max: 1.0,
                deviation: 1.0,
            }.how_likely(-2.0),
            AT_1
        );
    }
    #[test]
    fn test_stretch_deviation() {
        assert_eq!(
            GaussianCurve {
                mean: 0.0,
                max: 1.0,
                deviation: 2.0,
            }.how_likely(2.0),
            AT_1 / 2.0
        );
        assert_eq!(
            GaussianCurve {
                mean: 1.0,
                max: 1.0,
                deviation: 2.0,
            }.how_likely(3.0),
            AT_1 / 2.0
        );

        assert_eq!(
            GaussianCurve {
                mean: 1.0,
                max: 1.0,
                deviation: -2.0,
            }.how_likely(3.0),
            0.0
        );
    }

    #[test]
    fn test_scale_maximum() {
        assert_eq!(
            GaussianCurve {
                mean: 0.0,
                max: 0.5,
                deviation: 1.0,
            }.how_likely(1.0),
            AT_1 / 2.0
        );
    }
}
