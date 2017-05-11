extern crate rand;

pub use self::rand::Closed01;

pub use distribution::*;
pub use std::clone::*;

#[allow(dead_code)]
struct ContinuousUniform {
    start: f64,
    end: f64,
}

#[allow(dead_code)]
impl ContinuousUniform {
    fn new(start: f64, end: f64) -> ContinuousUniform {
        let lesser = if start <= end { start } else { end };
        let greater = if start >= end { start } else { end };

        ContinuousUniform {
            start: lesser,
            end: greater,
        }
    }
}

impl Distribution<f64> for ContinuousUniform {
    fn sample(&self) -> RandomVariable<f64> {
        let Closed01(prob) = rand::random::<Closed01<f64>>();
        let width = self.end - self.start;
        let samp = prob * (width as f64) + self.start as f64;

        RandomVariable { value: Cell::new(samp) }
    }

    fn mu(&self) -> f64 {
        (self.end - self.start) / 2.0f64
    }

    fn sigma(&self) -> f64 {
        ((self.end - self.start) / 12.0f64).sqrt()
    }

    fn pdf(&self, x: f64) -> f64 {
        if x >= self.start && x <= self.end {
            (self.end - self.start).recip()
        } else {
            0.0f64
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        if x < self.start {
            0.0f64
        } else if x <= self.end {
            (x - self.start) / (self.end - self.start)
        } else {
            1.0f64
        }
    }
}
