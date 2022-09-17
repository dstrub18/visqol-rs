use crate::fast_fourier_transform;
use crate::fft_manager::FftManager;
use num::complex::Complex64;

/// Calculate the maximum delay between to signals.
pub fn calculate_best_lag(signal_1: &[f64], signal_2: &[f64]) -> Option<i64> {
    let max_lag = ((signal_1.len().max(signal_2.len())) - 1) as i64;

    let point_wise_fft_vec =
        calculate_inverse_fft_pointwise_product(&mut signal_1.to_vec(), &mut signal_2.to_vec());
    // Negative errors
    let mut corrs = point_wise_fft_vec[point_wise_fft_vec.len() - max_lag as usize..].to_vec();
    // Positive errors
    let mut positives = point_wise_fft_vec[0..max_lag as usize + 1].to_vec();

    corrs.append(&mut positives);

    // Get maximum
    let best_corr = corrs[..].iter().max_by(|x, y| {
        x.abs()
            .partial_cmp(&y.abs())
            .expect("Failed to compute correlation")
    })?;

    let best_corr_idx = corrs.iter().position(|&r| r == *best_corr)?;

    Some(best_corr_idx as i64 - max_lag)
}

/// Calculates the pointwise inverse fft product of 2 signals
pub fn calculate_inverse_fft_pointwise_product(
    signal_1: &mut Vec<f64>,
    signal_2: &mut Vec<f64>,
) -> Vec<f64> {
    let biggest_length = signal_1.len().max(signal_2.len());

    match &signal_1.len().cmp(&signal_2.len()) {
        std::cmp::Ordering::Less => {
            signal_1.resize(biggest_length, 0.0);
        }
        std::cmp::Ordering::Greater => {
            signal_2.resize(biggest_length, 0.0);
        }
        _ => {}
    }
    let (_, exp) = frexp((signal_1.len() * 2 - 1) as f32);
    let fft_points = 2usize.pow(exp as u32);
    let mut manager = FftManager::new(fft_points);
    let point_wise_product =
        calculate_fft_pointwise_product(signal_1, signal_2, &mut manager, fft_points);

    fast_fourier_transform::inverse_1d_conj_sym(&mut manager, &point_wise_product)
}

/// Calculates the pointwise fft product of 2 signals
pub fn calculate_fft_pointwise_product(
    signal_1: &[f64],
    signal_2: &[f64],
    manager: &mut FftManager,
    fft_points: usize,
) -> Vec<Complex64> {
    let mut fft_signal_2 =
        fast_fourier_transform::forward_1d_from_points(manager, signal_2, fft_points);
    fft_signal_2
        .iter_mut()
        .for_each(|element| *element = element.conj());

    let fft_signal_1 =
        fast_fourier_transform::forward_1d_from_points(manager, signal_1, fft_points);
    fft_signal_1
        .iter()
        .zip(fft_signal_2.iter())
        .map(|(a, b)| a * b)
        .collect()
}

///
/// Returns the mantissa and the exponent of a given floating point value.
/// # Examples
///
/// ```
/// use visqol_rs::xcorr::frexp;
/// let (_, result) = frexp(27.0f32);
/// assert_eq!(result, 5);
/// ```
pub fn frexp(s: f32) -> (f32, i32) {
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
mod tests
{
    use ndarray::Array1;
    use super::*;

#[test]
fn test_best_lag_same_length() {
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
fn test_best_lag_ref_shorter() {
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
fn test_best_lag_ref_longer() {
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
fn test_negative_best_lag() {
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
    let (_, result) = frexp(27.0f32);
    let expected_result = 5;

    assert_eq!(result, expected_result);
}

}