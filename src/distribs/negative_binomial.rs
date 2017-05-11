extern crate rand;

pub use self::rand::Closed01;

pub use distribution::*;
pub use std::clone::*;
pub use util::math::*;

#[allow(dead_code)]
struct NegativeBinomial {
    p: f64,
    k: u64,
}

impl NegativeBinomial {
    fn new(probability: f64, num_successes: u64) -> NegativeBinomial {
        NegativeBinomial {
            p: probability,
            k: num_successes,
        }
    }
}

impl Distribution<u64> for NegativeBinomial {
    fn sample(&self) -> RandomVariable<u64> {
        let Closed01(prob) = rand::random::<Closed01<f64>>();
        let mut cum_prob: f64 = 0.0f64;
        let mut k: u64 = 0u64;

        for _ in 0..self.n {
            cum_prob += self.pdf(k);

            if cum_prob > prob {
                break;
            }

            k += 1
        }

        RandomVariable { value: Cell::new(k) }
    }

    fn mu(&self) -> f64 {
        (self.k as f64) / (self.p)
    }

    fn sigma(&self) -> f64 {
        (self.k as f64) * (1.0f64 - self.p) / (self.p.powi(2))
    }

    fn pdf(&self, x: u64) -> f64 {
        binomial_coeff(x - 1, self.k - 1) * self.p.powi(k) * (1.0f64 - p).powi(x - k)
    }

    fn cdf(&self, x: u64) -> f64 {
        (0..x).fold(0.0f64, |sum, next| sum + self.pdf(next))
    }
}
