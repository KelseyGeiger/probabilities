pub use std::cell::*;

trait Distribution<Sample: Sized, Range> {
    fn sample<'a>(&self) -> RandomVariable<'a, Sample, Range>;

    fn mu(self) -> f64;
    fn sigma(self) -> f64;

    fn range(self) -> Range;

    fn pdf(self, x: Sample) -> Option<f64>;
    fn cdf(self, x: Sample) -> Option<f64>;
}

#[allow(dead_code)]
struct RandomVariable<'dist, Sample: Sized + 'dist, Range: 'dist> {
    distribution: &'dist Distribution<Sample, Range>,
    value: Cell<Sample>,
}
