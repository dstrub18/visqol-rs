use std::error::Error;

use crate::audio_signal::AudioSignal;
use crate::math_utils;
use crate::spectrogram::Spectrogram;
use crate::wav_reader::WavFile;
use ndarray::{Array1, Array2, Axis, ShapeBuilder};
use num::complex::Complex64;
use num_traits::Zero;

// Constants
const SPL_REFERENCE_POINT: f64 = 0.00002;
const NOISE_FLOOR_RELATIVE_TO_PEAK_DB: f64 = 45.0;
const NOISE_FLOOR_ABSOLUTE_DB: f64 = -45.0;

/// Returns a copy of `degraded` which has the same SPL as `reference`.
pub fn scale_to_match_sound_pressure_level(
    reference: &AudioSignal,
    degraded: &AudioSignal,
) -> AudioSignal {
    let ref_spl = calculate_sound_pressure_level(reference);
    let deg_spl = calculate_sound_pressure_level(degraded);

    let scale_factor = 10.0f64.powf((ref_spl - deg_spl) / 20.0);
    let scaled_mat = degraded.data_matrix.clone() * scale_factor;
    AudioSignal::new(
        scaled_mat
            .as_slice()
            .expect("Failed to create AudioSignal from slice!"),
        degraded.sample_rate,
    )
}

/// Computes the sound pressure level of an audio signal in dB
fn calculate_sound_pressure_level(signal: &AudioSignal) -> f64 {
    let energy: f64 = signal
        .data_matrix
        .iter()
        .map(|element| element.powi(2))
        .sum();
    let sound_pressure = (energy / (signal.data_matrix.len()) as f64).sqrt();
    20.0 * ((sound_pressure / SPL_REFERENCE_POINT).log10())
}

/// Calculates the per-column sum of a 2d array and returns them as a 1d array
fn to_mono_matrix(sample_matrix: &Array2<f64>) -> Array1<f64> { sample_matrix.sum_axis(Axis(1)) }

/// Given a `file_path` to a wav file on disk, this file is loaded. If there are multiple channels, these are summed and normalized to 1 mono channel.
pub fn load_as_mono(file_path: &str) -> Result<AudioSignal, Box<dyn Error>> {
    let wav_reader = WavFile::open(file_path)?;

    let data_vector_float = math_utils::normalize_int16_to_double(&wav_reader.samples);
    let final_signal = extract_multichannel(wav_reader.num_channels as usize, &data_vector_float);

    let final_signal = to_mono_matrix(&final_signal);

    Ok(AudioSignal {
        data_matrix: final_signal / wav_reader.num_channels as f64,
        sample_rate: wav_reader.sample_rate,
    })
}

/// De-interleave an interleaved signal and returns them in a matrix. 1 row represents 1 channel.
fn extract_multichannel(num_channels: usize, interleaved_vector: &[f64]) -> Array2<f64> {
    assert!(interleaved_vector.len() % num_channels == 0);
    let sub_vector_size = interleaved_vector.len() / num_channels;
    Array2::from_shape_vec(
        (sub_vector_size, num_channels).strides((num_channels, 1)),
        interleaved_vector.to_vec(),
    )
    .expect("Failed to sum multichannel signal to mono signal!")
}

/// Scales 2 spectrograms to match their sound pressure levels.
pub fn prepare_spectrograms_for_comparison(
    reference: &mut Spectrogram,
    degraded: &mut Spectrogram,
) {
    reference.convert_to_db();
    degraded.convert_to_db();
    reference.raise_floor(NOISE_FLOOR_ABSOLUTE_DB);
    degraded.raise_floor(NOISE_FLOOR_ABSOLUTE_DB);

    reference.raise_floor_per_frame(NOISE_FLOOR_RELATIVE_TO_PEAK_DB, degraded);

    let ref_floor = reference.get_minimum();
    let deg_floor = degraded.get_minimum();
    let lowest_floor = ref_floor.min(deg_floor);

    reference.subtract_floor(lowest_floor);

    degraded.subtract_floor(lowest_floor);
}

/// Clones all elements of `float_vector` into the real elements of a complex vector and sets all imaginary parts to 0.0.
pub fn float_vec_to_real_valued_complex_vec(float_vector: &[f64]) -> Vec<Complex64> {
    let mut complex_vec = vec![Complex64::zero(); float_vector.len()];

    complex_vec
        .iter_mut()
        .enumerate()
        .for_each(|(index, element)| {
            element.re = float_vector[index];
        });

    complex_vec
}
/// Clones all elements of `complex_vector` into a vector of `f64` values, ommitting the imaginary parts.
pub fn real_valued_complex_vec_to_float_vec(complex_vector: &[Complex64]) -> Vec<f64> {
    let mut real_vec = vec![f64::zero(); complex_vector.len()];

    real_vec
        .iter_mut()
        .enumerate()
        .for_each(|(index, element)| {
            *element = complex_vector[index].re;
        });

    real_vec
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn mono_file_is_read_successfully() {
        let expected_mono_test_sample_rate = 48000;
        let expected_mono_test_num_rows = 131444;
        let expected_mono_test_num_cols = 1;
        let expected_mono_duration = 2.74;

        let tolerance = 0.01;

        let signal = load_as_mono("test_data/CA01_01.wav").unwrap();
        assert_eq!(signal.sample_rate, expected_mono_test_sample_rate);
        assert_eq!(signal.data_matrix.len(), expected_mono_test_num_rows);
        assert_eq!(signal.data_matrix.ndim(), expected_mono_test_num_cols);
        assert_abs_diff_eq!(
            signal.get_duration(),
            expected_mono_duration,
            epsilon = tolerance
        );
    }

    #[test]
    fn stereo_file_is_read_and_summed() {
        let expected_stereo_test_sample_rate = 48000;
        let expected_stereo_test_num_rows = 597784;
        let expected_stereo_test_num_cols = 1;
        let expected_stereo_duration = 12.45;
        let tolerance = 0.01;

        let signal =
            load_as_mono("test_data/conformance_testdata_subset/guitar48_stereo.wav").unwrap();
        assert_eq!(signal.sample_rate, expected_stereo_test_sample_rate);
        assert_eq!(signal.len() as u32, expected_stereo_test_num_rows);
        assert_eq!(
            signal.data_matrix.ndim() as u32,
            expected_stereo_test_num_cols
        );
        assert_abs_diff_eq!(
            signal.get_duration(),
            expected_stereo_duration,
            epsilon = tolerance
        );
        assert_abs_diff_eq!(signal[2], -0.000_015_258_789_062_5, epsilon = tolerance);
        assert_abs_diff_eq!(
            signal[597782],
            -0.000_259_399_414_062_5,
            epsilon = tolerance
        );
    }

    #[test]
    #[should_panic]
    fn loading_32_bit_quantization_fails() {
        load_as_mono("test_data/clean_speech/CA01_01_32bits.wav").unwrap();
    }

    #[test]
    #[should_panic]
    fn loading_8_bit_quantization_fails() {
        load_as_mono("test_data/clean_speech/CA01_01_8bits.wav").unwrap();
    }
}
