use approx::assert_abs_diff_eq;
use visqol_rs::{
    envelope::{calculate_upper_env, calculate_hilbert},
    fft_manager,
    misc_audio::load_as_mono,
    xcorr::{
        calculate_best_lag, calculate_fft_pointwise_product,
        calculate_inverse_fft_pointwise_product, frexp,
    },
};

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

    let result =
        calculate_fft_pointwise_product(&ref_signal_vec, &deg_signal_vec, &mut manager, fft_points);

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

    let result = calculate_best_lag(ref_signal.data_matrix.as_slice().unwrap(), deg_signal.data_matrix.as_slice().unwrap()).unwrap();

    assert_abs_diff_eq!(result, 0);
}
