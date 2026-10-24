use crate::fft_manager::FftManager;
use num::complex::Complex64;
use num::Zero;
use rustfft::FftPlanner;

/// Calculate the maximum delay between to signals.
pub fn calculate_best_lag(signal_1: &[f64], signal_2: &[f64]) -> Option<i64> {
    let max_lag = ((signal_1.len().max(signal_2.len())) - 1) as i64;

    let (inverse_fft, fft_points, inverse_fft_scale) =
        calculate_unscaled_inverse_fft_pointwise_product(signal_1, signal_2);
    let point_wise_fft_vec = &inverse_fft[..fft_points];

    // Negative errors followed by positive errors
    let corrs = point_wise_fft_vec[point_wise_fft_vec.len() - max_lag as usize..]
        .iter()
        .chain(&point_wise_fft_vec[0..max_lag as usize + 1])
        .map(|element| element.re * inverse_fft_scale);

    // Get maximum
    let best_corr = corrs.clone().max_by(|x, y| {
        x.abs()
            .partial_cmp(&y.abs())
            .expect("Failed to compute correlation")
    })?;

    let best_corr_idx = corrs.clone().position(|r| r == best_corr)?;

    Some(best_corr_idx as i64 - max_lag)
}

/// Calculates the pointwise inverse fft product of 2 signals
#[cfg(test)]
pub fn calculate_inverse_fft_pointwise_product(signal_1: &[f64], signal_2: &[f64]) -> Vec<f64> {
    let (inverse_fft, fft_points, inverse_fft_scale) =
        calculate_unscaled_inverse_fft_pointwise_product(signal_1, signal_2);

    inverse_fft[..fft_points]
        .iter()
        .map(|element| element.re * inverse_fft_scale)
        .collect()
}

/// Calculates the inverse fft of the pointwise fft product of 2 signals without applying the inverse fft scaling.
/// Returns the complex result, the number of valid points in it and the scale factor to apply to its real parts.
fn calculate_unscaled_inverse_fft_pointwise_product(
    signal_1: &[f64],
    signal_2: &[f64],
) -> (Vec<Complex64>, usize, f64) {
    let biggest_length = signal_1.len().max(signal_2.len());
    let (_, exp) = frexp((biggest_length * 2 - 1) as f64);
    let fft_points = 2usize.pow(exp as u32);
    let fft_size = FftManager::fft_size_for(fft_points);

    let (mut point_wise_product, mut output_buffer) =
        calculate_fft_pointwise_product_and_spent_buffer(signal_1, signal_2, fft_size);

    // The forward plan is dropped at this point, so only one plan is alive at a time.
    let inverse_fft = FftPlanner::<f64>::new().plan_fft_inverse(fft_size);
    let mut scratch = vec![Complex64::zero(); inverse_fft.get_outofplace_scratch_len()];
    inverse_fft.process_outofplace_with_scratch(
        &mut point_wise_product,
        &mut output_buffer,
        &mut scratch,
    );

    (output_buffer, fft_points, 1.0f64 / (fft_size as f64))
}

/// Calculates the pointwise fft product of 2 signals
#[cfg(test)]
pub fn calculate_fft_pointwise_product(
    signal_1: &[f64],
    signal_2: &[f64],
    manager: &FftManager,
) -> Vec<Complex64> {
    calculate_fft_pointwise_product_and_spent_buffer(signal_1, signal_2, manager.fft_size).0
}

/// Calculates the pointwise product of the spectrum of `signal_1` and the conjugated spectrum of `signal_2`, both zero-padded to `fft_size`.
/// Also returns the spent fft input buffer, so that the caller can reuse it instead of allocating a new one.
fn calculate_fft_pointwise_product_and_spent_buffer(
    signal_1: &[f64],
    signal_2: &[f64],
    fft_size: usize,
) -> (Vec<Complex64>, Vec<Complex64>) {
    let forward_fft = FftPlanner::<f64>::new().plan_fft_forward(fft_size);
    let mut scratch = vec![Complex64::zero(); forward_fft.get_outofplace_scratch_len()];

    // The out-of-place fft uses its input as scratch space, so the input buffer is refilled for each signal.
    let mut input_buffer = vec![Complex64::zero(); fft_size];

    copy_zero_padded(signal_2, &mut input_buffer);
    let mut fft_signal_2 = vec![Complex64::zero(); fft_size];
    forward_fft.process_outofplace_with_scratch(&mut input_buffer, &mut fft_signal_2, &mut scratch);

    copy_zero_padded(signal_1, &mut input_buffer);
    let mut fft_signal_1 = vec![Complex64::zero(); fft_size];
    forward_fft.process_outofplace_with_scratch(&mut input_buffer, &mut fft_signal_1, &mut scratch);

    fft_signal_1
        .iter_mut()
        .zip(&fft_signal_2)
        .for_each(|(a, b)| *a = *a * b.conj());

    (fft_signal_1, input_buffer)
}

/// Copies `signal` into the real parts of `buffer` and zeroes everything else.
fn copy_zero_padded(signal: &[f64], buffer: &mut [Complex64]) {
    let (signal_part, padding) = buffer.split_at_mut(signal.len());
    signal_part
        .iter_mut()
        .zip(signal)
        .for_each(|(element, &sample)| *element = Complex64::new(sample, 0.0));
    padding.fill(Complex64::zero());
}

///
/// Returns the mantissa and the exponent of a given floating point value.
pub fn frexp(s: f64) -> (f64, i32) {
    if 0.0 == s {
        (s, 0)
    } else {
        let lg = s.abs().log2();
        let x = (lg - lg.floor() - 1.0).exp2();
        let exp = lg.floor() + 1.0;
        (s.signum() * x, exp as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;

    #[test]
    fn best_lag_signals_have_equal_length() {
        let ref_signal = vec![
            2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0,
        ];
        let deg_signal_lag2 = vec![
            1.2, 0.1, -3.3, 0.1, 1.1, 2.2, 2.1, 7.1, 8.3, 6.8, 2.4, 2.2, 2.2, 2.1,
        ];

        assert_eq!(deg_signal_lag2.len(), 14);
        let ref_signal_mat = Array1::from_vec(ref_signal);
        let deg_signal_lag2_mat = Array1::from_vec(deg_signal_lag2);
        assert_eq!(ref_signal_mat.len(), deg_signal_lag2_mat.len());
        let best_lag = calculate_best_lag(
            ref_signal_mat.as_slice().unwrap(),
            deg_signal_lag2_mat.as_slice().unwrap(),
        )
        .unwrap();

        let expected_result = 2;
        assert_eq!(best_lag, expected_result);
    }

    #[test]
    fn best_lag_reference_is_shorter_than_degraded() {
        let ref_signal = vec![
            2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0,
        ];
        let deg_signal_lag2 = vec![
            1.2, 0.1, -3.3, 0.1, 1.1, 2.2, 2.1, 7.1, 8.3, 6.8, 2.4, 2.2, 2.2, 2.1, 2.0,
        ];

        assert!(ref_signal.len() < deg_signal_lag2.len());
        let ref_signal_mat = Array1::from_vec(ref_signal);
        let deg_signal_lag2_mat = Array1::from_vec(deg_signal_lag2);
        let best_lag = calculate_best_lag(
            ref_signal_mat.as_slice().unwrap(),
            deg_signal_lag2_mat.as_slice().unwrap(),
        )
        .unwrap();

        let expected_result = 2;
        assert_eq!(best_lag, expected_result);
    }

    #[test]
    fn best_lag_reference_is_longer_than_degraded() {
        let ref_signal = vec![
            2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0,
        ];
        let deg_signal_lag2 = vec![
            1.2, 0.1, -3.3, 0.1, 1.1, 2.2, 2.1, 7.1, 8.3, 6.8, 2.4, 2.2, 2.2,
        ];
        assert!(ref_signal.len() > deg_signal_lag2.len());

        let ref_signal_mat = Array1::from_vec(ref_signal);
        let deg_signal_lag2_mat = Array1::from_vec(deg_signal_lag2);
        let best_lag = calculate_best_lag(
            ref_signal_mat.as_slice().unwrap(),
            deg_signal_lag2_mat.as_slice().unwrap(),
        )
        .unwrap();

        let expected_result = 2;
        assert_eq!(best_lag, expected_result);
    }
    #[test]
    fn best_lag_is_negative() {
        let ref_signal = vec![
            2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0,
        ];
        let deg_signal_lag2 = vec![
            2.0, 2.0, 2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0,
        ];

        let ref_signal_mat = Array1::from_vec(ref_signal);
        let deg_signal_lag2_mat = Array1::from_vec(deg_signal_lag2);
        let best_lag = calculate_best_lag(
            ref_signal_mat.as_slice().unwrap(),
            deg_signal_lag2_mat.as_slice().unwrap(),
        )
        .unwrap();

        let expected_result = -2;
        assert_eq!(best_lag, expected_result);
    }

    #[test]
    fn test_frexp() {
        let (_, result) = frexp(27.0f64);
        let expected_result = 5;

        assert_eq!(result, expected_result);
    }
}
