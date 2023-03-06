use crate::audio_utils;
use crate::math_utils;
use num::complex::Complex64;
use num::Zero;
use rustfft::FftPlanner;

// Constants
const MIN_FFT_SIZE: usize = 32;

/// Wrapper around the `rustfft` library to perform basic fft operations.
pub struct FftManager {
    /// Planner to perform fft operations
    planner: FftPlanner<f64>,
    /// Length of the fft
    pub fft_size: usize,
    /// Scale factor to apply after inverse fft
    inverse_fft_scale: f64,
    /// Number of samples to apply fft on
    pub samples_per_channel: usize,
}

impl FftManager {
    /// Creates a new fft manager, computes internal variables from `samples_per_channel`
    pub fn new(samples_per_channel: usize) -> Self {
        let fft_size = math_utils::next_pow_two(samples_per_channel).max(MIN_FFT_SIZE);

        Self {
            planner: FftPlanner::<f64>::new(),
            fft_size,
            samples_per_channel,
            inverse_fft_scale: 1.0f64 / (fft_size as f64),
        }
    }

    /// Zero-pads `time_channel` if necessary, transforms its contents into the frequency domain and stores it in `freq_channel`
    pub fn freq_from_time_domain(
        &mut self,
        time_channel: &mut Vec<f64>,
        freq_channel: &mut [Complex64],
    ) {
        let real_to_complex = self.planner.plan_fft_forward(self.fft_size);
        if time_channel.len() == self.fft_size {
            let mut complex_time_domain =
                audio_utils::float_vec_to_real_valued_complex_vec(time_channel);
            let mut scratch_buffer =
                vec![Complex64::zero(); real_to_complex.get_outofplace_scratch_len()];
            real_to_complex.process_outofplace_with_scratch(
                &mut complex_time_domain[..],
                freq_channel,
                &mut scratch_buffer[..],
            );
        } else {
            time_channel.resize(self.fft_size, 0.0f64);
            let mut complex_time_domain =
                audio_utils::float_vec_to_real_valued_complex_vec(time_channel);
            let mut scratch_buffer =
                vec![Complex64::zero(); real_to_complex.get_outofplace_scratch_len()];
            real_to_complex.process_outofplace_with_scratch(
                &mut complex_time_domain[..],
                &mut freq_channel[..],
                &mut scratch_buffer[..],
            );
        }
    }

    /// Zero-pads `freq_channel` if necessary, transforms its contents into the time domain and stores it in `time_channel`
    pub fn time_from_freq_domain(
        &mut self,
        freq_channel: &mut [Complex64],
        time_channel: &mut Vec<f64>,
    ) {
        let complex_to_real = self.planner.plan_fft_inverse(self.fft_size);

        if time_channel.len() == self.fft_size {
            let mut scratch_buffer =
                vec![Complex64::zero(); complex_to_real.get_outofplace_scratch_len()];
            let mut complex_td = audio_utils::float_vec_to_real_valued_complex_vec(time_channel);
            complex_to_real.process_outofplace_with_scratch(
                freq_channel,
                &mut complex_td[..],
                &mut scratch_buffer[..],
            );
            *time_channel = audio_utils::real_valued_complex_vec_to_float_vec(&complex_td);
        } else {
            let mut scratch_buffer =
                vec![Complex64::zero(); complex_to_real.get_outofplace_scratch_len()];
            time_channel.resize(self.fft_size, f64::zero());
            let mut complex_td = audio_utils::float_vec_to_real_valued_complex_vec(time_channel);
            complex_to_real.process_outofplace_with_scratch(
                freq_channel,
                &mut complex_td[..],
                &mut scratch_buffer[..],
            );
            *time_channel = audio_utils::real_valued_complex_vec_to_float_vec(&complex_td);
        }
    }

    /// Multiplies each element in `time_channel` by `self.inverse_fft_scale`
    pub fn apply_reverse_fft_scaling(&self, time_channel: &mut [f64]) {
        time_channel.iter_mut().for_each(|x| {
            *x *= self.inverse_fft_scale;
        });
    }
}
