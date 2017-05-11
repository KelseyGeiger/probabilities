use std::f64::consts;

//Taken from https://codereview.stackexchange.com/a/116898
#[allow(dead_code)]
pub fn gamma(x: f64) -> f64 {
    const COEFFICIENTS: [f64; 29] = [-0.00000000000000000023,
                                     0.00000000000000000141,
                                     0.00000000000000000119,
                                     -0.00000000000000011813,
                                     0.00000000000000122678,
                                     -0.00000000000000534812,
                                     -0.00000000000002058326,
                                     0.00000000000051003703,
                                     -0.00000000000369680562,
                                     0.00000000000778226344,
                                     0.00000000010434267117,
                                     -0.00000000118127457049,
                                     0.00000000500200764447,
                                     0.00000000611609510448,
                                     -0.00000020563384169776,
                                     0.00000113302723198170,
                                     -0.00000125049348214267,
                                     -0.00002013485478078824,
                                     0.00012805028238811619,
                                     -0.00021524167411495097,
                                     -0.00116516759185906511,
                                     0.00721894324666309954,
                                     -0.00962197152787697356,
                                     -0.04219773455554433675,
                                     0.16653861138229148950,
                                     -0.04200263503409523553,
                                     -0.65587807152025388108,
                                     0.57721566490153286061,
                                     1.00000000000000000000];

    const INITIAL: f64 = 0.00000000000000000002;

    COEFFICIENTS
        .iter()
        .fold(INITIAL, |sum, coefficient| sum * (x - 1.0) + coefficient)
        .recip()
}

#[allow(dead_code)]
pub fn large_fact(n: u64) -> f64 {
    gamma((n + 1) as f64)
}

pub fn binomial_coeff(n: u64, k: u64) -> f64 {
    large_fact(n) / (large_fact(n - k) * large_fact(k))
}

pub fn erf(x: f64) -> f64 {
    const ERF_P: f64 = 0.3275911f64;
    const ERF_COEFF: [f64; 5] = [0.254829592,
                                 -0.284496736,
                                 1.421413741,
                                 -1.453152027,
                                 1.061405429];

    let t = (1.0f64 + ERF_P * x).recip();

    let mut sum: f64 = 0.0f64;

    for i in 0i32..4i32 {
        sum += ERF_COEFF[i as usize] * ERF_P.powi(i + 1);
    }

    let exp_factor = -(x.powi(2)).exp();

    1.0f64 - (sum * exp_factor)
}

pub fn phi(x: f64) -> f64 {
    0.5f64 * (1.0f64 + erf(x * 2.0f64.sqrt().recip()))
}
