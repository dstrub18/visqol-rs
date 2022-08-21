use crate::fast_fourier_transform;
use crate::fft_manager::FftManager;
use ndarray::Array1;
use num::complex::Complex64;

pub fn calculate_best_lag(signal_1: &Array1<f64>, signal_2: &Array1<f64>) -> Option<i64> {
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
///
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
