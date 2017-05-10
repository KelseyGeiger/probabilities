extern crate rand;

pub use self::rand::Closed01;

pub use distribution::*;
pub use std::clone::*;

#[allow(dead_code)]
struct DiscreteUniform {
    pub start: i64,
    pub end: i64,
}

#[allow(dead_code)]
impl DiscreteUniform {
    fn new(start: i64, end: i64) -> DiscreteUniform {
        let less = if start <= end { start } else { end };
        let greater = if start >= end { start } else { end };

        DiscreteUniform {
            start: less,
            end: greater,
        }
    }
}

impl Distribution<i64> for DiscreteUniform {
    fn sample(&self) -> RandomVariable<i64> {
        let Closed01(prob) = rand::random::<Closed01<f64>>();
        let width = self.end - self.start;
        let samp = prob * (width as f64) + self.start as f64;
        let mut value = self.start;

        while (value as f64) < samp {
            if (value + 1) as f64 > samp {
                break;
            }

            value += 1;
        }

        RandomVariable { value: Cell::new(value) }
    }

    fn mu(&self) -> f64 {
        let p = ((self.end - self.start) as f64).recip();

        //E(X) = Sum from x1 to xn of xi * p(xi)
        (self.start..self.end).fold(0.0f64, |sum, next| sum + (next as f64) * p)
    }

    fn sigma(&self) -> f64 {
        let p = ((self.end - self.start) as f64).recip();

        let ex_2 = self.mu();
        let ex_2 = ex_2 * ex_2;

        let e_x2 = (self.start..self.end).fold(0.0f64, |sum, next| {
            let x = next as f64;
            sum + x * x * p
        });

        //sigma(X) = sqrt(Var(X)) = sqrt(E( (X - E(X))^2 )) = sqrt(E(X^2) - E(X)^2)
        (e_x2 - ex_2).sqrt()
    }

    #[allow(unused_variables)]
    fn pdf(&self, x: i64) -> Option<f64> {
        Some(((self.end - self.start) as f64).recip())
    }

    fn cdf(&self, x: i64) -> Option<f64> {
        Some((x as f64) * ((self.end - self.start) as f64).recip())
    }
}
