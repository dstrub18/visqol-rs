use crate::fast_fourier_transform;
use crate::fft_manager::FftManager;
use ndarray::Array1;
use num::complex::Complex64;

/// Calculates the upper envelope for a given time domain signal.
pub fn calculate_upper_env(signal: &Array1<f64>) -> Option<ndarray::Array1<f64>> {
    let mean = signal.mean()?;
    let mut signal_centered = signal - mean;
    let hilbert = calculate_hilbert(signal_centered.as_slice_mut()?)?;

    let mut hilbert_amplitude = Array1::<f64>::zeros(hilbert.len());

    for (amplitude, h) in hilbert_amplitude.iter_mut().zip(&hilbert) {
        *amplitude = h.norm();
    }
    hilbert_amplitude += mean;
    Some(hilbert_amplitude)
}

/// Calculates the hilbert transform for a given time domain signal.
pub fn calculate_hilbert(signal: &mut [f64]) -> Option<Array1<Complex64>> {
    let mut fft_manager = FftManager::new(signal.len());
    let freq_domain_signal =
        fast_fourier_transform::forward_1d_from_matrix(&mut fft_manager, signal);

    let is_odd = signal.len() % 2 == 1;
    let is_non_empty = !signal.is_empty();

    // Set up scaling vector
    let mut hilbert_scaling = vec![0.0f64; freq_domain_signal.len()];
    hilbert_scaling[0] = 1.0;

    if !is_odd && is_non_empty {
        hilbert_scaling[signal.len() / 2] = 1.0;
    } else if is_odd && is_non_empty {
        hilbert_scaling[signal.len() / 2] = 2.0;
    }

    let n = if is_odd {
        (freq_domain_signal.len() + 1) / 2
    } else {
        freq_domain_signal.len() / 2
    };

    hilbert_scaling[1..n].fill(2.0);

    let mut element_wise_product = Array1::<Complex64>::zeros(freq_domain_signal.len());

    for i in 0..freq_domain_signal.len() {
        element_wise_product[i] = freq_domain_signal[i] * hilbert_scaling[i];
    }

    let mut hilbert =
        fast_fourier_transform::inverse_1d(&mut fft_manager, element_wise_product.as_slice()?);
    hilbert
        .iter_mut()
        .for_each(|element| *element = *element * 2.0 - 0.000001);
    Some(Array1::<Complex64>::from_vec(hilbert))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fft_manager,
        misc_audio::load_as_mono,
        xcorr::{
            calculate_best_lag, calculate_fft_pointwise_product,
            calculate_inverse_fft_pointwise_product, frexp,
        },
    };
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_hilbert_on_signal() {
        let mut signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav").unwrap();
        let result = calculate_hilbert(signal.data_matrix.as_slice_mut().unwrap()).unwrap();

        assert_abs_diff_eq!(result[0].re, 0.000_303_661_691_188_833, epsilon = 0.0001);
    }

    #[test]
    fn test_envelope_on_signal() {
        let signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav").unwrap();
        let result = calculate_upper_env(&signal.data_matrix).unwrap();

        assert_abs_diff_eq!(result[0], 0.00030159861338215923, epsilon = 0.0001);
    }

    #[test]
    fn test_xcorr_pointwise_prod_on_signal() {
        let ref_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav").unwrap();
        let ref_signal_vec = ref_signal.data_matrix.to_vec();
        let deg_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav").unwrap();
        let deg_signal_vec = deg_signal.data_matrix.to_vec();

        let (_, exponent) = frexp((ref_signal_vec.len() * 2 - 1) as f32);
        let fft_points = 2i32.pow(exponent as u32) as usize;
        let mut manager = fft_manager::FftManager::new(fft_points);

        let result = calculate_fft_pointwise_product(
            &ref_signal_vec,
            &deg_signal_vec,
            &mut manager,
            fft_points,
        );

        assert_abs_diff_eq!(result[0].re, 0.012231532484292984, epsilon = 0.001);
    }

    #[test]
    fn test_calculate_inverse_fft_pointwise_product() {
        let ref_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav").unwrap();
        let deg_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav").unwrap();

        let result = calculate_inverse_fft_pointwise_product(
            &mut ref_signal.data_matrix.to_vec(),
            &mut deg_signal.data_matrix.to_vec(),
        );

        assert_abs_diff_eq!(result[0], 79.66060597338944, epsilon = 0.0001);
    }

    #[test]
    fn test_calculate_best_lag() {
        let ref_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav").unwrap();
        let deg_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav").unwrap();

        let result = calculate_best_lag(
            ref_signal.data_matrix.as_slice().unwrap(),
            deg_signal.data_matrix.as_slice().unwrap(),
        )
        .unwrap();

        assert_abs_diff_eq!(result, 0);
    }
}
