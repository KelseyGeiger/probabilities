extern crate rand;

pub use self::rand::Closed01;

pub use distribs::distribution::*;
pub use std::clone::*;
pub use util::math::*;

#[allow(dead_code)]
pub struct Binomial {
    n: u64,
    p: f64,
}

#[allow(dead_code)]
impl Binomial {
    pub fn new(num: u64, probability: f64) -> Binomial {
        Binomial {
            n: num,
            p: probability,
        }
    }
}

impl Distribution<u64> for Binomial {
    fn sample(&self) -> RandomVariable<u64> {
        let Closed01(prob) = rand::random::<Closed01<f64>>();
        let mut cum_prob: f64 = 0.0f64;
        let mut k: u64 = 0u64;

        for _ in 0..self.n {
            cum_prob += self.pdf(k);

            println!("{}", cum_prob);

            if cum_prob > prob {
                break;
            }

            k += 1
        }

        RandomVariable { value: Cell::new(k) }
    }

    fn mu(&self) -> f64 {
        self.n as f64 * self.p as f64
    }

    fn sigma(&self) -> f64 {
        ((self.n as f64) * (self.p as f64) * (1.0f64 - (self.p as f64))).sqrt()
    }

    fn pdf(&self, x: u64) -> f64 {
        if x <= self.n {
            binomial_coeff(self.n, x) * self.p.powf(x as f64) *
            (1.0f64 - self.p).powf((self.n - x) as f64)
        } else {
            0.0f64
        }
    }

    fn cdf(&self, x: u64) -> f64 {
        if x <= self.n {
            (0..x).fold(0.0f64, |sum, next| sum + self.pdf(next))
        } else {
            1.0f64
        }
    }
}
