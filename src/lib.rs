pub mod util;
pub mod distribs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        use distribs::*;

        let disc_uni = DiscreteUniform::new(-5, 5);
        let cont_uni = ContinuousUniform::new(0.0f64, 1.0f64);
        let bin = Binomial::new(20, 0.5f64);
        let neg_bin = NegativeBinomial::new(0.6, 5);
        let geom = Geometric::new(0.5);
    }
}
