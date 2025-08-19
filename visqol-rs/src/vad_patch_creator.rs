use crate::patch_creator::PatchCreator;
use crate::visqol_error::VisqolError;
use crate::{analysis_window::AnalysisWindow, audio_signal::AudioSignal, math_utils, rms_vad};
use itertools::Itertools;
use ndarray::{s, Array2};

/// Computes patch indices from a spectrogram by analyzing voice acitivity in the time domain and rejecting patches which are considered silent.
pub struct VadPatchCreator {
    patch_size: usize,
    frames_with_va_threshold: f64,
}

impl PatchCreator for VadPatchCreator {
    fn create_ref_patch_indices(
        &self,
        spectrogram: &Array2<f64>,
        ref_signal: &AudioSignal,
        window: &AnalysisWindow,
    ) -> Result<std::vec::Vec<usize>, VisqolError> {
        let norm_mat = math_utils::normalize_signal(&ref_signal.data_matrix);
        let norm_sig = AudioSignal::new(
            norm_mat
                .as_slice()
                .expect("Failed to create AudioSignal from slice!"),
            ref_signal.sample_rate,
        );

        let frame_size = (window.size as f64 * window.overlap) as usize;
        let patch_sample_length = self.patch_size * frame_size;
        let spectrum_length = spectrogram.ncols();
        let first_patch_idx = self.patch_size / 2 - 1;
        let patch_count = (spectrum_length - first_patch_idx) / self.patch_size;
        let total_sample_count = patch_count * patch_sample_length;

        let mut ref_patch_indices = Vec::<usize>::with_capacity(patch_count);

        // Pass the reference signal to the VAD to determine which frames have voice
        // activity.
        let vad_result = self.get_voice_activity(
            norm_sig
                .data_matrix
                .as_slice()
                .ok_or(VisqolError::FailedToComputeVad)?,
            first_patch_idx,
            total_sample_count,
            frame_size,
        );

        let mut patch_idx = first_patch_idx;

        for patch in &vad_result.iter().chunks(self.patch_size) {
            let frames_with_va = patch.sum::<f64>();

            if frames_with_va >= self.frames_with_va_threshold {
                ref_patch_indices.push(patch_idx);
            }
            patch_idx += self.patch_size;
        }

        Ok(ref_patch_indices)
    }

    fn create_patches_from_indices(
        &self,
        spectrogram: &Array2<f64>,
        patch_indices: &[usize],
    ) -> Vec<Array2<f64>> {
        let mut patches = Vec::<Array2<f64>>::with_capacity(patch_indices.len());

        let mut patch: Array2<f64>;

        let mut end_col: usize;
        for start_col in patch_indices {
            end_col = start_col + self.patch_size;
            patch = spectrogram.slice(s![.., *start_col..end_col]).to_owned();
            patches.push(patch);
        }
        patches
    }
}

impl VadPatchCreator {
    /// Creates a new `VadPatchCreator` with the desired patch size.
    pub fn new(patch_size: usize) -> Self {
        Self {
            patch_size,
            frames_with_va_threshold: 1.0,
        }
    }

    /// Given a time domain signal, this function returns a vector with 1s indicating voice acitivity and 0s indicating the absence of acitivity.
    pub fn get_voice_activity(
        &self,
        signal: &[f64],
        start_sample: usize,
        total_samples: usize,
        frame_length: usize,
    ) -> Vec<f64> {
        let mut vad = rms_vad::RmsVad::default();

        let patch = &signal[start_sample..start_sample + total_samples];

        let mut frame = Vec::<i16>::with_capacity(frame_length);
        for patch_element in patch {
            let mut scaled_val = ((*patch_element * ((1 << 15) as f64)) as i16) as f64;
            scaled_val = (-(1 << 15) as f64)
                .max(1.0 * ((1 << 15) - 1) as f64)
                .min(scaled_val);
            frame.push(scaled_val as i16);

            if frame.len() == frame_length {
                vad.process_chunk(&frame);
                frame.clear();
            }
        }
        vad.get_vad_results()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis_window::AnalysisWindow;
    use crate::audio_utils::load_as_mono;
    use crate::gammatone_filterbank::GammatoneFilterbank;
    use crate::gammatone_spectrogram_builder::GammatoneSpectrogramBuilder;
    use crate::patch_creator::PatchCreator;
    use crate::spectrogram_builder::SpectrogramBuilder;

    #[test]
    fn clean_speech_vad() {
        const K_START_SAMPLE: usize = 14;
        const K_TOTAL_SAMPLE: usize = 115200;
        const K_FRAME_LEN: usize = 480;
        const K_CA01_01_VAD_RES_COUNT: usize = 240;

        let ref_signal = load_as_mono("test_data/clean_speech/CA01_01.wav").unwrap();

        let vad = VadPatchCreator::new(20);
        let res = vad.get_voice_activity(
            ref_signal.data_matrix.as_slice().unwrap(),
            K_START_SAMPLE,
            K_TOTAL_SAMPLE,
            K_FRAME_LEN,
        );
        assert_eq!(K_CA01_01_VAD_RES_COUNT, res.len());
    }

    #[test]
    fn patch_indices() {
        const _K_MINIMUM_FREQ: f64 = 50.0;
        const K_PATCH_SIZE: usize = 20;
        const _K_NUM_BANDS: usize = 21;

        let expected_patches = vec![9, 29, 49, 69, 89];
        let ref_signal = load_as_mono("test_data/clean_speech/CA01_01.wav").unwrap();

        let mut spectrogram_builder =
            GammatoneSpectrogramBuilder::new(GammatoneFilterbank::<21>::new(50.0), true);
        let window = AnalysisWindow::new(ref_signal.sample_rate, 0.25, 0.08);

        let spectrogram = spectrogram_builder.build(&ref_signal, &window).unwrap();

        let vad = VadPatchCreator::new(K_PATCH_SIZE);
        let patches = vad
            .create_ref_patch_indices(&spectrogram.data, &ref_signal, &window)
            .unwrap();

        assert_eq!(patches.len(), expected_patches.len());
        for (&a, b) in patches.iter().zip(expected_patches) {
            assert_eq!(a, b);
        }
    }
}
