use approx::assert_abs_diff_eq;
use ndarray::Axis;
use visqol_rs::{misc_audio::load_as_mono, envelope::{hilbert, calculate_upper_env}, fft_manager, xcorr::{frexp, calculate_fft_pointwise_product, calculate_inverse_fft_pointwise_product, calculate_best_lag}};

#[test]
fn test_hilbert_on_signal()
{
    let mut signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav");
    let result = hilbert(&mut signal.data_matrix);

    assert_abs_diff_eq!(result[0].re,  0.0003036616911888330, epsilon=0.0001);
}

#[test]
fn test_envelope_on_signal()
{
    let mut signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav");
    let result = calculate_upper_env(&mut signal.data_matrix);

    assert_abs_diff_eq!(result[0], 0.00030159861338215923, epsilon=0.0001);
}

#[test]
fn test_xcorr_pointwise_prod_on_signal()
{
    let ref_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav");
    let ref_signal_vec = ref_signal.data_matrix.to_vec();
    let deg_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav");
    let deg_signal_vec = deg_signal.data_matrix.to_vec();

    let (_, exponent) = frexp((ref_signal_vec.len() * 2 - 1) as f32);
    let fft_points = 2i32.pow(exponent as u32) as usize;
    let mut manager = fft_manager::FftManager::new(fft_points);

    let result = calculate_fft_pointwise_product(&ref_signal_vec, &deg_signal_vec, &mut manager, fft_points);

    assert_abs_diff_eq!(result[0].re, 0.012231532484292984, epsilon=0.001);
}


#[test]
fn test_calculate_inverse_fft_pointwise_product()
{
    let ref_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav");
    let deg_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav");

    let result = calculate_inverse_fft_pointwise_product(&&ref_signal.data_matrix, &&deg_signal.data_matrix);

    assert_abs_diff_eq!(result[0], 79.6557439500466, epsilon=0.0001);
}

#[test]
fn test_calculate_best_lag()
{
    let ref_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav");
    let deg_signal = load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav");

    let result = calculate_best_lag(&&ref_signal.data_matrix, &&deg_signal.data_matrix);

    assert_abs_diff_eq!(result, 0);
}