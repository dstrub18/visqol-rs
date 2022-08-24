use std::error::Error;

use crate::audio_signal::AudioSignal;
use crate::misc_math;
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
fn to_mono_matrix(sample_matrix: &Array2<f64>) -> Array1<f64> {
    sample_matrix.sum_axis(Axis(1))
}

/// Given a `file_path` to a wav file on disk, this file is loaded. If there are multiple channels, these are summed and normalized to 1 mono channel.
pub fn load_as_mono(file_path: &str) -> Result<AudioSignal, Box<dyn Error>> {
    let wav_reader = WavFile::open(file_path)?;

    let data_vector_float = misc_math::normalize_int16_to_double(&wav_reader.samples);
    let final_signal = extract_multichannel(wav_reader.num_channels as usize, &data_vector_float);

    let final_signal = to_mono_matrix(&final_signal);

    Ok(AudioSignal {
        data_matrix: final_signal / wav_reader.num_channels as f64,
        sample_rate: wav_reader.sample_rate,
    })
}

/// De-interleave an interleaved signal and returns them in a matrix. 1 row represents 1 channel.
fn extract_multichannel(num_channels: usize, interleaved_vector: &Vec<f64>) -> Array2<f64> {
    assert!(interleaved_vector.len() % num_channels as usize == 0);
    let sub_vector_size = interleaved_vector.len() / num_channels as usize;
    Array2::from_shape_vec(
        (sub_vector_size, num_channels).strides((num_channels, 1)),
        interleaved_vector.clone(),
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

/// Given a complex vector, this function will append its elements reversed and the imaginary part multiplied by -1.0. The 1st and last element of the original vector are ommitted in that process.
pub fn mirror_spectrum(spectrum: &mut Vec<Complex64>) {
    let nyquist_bin = Complex64::new(
        spectrum.last().expect("Failed to copy Nyqvist bin!").re,
        0.0,
    ); // Copy Nyqvist real part
    let zero_hz_bin = Complex64::new(spectrum[0].re, 0.0); // Copy 0 hz bin

    spectrum.pop(); //  Remove Nyqvist
    spectrum.remove(0); // Remove 0 hz

    let mut mirrored_spectrum = spectrum.clone();
    mirrored_spectrum.reverse();
    mirrored_spectrum.iter_mut().for_each(|element| {
        element.im *= -1.0;
    });
    // Push Nyqivst in middle of Vec
    spectrum.push(nyquist_bin);
    spectrum.extend(mirrored_spectrum);
    spectrum.insert(0, zero_hz_bin);
}
