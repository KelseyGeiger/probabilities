extern crate rand;

use crate::distribs::distribution::*;
use crate::util::math::*;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Hypergeometric {
    N: u64,
    m: u64,
    n: u64,
}

#[allow(dead_code)]
impl Hypergeometric {
    pub fn new(total: u64, n_different: u64, n_picked: u64) -> Hypergeometric {
        Hypergeometric {
            N: total,
            m: n_different,
            n: n_picked,
        }
    }
}

impl Distribution<u64> for Hypergeometric {
    fn sample(&self) -> RandomVariable<u64> {
        let prob = rand::random::<f64>();
        let low_lim = if self.n < self.m { self.n } else { self.m };
        let mut cum_prob: f64 = 0.0f64;
        let mut k: u64 = 0u64;

        for _ in 0..low_lim {
            cum_prob += self.pdf(k);

            if cum_prob > prob {
                break;
            }

            k += 1
        }

        RandomVariable { value: Cell::new(k) }
    }

    fn mu(&self) -> f64 {
        ((self.n * self.m) as f64) / (self.N as f64)
    }

    fn sigma(&self) -> f64 {
        let mean = self.mu();
        let failure = ((self.N - self.m) as f64) / (self.N as f64);
        let remaining = ((self.N - self.n) as f64) / ((self.N - 1) as f64);

        (mean * failure * remaining).sqrt()
    }

    fn pdf(&self, x: u64) -> f64 {
        binomial_coeff(self.m, x) * binomial_coeff(self.N - self.m, self.N - x) /
        binomial_coeff(self.N, self.n)
    }

    fn cdf(&self, x: u64) -> f64 {
        (0..x).fold(0.0f64, |sum, next| sum + self.pdf(next))
    }
}
