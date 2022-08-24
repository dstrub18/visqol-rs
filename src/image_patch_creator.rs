use crate::{
    analysis_window::AnalysisWindow, audio_signal::AudioSignal, patch_creator::PatchCreator,
    visqol_error::VisqolError,
};
use ndarray::{s, Array2};

/// Creates patches from a spectrogram by segmenting it into equally-sized matrices.
pub struct ImagePatchCreator {
    patch_size: usize,
}

impl PatchCreator for ImagePatchCreator {
    fn create_ref_patch_indices(
        &self,
        spectrogram: &Array2<f64>,
        _ref_signal: &AudioSignal,
        _window: &AnalysisWindow,
    ) -> Result<Vec<usize>, VisqolError> {
        self.create_ref_patch_indices_from_spectrogram(spectrogram)
    }

    fn create_patches_from_indices(
        &self,
        spectrogram: &Array2<f64>,
        patch_indices: &[usize],
    ) -> Vec<Array2<f64>> {
        let mut end_col: usize;

        let mut patches = Vec::<Array2<f64>>::new();

        let mut patch: Array2<f64>;

        for start_col in patch_indices {
            end_col = start_col + self.patch_size;
            patch = spectrogram.slice(s![.., *start_col..end_col]).to_owned();
            patches.push(patch);
        }
        patches
    }
}

impl ImagePatchCreator {
    /// Returns a new `ImagePatchCreator` with the desired patch size.
    pub fn new(patch_size: usize) -> Self { Self { patch_size } }

    /// Computes the start indices of each patch by segmenting the entire spectrogram into equally-sized patches.
    fn create_ref_patch_indices_from_spectrogram(
        &self,
        spectrogram: &Array2<f64>,
    ) -> Result<Vec<usize>, VisqolError> {
        let spectrum_length = spectrogram.ncols();
        let init_patch_index = self.patch_size / 2;

        if spectrum_length < self.patch_size + init_patch_index {
            return Err(VisqolError::ReferenceSpectrogramTooSmall {
                spectrogram_length: spectrum_length,
                minimum_required: self.patch_size - init_patch_index,
            });
        }

        let max_index = if init_patch_index < (spectrum_length - self.patch_size) {
            spectrum_length - self.patch_size
        } else {
            init_patch_index + 1
        };

        let mut ref_patch_indices = Vec::<usize>::with_capacity(spectrum_length / self.patch_size);

        for i in (init_patch_index..max_index).step_by(self.patch_size) {
            ref_patch_indices.push(i - 1);
        }
        Ok(ref_patch_indices)
    }
}
