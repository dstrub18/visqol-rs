#![allow(warnings)]
use crate::audio_utils::{
    float_vec_to_real_valued_complex_vec, real_valued_complex_vec_to_float_vec,
};
use ndarray::Array2;
use num::{complex::Complex64, Zero};

// Glasberg and Moore Parameters
const EAR_Q: f64 = 9.26449f64;
const MIN_BW: f64 = 24.7f64;
const ERB_ORDER: f64 = 1.0;

/// Computes the coefficients for an ERB filterbank.
pub fn make_filters<const NUM_BANDS: usize>(
    sample_rate: usize,
    low_freq: f64,
    high_freq: f64,
) -> (Array2<f64>, Vec<f64>) {
    let mut high_freq = high_freq;
    if (high_freq > sample_rate as f64 / 2.0) {
        log::warn!("EquivalentRectangularBandwidth::MakeFilters: high_freq >= (sample_rate / 2), for sample_rate={}, high_freq={}. Falling back to (sample_rate / 2)", sample_rate, high_freq);
        high_freq = sample_rate as f64 / 2.0;
    }

    let pi = std::f64::consts::PI;
    let cf = float_vec_to_real_valued_complex_vec(
        &calculate_uniform_center_freqs::<{ NUM_BANDS }>(low_freq, high_freq),
    );

    let mut B = [Complex64::zero(); NUM_BANDS];
    let mut B1 = [Complex64::zero(); NUM_BANDS];

    for (B_element, cf_element) in B.iter_mut().zip(&cf) {
        let erb =
            ((cf_element / EAR_Q).powf(ERB_ORDER) + MIN_BW.powf(ERB_ORDER)).powf(1.0 / ERB_ORDER);
        *B_element = 1.019 * 2.0 * pi * erb;
    }
    let t = 1.0 / sample_rate as f64;

    let mut exp_bt = [Complex64::zero(); NUM_BANDS];

    for (exp, b_element) in exp_bt.iter_mut().zip(&B) {
        *exp = (*b_element * t).exp();
    }

    let mut B1 = [Complex64::zero(); NUM_BANDS];
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

    let A0 = [t; NUM_BANDS];
    let A2 = [0.0f64; NUM_BANDS];
    let B0 = [1.0f64; NUM_BANDS];
    let mut vf_coeffs = ndarray::Array2::<f64>::zeros((NUM_BANDS, 10));
    // Setup matrix
    for i in 0..NUM_BANDS {
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
fn calculate_uniform_center_freqs<const NUM_BANDS: usize>(
    low_freq: f64,
    high_freq: f64,
) -> [f64; NUM_BANDS] {
    // Glasberg and Moore Parameters

    let a = -(EAR_Q * MIN_BW);
    let b = -((high_freq + EAR_Q * MIN_BW).ln());
    let c = (low_freq + EAR_Q * MIN_BW).ln();
    let d = high_freq + EAR_Q * MIN_BW;
    let e = (b + c) / NUM_BANDS as f64;
    let mut coefficients = [0.0; NUM_BANDS];
    for (i, coefficient) in coefficients.iter_mut().enumerate() {
        let f = ((i as f64 + 1.0) * e).exp() * d;
        *coefficient = a + f;
    }
    coefficients
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use ndarray::Axis;

    use super::*;
    #[test]
    fn erb_coefficients_are_computed_correctly() {
        let fs = 48000;
        const NUM_BANDS: usize = 32;
        let min_freq = 50.0f64;

        let (mut filter_coeffs, _) = make_filters::<NUM_BANDS>(fs, min_freq, fs as f64 / 2.0);

        let expected_filter_coefficients = vec![
            2.08333e-05,
            -2.10773e-05,
            -2.04216e-05,
            -2.08057e-05,
            -2.06932e-05,
            0.0,
            1.0,
            -1.99194,
            0.992003,
            3.66499e-10,
            2.08333e-05,
            -2.13371e-05,
            -2.01347e-05,
            -2.08391e-05,
            -2.06327e-05,
            0.0,
            1.0,
            -1.99065,
            0.990811,
            2.09642e-10,
            2.08333e-05,
            -2.16347e-05,
            -1.98045e-05,
            -2.08766e-05,
            -2.05626e-05,
            0.0,
            1.0,
            -1.98908,
            0.989443,
            1.2008e-10,
            2.08333e-05,
            -2.19753e-05,
            -1.94244e-05,
            -2.09187e-05,
            -2.0481e-05,
            0.0,
            1.0,
            -1.98719,
            0.987872,
            6.88196e-11,
            2.08333e-05,
            -2.23648e-05,
            -1.8987e-05,
            -2.09656e-05,
            -2.03861e-05,
            0.0,
            1.0,
            -1.98488,
            0.986069,
            3.94549e-11,
            2.08333e-05,
            -2.28097e-05,
            -1.84834e-05,
            -2.10177e-05,
            -2.02754e-05,
            0.0,
            1.0,
            -1.98207,
            0.983999,
            2.26268e-11,
            2.08333e-05,
            -2.33175e-05,
            -1.79036e-05,
            -2.1075e-05,
            -2.01461e-05,
            0.0,
            1.0,
            -1.97861,
            0.981626,
            1.29804e-11,
            2.08333e-05,
            -2.38964e-05,
            -1.7236e-05,
            -2.11376e-05,
            -1.99948e-05,
            0.0,
            1.0,
            -1.97436,
            0.978904,
            7.44929e-12,
            2.08333e-05,
            -2.45551e-05,
            -1.64674e-05,
            -2.12051e-05,
            -1.98174e-05,
            0.0,
            1.0,
            -1.96908,
            0.975784,
            4.27683e-12,
            2.08333e-05,
            -2.53035e-05,
            -1.55824e-05,
            -2.12768e-05,
            -1.9609e-05,
            0.0,
            1.0,
            -1.96252,
            0.972209,
            2.45661e-12,
            2.08333e-05,
            -2.61516e-05,
            -1.45634e-05,
            -2.13516e-05,
            -1.93634e-05,
            0.0,
            1.0,
            -1.95432,
            0.968115,
            1.41185e-12,
            2.08333e-05,
            -2.71104e-05,
            -1.33906e-05,
            -2.14274e-05,
            -1.90735e-05,
            0.0,
            1.0,
            -1.94404,
            0.96343,
            8.11924e-13,
            2.08333e-05,
            -2.81904e-05,
            -1.20411e-05,
            -2.15011e-05,
            -1.87303e-05,
            0.0,
            1.0,
            -1.93111,
            0.958071,
            4.67256e-13,
            2.08333e-05,
            -2.94021e-05,
            -1.04891e-05,
            -2.15681e-05,
            -1.83232e-05,
            0.0,
            1.0,
            -1.91478,
            0.951946,
            2.69126e-13,
            2.08333e-05,
            -3.07545e-05,
            -8.70611e-06,
            -2.16217e-05,
            -1.78388e-05,
            0.0,
            1.0,
            -1.89411,
            0.944953,
            1.55156e-13,
            2.08333e-05,
            -3.22539e-05,
            -6.66023e-06,
            -2.16526e-05,
            -1.72615e-05,
            0.0,
            1.0,
            -1.86788,
            0.936976,
            8.95482e-14,
            2.08333e-05,
            -3.39024e-05,
            -4.31725e-06,
            -2.16478e-05,
            -1.65718e-05,
            0.0,
            1.0,
            -1.83454,
            0.927888,
            5.17476e-14,
            2.08333e-05,
            -3.5695e-05,
            -1.64128e-06,
            -2.15895e-05,
            -1.57468e-05,
            0.0,
            1.0,
            -1.79214,
            0.917548,
            2.99468e-14,
            2.08333e-05,
            -3.7616e-05,
            1.40338e-06,
            -2.14536e-05,
            -1.4759e-05,
            0.0,
            1.0,
            -1.7382,
            0.905802,
            1.73591e-14,
            2.08333e-05,
            -3.96336e-05,
            4.84914e-06,
            -2.12082e-05,
            -1.35762e-05,
            0.0,
            1.0,
            -1.66965,
            0.892484,
            1.00816e-14,
            2.08333e-05,
            -4.16933e-05,
            8.72003e-06,
            -2.08114e-05,
            -1.21618e-05,
            0.0,
            1.0,
            -1.58272,
            0.877413,
            5.86779e-15,
            2.08333e-05,
            -4.37082e-05,
            1.30236e-05,
            -2.02091e-05,
            -1.04755e-05,
            0.0,
            1.0,
            -1.47286,
            0.8604,
            3.42376e-15,
            2.08333e-05,
            -4.55475e-05,
            1.77389e-05,
            -1.93334e-05,
            -8.47518e-06,
            0.0,
            1.0,
            -1.33481,
            0.841247,
            2.00343e-15,
            2.08333e-05,
            -4.7022e-05,
            2.27984e-05,
            -1.81014e-05,
            -6.12215e-06,
            0.0,
            1.0,
            -1.16273,
            0.819753,
            1.17616e-15,
            2.08333e-05,
            -4.78682e-05,
            2.80631e-05,
            -1.64164e-05,
            -3.38866e-06,
            0.0,
            1.0,
            -0.950643,
            0.795718,
            6.93091e-16,
            2.08333e-05,
            -4.7734e-05,
            3.32893e-05,
            -1.41731e-05,
            -2.71695e-07,
            0.0,
            1.0,
            -0.69335,
            0.768954,
            4.10182e-16,
            2.08333e-05,
            -4.61725e-05,
            3.80879e-05,
            -1.12707e-05,
            3.18611e-06,
            0.0,
            1.0,
            -0.388059,
            0.739294,
            2.43946e-16,
            2.08333e-05,
            -4.26543e-05,
            4.18827e-05,
            -7.63796e-06,
            6.86629e-06,
            0.0,
            1.0,
            -0.0370402,
            0.706603,
            1.45896e-16,
            2.08333e-05,
            -3.66218e-05,
            4.38804e-05,
            -3.27671e-06,
            1.05353e-05,
            0.0,
            1.0,
            0.348412,
            0.670801,
            8.78132e-17,
            2.08333e-05,
            -2.76126e-05,
            4.30861e-05,
            1.67174e-06,
            1.38017e-05,
            0.0,
            1.0,
            0.742727,
            0.631875,
            5.32319e-17,
            2.08333e-05,
            -1.54917e-05,
            3.84163e-05,
            6.8377e-06,
            1.60868e-05,
            0.0,
            1.0,
            1.10038,
            0.589903,
            3.25068e-17,
            2.08333e-05,
            -8.16567e-07,
            2.89922e-05,
            1.15306e-05,
            1.6645e-05,
            0.0,
            1.0,
            1.35243,
            0.545081,
            1.97665e-17,
        ];

        filter_coeffs.invert_axis(Axis(0));
        assert_eq!(filter_coeffs.shape()[1], 10);
        assert_eq!(filter_coeffs.shape()[0], NUM_BANDS);

        let exptected_filter_coeffs_mat =
            Array2::from_shape_vec((NUM_BANDS, 10), expected_filter_coefficients).unwrap();

        let epsilon = 0.0001;

        // Check if coefficients are the same
        for (&res, ex) in filter_coeffs.iter().zip(exptected_filter_coeffs_mat) {
            assert_abs_diff_eq!(res, ex, epsilon = epsilon);
        }
    }
}
