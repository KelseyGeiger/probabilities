pub use std::cell::*;
pub use std::clone::*;
pub use std::option::*;

pub trait Distribution<Sample: Sized> {
    fn sample(&self) -> RandomVariable<Sample>;

    fn mu(&self) -> f64;
    fn sigma(&self) -> f64;

    fn pdf(&self, x: Sample) -> Option<f64>;
    fn cdf(&self, x: Sample) -> Option<f64>;
}

#[allow(dead_code)]
pub struct RandomVariable<Sample: Sized> {
    pub value: Cell<Sample>,
}

impl<Sample: Sized + Copy> RandomVariable<Sample> {
    pub fn new(val: Sample) -> RandomVariable<Sample> {
        RandomVariable { value: Cell::new(val) }
    }
}
