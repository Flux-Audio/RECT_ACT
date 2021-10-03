// by Flux-Audio, some rights reserved
// This code is licensed under MIT license (see LICENSE.md for details)


/// Leaky ReLU activation function
/// 
/// # Parameters
/// - `x`: input
/// - `alpha`: rectification amount (0 = linear, 0.5 = half-wave rect, 1 = full rectification)
pub fn lrelu (x: f64, alpha: f64) -> f64 {
    ((1.0 - 2.0 * alpha) * x).max(x)
}

pub fn maxTanh (x: f64, alpha: f64) -> f64 {
    ((1.0 - 2.0 * alpha) * x).tanh().max(x)
}

pub fn softPlus (x: f64, alpha: f64) -> f64 {
    let four_a = 4.0 * alpha;
    let temp = (four_a + 1.0).log2();
    ((four_a + (3.0 * x).exp2()).log2() - temp) / ((four_a + 8.0).log2() - temp)
}

pub fn swish (x: f64, alpha: f64) -> f64 {
    let eight_a = 8.0 * alpha;
    (-eight_a).exp2() * (eight_a.exp2() + 1.0) * x / ((-eight_a * x).exp2() + 1.0)
}

pub fn mish (x: f64, alpha: f64) -> f64 {
    let temp1 = (-2.0 * alpha + 4.0).powf(2.0);
    let temp2 = (-2.0 * alpha + x.exp2() + 2.0).powf(2.0);
    (temp1 + 1.0) * x * (temp2 - 1.0) / ((temp1 - 1.0) * (temp2 + 1.0))
}

pub fn softTanh (x: f64, alpha: f64) -> f64 {
    -x * ((-8.0 + 7.875 * alpha) * ((1.0 + 5.0 * alpha) * x).exp()).tanh()
}