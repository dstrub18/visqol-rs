use ndarray::Array2;

use crate::{
    analysis_window::AnalysisWindow, audio_signal::AudioSignal, visqol_error::VisqolError,
};

pub trait PatchCreator {
    fn create_ref_patch_indices(
        &self,
        spectrogram: &Array2<f64>,
        ref_signal: &AudioSignal,
        window: &AnalysisWindow,
    ) -> Result<Vec<usize>, VisqolError>;
    fn create_patches_from_indices(
        &self,
        spectrogram: &Array2<f64>,
        patch_indices: &[usize],
    ) -> Vec<Array2<f64>>;
}
