use crate::analysis_window::AnalysisWindow;
use crate::equivalent_rectangular_bandwidth;
use crate::gammatone_filterbank::GammatoneFilterbank;
use crate::spectrogram::Spectrogram;
use crate::spectrogram_builder::SpectrogramBuilder;
use crate::{audio_signal::AudioSignal, visqol_error::VisqolError};
use ndarray::{Array2, Axis};

/// Produces a frequency domain representation from a time domain signal using a gammatone filterbank.
pub struct GammatoneSpectrogramBuilder {
    filter_bank: GammatoneFilterbank,
    speech_mode: bool,
}

impl SpectrogramBuilder for GammatoneSpectrogramBuilder {
    fn build(
        &mut self,
        signal: &AudioSignal,
        window: &AnalysisWindow,
    ) -> Result<Spectrogram, VisqolError> {
        let time_domain_signal = &signal.data_matrix.to_vec();
        let sample_rate = signal.sample_rate;
        let max_freq = if self.speech_mode {
            Self::SPEECH_MODE_MAX_FREQ
        } else {
            sample_rate / 2
        };

        // get gammatone coefficients
        let (mut filter_coeffs, mut center_freqs) = equivalent_rectangular_bandwidth::make_filters(
            sample_rate as usize,
            self.filter_bank.num_bands,
            self.filter_bank.min_freq,
            max_freq as f64,
        );
        filter_coeffs.invert_axis(Axis(0));
        self.filter_bank.set_filter_coefficients(&filter_coeffs);
        self.filter_bank.reset_filter_conditions();

        let hop_size = (window.size as f64 * window.overlap) as usize;

        if time_domain_signal.len() < window.size {
            return Err(VisqolError::TooFewSamples {
                found: time_domain_signal.len(),
                minimum_required: window.size,
            });
        }

        let num_cols = 1 + ((time_domain_signal.len() - window.size) / hop_size);
        let mut out_matrix = Array2::<f64>::zeros((self.filter_bank.num_bands, num_cols));

        for (index, frame) in time_domain_signal
            .windows(window.size)
            .step_by(hop_size)
            .enumerate()
        {
            self.filter_bank.reset_filter_conditions();
            let mut filtered_signal = self.filter_bank.apply_filter(frame);

            filtered_signal.map_inplace(|e| *e = *e * *e);

            let mut row_means = filtered_signal
                .mean_axis(Axis(1))
                .expect("Failed to compute means for gammatone spectrogram!");

            row_means.map_inplace(|e| {
                *e = e.sqrt();
            });

            for j in 0..row_means.to_vec().len() {
                out_matrix[(j, index)] = row_means[j];
            }
        }

        center_freqs.as_mut_slice().sort_by(|a, b| {
            a.partial_cmp(b)
                .expect("Failed to sort center frequencies!")
        });
        Ok(Spectrogram::new(out_matrix, center_freqs))
    }
}

impl GammatoneSpectrogramBuilder {
    const SPEECH_MODE_MAX_FREQ: u32 = 8000;

    /// Creates a new gammatone spectrogram builder with the given gammatone filterbank.
    /// If `use_speech_mode` is set to `true`, the maximum frequency is determined to be 8000 Hz.
    pub fn new(filter_bank: GammatoneFilterbank, use_speech_mode: bool) -> Self {
        Self {
            filter_bank,
            speech_mode: use_speech_mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis_window::AnalysisWindow;
    use crate::audio_utils;
    use crate::gammatone_filterbank::GammatoneFilterbank;
    use crate::spectrogram_builder::SpectrogramBuilder;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_spec_builder() {
        // Fixed parameters
        const MINIMUM_FREQ: f64 = 50.0;
        const NUM_BANDS: usize = 32;
        const OVERLAP: f64 = 0.25;

        const REF_SPECTRO_NUM_COLS: usize = 802;

        let signal_ref = audio_utils::load_as_mono(
            "test_data/conformance_testdata_subset/contrabassoon48_stereo.wav",
        )
        .unwrap();
        let filter_bank = GammatoneFilterbank::new::<{ NUM_BANDS }>(MINIMUM_FREQ);
        let window = AnalysisWindow::new(signal_ref.sample_rate, OVERLAP, 0.08);

        let mut spectro_builder = GammatoneSpectrogramBuilder::new(filter_bank, false);
        let spectrogram_ref = spectro_builder.build(&signal_ref, &window).unwrap();

        // Check 1st element
        assert_abs_diff_eq!(spectrogram_ref.data[(0, 0)], 9.44161e-05, epsilon = 0.00001);
        // Check dimensions
        assert_eq!(spectrogram_ref.data.ncols(), REF_SPECTRO_NUM_COLS);
    }
}
