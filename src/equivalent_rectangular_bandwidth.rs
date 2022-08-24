#![allow(warnings)]
use crate::misc_audio::{
    float_vec_to_real_valued_complex_vec, real_valued_complex_vec_to_float_vec,
};
use log;
use ndarray::Array2;
use num::{complex::Complex64, Zero};

// Glasberg and Moore Parameters
const EAR_Q: f64 = 9.26449f64;
const MIN_BW: f64 = 24.7f64;
const ERB_ORDER: f64 = 1.0;

/// Computes the coefficients for an ERB filterbank.
pub fn make_filters(
    sample_rate: usize,
    num_bands: usize,
    low_freq: f64,
    high_freq: f64,
) -> (Array2<f64>, Vec<f64>) {
    let mut high_freq = high_freq;
    if (high_freq > sample_rate as f64 / 2.0) {
        log::warn!("EquivalentRectangularBandwidth::MakeFilters: high_freq >= (sample_rate / 2), for sample_rate={}, high_freq={}. Falling back to (sample_rate / 2)", sample_rate, high_freq);
        high_freq = sample_rate as f64 / 2.0;
    }

    let pi = std::f64::consts::PI;
    let cf = float_vec_to_real_valued_complex_vec(&calculate_uniform_center_freqs(
        low_freq, high_freq, num_bands,
    ));

    let mut B = vec![Complex64::zero(); num_bands];
    let mut B1 = vec![Complex64::zero(); num_bands];

    for (B_element, cf_element) in B.iter_mut().zip(&cf) {
        let erb =
            ((cf_element / EAR_Q).powf(ERB_ORDER) + MIN_BW.powf(ERB_ORDER)).powf(1.0 / ERB_ORDER);
        *B_element = 1.019 * 2.0 * pi * erb;
    }
    let t = 1.0 / sample_rate as f64;

    let mut exp_bt = vec![Complex64::zero(); num_bands];

    for (exp, b_element) in exp_bt.iter_mut().zip(&B) {
        *exp = (*b_element * t).exp();
    }

    let mut B1 = vec![Complex64::zero(); num_bands];
    for i in 0..B1.len() {
        B1[i] = -2.0 * (2.0 * cf[i] * pi * t).cos() / exp_bt[i];
    }

    // b2
    let mut B2 = B.clone();
    B2.iter_mut()
        .for_each(|element| *element = (*element * t * -2.0).exp());

    // b1
    let mut b1 = cf.clone();
    b1.iter_mut()
        .for_each(|element| *element = (*element * 2.0 * pi * t).sin() * t);

    let mut bPos = b1.clone();
    bPos.iter_mut()
        .for_each(|element| *element = *element * 2.0 * (3.0 + 2.0f64.powf(1.5)).sqrt());

    let mut bNeg = b1.clone();
    bNeg.iter_mut()
        .for_each(|element| *element = *element * 2.0 * (3.0 + -(2.0f64.powf(1.5))).sqrt());

    let mut a = cf.clone();
    a.iter_mut()
        .for_each(|element| *element = (*element * 2.0 * pi * t).cos() * 2.0 * t);

    let mut A11 = vec![Complex64::zero(); a.len()];
    A11.iter_mut()
        .enumerate()
        .for_each(|(i, element)| *element = -(a[i] / exp_bt[i] + bPos[i] / exp_bt[i]) / 2.0f64);

    let mut A12 = vec![Complex64::zero(); a.len()];
    A12.iter_mut()
        .enumerate()
        .for_each(|(i, element)| *element = -(a[i] / exp_bt[i] - bPos[i] / exp_bt[i]) / 2.0f64);

    let mut A13 = vec![Complex64::zero(); a.len()];
    A13.iter_mut()
        .enumerate()
        .for_each(|(i, element)| *element = -(a[i] / exp_bt[i] + bNeg[i] / exp_bt[i]) / 2.0f64);

    let mut A14 = vec![Complex64::zero(); a.len()];
    A14.iter_mut()
        .enumerate()
        .for_each(|(i, element)| *element = -(a[i] / exp_bt[i] - bNeg[i] / exp_bt[i]) / 2.0f64);

    // setup gain variables
    let i = Complex64::new(0.0, 1.0);
    let p1 = 2.0f64.powf(3.0 / 2.0);
    let s1 = (3.0 - p1).sqrt();
    let s2 = (3.0 + p1).sqrt();
    let mut xExp = cf.clone();
    xExp.iter_mut()
        .for_each(|element| *element = (4.0 * i * *element * pi * t).exp());

    let mut x01 = xExp.clone();
    x01.iter_mut()
        .for_each(|element| *element = -2.0 * *element * t);

    let mut x02 = cf.clone();
    x02.iter_mut().zip(&B).for_each(|(x02_element, B_element)| {
        *x02_element = 2.0 * (-(B_element * t) + 2.0 * i * *x02_element * pi * t).exp() * t
    });

    let mut xcos = cf.clone();
    xcos.iter_mut()
        .for_each(|element| *element = (2.0 * *element * pi * t).cos());
    let mut xsin = cf.clone();
    xsin.iter_mut()
        .for_each(|element| *element = (2.0 * *element * pi * t).sin());

    // calculate gain
    let mut x12 = xcos.clone();
    x12.iter_mut()
        .zip(&xsin)
        .for_each(|(cos_element, sin_element)| *cos_element = *cos_element - (s1 * sin_element));

    let mut x1 = x01.clone();
    x1.iter_mut()
        .zip(&x02)
        .zip(x12)
        .for_each(|((element, x02_element), x12_element)| {
            *element = *element + (x02_element * x12_element)
        });

    let mut x22 = xcos.clone();
    x22.iter_mut()
        .zip(&xsin)
        .for_each(|(cos_element, sin_element)| *cos_element = *cos_element + (s1 * sin_element));

    let mut x2 = x01.clone();
    x2.iter_mut()
        .zip(&x02)
        .zip(&x22)
        .for_each(|((element, x02_element), x22_element)| {
            *element = *element + (x02_element * x22_element)
        });

    let mut x32 = xcos.clone();
    x32.iter_mut()
        .zip(&xsin)
        .for_each(|(cos_element, sin_element)| *cos_element = *cos_element - (s2 * sin_element));

    let mut x3 = x01.clone();
    x3.iter_mut()
        .zip(&x02)
        .zip(&x32)
        .for_each(|((element, x02_element), x32_element)| {
            *element = *element + (x02_element * x32_element)
        });

    let mut x42 = xcos.clone();
    x42.iter_mut()
        .zip(&xsin)
        .for_each(|(cos_element, sin_element)| *cos_element = *cos_element + (s2 * sin_element));

    let mut x4 = x01.clone();
    x4.iter_mut()
        .zip(&x02)
        .zip(&x42)
        .for_each(|((element, x02_element), x42_element)| {
            *element = *element + (x02_element * x42_element)
        });

    let mut x5 = B.clone();
    x5.iter_mut()
        .zip(&xExp)
        .for_each(|(element, xExp_element)| {
            *element = (-2.0 / (2.0 * *element * t).exp()) - 2.0 * xExp_element
                + (2.0 * (1.0 + xExp_element)) / (*element * t).exp()
        });

    let mut y = x5.clone();
    y.iter_mut()
        .for_each(|element| *element = element.powf(4.0));

    let mut gain = vec![0.0f64; x01.len()];
    for i in 0..gain.len() {
        gain[i] = ((x1[i] * x2[i] * x3[i] * x4[i]) / x5[i].powf(4.0)).norm();
    }

    let A0 = vec![t; num_bands];
    let A2 = vec![0.0f64; num_bands];
    let B0 = vec![1.0f64; num_bands];
    let mut vf_coeffs = ndarray::Array2::<f64>::zeros((num_bands, 10));
    // Setup matrix
    for i in 0..num_bands {
        vf_coeffs[(i, 0)] = A0[i];
        vf_coeffs[(i, 1)] = A11[i].re;
        vf_coeffs[(i, 2)] = A12[i].re;
        vf_coeffs[(i, 3)] = A13[i].re;
        vf_coeffs[(i, 4)] = A14[i].re;
        vf_coeffs[(i, 5)] = A2[i];
        vf_coeffs[(i, 6)] = B0[i];
        vf_coeffs[(i, 7)] = B1[i].re;
        vf_coeffs[(i, 8)] = B2[i].re;
        vf_coeffs[(i, 9)] = gain[i];
    }

    (vf_coeffs, real_valued_complex_vec_to_float_vec(&cf))
}

/// Given a lower frequency boundary, a higher frequency boundary and the number of bands, this function calculates the center frequencies on an ERB scale.
fn calculate_uniform_center_freqs(low_freq: f64, high_freq: f64, num_bands: usize) -> Vec<f64> {
    // Glasberg and Moore Parameters

    let a = -(EAR_Q * MIN_BW);
    let b = -((high_freq + EAR_Q * MIN_BW).ln());
    let c = (low_freq + EAR_Q * MIN_BW).ln();
    let d = high_freq + EAR_Q * MIN_BW;
    let e = (b + c) / num_bands as f64;
    let mut coefficients = vec![0.0; num_bands];
    for (i, coefficient) in coefficients.iter_mut().enumerate() {
        let f = ((i as f64 + 1.0) * e).exp() * d;
        *coefficient = a + f;
    }
    coefficients
}
