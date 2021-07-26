extern crate rand;

use crate::distribs::distribution::*;
use crate::util::math::*;
use std::f64::consts;

#[allow(dead_code)]
pub struct Gaussian {
    mu: f64,
    sigma: f64,
}

impl Gaussian {
    pub fn new(avg: f64, variance: f64) -> Gaussian {
        Gaussian {
            mu: avg,
            sigma: variance.sqrt(),
        }
    }
}

impl Distribution<f64> for Gaussian {
    //Using the Box–Muller transform (https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform)
    fn sample(&self) -> RandomVariable<f64> {
        let u_1 = rand::random::<f64>();
        let u_2 = rand::random::<f64>();

        let u_1_term = (-2.0f64 * (u_1.ln())).sqrt();
        let u_2_term = (2.0f64 * consts::PI * u_2).cos();

        RandomVariable { value: Cell::new(self.sigma * (u_1_term * u_2_term) + self.mu) }
    }

    fn mu(&self) -> f64 {
        self.mu
    }
    fn sigma(&self) -> f64 {
        self.sigma
    }

    fn pdf(&self, x: f64) -> f64 {
        let factor = (2.0f64 * self.sigma * self.sigma * consts::PI)
            .sqrt()
            .recip();
        let exp_num = -((x - self.mu).powi(2));
        let exp_denom = 2.0f64 * self.sigma * self.sigma;

        factor * (exp_num / exp_denom).exp()
    }

    fn cdf(&self, x: f64) -> f64 {
        phi((x - self.mu) / self.sigma)
    }
}
