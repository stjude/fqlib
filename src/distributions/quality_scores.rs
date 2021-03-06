use rand::{distributions::Distribution, Rng};
use rand_distr::Normal;

const MIN: f64 = 0.0;
const MAX: f64 = 41.0;

const MEAN: f64 = (MIN + MAX) as f64 / 2.0;
// std_dev = sqrt(MEAN / 3.0)
const STD_DEV: f64 = 2.61;

pub struct QualityScores {
    distribution: Normal<f64>,
}

impl Default for QualityScores {
    fn default() -> Self {
        Self {
            // Std. dev. is never < 0.0.
            distribution: Normal::new(MEAN, STD_DEV).unwrap(),
        }
    }
}

impl Distribution<u8> for QualityScores {
    fn sample<R>(&self, rng: &mut R) -> u8
    where
        R: Rng + ?Sized,
    {
        let n = self.distribution.sample(rng);
        let score = clamp(n, MIN, MAX).round();
        score as u8
    }
}

fn clamp(n: f64, min: f64, max: f64) -> f64 {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(0.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp(1.0, 0.0, 1.0), 1.0);
        assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(2.0, 0.0, 1.0), 1.0);
    }
}
