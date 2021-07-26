extern crate rand;

use crate::distribs::distribution::*;
use std::cell::Cell;

#[allow(dead_code)]
pub struct Geometric {
    p: f64,
}

#[allow(dead_code)]
impl Geometric {
    pub fn new(prob: f64) -> Geometric {
        Geometric { p: prob }
    }
}

impl Distribution<u64> for Geometric {
    fn sample(&self) -> RandomVariable<u64> {
        let prob = rand::random::<f64>();
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
        self.p.recip()
    }

    fn sigma(&self) -> f64 {
        (1.0f64 - self.p) * self.p.powi(2).recip()
    }

    fn pdf(&self, x: u64) -> f64 {
        (1.0f64 - self.p).powf((x - 1) as f64) * self.p
    }

    fn cdf(&self, x: u64) -> f64 {
        1.0f64 - (1.0f64 - self.p).powf(x as f64)
    }
}
