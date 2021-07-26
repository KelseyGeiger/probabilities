extern crate rand;

pub use self::rand::Closed01;

use crate::distribs::distribution::*;
use std::cell::Cell;

#[allow(dead_code)]
pub struct Exponential {
    lambda: f64,
}

impl Exponential {
    pub fn new(rate: f64) -> Exponential {
        Exponential { lambda: rate }
    }
}

impl Distribution<f64> for Exponential {
    //Using the Inverse Transfom Sampling method (https://en.wikipedia.org/wiki/Inverse_transform_sampling)
    fn sample(&self) -> RandomVariable<f64> {
        let Closed01(prob) = rand::random::<Closed01<f64>>();
        let factor = -self.lambda.recip();

        RandomVariable { value: Cell::new(factor * (1.0f64 - prob).ln()) }
    }

    fn mu(&self) -> f64 {
        self.lambda.recip()
    }

    fn sigma(&self) -> f64 {
        self.lambda.recip().powi(2)
    }

    fn pdf(&self, x: f64) -> f64 {
        self.lambda * (-self.lambda * x).exp()
    }

    fn cdf(&self, x: f64) -> f64 {
        1.0f64 - (-self.lambda * x).exp()
    }
}
