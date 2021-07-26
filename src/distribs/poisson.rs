extern crate rand;

pub use self::rand::Closed01;

use crate::distribs::distribution::*;
use crate::util::math::*;
use std::cell::Cell;

#[allow(dead_code)]
pub struct Poisson {
    lambda: f64,
}

#[allow(dead_code)]
impl Poisson {
    pub fn new(rate: f64) -> Poisson {
        Poisson { lambda: rate }
    }
}

impl Distribution<u64> for Poisson {
    fn sample(&self) -> RandomVariable<u64> {
        let Closed01(prob) = rand::random::<Closed01<f64>>();
        let mut cum_prob: f64 = 0.0f64;
        let mut k: u64 = 0u64;

        for _ in 0..(u64::max_value() - 1) {
            cum_prob += self.pdf(k);

            if cum_prob > prob {
                break;
            }

            k += 1
        }

        RandomVariable { value: Cell::new(k) }
    }

    fn mu(&self) -> f64 {
        self.lambda
    }

    fn sigma(&self) -> f64 {
        self.lambda.sqrt()
    }

    fn pdf(&self, x: u64) -> f64 {
        (-self.lambda).exp() * (self.lambda.powf(x as f64)) / large_fact(x)
    }

    fn cdf(&self, x: u64) -> f64 {
        (0..x).fold(0.0f64, |sum, next| sum + self.pdf(next))
    }
}
