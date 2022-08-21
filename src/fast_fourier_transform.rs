use crate::fft_manager::FftManager;
use crate::misc_audio::real_valued_complex_vec_to_float_vec;
use num::complex::Complex64;
use num::Zero;

pub fn forward_1d_from_matrix(fft_manager: &mut FftManager, in_matrix: &[f64]) -> Vec<Complex64> {
    let mut temp_time_buffer = in_matrix.to_vec();

    let mut temp_freq_buffer = vec![Complex64::zero(); fft_manager.fft_size];
    fft_manager.freq_from_time_domain(&mut temp_time_buffer, &mut temp_freq_buffer);

    temp_freq_buffer
}

pub fn forward_1d_from_points(
    fft_manager: &mut FftManager,
    in_matrix: &[f64],
    points: usize,
) -> Vec<Complex64> {
    let num_points_to_append = points - in_matrix.len();
    let mut signal = in_matrix.to_vec();
    signal.extend(vec![0.0; num_points_to_append]);

    forward_1d_from_matrix(fft_manager, &signal)
}

pub fn inverse_1d(fft_manager: &mut FftManager, in_matrix: &[Complex64]) -> Vec<Complex64> {
    let mut temp_freq_buffer = in_matrix.to_vec();
    let mut temp_time_buffer = vec![f64::zero(); fft_manager.samples_per_channel];
    fft_manager.time_from_freq_domain(&mut temp_freq_buffer, &mut temp_time_buffer);
    fft_manager.apply_reverse_fft_scaling(&mut temp_time_buffer);

    // This makes very little sense but oh well...
    let mut out_vec = vec![Complex64::zero(); fft_manager.samples_per_channel];
    for (i, elem) in out_vec.iter_mut().enumerate() {
        elem.re = temp_time_buffer[i];
    }
    out_vec
}

pub fn inverse_1d_conj_sym(fft_manager: &mut FftManager, in_matrix: &[Complex64]) -> Vec<f64> {
    let inverse = inverse_1d(fft_manager, in_matrix);

    real_valued_complex_vec_to_float_vec(&inverse)
}
