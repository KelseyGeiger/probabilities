pub mod util;
pub mod distribs;

pub use util::*;
pub use distribs::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let disc_uni = DiscreteUniform::new(-5, 5);
        let cont_uni = ContinuousUniform::new(0, 1);
        let bin = Binomial::new(20, 0.5f64);
        let neg_bin = NegativeBinomial::new(0.6, 5);
        let geom = Geometric::new(0.5);
    }
}
